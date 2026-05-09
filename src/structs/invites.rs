// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord invites

use serde::{Deserialize, Serialize, ser::Serializer, de::Deserializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use chrono::{DateTime, Utc};
use bitflags::bitflags;
use tokio_util::bytes::Bytes;

use super::{
  Snowflake,
  applications::Application,
  channels::Channel,
  guilds::{Guild, GuildScheduledEvent},
  interactions::Attachments,
  messages::Attachment,
  roles::Role,
  users::User,
  utils::File,
};
use crate::rest::{Rest, RestError};

/// Discord Invite Object
#[derive(Deserialize, Clone, Debug)]
pub struct Invite {
  /// The [type of invite](InviteType)
  #[serde(rename = "type")]
  pub invite_type: InviteType,
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
  /// [guild invite flags](InviteFlags) for guild invites
  pub flags: Option<InviteFlags>,
  /// The roles assigned to the user upon accepting the invite.
  pub roles: Option<Vec<Role>>,

  // Invite Metadata Extension:

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

/// Special partial invite object from [`get_vanity_url`](Guild::get_vanity_url)
#[derive(Deserialize, Clone, Debug)]
pub struct VanityUrlInvite {
  /// The invite code (`None` if vanity url is not set)
  pub code: Option<String>,
  /// Number of times this invite has been used
  pub uses: Option<i64>,
}

bitflags! {
  /// Bitflags for Discord Invite Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct InviteFlags: u32 {
    /// This invite is a guest invite for a voice channel
    const IS_GUEST_INVITE = 1 << 0;
  }
}

/// Discord Invite Types
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InviteType {
  /// Guild
  GUILD = 0,
  /// Group DM
  GROUP_DM = 1,
  /// Friend
  FRIEND = 2,
  /// Invite type that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
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
  #[serde(other)]
  UNKNOWN,
}

/// Parameters for creating an invite with [`create_invite`](Channel::create_invite)
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
  /// A csv file with a single column of user IDs for all the users able to accept this invite
  #[serde(skip_serializing)]
  pub target_users_file: Option<File>,
  /// The role ID(s) for roles in the guild given to the users that accept this invite
  #[serde(skip_serializing_if = "Option::is_none")]
  pub role_ids: Option<Vec<Snowflake>>,
}

/// Response from [`get_target_users_job_status`](Invite::get_target_users_job_status)
#[derive(Deserialize, Clone, Debug)]
pub struct InviteTargetUsersJobStatus {
  /// Status of the job
  pub status: InviteTargetUsersJobStatusCode,
  /// Total users specified
  pub total_users: i64,
  /// Amount of users that have been processed
  pub processed_users: i64,
  /// Date the job was created
  pub created_at: DateTime<Utc>,
  /// Date the job was completed
  pub completed_at: Option<DateTime<Utc>>,
  /// Error message from the job
  pub error_message: Option<String>,
}

/// Discord Invite Target Users Job Status Codes
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum InviteTargetUsersJobStatusCode {
  /// The default value.
  UNSPECIFIED = 0,
  /// The job is still being processed.
  PROCESSING = 1,
  /// The job has been completed successfully.
  COMPLETED = 2,
  /// The job has failed, see `error_message` field for more details.
  FAILED = 3,
  /// Status code that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

impl Invite {
  /// Fetch an invite with the invite code
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::invites::Invite;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let invite = Invite::fetch(&input.rest, "discord-developers").await?;
  /// # }
  /// ```
  pub async fn fetch<T: ToString>(rest: &Rest, code: T) -> Result<Invite, RestError> {
    rest.get(format!("invites/{}", code.to_string())).await
  }

  /// Delete an invite
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # use slashook::structs::invites::CreateInviteOptions;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let channel = input.channel.unwrap();
  /// # let options = CreateInviteOptions::new().set_max_uses(1);
  /// # let invite = channel.create_invite(&input.rest, options).await?;
  /// invite.delete(&input.rest).await?;
  /// # }
  /// ```
  pub async fn delete(&self, rest: &Rest) -> Result<Invite, RestError> {
    rest.delete(format!("invites/{}", self.code)).await
  }

  /// Get the target users of an invite
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # use slashook::structs::invites::CreateInviteOptions;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let channel = input.channel.unwrap();
  /// # let options = CreateInviteOptions::new().set_max_uses(1);
  /// # let invite = channel.create_invite(&input.rest, options).await?;
  /// let bytes = invite.get_target_users(&input.rest).await?;
  /// let target_users = str::from_utf8(&bytes)?.split_terminator("\r\n").skip(1);
  /// # }
  /// ```
  pub async fn get_target_users(&self, rest: &Rest) -> Result<Bytes, RestError> {
    rest.get_raw(format!("invites/{}/target-users", self.code)).await
  }

  /// Update the target users of an invite
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # use slashook::structs::invites::CreateInviteOptions;
  /// # use slashook::structs::utils::File;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let channel = input.channel.unwrap();
  /// # let options = CreateInviteOptions::new().set_max_uses(1);
  /// # let invite = channel.create_invite(&input.rest, options).await?;
  /// let file = File::new("users.csv", "933795693162799156\n545364944258990091\n520953716610957312");
  /// invite.update_target_users(&input.rest, file).await?;
  /// # }
  /// ```
  pub async fn update_target_users(&self, rest: &Rest, mut file: File) -> Result<(), RestError> {
    file._part_name = Some("target_users_file".to_string());
    rest.put_files(format!("invites/{}/target-users", self.code), CreateInviteOptions::new(), vec![file]).await
  }

  /// Get the target users job status
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # use slashook::structs::invites::CreateInviteOptions;
  /// # use slashook::structs::utils::File;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let channel = input.channel.unwrap();
  /// let file = File::new("users.csv", "933795693162799156\n545364944258990091\n520953716610957312");
  /// let options = CreateInviteOptions::new().set_target_users_file(file);
  /// let invite = channel.create_invite(&input.rest, options).await?;
  /// let job_status = invite.get_target_users_job_status(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_target_users_job_status(&self, rest: &Rest) -> Result<InviteTargetUsersJobStatus, RestError> {
    rest.get(format!("invites/{}/target-users/job-status", self.code)).await
  }
}

impl CreateInviteOptions {
  /// Creates a new empty `CreateInviteOptions`
  pub fn new() -> Self {
    Self {
      max_age: None,
      max_uses: None,
      temporary: None,
      unique: None,
      target_type: None,
      target_user_id: None,
      target_application_id: None,
      target_users_file: None,
      role_ids: None,
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

  /// Sets the target users file
  pub fn set_target_users_file(mut self, file: File) -> Self {
    self.target_users_file = Some(file);
    self
  }

  /// Adds a role id
  pub fn add_role_id<T: ToString>(mut self, role_id: T) -> Self {
    let mut role_ids = self.role_ids.unwrap_or_default();
    role_ids.push(role_id.to_string());
    self.role_ids = Some(role_ids);
    self
  }

  /// Sets the role ids
  pub fn set_role_ids<T: ToString>(mut self, role_ids: Vec<T>) -> Self {
    self.role_ids = Some(role_ids.into_iter().map(|t| t.to_string()).collect());
    self
  }
}

impl Attachments for CreateInviteOptions {
  fn take_attachments(&mut self) -> Vec<Attachment> {
    Vec::new()
  }

  fn set_attachments(&mut self, _attachments: Vec<Attachment>) -> &mut Self {
    self
  }
}

impl<'de> Deserialize<'de> for InviteFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}

impl Serialize for InviteFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}
