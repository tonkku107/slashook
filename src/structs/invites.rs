// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord invites

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use chrono::{DateTime, Utc};
use super::{
  Snowflake,
  applications::Application,
  channels::Channel,
  guilds::{Guild, GuildScheduledEvent},
  users::User,
};

/// Discord Invite Object
#[derive(Deserialize, Clone, Debug)]
pub struct Invite {
  /// The invite code (unique ID)
  pub code: String,
  /// The guild this invite is for
  pub guild: Option<Guild>,
  /// The channel this invite is for
  pub channel: Option<Channel>,
  /// The user who created the invite
  pub inviter: Option<User>,
  /// The [type of target](TargetType) for this voice channel invite
  pub target_type: Option<TargetType>,
  /// The user whose stream to display for this voice channel stream invite
  pub target_user: Option<User>,
  /// The embedded application to open for this voice channel embedded application invite
  pub target_application: Option<Application>,
  /// Approximate count of online members, returned from the `GET /invites/<code>` endpoint when `with_counts` is `true`
  pub approximate_presence_count: Option<i64>,
  /// Approximate count of total members, returned from the `GET /invites/<code>` endpoint when `with_counts` is `true`
  pub approximate_member_count: Option<i64>,
  /// The expiration date of this invite, returned from the `GET /invites/<code>` endpoint when `with_expiration` is `true`
  pub expires_at: Option<DateTime<Utc>>,
  /// Guild scheduled event data, only included if `guild_scheduled_event_id` contains a valid guild scheduled event id
  pub guild_scheduled_event: Option<GuildScheduledEvent>,

  /// Number of times this invite has been used
  pub uses: Option<i64>,
  /// Max number of times this invite can be used
  pub max_uses: Option<i64>,
  /// Duration (in seconds) after which the invite expires
  pub max_age: Option<i64>,
  /// Whether this invite only grants temporary membership
  pub temporary: Option<bool>,
  /// When this invite was created
  pub created_at: Option<DateTime<Utc>>,
}

/// Discord Invite Target Types
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum TargetType {
  /// Stream target
  STREAM = 1,
  /// Embedded application target
  EMBEDDED_APPLICATION = 2,
  /// Target type that hasn't been implemented yet
  UNKNOWN
}

/// Parameters for creating an invite with [create_invite](super::channels::Channel::create_invite)
#[derive(Serialize, Default, Clone, Debug)]
pub struct CreateInviteOptions {
  /// Duration of invite in seconds before expiry, or 0 for never. between 0 and 604800 (7 days). Defaults to 86400 (24 hours)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_age: Option<i64>,
  /// Max number of uses or 0 for unlimited. between 0 and 100. Default to 0
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_uses: Option<i64>,
  /// Whether this invite only grants temporary membership. Defaults to false
  #[serde(skip_serializing_if = "Option::is_none")]
  pub temporary: Option<bool>,
  /// If true, don't try to reuse a similar invite (useful for creating many unique one time use invites). Defaults to false
  #[serde(skip_serializing_if = "Option::is_none")]
  pub unique: Option<bool>,
  /// The [type of target](TargetType) for this voice channel invite.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub target_type: Option<TargetType>,
  /// The id of the user whose stream to display for this invite, required if `target_type` is `STREAM`, the user must be streaming in the channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub target_user_id: Option<Snowflake>,
  /// The id of the embedded application to open for this invite, required if `target_type` is `EMBEDDED_APPLICATION`, the application must have the `EMBEDDED` flag
  #[serde(skip_serializing_if = "Option::is_none")]
  pub target_application_id: Option<Snowflake>,
}

impl CreateInviteOptions {
  /// Creates a new empty CreateInviteOptions
  pub fn new() -> Self {
    Self {
      max_age: None,
      max_uses: None,
      temporary: None,
      unique: None,
      target_type: None,
      target_user_id: None,
      target_application_id: None,
    }
  }

  /// Sets the max age
  pub fn set_max_age(mut self, max_age: i64) -> Self {
    self.max_age = Some(max_age);
    self
  }

  /// Sets the max uses
  pub fn set_max_uses(mut self, max_uses: i64) -> Self {
    self.max_uses = Some(max_uses);
    self
  }

  /// Sets temporary
  pub fn set_temporary(mut self, temporary: bool) -> Self {
    self.temporary = Some(temporary);
    self
  }

  /// Sets unique
  pub fn set_unique(mut self, unique: bool) -> Self {
    self.unique = Some(unique);
    self
  }

  /// Sets the target type
  pub fn set_target_type(mut self, target_type: TargetType) -> Self {
    self.target_type = Some(target_type);
    self
  }

  /// Sets the target user id
  pub fn set_target_user_id<T: ToString>(mut self, target: T) -> Self {
    self.target_user_id = Some(target.to_string());
    self
  }

  /// Sets the target application id
  pub fn set_target_application_id<T: ToString>(mut self, target: T) -> Self {
    self.target_application_id = Some(target.to_string());
    self
  }
}
