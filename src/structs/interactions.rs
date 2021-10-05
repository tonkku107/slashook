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
  channels::{Channel, Message, MessageFlags, AllowedMentions},
  components::{Component, ComponentType}
};
use crate::commands::MessageResponse;

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
  pub message: Option<Message>
}

#[doc(hidden)]
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InteractionType {
  PING = 1,
  APPLICATION_COMMAND = 2,
  MESSAGE_COMPONENT = 3,
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
  pub target_id: Option<Snowflake>
}

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

#[doc(hidden)]
#[derive(Deserialize, Clone, Debug)]
pub struct InteractionDataResolved {
  pub users: Option<HashMap<Snowflake, User>>,
  pub members: Option<HashMap<Snowflake, GuildMember>>,
  pub roles: Option<HashMap<Snowflake, Role>>,
  pub channels: Option<HashMap<Snowflake, Channel>>,
  pub messages: Option<HashMap<Snowflake, Message>>
}

#[doc(hidden)]
#[derive(Deserialize, Clone, Debug)]
pub struct InteractionOption {
  pub name: String,
  #[serde(rename = "type")]
  pub option_type: InteractionOptionType,
  pub value: Option<Value>,
  pub options: Option<Vec<InteractionOption>>
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
  UPDATE_MESSAGE = 7
}

#[doc(hidden)]
#[derive(Serialize, Clone, Debug)]
pub struct InteractionCallbackData {
  pub tts: Option<bool>,
  pub content: Option<String>,
  pub embeds: Option<Vec<Embed>>,
  pub allowed_mentions: Option<AllowedMentions>,
  pub flags: Option<MessageFlags>,
  pub components: Option<Vec<Component>>
}

#[doc(hidden)]
impl From<MessageResponse> for InteractionCallbackData {
  fn from(msg: MessageResponse) -> InteractionCallbackData {
    let mut flags = MessageFlags::empty();
    if msg.ephemeral { flags.insert(MessageFlags::EPHEMERAL) }
    InteractionCallbackData {
      tts: msg.tts,
      content: msg.content,
      flags: Some(flags),
      embeds: msg.embeds,
      components: msg.components,
      allowed_mentions: msg.allowed_mentions
    }
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
      Self::Other(o) => write!(f, "{}", o)
    }
  }
}
