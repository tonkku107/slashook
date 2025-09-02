// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::structs::{
  components::{Component, Components},
  embeds::Embed,
  interactions::{ApplicationCommandOptionChoice, Attachments, InteractionCallbackData},
  messages::{AllowedMentions, Attachment, Message, MessageFlags, MessageReference},
  polls::PollCreateRequest,
  utils::File, Snowflake,
};
use serde::Serialize;
use crate::tokio::sync::mpsc;
use crate::rest::{Rest, RestError};

/// Error for when a response failed due to the interaction having been responded to already.
#[derive(Debug)]
pub struct InteractionResponseError;
impl std::fmt::Display for InteractionResponseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Interaction has already been responded to.")
  }
}
impl std::error::Error for InteractionResponseError { }

/// Message that can be sent as a response to a command or other interaction
///
/// This struct can be easily constructed from a `str`, `String`, [`Embed`](crate::structs::embeds::Embed), [`Components`](crate::structs::components::Components),
/// [`File`](crate::structs::utils::File) or [`PollCreateRequest`](crate::structs::polls::PollCreateRequest)
/// with the `From` trait
#[derive(Serialize, Clone, Debug)]
pub struct MessageResponse {
  /// Whether the response is TTS
  pub tts: Option<bool>,
  /// Message content (up to 2000 characters)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  /// Up to 10 embeds (up to 6000 characters)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub embeds: Option<Vec<Embed>>,
  /// [Allowed mentions](AllowedMentions) object
  #[serde(skip_serializing_if = "Option::is_none")]
  pub allowed_mentions: Option<AllowedMentions>,
  /// [Message flags](MessageFlags) combined as a bitfield
  /// (only [SUPPRESS_EMBEDS](MessageFlags::SUPPRESS_EMBEDS), [EPHEMERAL](MessageFlags::EPHEMERAL), and [SUPPRESS_NOTIFICATIONS](MessageFlags::SUPPRESS_NOTIFICATIONS) can be set for interaction responses)
  pub flags: Option<MessageFlags>,
  /// Message components
  #[serde(skip_serializing_if = "Option::is_none")]
  pub components: Option<Vec<Component>>,
  /// Attachment objects with filename and description.\
  /// Use `files` if you want to upload an attachment.\
  /// This is for partial attachment objects indicating which to keep when editing.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub attachments: Option<Vec<Attachment>>,
  /// Details about the poll
  #[serde(skip_serializing_if = "Option::is_none")]
  pub poll: Option<PollCreateRequest>,
  /// Include to make your message a reply or a forward (not available for interaction responses)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub message_reference: Option<MessageReference>,
  /// IDs of up to 3 stickers in the server to send in the message
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sticker_ids: Option<Vec<Snowflake>>,
  /// Up to 10 files to send with the response
  #[serde(skip_serializing)]
  pub files: Option<Vec<File>>,
}

impl MessageResponse {
  /// Set the value of tts for the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// let response = MessageResponse::from("This message is text to speech")
  ///   .set_tts(true);
  /// assert_eq!(response.tts, Some(true));
  /// ```
  pub fn set_tts(mut self, tts: bool) -> Self {
    self.tts = Some(tts);
    self
  }

  /// Set the content of the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// let response = MessageResponse::from("This content will be replaced")
  ///   .set_content("I rule the world!");
  /// assert_eq!(response.content, Some(String::from("I rule the world!")));
  /// ```
  pub fn set_content<T: ToString>(mut self, content: T) -> Self {
    self.content = Some(content.to_string());
    self
  }

  /// Add an embed to the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new().set_description("This is an embed");
  /// let response = MessageResponse::from("Look at my embed:")
  ///   .add_embed(embed);
  /// assert_eq!(response.embeds.unwrap()[0].description, Some(String::from("This is an embed")));
  /// ```
  pub fn add_embed(mut self, embed: Embed) -> Self {
    let mut embeds = self.embeds.unwrap_or_default();
    embeds.push(embed);
    self.embeds = Some(embeds);
    self
  }

  /// Clear embeds from the message. Sets embeds to an empty Vec which also clears embeds when editing.
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// let response = MessageResponse::from("Embeds cleared")
  ///   .clear_embeds();
  /// assert_eq!(response.embeds.unwrap().len(), 0);
  /// ```
  pub fn clear_embeds(mut self) -> Self {
    self.embeds = Some(Vec::new());
    self
  }

  /// Set the allowed mentions for the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::{AllowedMentions, AllowedMentionType};
  /// let allowed_mentions = AllowedMentions::new().add_parse(AllowedMentionType::USERS);
  /// let response = MessageResponse::from("<@1234> Get pinged. Not @everyone or <@&1235> tho.")
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn set_allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
    self.allowed_mentions = Some(allowed_mentions);
    self
  }

  /// Set the ephemeralness of the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::MessageFlags;
  /// let response = MessageResponse::from("This is for your eyes only!")
  ///   .set_ephemeral(true);
  /// assert_eq!(response.flags.unwrap().contains(MessageFlags::EPHEMERAL), true);
  /// ```
  pub fn set_ephemeral(mut self, ephemeral: bool) -> Self {
    let mut flags = self.flags.unwrap_or_else(MessageFlags::empty);
    flags.set(MessageFlags::EPHEMERAL, ephemeral);
    self.flags = Some(flags);
    self
  }

  /// Set suppress embeds flag
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::MessageFlags;
  /// let response = MessageResponse::from("No embeds here")
  ///   .set_suppress_embeds(true);
  /// assert_eq!(response.flags.unwrap().contains(MessageFlags::SUPPRESS_EMBEDS), true);
  /// ```
  pub fn set_suppress_embeds(mut self, suppress: bool) -> Self {
    let mut flags = self.flags.unwrap_or_else(MessageFlags::empty);
    flags.set(MessageFlags::SUPPRESS_EMBEDS, suppress);
    self.flags = Some(flags);
    self
  }

  /// Set suppress notifications flag
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::MessageFlags;
  /// let response = MessageResponse::from("No notifications @here")
  ///   .set_suppress_notifications(true);
  /// assert_eq!(response.flags.unwrap().contains(MessageFlags::SUPPRESS_NOTIFICATIONS), true);
  /// ```
  pub fn set_suppress_notifications(mut self, suppress: bool) -> Self {
    let mut flags = self.flags.unwrap_or_else(MessageFlags::empty);
    flags.set(MessageFlags::SUPPRESS_NOTIFICATIONS, suppress);
    self.flags = Some(flags);
    self
  }

  /// Set voice message flag
  /// ```no_run
  /// # use slashook::commands::{MessageResponse, CmdResult};
  /// # use slashook::structs::{messages::MessageFlags, utils::File};
  /// # use slashook::tokio::fs::File as TokioFile;
  /// # #[slashook::main]
  /// # async fn main() -> CmdResult {
  /// let file = TokioFile::open("audio.ogg").await?;
  /// let audio_file = File::from_file("audio.ogg", file).await?
  ///   .set_duration_secs(1.1799999475479126)
  ///   .set_waveform("AAM1YAAAAAAAAAA=");
  /// let response = MessageResponse::from(audio_file).set_as_voice_message(true);
  /// assert_eq!(response.flags.unwrap().contains(MessageFlags::IS_VOICE_MESSAGE), true);
  /// # Ok(())
  /// # }
  /// ```
  pub fn set_as_voice_message(mut self, is_voice_message: bool) -> Self {
    let mut flags = self.flags.unwrap_or_else(MessageFlags::empty);
    flags.set(MessageFlags::IS_VOICE_MESSAGE, is_voice_message);
    self.flags = Some(flags);
    self
  }

  /// Use Components V2 on the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::components::{Components, TextDisplay, Container};
  /// let text = TextDisplay::new("some text");
  /// let inner_text = TextDisplay::new("more text");
  /// let container = Container::new().add_component(inner_text);
  /// let components = Components::empty()
  ///   .add_component(text)
  ///   .add_component(container);
  /// let response = MessageResponse::from(components)
  ///   .set_components_v2(true);
  /// ```
  pub fn set_components_v2(mut self, components_v2: bool) -> Self {
    let mut flags = self.flags.unwrap_or_else(MessageFlags::empty);
    flags.set(MessageFlags::IS_COMPONENTS_V2, components_v2);
    self.flags = Some(flags);
    self
  }

  /// Set the components on the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::components::{Components, Button, ButtonStyle};
  /// let button = Button::new()
  ///   .set_style(ButtonStyle::DANGER)
  ///   .set_label("Do not press!")
  ///   .set_id("example_button", "danger");
  /// let components = Components::new().add_button(button);
  /// let response = MessageResponse::from("Ooh! A big red button!")
  ///   .set_components(components);
  /// ```
  pub fn set_components(mut self, components: Components) -> Self {
    self.components = Some(components.0);
    self
  }

  /// Add a file to be sent with the message
  /// ```no_run
  /// # use slashook::commands::{MessageResponse, CmdResult};
  /// # use slashook::structs::utils::File;
  /// use slashook::tokio::fs::File as TokioFile;
  /// # #[slashook::main]
  /// # async fn main() -> CmdResult {
  /// let file = TokioFile::open("cat.png").await?;
  /// let msg_file = File::from_file("cat.png", file).await?
  ///   .set_description("Picture of my cute cat!");
  /// let response = MessageResponse::from("Here's a picture of my cat")
  ///   .add_file(msg_file);
  /// # Ok(())
  /// # }
  /// ```
  pub fn add_file(mut self, file: File) -> Self {
    let mut files = self.files.unwrap_or_default();
    files.push(file);
    self.files = Some(files);
    self
  }

  /// Keep an existing attachment when editing
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::utils::File;
  /// # use slashook::tokio::fs::File as TokioFile;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg_file = File::from_file("cat.png", TokioFile::open("cat.png").await?).await?;
  /// let msg_file2 = File::from_file("cat2.png", TokioFile::open("cat2.png").await?).await?;
  ///
  /// res.defer(false).await?;
  ///
  /// let response = MessageResponse::from("Here's a picture of my cat")
  ///   .add_file(msg_file);
  /// let msg = res.send_followup_message(response).await?;
  ///
  /// let edit_response = MessageResponse::from("And I added the other cat too!")
  ///   .keep_attachment(&msg.attachments.get(0).unwrap().id)
  ///   .add_file(msg_file2);
  /// res.edit_original_message(edit_response).await?;
  /// # }
  /// ```
  pub fn keep_attachment<T: ToString>(mut self, attachment_id: T) -> Self {
    let mut attachments = self.attachments.unwrap_or_default();
    attachments.push(Attachment::keep_with_id(attachment_id));
    self.attachments = Some(attachments);
    self
  }

  /// Clear attachments from the message. Sets attachments to an empty Vec which also deletes attachments when editing.
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// let response = MessageResponse::from("Attachments deleted")
  ///   .clear_attachments();
  /// assert_eq!(response.attachments.unwrap().len(), 0);
  /// ```
  pub fn clear_attachments(mut self) -> Self {
    self.attachments = Some(Vec::new());
    self
  }

  /// Add a poll to the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::{polls::{PollCreateRequest, PollAnswer}, Emoji};
  /// let response = MessageResponse::from("This message will contain a poll!")
  ///   .set_poll(PollCreateRequest::new("Is this a good poll?")
  ///     .add_answer(PollAnswer::new().set_text("Yes").set_emoji(Emoji::new_standard_emoji("✅")))
  ///     .add_answer(PollAnswer::from("No").set_emoji(Emoji::new_custom_emoji("567088349484023818", "redtick", false)))
  ///     .add_answer("Maybe")
  ///     .set_duration(1)
  ///   );
  /// ```
  pub fn set_poll(mut self, poll: PollCreateRequest) -> Self {
    self.poll = Some(poll);
    self
  }

  /// Set a reply or forward
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::MessageReference;
  /// let response = MessageResponse::from("This is a reply")
  ///   .set_message_reference(MessageReference::new_reply("916413462467465246"));
  /// assert_eq!(response.message_reference.unwrap().message_id, Some(String::from("916413462467465246")));
  /// ```
  pub fn set_message_reference(mut self, message_reference: MessageReference) -> Self {
    self.message_reference = Some(message_reference);
    self
  }

  /// Add a sticker to the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// let response = MessageResponse::from(":)")
  ///   .add_sticker("749044136589393960");
  /// assert_eq!(response.sticker_ids.unwrap().remove(0), String::from("749044136589393960"));
  /// ```
  pub fn add_sticker<T: ToString>(mut self, sticker_id: T) -> Self {
    let mut sticker_ids = self.sticker_ids.unwrap_or_default();
    sticker_ids.push(sticker_id.to_string());
    self.sticker_ids = Some(sticker_ids);
    self
  }
}

/// A modal that can be opened for user input
#[derive(Clone, Debug)]
pub struct Modal {
  /// a developer-defined identifier for the component, max 100 characters
  pub custom_id: String,
  /// The title of the popup modal
  pub title: String,
  /// The components that make up the modal
  pub components: Vec<Component>
}

impl Modal {
  /// Creates a new modal.\
  /// The command argument is used by the library to choose which command to run when the modal is submitted.
  /// The custom_id is formatted as `command/id`
  /// ```
  /// # use slashook::commands::Modal;
  /// let modal = Modal::new("example_modal", "modal1", "Please fill this form");
  /// ```
  pub fn new<T: ToString, U: ToString, V: ToString>(command: T, id: U, title: V) -> Self {
    Self {
      custom_id: format!("{}/{}", command.to_string(), id.to_string()),
      title: title.to_string(),
      components: Vec::new()
    }
  }

  /// Set the components on the modal
  /// ```
  /// # use slashook::commands::Modal;
  /// # use slashook::structs::components::{Components, TextInput, SelectMenu, SelectMenuType, SelectOption, Label};
  /// let text_input = TextInput::new()
  ///   .set_id("input");
  /// let select_menu = SelectMenu::new(SelectMenuType::STRING)
  ///   .add_option(SelectOption::new("Yes", "yes"))
  ///   .add_option(SelectOption::new("No", "no"))
  ///   .set_id("", "select");
  /// let components = Components::new_label(
  ///   Label::new("Tell us something").set_description("It could be anything")
  /// )
  ///   .add_text_input(text_input)
  ///   .add_label(Label::new("Yes or No"))
  ///   .add_select_menu(select_menu);
  /// let modal = Modal::new("example_modal", "modal1", "Please fill this form")
  ///   .set_components(components);
  /// ```
  pub fn set_components(mut self, components: Components) -> Self {
    self.components = components.0;
    self
  }
}

#[derive(Debug)]
pub enum CommandResponse {
  DeferMessage(MessageFlags),
  SendMessage(MessageResponse),
  DeferUpdate,
  UpdateMessage(MessageResponse),
  AutocompleteResult(Vec<ApplicationCommandOptionChoice>),
  Modal(Modal),
  LaunchActivity,
}

/// Struct with methods for responding to interactions
#[derive(Debug)]
pub struct CommandResponder {
  pub(crate) tx: mpsc::UnboundedSender<CommandResponse>,
  pub(crate) id: String,
  pub(crate) token: String,
  pub(crate) rest: Rest
}

impl CommandResponder {
  /// Respond to an interaction with a message.\
  /// If interaction has already been responded to, this function will call [`send_followup_message`](CommandResponder::send_followup_message) instead and a message can only be returned in this case.
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// ##[command(name = "example", description = "An example command")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("Hello!").await?;
  /// }
  /// ```
  pub async fn send_message<T: Into<MessageResponse>>(&self, response: T) -> Result<Option<Message>, RestError> {
    let response = response.into();
    match self.tx.send(CommandResponse::SendMessage(response)) {
      Ok(_) => {
        self.tx.closed().await;
        Ok(None)
      },
      Err(err) => {
        if let CommandResponse::SendMessage(response) = err.0 {
          return self.send_followup_message(response).await.map(Some);
        }
        Ok(None)
      }
    }
  }

  /// Respond to an interaction by editing the original message.\
  /// If interaction has already been responded to, this function will call [`edit_original_message`](CommandResponder::edit_original_message) instead and a message can only be returned in this case.
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// # use slashook::structs::components::{Components, Button};
  /// ##[command(name = "example_button", ignore = true)]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.update_message("Button was clicked!").await?;
  /// }
  /// ```
  pub async fn update_message<T: Into<MessageResponse>>(&self, response: T) -> Result<Option<Message>, RestError> {
    let response = response.into();
    match self.tx.send(CommandResponse::UpdateMessage(response)) {
      Ok(_) => {
        self.tx.closed().await;
        Ok(None)
      },
      Err(err) => {
        if let CommandResponse::UpdateMessage(response) = err.0 {
          return self.edit_original_message(response).await.map(Some);
        }
        Ok(None)
      }
    }
  }

  /// Give yourself more execution time.\
  /// If you don't respond within 3 seconds, Discord will disconnect and tell the user the interaction failed to run.
  /// By deferring, Discord will tell the user your bot is "thinking" and allow you to take your time. You can use the `send_followup_message` or `edit_original_message` methods to send the response.\
  /// The ephemeralness set here will be passed on to your first follow-up, no matter what ephemeralness you set there.
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command(name = "example", description = "An example command")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.defer(false).await?;
  ///   // Do something that takes longer than 3s
  ///   res.send_followup_message("Thank you for your patience!").await?;
  /// }
  /// ```
  pub async fn defer(&self, ephemeral: bool) -> Result<(), InteractionResponseError> {
    let mut flags = MessageFlags::empty();
    flags.set(MessageFlags::EPHEMERAL, ephemeral);
    self.tx.send(CommandResponse::DeferMessage(flags)).map_err(|_| InteractionResponseError)?;
    self.tx.closed().await;
    Ok(())
  }

  /// Much like `defer` but for component interactions and it shows nothing visibly to the user.
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command(name = "example_button", ignore = true)]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.defer_update().await?;
  ///   // Do something that takes longer than 3s
  ///   res.edit_original_message("Finally it changed!").await?;
  /// }
  /// ```
  pub async fn defer_update(&self) -> Result<(), InteractionResponseError> {
    self.tx.send(CommandResponse::DeferUpdate).map_err(|_| InteractionResponseError)?;
    self.tx.closed().await;
    Ok(())
  }

  /// Respond to an autocomplete interaction with autocomplete choices
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// # use slashook::structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType};
  /// ##[command(name = "example", description = "An example command", options = [{
  ///   name = "choice", description = "Choose an option",
  ///   autocomplete = true, option_type = InteractionOptionType::STRING
  /// }])]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   if input.is_autocomplete() {
  ///     let search = input.args.get(&input.focused.unwrap()).unwrap().as_string().unwrap();
  ///     // Use the current input to fetch or filter choices
  ///     let choices = vec![
  ///       ApplicationCommandOptionChoice::new("An autocompleted choice", "autocomplete1"),
  ///       ApplicationCommandOptionChoice::new("Another autocompleted choice", "autocomplete2")
  ///     ];
  ///     return res.autocomplete(choices).await?;
  ///   }
  /// }
  /// ```
  pub async fn autocomplete(&self, results: Vec<ApplicationCommandOptionChoice>) -> Result<(), InteractionResponseError> {
    self.tx.send(CommandResponse::AutocompleteResult(results)).map_err(|_| InteractionResponseError)?;
    self.tx.closed().await;
    Ok(())
  }

  /// Respond to an interaction with a modal
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse, Modal};
  /// # use slashook::structs::components::{Components, TextInput, SelectMenu, SelectMenuType, SelectOption, Label};
  /// ##[command(name = "example", description = "An example command")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   let text_input = TextInput::new()
  ///     .set_id("input");
  ///   let select_menu = SelectMenu::new(SelectMenuType::STRING)
  ///     .add_option(SelectOption::new("Yes", "yes"))
  ///     .add_option(SelectOption::new("No", "no"))
  ///     .set_id("", "select");
  ///   let components = Components::new_label(
  ///     Label::new("Tell us something").set_description("It could be anything")
  ///   )
  ///     .add_text_input(text_input)
  ///     .add_label(Label::new("Yes or No"))
  ///     .add_select_menu(select_menu);
  ///   let modal = Modal::new("example_modal", "modal1", "Please fill this form")
  ///     .set_components(components);
  ///   return res.open_modal(modal).await?;
  /// }
  /// ```
  pub async fn open_modal(&self, modal: Modal) -> Result<(), InteractionResponseError> {
    self.tx.send(CommandResponse::Modal(modal)).map_err(|_| InteractionResponseError)?;
    self.tx.closed().await;
    Ok(())
  }

  /// Respond to an interaction by launching the activity associated with the app.
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::interactions::{ApplicationCommandType, ApplicationCommandHandlerType};
  /// ##[command(
  ///   name = "launch",
  ///   command_type = ApplicationCommandType::PRIMARY_ENTRY_POINT,
  ///   handler = ApplicationCommandHandlerType::APP_HANDLER
  /// )]
  /// fn launch(input: CommandInput, res: CommandResponder) {
  ///   return res.launch_activity().await?;
  /// }
  /// ```
  pub async fn launch_activity(&self) -> Result<(), InteractionResponseError> {
    self.tx.send(CommandResponse::LaunchActivity).map_err(|_| InteractionResponseError)?;
    self.tx.closed().await;
    Ok(())
  }

  /// Send more messages after the initial response
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command(name = "example", description = "An example command")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("First message!").await?;
  ///   res.send_followup_message("Second message!").await?;
  /// }
  /// ```
  pub async fn send_followup_message<T: Into<MessageResponse>>(&self, response: T) -> Result<Message, RestError> {
    let mut response = response.into();
    let files = response.files.take();
    let msg: InteractionCallbackData = response.into();
    let path = format!("webhooks/{}/{}", self.id, self.token);
    if let Some(files) = files {
      self.rest.post_files(path, msg, files).await
    } else {
      self.rest.post(path, msg).await
    }
  }

  /// Edits a follow-up message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command(name = "example", description = "An example command")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("First message!").await?;
  ///   let msg = res.send_followup_message("Second message!").await?;
  ///   res.edit_followup_message(msg.id.unwrap(), "Second message but edited!").await?;
  /// }
  /// ```
  pub async fn edit_followup_message<T: Into<MessageResponse>>(&self, id: String, response: T) -> Result<Message, RestError> {
    let mut response = response.into();
    let files = response.files.take();
    let msg: InteractionCallbackData = response.into();
    let path = format!("webhooks/{}/{}/messages/{}", self.id, self.token, id);
    if let Some(files) = files {
      self.rest.patch_files(path, msg, files).await
    } else {
      self.rest.patch(path, msg).await
    }
  }

  /// Edits the original message\
  /// Same as running `edit_followup_message` with id of `@original`
  pub async fn edit_original_message<T: Into<MessageResponse>>(&self, response: T) -> Result<Message, RestError> {
    self.edit_followup_message(String::from("@original"), response).await
  }

  /// Gets a follow-up message
  pub async fn get_followup_message(&self, id: String) -> Result<Message, RestError> {
    self.rest.get(format!("webhooks/{}/{}/messages/{}", self.id, self.token, id)).await
  }

  /// Gets the original message\
  /// Same as running `get_followup_message` with id of `@original`
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command(name = "example", description = "An example command")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("First message!").await?;
  ///   let msg = res.get_original_message().await?;
  ///   println!("I responded with {}", msg.content);
  /// }
  /// ```
  pub async fn get_original_message(&self) -> Result<Message, RestError> {
    self.get_followup_message(String::from("@original")).await
  }

  /// Deletes a follow-up message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command(name = "example", description = "An example command")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("First message!").await?;
  ///   let msg = res.send_followup_message("If you see me say hi").await?;
  ///   res.delete_followup_message(msg.id.unwrap()).await?;
  /// }
  /// ```
  pub async fn delete_followup_message(&self, id: String) -> Result<(), RestError> {
    self.rest.delete(format!("webhooks/{}/{}/messages/{}", self.id, self.token, id)).await
  }

  /// Deletes the original message\
  /// Same as running `delete_followup_message` with id of `@original`
  pub async fn delete_original_message(&self) -> Result<(), RestError> {
    self.delete_followup_message(String::from("@original")).await
  }
}

impl From<&str> for MessageResponse {
  fn from(s: &str) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: Some(String::from(s)),
      embeds: None,
      allowed_mentions: None,
      flags: None,
      components: None,
      attachments: None,
      poll: None,
      message_reference: None,
      sticker_ids: None,
      files: None,
    }
  }
}

impl From<String> for MessageResponse {
  fn from(s: String) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: Some(s),
      embeds: None,
      allowed_mentions: None,
      flags: None,
      components: None,
      attachments: None,
      poll: None,
      message_reference: None,
      sticker_ids: None,
      files: None,
    }
  }
}

impl From<Embed> for MessageResponse {
  fn from(e: Embed) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      embeds: Some(vec![e]),
      allowed_mentions: None,
      flags: None,
      components: None,
      attachments: None,
      poll: None,
      message_reference: None,
      sticker_ids: None,
      files: None,
    }
  }
}

impl From<Vec<Embed>> for MessageResponse {
  fn from(e: Vec<Embed>) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      embeds: Some(e),
      allowed_mentions: None,
      flags: None,
      components: None,
      attachments: None,
      poll: None,
      message_reference: None,
      sticker_ids: None,
      files: None,
    }
  }
}

impl From<Components> for MessageResponse {
  fn from(c: Components) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      embeds: None,
      allowed_mentions: None,
      flags: None,
      components: Some(c.0),
      attachments: None,
      poll: None,
      message_reference: None,
      sticker_ids: None,
      files: None,
    }
  }
}

impl From<File> for MessageResponse {
  fn from(f: File) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      embeds: None,
      allowed_mentions: None,
      flags: None,
      components: None,
      attachments: None,
      poll: None,
      message_reference: None,
      sticker_ids: None,
      files: Some(vec![f]),
    }
  }
}

impl From<Vec<File>> for MessageResponse {
  fn from(f: Vec<File>) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      embeds: None,
      allowed_mentions: None,
      flags: None,
      components: None,
      attachments: None,
      poll: None,
      message_reference: None,
      sticker_ids: None,
      files: Some(f),
    }
  }
}

impl From<PollCreateRequest> for MessageResponse {
  fn from(poll: PollCreateRequest) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      embeds: None,
      allowed_mentions: None,
      flags: None,
      components: None,
      attachments: None,
      poll: Some(poll),
      message_reference: None,
      sticker_ids: None,
      files: None,
    }
  }
}

impl From<MessageReference> for MessageResponse {
  fn from(reference: MessageReference) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      embeds: None,
      allowed_mentions: None,
      flags: None,
      components: None,
      attachments: None,
      poll: None,
      message_reference: Some(reference),
      sticker_ids: None,
      files: None,
    }
  }
}

impl Attachments for MessageResponse {
  fn take_attachments(&mut self) -> Vec<Attachment> {
    self.attachments.take().unwrap_or_default()
  }

  fn set_attachments(&mut self, attachments: Vec<Attachment>) -> &mut Self {
    self.attachments = Some(attachments);
    self
  }
}
