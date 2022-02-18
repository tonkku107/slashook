// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs used in creating commands

mod responder;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  marker::Send,
  future::Future
};
use rocket::futures::future::BoxFuture;
use super::structs::{
  interactions::{
    Interaction, InteractionType, ApplicationCommandType, InteractionDataResolved, InteractionOption, InteractionOptionType,
    InteractionCallback, InteractionCallbackType,
    OptionValue
  },
  components::{Component, ComponentType},
  channels::{Message, MessageFlags},
  users::User,
  guilds::GuildMember,
  Snowflake
};
use super::tokio::{spawn, sync::{mpsc, oneshot}};
pub use responder::{MessageResponse, CommandResponder, Modal};
use responder::CommandResponse;
use crate::rest::Rest;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// The `Result` types expected from a command function
///
/// Since all the responses are sent via methods on [CommandResponder], we don't expect anything special on success.
/// Error can be anything that implements Error (boxed) which is useful for using `?` to handle errors.
pub type CmdResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// A trait for Command functions
///
/// A trait that allows requiring an `async fn(CommandInput, CommandResponder) -> CmdResult` in the [Command] struct.\
/// The function must also be `Send` as they can be transferred between threads
pub trait AsyncCmdFn: Send {
  /// A method that calls the function
  fn call(&self, input: CommandInput, responder: CommandResponder) -> BoxFuture<'static, CmdResult>;
}
impl<T, F> AsyncCmdFn for T
where
  T: Fn(CommandInput, CommandResponder) -> F + Send,
  F: Future<Output = CmdResult> + Send + 'static,
{
  fn call(&self, input: CommandInput, responder: CommandResponder) -> BoxFuture<'static, CmdResult> {
    Box::pin(self(input, responder))
  }
}

/// A struct representing a command that can be executed
///
/// **NOTE: This struct is usually constructed with the help of the [command attribute macro](macro@crate::command)**
pub struct Command {
  /// A handler function for the command
  pub func: Box<dyn AsyncCmdFn>,
  /// The name of the command
  pub name: String
}

/// Values passed as inputs for your command
#[derive(Clone, Debug)]
pub struct CommandInput {
  /// The type of the interaction this command was called for
  pub interaction_type: InteractionType,
  /// The type of the command on command interactions
  pub command_type: Option<ApplicationCommandType>,
  /// The type of the component on component interactions
  pub component_type: Option<ComponentType>,
  /// Name of the command that was executed
  pub command: String,
  /// Sub command that was executed
  ///
  /// Only included in chat input commands
  pub sub_command: Option<String>,
  /// Sub command group the sub command belongs in
  ///
  /// Only included in chat input commands
  pub sub_command_group: Option<String>,
  /// Arguments or modal inputs the user of your command filled./
  /// The key is the name of the argument or custom_id of the component
  pub args: HashMap<String, OptionValue>,
  /// user, member, role, channel and message objects resolved from arguments by Discord
  ///
  /// Not included in context menu commands, see the `target_*` fields.
  pub resolved: Option<InteractionDataResolved>,
  /// The ID of the guild the command was sent from
  pub guild_id: Option<Snowflake>,
  /// The ID of the channel the command was sent from
  pub channel_id: Option<Snowflake>,
  /// The user that ran the command
  pub user: User,
  /// If the command was executed in a guild, the member object of the user
  pub member: Option<GuildMember>,
  /// Message the interaction was executed on
  ///
  /// Only included in component interactions
  pub message: Option<Message>,
  /// The target user of a context menu command
  pub target_user: Option<User>,
  /// The target member of a context menu command
  pub target_member: Option<GuildMember>,
  /// The target message of a context menu command
  pub target_message: Option<Message>,
  /// Custom ID of the component
  ///
  /// Only included in component interactions
  pub custom_id: Option<String>,
  /// Chosen values from a Select Menu
  ///
  /// Only included in Select Menu component interactions
  pub values: Option<Vec<String>>,
  /// The argument currently in focus
  ///
  /// Only included in command autocomplete interactions
  pub focused: Option<String>,
  /// The selected [language](https://discord.com/developers/docs/reference#locales) of the user
  pub locale: String,
  /// The guild's preferred locale
  pub guild_locale: Option<String>,
  /// Handler for Discord API calls
  pub rest: Rest,
}

pub(crate) struct CommandHandler {
  commands: HashMap<String, Arc<Mutex<Command>>>
}

impl CommandHandler {
  pub fn new() -> Self {
    Self {
      commands: HashMap::new()
    }
  }

  pub fn add(&mut self, command: Command) {
    self.commands.insert(command.name.clone(), Arc::new(Mutex::new(command)));
  }

  pub async fn rocket_bridge(self: &Arc<Self>, mut receiver: mpsc::UnboundedReceiver::<RocketCommand>) {
    while let Some(command) = receiver.recv().await {
      let command_handler = self.clone();
      spawn(async move {
        let RocketCommand(interaction, bot_token, handler_send) = command;

        let value = if let
        InteractionType::APPLICATION_COMMAND |
        InteractionType::MESSAGE_COMPONENT |
        InteractionType::APPLICATION_COMMAND_AUTOCOMPLETE |
        InteractionType::MODAL_SUBMIT = interaction.interaction_type {
          command_handler.handle_command(interaction, bot_token).await.map_err(|_| ())
        } else {
          Err(())
        };

        handler_send.send(value).unwrap();
      });
    }
  }

  fn parse_options(&self, options: Vec<InteractionOption>, resolved: &Option<InteractionDataResolved>, mut input: &mut CommandInput) {
    for option in options.into_iter() {
      let option_value = match option.option_type {
        InteractionOptionType::SUB_COMMAND_GROUP => {
          input.sub_command_group = Some(option.name);
          return self.parse_options(option.options.expect("Sub command group is missing options"), resolved, input)
        },
        InteractionOptionType::SUB_COMMAND => {
          input.sub_command = Some(option.name);
          if option.options.is_none() { return }
          return self.parse_options(option.options.unwrap(), resolved, input)
        },

        InteractionOptionType::STRING => OptionValue::String(option.value.unwrap().as_str().unwrap().to_string()),
        InteractionOptionType::INTEGER => OptionValue::Integer(option.value.unwrap().as_i64().unwrap()),
        InteractionOptionType::BOOLEAN => OptionValue::Boolean(option.value.unwrap().as_bool().unwrap()),
        InteractionOptionType::USER => OptionValue::User(resolved.as_ref().unwrap().users.as_ref().unwrap().get(option.value.unwrap().as_str().unwrap()).unwrap().clone()),
        InteractionOptionType::CHANNEL => OptionValue::Channel(Box::new(resolved.as_ref().unwrap().channels.as_ref().unwrap().get(option.value.unwrap().as_str().unwrap()).unwrap().clone())),
        InteractionOptionType::ROLE => OptionValue::Role(resolved.as_ref().unwrap().roles.as_ref().unwrap().get(option.value.unwrap().as_str().unwrap()).unwrap().clone()),
        InteractionOptionType::MENTIONABLE => self.parse_mentionable(resolved, &option),
        InteractionOptionType::NUMBER => OptionValue::Number(option.value.unwrap().as_f64().unwrap()),
        InteractionOptionType::ATTACHMENT => OptionValue::Attachment(resolved.as_ref().unwrap().attachments.as_ref().unwrap().get(option.value.unwrap().as_str().unwrap()).unwrap().clone()),
        _ => OptionValue::Other(option.value.unwrap())
      };
      if option.focused.unwrap_or_default() {
        input.focused = Some(option.name.clone());
      }
      input.args.insert(option.name, option_value);
    }
  }

  fn parse_component_values(&self, components: Vec<Component>, input: &mut CommandInput) {
    for component in components.into_iter() {
      match component {
        Component::ActionRow(action_row) => {
          self.parse_component_values(action_row.components, input);
        },
        Component::TextInput(text_input) => {
          let value = OptionValue::String(text_input.value.unwrap_or_default());
          input.args.insert(text_input.custom_id, value);
        },
        _ => {}
      }
    }
  }

  fn parse_mentionable(&self, resolved: &Option<InteractionDataResolved>, option: &InteractionOption) -> OptionValue {
    let mut found_value = None;
    if let Some(users) = &resolved.as_ref().unwrap().users {
      if let Some(user) = users.get(option.value.as_ref().unwrap().as_str().unwrap()) {
        found_value = Some(OptionValue::User(user.clone()))
      }
    } else if let Some(roles) = &resolved.as_ref().unwrap().roles {
      if let Some(role) = roles.get(option.value.as_ref().unwrap().as_str().unwrap()) {
        found_value = Some(OptionValue::Role(role.clone()))
      }
    }
    if let Some(value) = found_value {
      value
    } else {
      panic!("Could not resolve mentionable");
    }
  }

  fn parse_resolved(&self, resolved: Option<InteractionDataResolved>, target_id: Option<String>, mut input: &mut CommandInput) {
    match input.command_type.as_ref().unwrap() {
      ApplicationCommandType::USER => {
        let target_id = target_id.expect("User context menu command has no target");
        let resolved = resolved.expect("User context menu command has no resolved");
        let mut resolved_users = resolved.users.expect("User context menu command has no resolved users");
        let user = resolved_users.remove(&target_id).expect("Target user not found");
        let mut resolved_members = resolved.members.expect("User context menu command has no resolved members");
        let member = resolved_members.remove(&target_id).expect("Target member not found");
        input.target_user = Some(user);
        input.target_member = Some(member);
      },
      ApplicationCommandType::MESSAGE => {
        let target_id = target_id.expect("Message context menu command has no target");
        let resolved = resolved.expect("Message context menu command has no resolved");
        let mut resolved_messages = resolved.messages.expect("Message context menu command has no resolved messages");
        let message = resolved_messages.remove(&target_id).expect("Target message not found");
        input.target_message = Some(message);
      },
      _ => {
        input.resolved = resolved;
      }
    }
  }

  fn parse_user(&self, user: Option<User>, member: &Option<GuildMember>) -> User {
    member.as_ref().map_or_else(|| user.unwrap(), |m| m.user.clone().unwrap())
  }

  async fn spawn_command(&self, command: Arc<Mutex<Command>>, id: String, token: String, input: CommandInput) -> Result<CommandResponse> {
    let (tx, mut rx) = mpsc::unbounded_channel::<CommandResponse>();
    let responder = CommandResponder { tx, id, token };

    spawn(async move {
      let fut = command.lock().unwrap().func.call(input, responder);
      if let Err(err) = fut.await {
        println!("Error returned from command handler: {:?}", err);
      }
    });

    let response = rx.recv().await.ok_or("Senders gone")?;
    rx.close();

    Ok(response)
  }

  fn format_response(&self, response: CommandResponse) -> Result<InteractionCallback> {
    match response {
      CommandResponse::DeferMessage(ephemeral) => {
        let mut flags = MessageFlags::empty();
        if ephemeral { flags.insert(MessageFlags::EPHEMERAL) };
        Ok(InteractionCallback {
          response_type: InteractionCallbackType::DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE,
          data: Some(flags.into())
        })
      },

      CommandResponse::DeferUpdate => {
        Ok(InteractionCallback {
          response_type: InteractionCallbackType::DEFERRED_UPDATE_MESSAGE,
          data: None
        })
      }

      CommandResponse::SendMessage(msg) => {
        Ok(InteractionCallback {
          response_type: InteractionCallbackType::CHANNEL_MESSAGE_WITH_SOURCE,
          data: Some(msg.into())
        })
      },

      CommandResponse::UpdateMessage(msg) => {
        Ok(InteractionCallback {
          response_type: InteractionCallbackType::UPDATE_MESSAGE,
          data: Some(msg.into())
        })
      },

      CommandResponse::AutocompleteResult(results) => {
        Ok(InteractionCallback {
          response_type: InteractionCallbackType::APPLICATION_COMMAND_AUTOCOMPLETE_RESULT,
          data: Some(results.into())
        })
      },

      CommandResponse::Modal(modal) => {
        Ok(InteractionCallback {
          response_type: InteractionCallbackType::MODAL,
          data: Some(modal.into())
        })
      }

    }
  }

  pub async fn handle_command(&self, interaction: Interaction, bot_token: Option<String>) -> Result<InteractionCallback> {
    let data = interaction.data.ok_or("Interaction has no data")?;

    let (name, custom_id): (String, Option<String>) = match interaction.interaction_type {
      InteractionType::APPLICATION_COMMAND | InteractionType::APPLICATION_COMMAND_AUTOCOMPLETE => {
        (data.name.ok_or("Command should have a name")?, None)
      },
      InteractionType::MESSAGE_COMPONENT | InteractionType::MODAL_SUBMIT => {
        let custom_id = data.custom_id.ok_or("Component interaction should have a custom_id")?;
        let (command_name, rest_id) = custom_id.as_str().split_once("/").ok_or("Invalid custom_id")?;
        (command_name.to_string(), Some(rest_id.to_string()))
      },
      _ => panic!("This type shouldn't be handled here")
    };

    let command = self.commands.get(&name).ok_or("Command not found")?;
    let task_command = command.clone();

    let mut input = CommandInput {
      interaction_type: interaction.interaction_type,
      command_type: data.command_type,
      component_type: data.component_type,
      command: name,
      sub_command: None,
      sub_command_group: None,
      args: HashMap::new(),
      resolved: None,
      guild_id: interaction.guild_id,
      channel_id: interaction.channel_id,
      user: self.parse_user(interaction.user, &interaction.member),
      member: interaction.member,
      message: interaction.message,
      target_user: None,
      target_member: None,
      target_message: None,
      custom_id,
      values: data.values,
      focused: None,
      locale: interaction.locale.expect("No locale in interaction"),
      guild_locale: interaction.guild_locale,
      rest: Rest::with_optional_token(bot_token)
    };

    if let Some(options) = data.options {
      self.parse_options(options, &data.resolved, &mut input);
    }

    if let Some(components) = data.components {
      self.parse_component_values(components, &mut input);
    }

    if input.command_type.is_some() {
      self.parse_resolved(data.resolved, data.target_id, &mut input);
    }

    let response = self.spawn_command(task_command, interaction.application_id, interaction.token, input).await?;
    self.format_response(response)
  }
}

impl CommandInput {
  /// Returns true if the interaction is for an executed command
  pub fn is_command(&self) -> bool {
    matches!(self.interaction_type, InteractionType::APPLICATION_COMMAND)
  }

  /// Returns true if the interaction is for a chat input command
  pub fn is_chat_input(&self) -> bool {
    self.command_type.as_ref().map_or(false, |t| matches!(t, ApplicationCommandType::CHAT_INPUT))
  }

  /// Returns true if the interaction is for a user context menu
  pub fn is_user_context(&self) -> bool {
    self.command_type.as_ref().map_or(false, |t| matches!(t, ApplicationCommandType::USER))
  }

  /// Returns true if the interaction is for a message context menu
  pub fn is_message_context(&self) -> bool {
    self.command_type.as_ref().map_or(false, |t| matches!(t, ApplicationCommandType::MESSAGE))
  }

  /// Returns true if the interaction is for a message component
  pub fn is_component(&self) -> bool {
    matches!(self.interaction_type, InteractionType::MESSAGE_COMPONENT)
  }

  /// Returns true if the interaction is for a clicked button
  pub fn is_button(&self) -> bool {
    self.component_type.as_ref().map_or(false, |t| matches!(t, ComponentType::BUTTON))
  }

  /// Returns true if the interaction is for a selecting from a select menu
  pub fn is_select_menu(&self) -> bool {
    self.component_type.as_ref().map_or(false, |t| matches!(t, ComponentType::SELECT_MENU))
  }

  /// Returns true if the interaction is for autocompletion
  pub fn is_autocomplete(&self) -> bool {
    matches!(self.interaction_type, InteractionType::APPLICATION_COMMAND_AUTOCOMPLETE)
  }

  /// Returns true if the interaction is for a modal submission
  pub fn is_modal_submit(&self) -> bool {
    matches!(self.interaction_type, InteractionType::MODAL_SUBMIT)
  }
}

#[derive(Debug)]
pub(crate) struct RocketCommand(pub Interaction, pub Option<String>, pub oneshot::Sender::<std::result::Result<InteractionCallback, ()>>);
