// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs used for handling commands

use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
use crate::tokio::{spawn, sync::{mpsc, oneshot}};
use anyhow::{anyhow, bail, Context};

use crate::structs::{
  interactions::{
    ApplicationCommand,
    Interaction, InteractionType, ApplicationCommandType, InteractionDataResolved, InteractionOption, InteractionOptionType,
    InteractionCallback,
    OptionValue
  },
  components::{Component, ComponentType},
  channels::Message,
  users::User,
  guilds::GuildMember,
  Snowflake,
  Permissions
};
use super::{Command, responder::{CommandResponder, CommandResponse}};
use crate::rest::Rest;

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
  /// Subcommand that was executed
  ///
  /// Only included in chat input commands
  pub subcommand: Option<String>,
  /// Subcommand group the subcommand belongs in
  ///
  /// Only included in chat input commands
  pub subcommand_group: Option<String>,
  /// Arguments or modal inputs the user of your command filled.\
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
  /// Resolved values from a select menu
  ///
  /// Only included in User, Role, Mentionable and Channel Select Menu component interactions
  pub resolved_values: Option<Vec<OptionValue>>,
  /// The argument currently in focus
  ///
  /// Only included in command autocomplete interactions
  pub focused: Option<String>,
  /// Permissions the app or bot has within the channel the interaction was sent from
  pub app_permissions: Option<Permissions>,
  /// The selected [language](https://discord.com/developers/docs/reference#locales) of the user
  pub locale: String,
  /// The guild's preferred locale
  pub guild_locale: Option<String>,
  /// Handler for Discord API calls
  pub rest: Rest,
}

pub(crate) struct CommandHandler {
  pub(crate) commands: HashMap<String, Arc<Mutex<Command>>>
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

  pub fn convert_commands(&self) -> anyhow::Result<Vec<ApplicationCommand>> {
    let mut vec = Vec::new();

    for c in self.commands.values() {
      let command = &*c.lock().map_err(|_| anyhow::Error::msg("Command had been poisoned"))?;
      if !command.ignore {
        vec.push(command.clone().try_into()?);
      }
    }

    Ok(vec)
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
          command_handler.handle_command(interaction, bot_token).await
        } else {
          Err(anyhow!("Unexpected InteractionType in rocket_bridge"))
        };

        handler_send.send(value).unwrap();
      });
    }
  }

  fn parse_options(&self, options: Vec<InteractionOption>, resolved: &Option<InteractionDataResolved>, input: &mut CommandInput) -> anyhow::Result<()> {
    for option in options.into_iter() {
      let option_value = match option.option_type {
        InteractionOptionType::SUB_COMMAND_GROUP => {
          input.subcommand_group = Some(option.name);
          return self.parse_options(option.options.context("Subcommand group has no subcommands")?, resolved, input)
        },
        InteractionOptionType::SUB_COMMAND => {
          input.subcommand = Some(option.name);
          if option.options.is_none() { return Ok(()) }
          return self.parse_options(option.options.unwrap(), resolved, input)
        },

        InteractionOptionType::STRING => OptionValue::String(
          option.value.context("String option has no value")?
          .as_str().context("String option value is not a string")?
          .to_string()
        ),
        InteractionOptionType::INTEGER => OptionValue::Integer(
          option.value.context("Integer option has no value")?
          .as_i64().context("Integer option value is not an integer")?
        ),
        InteractionOptionType::BOOLEAN => OptionValue::Boolean(
          option.value.context("Boolean option has no value")?
          .as_bool().context("Boolean option value is not a boolean")?
        ),
        InteractionOptionType::USER => OptionValue::User(
          resolved.as_ref().context("User option provided but no resolved object")?
          .users.as_ref().context("User option provided but no resolved users object")?
          .get(
            option.value.context("User option has no value")?
            .as_str().context("User option value is not a string (user id)")?
          ).context("User option provided but no matching resolved user found")?
          .clone()
        ),
        InteractionOptionType::CHANNEL => OptionValue::Channel(Box::new(
          resolved.as_ref().context("Channel option provided but no resolved object")?
          .channels.as_ref().context("Channel option provided but not resolved channels object")?
          .get(
            option.value.context("Channel option has no value")?
            .as_str().context("Channel option value is not a string (channel id)")?
          ).context("Channel option provided but no matching resolved channel found")?
          .clone()
        )),
        InteractionOptionType::ROLE => OptionValue::Role(
          resolved.as_ref().context("Role option provided but no resolved object")?
          .roles.as_ref().context("Role option provided but no resolved roles object")?
          .get(
            option.value.context("Role option has no value")?
            .as_str().context("Role option value is not a string (role id)")?
          ).context("Role option provided but no matching resolved role found")?
          .clone()
        ),
        InteractionOptionType::MENTIONABLE => self.parse_mentionable(
          resolved.as_ref().context("Mentionable option provided but no resolved object")?,
          option.value.as_ref().context("Mentionable option has no value")?.as_str().context("Mentionable option value is not a string (user or role id)")?
        )?,
        InteractionOptionType::NUMBER => OptionValue::Number(
          option.value.context("Number option has no value")?
          .as_f64().context("Number option value is not a number")?
        ),
        InteractionOptionType::ATTACHMENT => OptionValue::Attachment(
          resolved.as_ref().context("Attachment option provided but no resolved object")?
          .attachments.as_ref().context("Attachment option provided but no resolved attachments object")?
          .get(
            option.value.context("Attachment option has no value")?
            .as_str().context("Attachment option value is not a string (attachment id)")?
          ).context("Attachment option provided but no matching resolved attachment found")?
          .clone()
        ),
        _ => OptionValue::Other(option.value.unwrap_or_default())
      };
      if option.focused.unwrap_or_default() {
        input.focused = Some(option.name.clone());
      }
      input.args.insert(option.name, option_value);
    }
    Ok(())
  }

  fn parse_select_values(&self, values: Vec<String>, resolved: &Option<InteractionDataResolved>, input: &mut CommandInput) -> anyhow::Result<()> {
    let mut resolved_values = Vec::new();
    match input.component_type.as_ref().context("Somehow trying to parse values without a component type")? {
      ComponentType::USER_SELECT => {
        for value in values.iter() {
          resolved_values.push(OptionValue::User(
            resolved.as_ref().context("User select provided but no resolved object")?
            .users.as_ref().context("User select provided but no resolved users object")?
            .get(value).context("User select provided but no matching resolved user found")?
            .clone()
          ));
        }
      },
      ComponentType::ROLE_SELECT => {
        for value in values.iter() {
          resolved_values.push(OptionValue::Role(
            resolved.as_ref().context("Role select provided but no resolved object")?
            .roles.as_ref().context("Role select provided but no resolved roles object")?
            .get(value).context("Role select provided but no matching resolved role found")?
            .clone()
          ));
        }
      },
      ComponentType::MENTIONABLE_SELECT => {
        for value in values.iter() {
          resolved_values.push(
            self.parse_mentionable(resolved.as_ref().context("Mentionable select provided but no resolved object")?, value)?
          )
        }
      },
      ComponentType::CHANNEL_SELECT => {
        for value in values.iter() {
          resolved_values.push(OptionValue::Channel(
            Box::new(resolved.as_ref().context("Channel select provided but no resolved object")?
            .channels.as_ref().context("Channel select provided but no resolved channels object")?
            .get(value).context("Channel select provided but no matching resolved channel found")?
            .clone())
          ));
        }
      }
      _ => {},
    };
    input.values = Some(values);
    input.resolved_values = Some(resolved_values);
    Ok(())
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

  fn parse_mentionable(&self, resolved: &InteractionDataResolved, option_value: &str) -> anyhow::Result<OptionValue> {
    let mut found_value = None;
    if let Some(users) = &resolved.users {
      if let Some(user) = users.get(option_value) {
        found_value = Some(OptionValue::User(user.clone()))
      }
    }
    if let Some(roles) = &resolved.roles {
      if let Some(role) = roles.get(option_value) {
        found_value = Some(OptionValue::Role(role.clone()))
      }
    }
    if let Some(value) = found_value {
      Ok(value)
    } else {
      bail!("Mentionable option provided but no matching resolved user or role found");
    }
  }

  fn parse_resolved(&self, resolved: Option<InteractionDataResolved>, target_id: Option<String>, input: &mut CommandInput) -> anyhow::Result<()> {
    match input.command_type.as_ref().context("Somehow trying to parse resolved without a command type")? {
      ApplicationCommandType::USER => {
        let target_id = target_id.context("User context menu command has no target")?;
        let resolved = resolved.context("User context menu command has no resolved")?;
        let mut resolved_users = resolved.users.context("User context menu command has no resolved users")?;
        let user = resolved_users.remove(&target_id);
        let mut member = None;
        if let Some(mut resolved_members) = resolved.members {
          member = resolved_members.remove(&target_id);
        }
        input.target_user = user;
        input.target_member = member;
      },
      ApplicationCommandType::MESSAGE => {
        let target_id = target_id.context("Message context menu command has no target")?;
        let resolved = resolved.context("Message context menu command has no resolved")?;
        let mut resolved_messages = resolved.messages.context("Message context menu command has no resolved messages")?;
        let message = resolved_messages.remove(&target_id);
        input.target_message = message;
      },
      _ => {
        input.resolved = resolved;
      }
    }
    Ok(())
  }

  fn parse_user(&self, user: Option<User>, member: &Option<GuildMember>) -> anyhow::Result<User> {
    member.as_ref().map_or_else(|| user.context("No member or user provided"), |m| m.user.clone().context("No user object in member object"))
  }

  async fn spawn_command(&self, command: Arc<Mutex<Command>>, id: String, token: String, input: CommandInput) -> anyhow::Result<CommandResponse> {
    let (tx, mut rx) = mpsc::unbounded_channel::<CommandResponse>();
    let responder = CommandResponder {
      tx,
      id,
      token,
      rest: Rest::new()
    };

    spawn(async move {
      let fut = command.lock().unwrap().func.call(input, responder);
      if let Err(err) = fut.await {
        eprintln!("Error returned from command handler: {:?}", err);
      }
    });

    let response = rx.recv().await.context("Command handler finished without responding")?;
    rx.close();

    Ok(response)
  }

  pub async fn handle_command(&self, interaction: Interaction, bot_token: Option<String>) -> anyhow::Result<InteractionCallback> {
    let data = interaction.data.context("Interaction has no data")?;

    let (name, custom_id): (String, Option<String>) = match interaction.interaction_type {
      InteractionType::APPLICATION_COMMAND | InteractionType::APPLICATION_COMMAND_AUTOCOMPLETE => {
        (data.name.context("Command interaction is missing a command name")?, None)
      },
      InteractionType::MESSAGE_COMPONENT | InteractionType::MODAL_SUBMIT => {
        let custom_id = data.custom_id.context("Component interaction is missing a custom_id")?;
        let (command_name, rest_id) = custom_id.split_once('/').with_context(|| format!("Received custom_id ({}) is not in the correct format", custom_id))?;
        (command_name.to_string(), Some(rest_id.to_string()))
      },
      _ => bail!("Unexpected InteractionType in handle_command")
    };

    let command = self.commands.get(&name).with_context(|| format!("Received command ({}) has no registered command handler", name))?;
    let task_command = command.clone();

    let mut input = CommandInput {
      interaction_type: interaction.interaction_type,
      command_type: data.command_type,
      component_type: data.component_type,
      command: name,
      subcommand: None,
      subcommand_group: None,
      args: HashMap::new(),
      resolved: None,
      guild_id: interaction.guild_id,
      channel_id: interaction.channel_id,
      user: self.parse_user(interaction.user, &interaction.member)?,
      member: interaction.member,
      message: interaction.message,
      target_user: None,
      target_member: None,
      target_message: None,
      custom_id,
      values: None,
      resolved_values: None,
      focused: None,
      app_permissions: interaction.app_permissions,
      locale: interaction.locale.context("Interaction didn't include a locale")?,
      guild_locale: interaction.guild_locale,
      rest: Rest::with_optional_token(bot_token)
    };

    if let Some(options) = data.options {
      self.parse_options(options, &data.resolved, &mut input)?;
    }

    if let Some(components) = data.components {
      self.parse_component_values(components, &mut input);
    }

    if let Some(values) = data.values {
      self.parse_select_values(values, &data.resolved, &mut input)?;
    }

    if input.command_type.is_some() {
      self.parse_resolved(data.resolved, data.target_id, &mut input)?;
    }

    let response = self.spawn_command(task_command, interaction.application_id, interaction.token, input).await?;
    Ok(response.into())
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

  /// Returns true if the interaction is for a string select menu
  pub fn is_string_select(&self) -> bool {
    self.component_type.as_ref().map_or(false, |t| matches!(t, ComponentType::STRING_SELECT))
  }

  /// Returns true if the interaction is for a user select menu
  pub fn is_user_select(&self) -> bool {
    self.component_type.as_ref().map_or(false, |t| matches!(t, ComponentType::USER_SELECT))
  }

  /// Returns true if the interaction is for a role select menu
  pub fn is_role_select(&self) -> bool {
    self.component_type.as_ref().map_or(false, |t| matches!(t, ComponentType::ROLE_SELECT))
  }

  /// Returns true if the interaction is for a mentionable select menu
  pub fn is_mentionable_select(&self) -> bool {
    self.component_type.as_ref().map_or(false, |t| matches!(t, ComponentType::MENTIONABLE_SELECT))
  }

  /// Returns true if the interaction is for a channel select menu
  pub fn is_channel_select(&self) -> bool {
    self.component_type.as_ref().map_or(false, |t| matches!(t, ComponentType::CHANNEL_SELECT))
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
pub(crate) struct RocketCommand(pub Interaction, pub Option<String>, pub oneshot::Sender::<anyhow::Result<InteractionCallback>>);
