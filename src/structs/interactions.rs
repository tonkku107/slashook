// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord interactions

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_json::Value;
use std::collections::HashMap;
use super::{
  Snowflake,
  embeds::Embed,
  users::User,
  guilds::{GuildMember, Role},
  channels::{Channel, Message, MessageFlags, AllowedMentions, Attachment, ChannelType},
  components::{Component, ComponentType},
  monetization::Entitlement,
  utils::File,
  Permissions
};
use crate::{
  rest::{Rest, RestError},
  commands::{MessageResponse, Modal, responder::CommandResponse},
};

/// Discord ApplicationCommand Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationCommand {
  /// Unique ID of command
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Snowflake>,
  /// [Type of command](ApplicationCommandType), defaults to `CHAT_INPUT`
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub command_type: Option<ApplicationCommandType>,
  /// ID of the parent application
  #[serde(skip_serializing_if = "Option::is_none")]
  pub application_id: Option<Snowflake>,
  /// Guild ID of the command, if not global
  #[serde(skip_serializing_if = "Option::is_none")]
  pub guild_id: Option<Snowflake>,
  /// [Name of command](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-naming), 1-32 characters
  pub name: String,
  /// Localization dictionary for `name` field. Values follow the same restrictions as `name`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name_localizations: Option<HashMap<String, String>>,
  /// Description for `CHAT_INPUT` commands, 1-100 characters. Empty string for `USER` and `MESSAGE` commands
  pub description: String,
  /// Localization dictionary for `description` field. Values follow the same restrictions as `description`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description_localizations: Option<HashMap<String, String>>,
  /// Parameters for the command, max of 25
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<Vec<ApplicationCommandOption>>,
  /// Set of [permissions](Permissions) represented as a bit set
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_member_permissions: Option<Permissions>,
  /// Indicates whether the command is available in DMs with the app, only for globally-scoped commands. By default, commands are visible.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dm_permission: Option<bool>,
  /// Indicates whether the command is age-restricted, defaults to `false`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nsfw: Option<bool>,
  /// Autoincrementing version identifier updated during substantial record changes
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<Snowflake>,
}

/// Discord Application Command Types
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ApplicationCommandType {
  /// Slash commands; a text-based command that shows up when a user types `/`
  CHAT_INPUT = 1,
  /// A UI-based command that shows up when you right click or tap on a user
  USER = 2,
  /// A UI-based command that shows up when you right click or tap on a message
  MESSAGE = 3,
  /// An application command type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Application Command Option Object
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ApplicationCommandOption {
  /// Type of option
  #[serde(rename = "type")]
  pub option_type: InteractionOptionType,
  /// 1-32 character name
  pub name: String,
  /// Localization dictionary for the `name` field. Values follow the same restrictions as `name`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name_localizations: Option<HashMap<String, String>>,
  /// 1-100 character description
  pub description: String,
  /// Localization dictionary for the `description` field. Values follow the same restrictions as `description`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description_localizations: Option<HashMap<String, String>>,
  /// If the parameter is required or optional--default `false`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub required: Option<bool>,
  /// Choices for `STRING`, `INTEGER`, and `NUMBER` types for the user to pick from, max 25
  #[serde(skip_serializing_if = "Option::is_none")]
  pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
  /// If the option is a subcommand or subcommand group type, these nested options will be the parameters
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<Vec<ApplicationCommandOption>>,
  /// If the option is a channel type, the channels shown will be restricted to these types
  #[serde(skip_serializing_if = "Option::is_none")]
  pub channel_types: Option<Vec<ChannelType>>,
  /// If the option is an `INTEGER` or `NUMBER` type, the minimum value permitted
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_value: Option<f64>,
  /// If the option is an `INTEGER` or `NUMBER` type, the maximum value permitted
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_value: Option<f64>,
  /// For option type `STRING`, the minimum allowed length (minimum of `0`, maximum of `6000`)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_length: Option<i64>,
  /// For option type `STRING`, the maximum allowed length (minimum of `1`, maximum of `6000`)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_length: Option<i64>,
  /// If autocomplete interactions are enabled for this `STRING`, `INTEGER`, or `NUMBER` type option
  #[serde(skip_serializing_if = "Option::is_none")]
  pub autocomplete: Option<bool>,
}

/// Discord Application Command Option Choice Object
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ApplicationCommandOptionChoice {
  /// 1-100 character choice name
  pub name: String,
  /// Localization dictionary for the name field. Values follow the same restrictions as name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name_localizations: Option<HashMap<String, String>>,
  /// Value of the choice, up to 100 characters if string
  pub value: Value,
}

#[doc(hidden)]
#[derive(Deserialize, Clone, Debug)]
pub struct Interaction {
  pub id: Snowflake,
  pub application_id: Snowflake,
  #[serde(rename = "type")]
  pub interaction_type: InteractionType,
  pub data: Option<InteractionData>,
  pub guild_id: Option<Snowflake>,
  pub channel: Option<Channel>,
  pub channel_id: Option<Snowflake>,
  pub member: Option<GuildMember>,
  pub user: Option<User>,
  pub token: String,
  pub version: u8,
  pub message: Option<Message>,
  pub app_permissions: Option<Permissions>,
  pub locale: Option<String>,
  pub guild_locale: Option<String>,
  pub entitlements: Vec<Entitlement>,
}

/// Discord Interaction Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionType {
  /// Ping interaction
  PING = 1,
  /// Application command interaction
  APPLICATION_COMMAND = 2,
  /// Message component interaction
  MESSAGE_COMPONENT = 3,
  /// Autocomplete interaction
  APPLICATION_COMMAND_AUTOCOMPLETE = 4,
  /// Modal submit interaction
  MODAL_SUBMIT = 5,
  /// Interaction type that hasn't been implemented yet
  UNKNOWN
}

#[doc(hidden)]
#[derive(Deserialize, Clone, Debug)]
pub struct InteractionData {
  pub id: Option<Snowflake>,
  pub name: Option<String>,
  #[serde(rename = "type")]
  pub command_type: Option<ApplicationCommandType>,
  pub resolved: Option<InteractionDataResolved>,
  pub options: Option<Vec<InteractionOption>>,
  pub guild_id: Option<Snowflake>,
  pub target_id: Option<Snowflake>,
  pub custom_id: Option<String>,
  pub component_type: Option<ComponentType>,
  pub values: Option<Vec<String>>,
  pub components: Option<Vec<Component>>
}

/// Discord Interaction Data Resolved Object
#[derive(Deserialize, Clone, Debug)]
pub struct InteractionDataResolved {
  /// The ids and User objects
  pub users: Option<HashMap<Snowflake, User>>,
  /// The ids and partial Member objects
  pub members: Option<HashMap<Snowflake, GuildMember>>,
  /// The ids and Role objects
  pub roles: Option<HashMap<Snowflake, Role>>,
  /// The ids and partial Channel objects
  pub channels: Option<HashMap<Snowflake, Channel>>,
  /// The ids and partial Message objects
  pub messages: Option<HashMap<Snowflake, Message>>,
  /// The ids and attachment objects
  pub attachments: Option<HashMap<Snowflake, Attachment>>
}

#[doc(hidden)]
#[derive(Deserialize, Clone, Debug)]
pub struct InteractionOption {
  pub name: String,
  #[serde(rename = "type")]
  pub option_type: InteractionOptionType,
  pub value: Option<Value>,
  pub options: Option<Vec<InteractionOption>>,
  pub focused: Option<bool>
}

/// Discord Application Command Option Type
#[derive(Serialize_repr, Deserialize_repr, Default, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionOptionType {
  /// A subcommand
  SUB_COMMAND = 1,
  /// A subcommand group
  SUB_COMMAND_GROUP = 2,
  /// A string
  STRING = 3,
  /// An integer, Any integer between -2^53 and 2^53
  INTEGER = 4,
  /// A boolean
  BOOLEAN = 5,
  /// A user
  USER = 6,
  /// A channel, Includes all channel types + categories
  CHANNEL = 7,
  /// A role
  ROLE = 8,
  /// A mentionable, Includes users and roles
  MENTIONABLE = 9,
  /// A number, Any double between -2^53 and 2^53
  NUMBER = 10,
  /// An attachment object
  ATTACHMENT = 11,
  #[default]
  /// An unknown option type that hasn't been implemented yet
  UNKNOWN
}

/// Represents the possible values from command arguments
#[derive(Clone, Debug)]
pub enum OptionValue {
  /// Represents a string value
  String(String),
  /// Represents an integer value
  Integer(i64),
  /// Represents a boolean value
  Boolean(bool),
  /// Represents a user value
  User(User),
  /// Represents a channel value
  Channel(Box<Channel>),
  /// Represents a role channe
  Role(Role),
  /// Represents a number value
  Number(f64),
  /// Represents an attachment value
  Attachment(Attachment),
  /// Represents any unknown value
  Other(Value)
}

#[doc(hidden)]
#[derive(Serialize, Clone, Debug)]
pub struct InteractionCallback {
  #[serde(rename = "type")]
  pub response_type: InteractionCallbackType,
  pub data: Option<InteractionCallbackData>
}

#[doc(hidden)]
#[derive(Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionCallbackType {
  PONG = 1,
  CHANNEL_MESSAGE_WITH_SOURCE = 4,
  DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE = 5,
  DEFERRED_UPDATE_MESSAGE = 6,
  UPDATE_MESSAGE = 7,
  APPLICATION_COMMAND_AUTOCOMPLETE_RESULT = 8,
  MODAL = 9,
  PREMIUM_REQUIRED = 10,
}

#[doc(hidden)]
#[derive(Serialize, Clone, Debug)]
pub struct InteractionCallbackData {
  pub tts: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub embeds: Option<Vec<Embed>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub allowed_mentions: Option<AllowedMentions>,
  pub flags: Option<MessageFlags>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub components: Option<Vec<Component>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub attachments: Option<Vec<Attachment>>,
  pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
  pub custom_id: Option<String>,
  pub title: Option<String>,
  #[serde(skip_serializing)]
  pub files: Option<Vec<File>>
}

impl ApplicationCommand {
  /// Takes a list of application commands, overwriting the existing global command list for this application.
  pub async fn bulk_overwrite_global_commands<T: ToString>(rest: &Rest, application_id: T, commands: Vec<Self>) -> Result<Vec<Self>, RestError> {
    rest.put(format!("/applications/{}/commands", application_id.to_string()), commands).await
  }

  /// Takes a list of application commands, overwriting the existing command list for this application for the targeted guild.
  pub async fn bulk_overwrite_guild_commands<T: ToString, U: ToString>(rest: &Rest, application_id: T, guild_id: U, commands: Vec<Self>) -> Result<Vec<Self>, RestError> {
    rest.put(format!("/applications/{}/guilds/{}/commands", application_id.to_string(), guild_id.to_string()), commands).await
  }
}

impl TryFrom<u8> for ApplicationCommandType {
  type Error = serde_json::Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    serde_json::from_value(value.into())
  }
}

impl TryFrom<u8> for InteractionOptionType {
  type Error = serde_json::Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    serde_json::from_value(value.into())
  }
}

#[doc(hidden)]
impl From<CommandResponse> for InteractionCallback {
  fn from(response: CommandResponse) -> InteractionCallback {
    match response {
      CommandResponse::DeferMessage(flags) => {
        InteractionCallback {
          response_type: InteractionCallbackType::DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE,
          data: Some(flags.into())
        }
      },

      CommandResponse::DeferUpdate => {
        InteractionCallback {
          response_type: InteractionCallbackType::DEFERRED_UPDATE_MESSAGE,
          data: None
        }
      }

      CommandResponse::SendMessage(msg) => {
        InteractionCallback {
          response_type: InteractionCallbackType::CHANNEL_MESSAGE_WITH_SOURCE,
          data: Some(msg.into())
        }
      },

      CommandResponse::UpdateMessage(msg) => {
        InteractionCallback {
          response_type: InteractionCallbackType::UPDATE_MESSAGE,
          data: Some(msg.into())
        }
      },

      CommandResponse::AutocompleteResult(results) => {
        InteractionCallback {
          response_type: InteractionCallbackType::APPLICATION_COMMAND_AUTOCOMPLETE_RESULT,
          data: Some(results.into())
        }
      },

      CommandResponse::Modal(modal) => {
        InteractionCallback {
          response_type: InteractionCallbackType::MODAL,
          data: Some(modal.into())
        }
      },

      CommandResponse::PremiumRequired => {
        InteractionCallback {
          response_type: InteractionCallbackType::PREMIUM_REQUIRED,
          data: None
        }
      },

    }
  }
}

#[doc(hidden)]
impl From<MessageResponse> for InteractionCallbackData {
  fn from(msg: MessageResponse) -> InteractionCallbackData {
    InteractionCallbackData {
      tts: msg.tts,
      content: msg.content,
      flags: msg.flags,
      embeds: msg.embeds,
      components: msg.components,
      attachments: msg.attachments,
      allowed_mentions: msg.allowed_mentions,
      choices: None,
      custom_id: None,
      title: None,
      files: msg.files
    }
  }
}

#[doc(hidden)]
impl From<MessageFlags> for InteractionCallbackData {
  fn from(flags: MessageFlags) -> InteractionCallbackData {
    InteractionCallbackData {
      tts: None,
      content: None,
      flags: Some(flags),
      embeds: None,
      components: None,
      attachments: None,
      allowed_mentions: None,
      choices: None,
      custom_id: None,
      title: None,
      files: None
    }
  }
}

#[doc(hidden)]
impl From<Vec<ApplicationCommandOptionChoice>> for InteractionCallbackData {
  fn from(results: Vec<ApplicationCommandOptionChoice>) -> InteractionCallbackData {
    InteractionCallbackData {
      tts: None,
      content: None,
      flags: None,
      embeds: None,
      components: None,
      attachments: None,
      allowed_mentions: None,
      choices: Some(results),
      custom_id: None,
      title: None,
      files: None
    }
  }
}

#[doc(hidden)]
impl From<Modal> for InteractionCallbackData {
  fn from(modal: Modal) -> InteractionCallbackData {
    InteractionCallbackData {
      tts: None,
      content: None,
      flags: None,
      embeds: None,
      components: Some(modal.components),
      attachments: None,
      allowed_mentions: None,
      choices: None,
      custom_id: Some(modal.custom_id),
      title: Some(modal.title),
      files: None
    }
  }
}

/// Trait for structs that have an [Attachment] Vec.
/// Functions for use with [post_files](crate::rest::Rest::post_files) and [patch_files](crate::rest::Rest::patch_files)
pub trait Attachments {
  /// Returns the attachments that have been set and possibly removes the originals.
  fn take_attachments(&mut self) -> Vec<Attachment>;

  /// Sets updated attachments.
  fn set_attachments(&mut self, attachments: Vec<Attachment>) -> &mut Self;
}

impl Attachments for InteractionCallbackData {
  fn take_attachments(&mut self) -> Vec<Attachment> {
    self.attachments.take().unwrap_or_default()
  }

  fn set_attachments(&mut self, attachments: Vec<Attachment>) -> &mut Self {
    self.attachments = Some(attachments);
    self
  }
}

impl OptionValue {
  /// Returns true if the value is a string. Returns false otherwise.
  pub fn is_string(&self) -> bool {
    matches!(self, Self::String(_))
  }

  /// If the value is a string, returns the String. Returns None otherwise.
  pub fn as_string(&self) -> Option<String> {
    match self {
      Self::String(s) => Some(s.to_string()),
      _ => None
    }
  }

  /// Returns true if the value is an integer. Returns false otherwise.
  pub fn is_i64(&self) -> bool {
    matches!(self, Self::Integer(_))
  }

  /// If the value is an integer, returns the i64. Returns None otherwise.
  pub fn as_i64(&self) -> Option<i64> {
    match self {
      Self::Integer(i) => Some(*i),
      _ => None
    }
  }

  /// Returns true if the value is a number. Returns false otherwise.
  pub fn is_f64(&self) -> bool {
    matches!(self, Self::Number(_))
  }

  /// If the value is a number, returns the f64. Returns None otherwise.
  pub fn as_f64(&self) -> Option<f64> {
    match self {
      Self::Number(n) => Some(*n),
      _ => None
    }
  }

  /// Returns true if the value is a boolean. Returns false otherwise.
  pub fn is_bool(&self) -> bool {
    matches!(self, Self::Boolean(_))
  }

  /// If the value is a boolean, returns the bool. Returns None otherwise.
  pub fn as_bool(&self) -> Option<bool> {
    match self {
      Self::Boolean(b) => Some(*b),
      _ => None
    }
  }

  /// Returns true if the value is a user. Returns false otherwise.
  pub fn is_user(&self) -> bool {
    matches!(self, Self::Boolean(_))
  }

  /// If the value is a user, returns the User. Returns None otherwise.
  pub fn as_user(&self) -> Option<&User> {
    match self {
      Self::User(u) => Some(u),
      _ => None
    }
  }

  /// Returns true if the value is a channel. Returns false otherwise.
  pub fn is_channel(&self) -> bool {
    matches!(self, Self::Channel(_))
  }

  /// If the value is a channel, returns the Channel. Returns None otherwise.
  pub fn as_channel(&self) -> Option<&Channel> {
    match self {
      Self::Channel(c) => Some(c),
      _ => None
    }
  }

  /// Returns true if the value is a role. Returns false otherwise.
  pub fn is_role(&self) -> bool {
    matches!(self, Self::Role(_))
  }

  /// If the value is a role, returns the Role. Returns None otherwise.
  pub fn as_role(&self) -> Option<&Role> {
    match self {
      Self::Role(r) => Some(r),
      _ => None
    }
  }

  /// Returns true if the value is an attachment. Returns false otherwise.
  pub fn is_attachment(&self) -> bool {
    matches!(self, Self::Attachment(_))
  }

  /// If the value is an attachment, returns the Attachment. Returns None otherwise.
  pub fn as_attachment(&self) -> Option<&Attachment> {
    match self {
      Self::Attachment(a) => Some(a),
      _ => None
    }
  }
}

impl ApplicationCommandOptionChoice {
  /// Creates a new choice with a name and value
  pub fn new<T: ToString, U: Into<Value>>(name: T, value: U) -> Self {
    Self {
      name: name.to_string(),
      name_localizations: None,
      value: value.into()
    }
  }
}

impl std::fmt::Display for OptionValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::String(s) => write!(f, "\"{}\"", s),
      Self::Integer(i) => write!(f, "{}", i),
      Self::Boolean(b) => write!(f, "{}", b),
      Self::User(u) => write!(f, "\"{}\"", u.id),
      Self::Channel(c) => write!(f, "\"{}\"", c.id),
      Self::Role(r) => write!(f, "\"{}\"", r.id),
      Self::Number(n) => write!(f, "{}", n),
      Self::Attachment(a) => write!(f, "{}", a.url),
      Self::Other(o) => write!(f, "{}", o)
    }
  }
}
