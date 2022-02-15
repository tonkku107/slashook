// Copyright 2021 slashook Developers
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
  channels::{Channel, Message, MessageFlags, AllowedMentions, Attachment},
  components::{Component, ComponentType}
};
use crate::commands::{MessageResponse, Modal};

#[doc(hidden)]
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ApplicationCommandType {
  CHAT_INPUT = 1,
  USER = 2,
  MESSAGE = 3,
  UNKNOWN
}

/// Discord Application Command Option Choice Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationCommandOptionChoice {
  /// 1-100 character choice name
  pub name: String,
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
  pub channel_id: Option<Snowflake>,
  pub member: Option<GuildMember>,
  pub user: Option<User>,
  pub token: String,
  pub version: u8,
  pub message: Option<Message>,
  pub locale: Option<String>,
  pub guild_locale: Option<String>
}

/// Discord Interaction Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionType {
  PING = 1,
  APPLICATION_COMMAND = 2,
  MESSAGE_COMPONENT = 3,
  APPLICATION_COMMAND_AUTOCOMPLETE = 4,
  MODAL_SUBMIT = 5,
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
  pub custom_id: Option<String>,
  pub component_type: Option<ComponentType>,
  pub values: Option<Vec<String>>,
  pub target_id: Option<Snowflake>,
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

#[doc(hidden)]
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionOptionType {
  SUB_COMMAND = 1,
  SUB_COMMAND_GROUP = 2,
  STRING = 3,
  INTEGER = 4,
  BOOLEAN = 5,
  USER = 6,
  CHANNEL = 7,
  ROLE = 8,
  MENTIONABLE = 9,
  NUMBER = 10,
  ATTACHMENT = 11,
  UNKNOWN
}

/// Represents the possible values from command arguments
#[derive(Clone, Debug)]
pub enum OptionValue {
  String(String),
  Integer(i64),
  Boolean(bool),
  User(User),
  Channel(Box<Channel>),
  Role(Role),
  Number(f64),
  Attachment(Attachment),
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
  MODAL = 9
}

#[doc(hidden)]
#[derive(Serialize, Clone, Debug)]
pub struct InteractionCallbackData {
  pub tts: Option<bool>,
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
      title: None
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
      title: None
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
      title: None
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
      title: Some(modal.title)
    }
  }
}

/// Trait for structs that have an [Attachment](crate::structs::channels::Attachment) Vec.
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
    Self { name: name.to_string(), value: value.into() }
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
