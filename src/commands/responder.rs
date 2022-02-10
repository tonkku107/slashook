// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::structs::{
  embeds::Embed,
  interactions::{InteractionCallbackData, ApplicationCommandOptionChoice, Attachments},
  channels::{Message, AllowedMentions, Attachment},
  components::{Component, Components},
  utils::File
};
use serde::Serialize;
use crate::tokio::sync::mpsc;
use crate::rest::{Rest, RestError};

type SimpleResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Message that can be sent as a response to a command or other interaction
///
/// This struct can be easily constructed from a `str`, `String`, [`Embed`](crate::structs::embeds::Embed) or [`Components`](crate::structs::components::Components)
/// with the `From` trait
#[derive(Serialize, Clone, Debug)]
pub struct MessageResponse {
  /// Should the response is TTS or not
  pub tts: Option<bool>,
  /// Content of the message
  pub content: Option<String>,
  /// Should only the user receiving the message be able to see it
  #[serde(skip_serializing)]
  pub ephemeral: bool,
  /// Up to 10 embeds to send with the response
  pub embeds: Option<Vec<Embed>>,
  /// Components to send with the response
  pub components: Option<Vec<Component>>,
  /// Partial attachment objects indicating which to keep when editing.
  pub attachments: Option<Vec<Attachment>>,
  /// Which mentions should be parsed
  pub allowed_mentions: Option<AllowedMentions>,
  /// Up to 10 files to send with the response
  ///
  /// Only available for follow-up responses
  #[serde(skip_serializing)]
  pub files: Option<Vec<File>>
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

  /// Set the ephemeralness of the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// let response = MessageResponse::from("This is for your eyes only!")
  ///   .set_ephemeral(true);
  /// assert_eq!(response.ephemeral, true);
  /// ```
  pub fn set_ephemeral(mut self, ephemeral: bool) -> Self {
    self.ephemeral = ephemeral;
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
    self.components = Some(components.components);
    self
  }

  /// Set the allowed mentions for the message
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::channels::{AllowedMentions, AllowedMentionType};
  /// let allowed_mentions = AllowedMentions::new().add_parse(AllowedMentionType::users);
  /// let response = MessageResponse::from("<@1234> Get pinged. Not @everyone or <@&1235> tho.")
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn set_allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
    self.allowed_mentions = Some(allowed_mentions);
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
  pub fn keep_attachment<T: ToString>(mut self, attachment_id: T) -> Self {
    let mut attachments = self.attachments.unwrap_or_default();
    attachments.push(Attachment::keep_with_id(attachment_id));
    self.attachments = Some(attachments);
    self
  }
}

#[derive(Debug)]
pub enum CommandResponse {
  DeferMessage(bool),
  SendMessage(MessageResponse),
  DeferUpdate,
  UpdateMessage(MessageResponse),
  AutocompleteResult(Vec<ApplicationCommandOptionChoice>)
}

/// Struct with methods for responding to interactions
#[derive(Debug)]
pub struct CommandResponder {
  pub(crate) tx: mpsc::UnboundedSender<CommandResponse>,
  pub(crate) id: String,
  pub(crate) token: String,
}

impl CommandResponder {
  /// Respond to an interaction with a message.\
  /// Only valid for the first response. If you've already responded or deferred once, use the follow-up methods
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// ##[command("example")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("Hello!")?;
  /// }
  /// ```
  pub fn send_message<T: Into<MessageResponse>>(&self, response: T) -> SimpleResult<()> {
    let response = response.into();
    self.tx.send(CommandResponse::SendMessage(response))?;
    // TODO: Figure out why the sender doesn't realize it is closed and forward further send_message calls to send_followup_message
    Ok(())
  }

  /// Respond to an interaction by editing the original message.\
  /// Only valid for the first response to a component interaction. If you've already responded or deferred once, use the follow-up methods
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// # use slashook::structs::components::{Components, Button};
  /// ##[command("example_button")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.update_message("Button was clicked!")?;
  /// }
  /// ```
  pub fn update_message<T: Into<MessageResponse>>(&self, response: T) -> SimpleResult<()> {
    let response = response.into();
    self.tx.send(CommandResponse::UpdateMessage(response))?;
    Ok(())
  }

  /// Give yourself more execution time.\
  /// If you don't respond within 3 seconds, Discord will disconnect and tell the user the interaction failed to run.
  /// By deferring, Discord will tell the user your bot is "thinking" and allow you to take your time. You can use the `send_followup_message` or `edit_original_message` methods to send the response.\
  /// The ephemeralness set here will be passed on to your first follow-up, no matter what ephemeralness you set there.
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command("example")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.defer(false)?;
  ///   // Do something that takes longer than 3s
  ///   res.send_followup_message("Thank you for your patience!").await?;
  /// }
  /// ```
  pub fn defer(&self, ephemeral: bool) -> SimpleResult<()> {
    self.tx.send(CommandResponse::DeferMessage(ephemeral))?;
    Ok(())
  }

  /// Much like `defer` but for component interactions and it shows nothing visibly to the user.
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command("example_button")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.defer_update()?;
  ///   // Do something that takes longer than 3s
  ///   res.edit_original_message("Finally it changed!").await?;
  /// }
  /// ```
  pub fn defer_update(&self) -> SimpleResult<()> {
    self.tx.send(CommandResponse::DeferUpdate)?;
    Ok(())
  }

  /// Respond to an autocomplete interaction with autocomplete choices
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// # use slashook::structs::interactions::ApplicationCommandOptionChoice;
  /// ##[command("example")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   if input.is_autocomplete() {
  ///     let search = input.args.get(&input.focused.unwrap()).unwrap().as_string().unwrap();
  ///     // Use the current input to fetch or filter choices
  ///     let choices = vec![
  ///       ApplicationCommandOptionChoice::new("An autocompleted choice", "autocomplete1"),
  ///       ApplicationCommandOptionChoice::new("Another autocompleted choice", "autocomplete2")
  ///     ];
  ///     return res.autocomplete(choices)?;
  ///   }
  /// }
  /// ```
  pub fn autocomplete(&self, results: Vec<ApplicationCommandOptionChoice>) -> SimpleResult<()> {
    self.tx.send(CommandResponse::AutocompleteResult(results))?;
    Ok(())
  }

  /// Send more messages after the initial response
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command("example")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("First message!")?;
  ///   res.send_followup_message("Second message!").await?;
  /// }
  /// ```
  pub async fn send_followup_message<T: Into<MessageResponse>>(&self, response: T) -> Result<Message, RestError> {
    let mut response = response.into();
    let files = response.files;
    response.files = None;
    let msg: InteractionCallbackData = response.into();
    let path = format!("webhooks/{}/{}", self.id, self.token);
    if let Some(files) = files {
      Rest::new().post_files(path, msg, files).await
    } else {
      Rest::new().post(path, msg).await
    }
  }

  /// Edits a follow-up message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command("example")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("First message!")?;
  ///   let msg = res.send_followup_message("Second message!").await?;
  ///   res.edit_followup_message(msg.id, "Second message but edited!").await?;
  /// }
  /// ```
  pub async fn edit_followup_message<T: Into<MessageResponse>>(&self, id: String, response: T) -> Result<Message, RestError> {
    let mut response = response.into();
    let files = response.files;
    response.files = None;
    let msg: InteractionCallbackData = response.into();
    let path = format!("webhooks/{}/{}/messages/{}", self.id, self.token, id);
    if let Some(files) = files {
      Rest::new().patch_files(path, msg, files).await
    } else {
      Rest::new().patch(path, msg).await
    }
  }

  /// Edits the original message\
  /// Same as running `edit_followup_message` with id of `@original`
  pub async fn edit_original_message<T: Into<MessageResponse>>(&self, response: T) -> Result<Message, RestError> {
    self.edit_followup_message(String::from("@original"), response).await
  }

  /// Gets a follow-up message
  pub async fn get_followup_message(&self, id: String) -> Result<Message, RestError> {
    Rest::new().get(format!("webhooks/{}/{}/messages/{}", self.id, self.token, id)).await
  }

  /// Gets the original message\
  /// Same as running `get_followup_message` with id of `@original`
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command("example")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("First message!")?;
  ///   let msg = res.get_original_message().await?;
  ///   println!("I responded with {}", msg.content);
  /// }
  /// ```
  pub async fn get_original_message(&self) -> Result<Message, RestError> {
    self.get_followup_message(String::from("@original")).await
  }

  /// Deletes a follow-up message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
  /// ##[command("example")]
  /// fn example(input: CommandInput, res: CommandResponder) {
  ///   res.send_message("First message!")?;
  ///   let msg = res.send_followup_message("If you see me say hi").await?;
  ///   res.delete_followup_message(msg.id).await?;
  /// }
  /// ```
  pub async fn delete_followup_message(&self, id: String) -> Result<(), RestError> {
    Rest::new().delete(format!("webhooks/{}/{}/messages/{}", self.id, self.token, id)).await
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
      ephemeral: false,
      embeds: None,
      components: None,
      attachments: None,
      allowed_mentions: None,
      files: None
    }
  }
}

impl From<String> for MessageResponse {
  fn from(s: String) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: Some(s),
      ephemeral: false,
      embeds: None,
      components: None,
      attachments: None,
      allowed_mentions: None,
      files: None
    }
  }
}

impl From<Embed> for MessageResponse {
  fn from(e: Embed) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      ephemeral: false,
      embeds: Some(vec![e]),
      components: None,
      attachments: None,
      allowed_mentions: None,
      files: None
    }
  }
}

impl From<Components> for MessageResponse {
  fn from(c: Components) -> MessageResponse {
    MessageResponse {
      tts: Some(false),
      content: None,
      ephemeral: false,
      embeds: None,
      components: Some(c.components),
      attachments: None,
      allowed_mentions: None,
      files: None
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
