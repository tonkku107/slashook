// Copyright 2024 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord channels

use serde::{Deserialize, de::Deserializer};
use serde::{Serialize, ser::Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_json::{Value, json};
use super::{
  Snowflake,
  guilds::GuildMember,
  interactions::Attachments,
  invites::{Invite, CreateInviteOptions},
  messages::{Message, MessageFetchOptions, Attachment},
  permissions::Permissions,
  users::User,
};
use crate::{
  rest::{Rest, RestError},
  commands::MessageResponse
};
use chrono::{DateTime, Utc};
use bitflags::bitflags;

/// Discord Channel Object
#[derive(Deserialize, Clone, Debug)]
pub struct Channel {
  /// The id of this channel
  pub id: Snowflake,
  /// The [type of channel](ChannelType)
  #[serde(rename = "type")]
  pub channel_type: ChannelType,
  /// The id of the guild (may be missing for some channel objects received over gateway guild dispatches)
  pub guild_id: Option<Snowflake>,
  /// Sorting position of the channel
  pub position: Option<i64>,
  /// Explicit permission overwrites for members and roles
  pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
  /// The name of the channel (1-100 characters)
  pub name: Option<String>,
  /// The channel topic (0-4096 characters for `GUILD_FORUM` channels, 0-1024 characters for all others)
  pub topic: Option<String>,
  /// Whether the channel is nsfw
  pub nsfw: Option<bool>,
  /// The id of the last message sent in this channel (or thread for `GUILD_FORUM` channels) (may not point to an existing or valid message or thread)
  pub last_message_id: Option<Snowflake>,
  /// The bitrate (in bits) of the voice channel
  pub bitrate: Option<i64>,
  /// The user limit of the voice channel
  pub user_limit: Option<i64>,
  /// Amount of seconds a user has to wait before sending another message (0-21600); bots, as well as users with the permission `manage_messages` or `manage_channel`, are unaffected
  pub rate_limit_per_user: Option<i64>,
  /// The recipients of the DM
  pub recipients: Option<Vec<User>>,
  /// Icon hash of the group DM
  pub icon: Option<String>,
  /// Id of the creator of the group DM or thread
  pub owner_id: Option<Snowflake>,
  /// Application id of the group DM creator if it is bot-created
  pub application_id: Option<Snowflake>,
  /// For group DM channels: whether the channel is managed by an application via the `gdm.join` OAuth2 scope
  pub managed: Option<bool>,
  /// For guild channels: id of the parent category for a channel (each parent category can contain up to 50 channels), for threads: id of the text channel this thread was created
  pub parent_id: Option<Snowflake>,
  /// When the last pinned message was pinned. This may be `None` in events such as `GUILD_CREATE` when a message is not pinned.
  pub last_pin_timestamp: Option<DateTime<Utc>>,
  /// [Voice region](https://discord.com/developers/docs/resources/voice#voice-region-object) id for the voice channel, automatic when set to None
  pub rtc_region: Option<String>,
  /// The camera [video quality mode](VideoQualityMode) of the voice channel, `AUTO` when not present
  pub video_quality_mode: Option<VideoQualityMode>,
  /// Number of messages (not including the initial message or deleted messages) in a thread.
  pub message_count: Option<i64>,
  /// An approximate count of users in a thread, stops counting at 50
  pub member_count: Option<i64>,
  /// Thread-specific fields not needed by other channels
  pub thread_metadata: Option<ThreadMetadata>,
  /// Thread member object for the current user, if they have joined the thread, only included on certain API endpoints
  pub member: Option<ThreadMember>,
  /// Default duration, copied onto newly created threads, in minutes, threads will stop showing in the channel list after the specified period of inactivity, can be set to: 60, 1440, 4320, 10080
  pub default_auto_archive_duration: Option<i64>,
  /// Computed permissions for the invoking user in the channel, including overwrites, only included when part of the `resolved` data received on a slash command interaction
  pub permissions: Option<Permissions>,
  /// [Channel flags](ChannelFlags) combined as a [bitfield](https://en.wikipedia.org/wiki/Bit_field)
  pub flags: Option<ChannelFlags>,
  /// Number of messages ever sent in a thread, it's similar to `message_count` on message creation, but will not decrement the number when a message is deleted
  pub total_message_sent: Option<i64>,
  /// The set of tags that can be used in a `GUILD_FORUM` channel
  pub available_tags: Option<Vec<ForumTag>>,
  /// The IDs of the set of tags that have been applied to a thread in a `GUILD_FORUM` channel
  pub applied_tags: Option<Vec<Snowflake>>,
  /// The emoji to show in the add reaction button on a thread in a `GUILD_FORUM` channel
  pub default_reaction_emoji: Option<DefaultReaction>,
  /// The initial `rate_limit_per_user` to set on newly created threads in a channel. This field is copied to the thread at creation time and does not live update.
  pub default_thread_rate_limit_per_user: Option<i64>,
  /// The [default sort order type](SortOrderType) used to order posts in `GUILD_FORUM` channels. Defaults to `None`, which indicates a preferred sort order hasn't been set by a channel admin
  pub default_sort_order: Option<SortOrderType>,
  /// The [default forum layout view](ForumLayoutType) used to display posts in `GUILD_FORUM` channels. Defaults to `NOT_SET`, which indicates a layout view has not been set by a channel admin
  pub default_forum_layout: Option<ForumLayoutType>,
  /// A nested message object returned when creating forum or media posts
  pub message: Option<Box<Message>>,
}

/// Discord Channel Types
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ChannelType {
  /// A text channel within a server
  GUILD_TEXT = 0,
  /// A direct message between users
  DM = 1,
  /// A voice channel within a server
  GUILD_VOICE = 2,
  /// A direct message between multiple users
  GROUP_DM = 3,
  /// An [organizational category](https://support.discord.com/hc/en-us/articles/115001580171-Channel-Categories-101) that contains up to 50 channels
  GUILD_CATEGORY = 4,
  /// A channel that [users can follow and crosspost into their own server](https://support.discord.com/hc/en-us/articles/360032008192) (formerly news channels)
  GUILD_ANNOUNCEMENT = 5,
  /// A temporary sub-channel within a GUILD_ANNOUNCEMENT channel
  ANNOUNCEMENT_THREAD = 10,
  /// A temporary sub-channel within a GUILD_TEXT or GUILD_FORUM channel
  GUILD_PUBLIC_THREAD = 11,
  /// A temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
  GUILD_PRIVATE_THREAD = 12,
  /// A voice channel for [hosting events with an audience](https://support.discord.com/hc/en-us/articles/1500005513722)
  GUILD_STAGE_VOICE = 13,
  /// The channel in a [hub](https://support.discord.com/hc/en-us/articles/4406046651927-Discord-Student-Hubs-FAQ) containing the listed servers
  GUILD_DIRECTORY = 14,
  /// Channel that can only contain threads
  GUILD_FORUM = 15,
  /// Channel type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Permission Overwrite Object
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PermissionOverwrite {
  /// Role or user id
  pub id: Snowflake,
  /// Either ROLE or MEMBER
  #[serde(rename = "type")]
  pub overwrite_type: PermissionOverwriteType,
  /// Permission bit set
  pub allow: Permissions,
  /// Permission bit set
  pub deny: Permissions
}

/// Discord Permission Overwrite Types
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PermissionOverwriteType {
  /// Permission overwrite for a role
  ROLE = 0,
  /// Permission overwrite for a member
  MEMBER = 1,
  /// Permission overwrite type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Video Quality Modes
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum VideoQualityMode {
  /// Discord chooses the quality for optimal performance
  AUTO = 1,
  /// 720p
  FULL = 2,
  /// Video quality mode that hasn't been implemented yet
  UNKNOWN
}

/// Discord Thread Metadata Object
#[derive(Deserialize, Clone, Debug)]
pub struct ThreadMetadata {
  /// Whether the thread is archived
  pub archived: bool,
  /// The thread will stop showing in the channel list after `auto_archive_duration` minutes of inactivity, can be set to: 60, 1440, 4320, 10080
  pub auto_archive_duration: i64,
  /// Timestamp when the thread's archive status was last changed, used for calculating recent activity
  pub archive_timestamp: DateTime<Utc>,
  /// Whether the thread is locked; when a thread is locked, only users with MANAGE_THREADS can unarchive it
  pub locked: bool,
  /// Whether non-moderators can add other non-moderators to a thread; only available on private threads
  pub invitable: Option<bool>,
  /// Timestamp when the thread was created; only populated for threads created after 2022-01-09
  pub create_timestamp: Option<DateTime<Utc>>
}

/// Discord Thread Member Object
#[derive(Deserialize, Clone, Debug)]
pub struct ThreadMember {
  /// The id of the thread
  pub id: Option<Snowflake>,
  /// The id of the user
  pub user_id: Option<Snowflake>,
  /// Time the user last joined the thread
  pub join_timestamp: DateTime<Utc>,
  /// Any user-thread settings, currently only used for notifications
  pub flags: i64,
  /// Additional information about the user
  pub member: Option<GuildMember>,
}

bitflags! {
  /// Bitflags for Discord Channel Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct ChannelFlags: u32 {
    /// This thread is pinned to the top of its parent `GUILD_FORUM` channel
    const PINNED = 1 << 1;
    /// Whether a tag is required to be specified when creating a thread in a `GUILD_FORUM` channel. Tags are specified in the `applied_tags` field.
    const REQUIRE_TAG = 1 << 4;
  }
}

/// Discord Forum Tag Object
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ForumTag {
  /// The id of the tag
  pub id: Snowflake,
  /// The name of the tag (0-20 characters)
  pub name: String,
  /// Whether this tag can only be added to or removed from threads by a member with the `MANAGE_THREADS` permission
  pub moderated: bool,
  /// The id of a guild's custom emoji
  pub emoji_id: Option<Snowflake>,
  /// The unicode character of the emoji
  pub emoji_name: Option<String>,
}

/// Discord Default Reaction Object
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DefaultReaction {
  /// The id of a guild's custom emoji
  pub emoji_id: Option<Snowflake>,
  /// The unicode character of the emoji
  pub emoji_name: Option<String>,
}

/// Discord Sort Order Types
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum SortOrderType {
  /// Sort forum posts by activity
  LATEST_ACTIVITY = 0,
  /// Sort forum posts by creation time (from most recent to oldest)
  CREATION_DATE = 1,
  /// Sort order type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Sort Order Types
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ForumLayoutType {
  /// No default has been set for forum channel
  NOT_SET = 0,
  /// Display posts as a list
  LIST_VIEW = 1,
  /// Display posts as a collection of tiles
  GALLERY_VIEW = 2,
  /// Forum layout type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Followed Channel Object
#[derive(Deserialize, Clone, Debug)]
pub struct FollowedChannel {
  /// Source channel id
  pub channel_id: Snowflake,
  /// Created target webhook id
  pub webhook_id: Snowflake,
}

/// Parameters for modifying a channel with [modify](Channel::modify).
#[derive(Serialize, Default, Clone, Debug)]
pub struct ChannelModifyOptions {
  /// 1-100 character channel name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// Base64 encoded icon
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon: Option<String>,
  /// The [type of channel](ChannelType); only conversion between text and announcement is supported and only in guilds with the "NEWS" feature
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub channel_type: Option<ChannelType>,
  /// The position of the channel in the left-hand listing
  #[serde(skip_serializing_if = "Option::is_none")]
  pub position: Option<i64>,
  /// 0-1024 character channel topic (0-4096 characters for `GUILD_FORUM` channels)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub topic: Option<String>,
  /// Whether the channel is nsfw
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nsfw: Option<bool>,
  /// Amount of seconds a user has to wait before sending another message (0-21600); bots, as well as users with the permission `manage_messages` or `manage_channel`, are unaffected
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rate_limit_per_user: Option<i64>,
  /// The bitrate (in bits) of the voice or stage channel; min 8000
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bitrate: Option<i64>,
  /// The user limit of the voice channel; 0 refers to no limit, 1 to 99 refers to a user limit
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_limit: Option<i64>,
  /// Channel or category-specific permissions
  #[serde(skip_serializing_if = "Option::is_none")]
  pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
  /// Id of the new parent category for a channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<Snowflake>,
  /// Channel [voice region](https://discord.com/developers/docs/resources/voice#voice-region-object) id, automatic when set to None
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rtc_region: Option<Option<String>>,
  /// The camera [video quality mode](VideoQualityMode) of the voice channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub video_quality_mode: Option<VideoQualityMode>,
  /// The default duration that the clients use (not the API) for newly created threads in the channel, in minutes, to automatically archive the thread after recent activity
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_auto_archive_duration: Option<i64>,
  /// [Channel flags](ChannelFlags) combined as a [bitfield](https://en.wikipedia.org/wiki/Bit_field). Currently only `REQUIRE_TAG` is supported in forum channels and `PINNED` can only be set for threads in forum channels.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flags: Option<ChannelFlags>,
  /// The set of tags that can be used in a `GUILD_FORUM` channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub available_tags: Option<Vec<ForumTag>>,
  /// The emoji to show in the add reaction button on a thread in a `GUILD_FORUM` channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_reaction_emoji: Option<DefaultReaction>,
  /// The initial `rate_limit_per_user` to set on newly created threads in a channel. This field is copied to the thread at creation time and does not live update.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_thread_rate_limit_per_user: Option<i64>,
  /// The [default sort order type](SortOrderType) used to order posts in `GUILD_FORUM` channels
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_sort_order: Option<SortOrderType>,
  /// The [default forum layout type](ForumLayoutType) used to display posts in `GUILD_FORUM` channels
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_forum_layout: Option<ForumLayoutType>,
  /// Whether the thread is archived
  #[serde(skip_serializing_if = "Option::is_none")]
  pub archived: Option<bool>,
  /// The thread will stop showing in the channel list after `auto_archive_duration` minutes of inactivity, can be set to: 60, 1440, 4320, 10080
  #[serde(skip_serializing_if = "Option::is_none")]
  pub auto_archive_duration: Option<i64>,
  /// Whether the thread is locked; when a thread is locked, only users with `MANAGE_THREADS` can unarchive it
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locked: Option<bool>,
  /// Whether non-moderators can add other non-moderators to a thread; only available on private threads
  #[serde(skip_serializing_if = "Option::is_none")]
  pub invitable: Option<bool>,
  /// The IDs of the set of tags that have been applied to a thread in a `GUILD_FORUM` channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub applied_tags: Option<Vec<Snowflake>>,
}

/// Options for creating threads
#[derive(Serialize, Default, Clone, Debug)]
pub struct ThreadCreateOptions {
  /// 1-100 character channel name
  pub name: String,
  /// The thread will stop showing in the channel list after `auto_archive_duration` minutes of inactivity, can be set to: 60, 1440, 4320, 10080
  pub auto_archive_duration: Option<i64>,
  /// The [type of thread](ChannelType) to create.
  #[serde(rename = "type")]
  pub thread_type: Option<ChannelType>,
  /// Whether non-moderators can add other non-moderators to a thread; only available when creating a private thread
  pub invitable: Option<bool>,
  /// Amount of seconds a user has to wait before sending another message (0-21600)
  pub rate_limit_per_user: Option<i64>,
  /// Contents of the first message in the forum/media thread. Required to create a thread in a `GUILD_FORUM` or a `GUILD_MEDIA` channel
  pub message: Option<MessageResponse>,
  /// The IDs of the set of tags that have been applied to a thread in a `GUILD_FORUM` or a `GUILD_MEDIA` channel
  pub applied_tags: Option<Vec<Snowflake>>,
}

/// Options for fetching thread members
#[derive(Serialize, Default, Clone, Debug)]
pub struct ThreadMemberOptions {
  /// Whether to include a [guild member](GuildMember) object for each thread member
  pub with_member: Option<bool>,
  /// Get thread members after this user ID
  pub after: Option<Snowflake>,
  /// Max number of thread members to return (1-100). Defaults to 100.
  pub limit: Option<i64>,
}

/// Options for fetching a list of threads
#[derive(Serialize, Default, Clone, Debug)]
pub struct ThreadListOptions {
  /// Returns threads archived before this timestamp
  pub before: Option<DateTime<Utc>>,
  /// Optional maximum number of threads to return
  pub limit: Option<i64>,
}

/// Discord thread list response object
#[derive(Deserialize, Clone, Debug)]
pub struct ThreadListResponse {
  /// The threads
  pub threads: Vec<Channel>,
  /// A thread member object for each returned thread the current user has joined
  pub members: Vec<ThreadMember>,
  /// Whether there are potentially additional threads that could be returned on a subsequent call
  pub has_more: bool
}

impl Channel {
  /// Fetch a channel with a channel ID
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// # }
  /// ```
  pub async fn fetch<T: ToString>(rest: &Rest, channel_id: T) -> Result<Self, RestError> {
    rest.get(format!("channels/{}", channel_id.to_string())).await
  }

  /// Edits a channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::{Channel, ChannelModifyOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// let options = ChannelModifyOptions::new().set_topic("Cool channel");
  /// let modified_channel = channel.modify(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn modify(&self, rest: &Rest, options: ChannelModifyOptions) -> Result<Self, RestError> {
    rest.patch(format!("channels/{}", self.id), options).await
  }

  /// Deletes a channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// channel.delete(&input.rest).await?;
  /// # }
  /// ```
  pub async fn delete(&self, rest: &Rest) -> Result<Self, RestError> {
    rest.delete(format!("channels/{}", self.id)).await
  }

  /// Fetch multiple messages from this channel\
  /// See also [`Message::fetch_many`](Message::fetch_many)
  pub async fn fetch_messages(&self, rest: &Rest, options: MessageFetchOptions) -> Result<Vec<Message>, RestError> {
    Message::fetch_many(rest, &self.id, options).await
  }

  /// Fetch a message from this channel with a message ID\
  /// See also [`Message::fetch`](Message::fetch)
  pub async fn fetch_message<T: ToString>(&self, rest: &Rest, message_id: T) -> Result<Message, RestError> {
    Message::fetch(rest, &self.id, message_id).await
  }

  /// Send a new message to this channel\
  /// See also [`Message::create`](Message::create)
  pub async fn create_message<T: Into<MessageResponse>>(&self, rest: &Rest, message: T) -> Result<Message, RestError> {
    Message::create(rest, &self.id, message).await
  }

  /// Delete multiple messages from this channel.\
  /// 2-100 message IDs can be provided at once.
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// let to_delete = vec![String::from("916411877410603008"), String::from("916413462467465246")];
  /// channel.bulk_delete_messages(&input.rest, to_delete).await?;
  /// # }
  /// ```
  pub async fn bulk_delete_messages(&self, rest: &Rest, messages: Vec<Snowflake>) -> Result<(), RestError> {
    let body = json!({ "messages": messages });
    rest.post(format!("channels/{}/messages/bulk-delete", self.id), body).await
  }

  /// Edits a permission overwrite
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::{Channel, PermissionOverwrite, PermissionOverwriteType};
  /// # use slashook::structs::Permissions;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// let overwrite = PermissionOverwrite {
  ///   id: String::from("53908232506183680"),
  ///   overwrite_type: PermissionOverwriteType::MEMBER,
  ///   allow: Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES,
  ///   deny: Permissions::empty()
  /// };
  /// channel.edit_channel_permission(&input.rest, overwrite).await?;
  /// # }
  /// ```
  pub async fn edit_channel_permission(&self, rest: &Rest, overwrite: PermissionOverwrite) -> Result<(), RestError> {
    rest.put(format!("channels/{}/permissions/{}", self.id, overwrite.id), overwrite).await
  }

  /// Deletes a permission overwrite
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// channel.delete_channel_permission(&input.rest, "53908232506183680").await?;
  /// # }
  /// ```
  pub async fn delete_channel_permission<T: ToString>(&self, rest: &Rest, overwrite_id: T) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/permissions/{}", self.id, overwrite_id.to_string())).await
  }

  /// Gets invites for this channel
  pub async fn get_invites(&self, rest: &Rest) -> Result<Vec<Invite>, RestError> {
    rest.get(format!("channels/{}/invites", self.id)).await
  }

  /// Creates a new invite for this channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # use slashook::structs::invites::CreateInviteOptions;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// let options = CreateInviteOptions::new().set_max_uses(1);
  /// let invite = channel.create_invite(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn create_invite(&self, rest: &Rest, options: CreateInviteOptions) -> Result<Invite, RestError> {
    rest.post(format!("channels/{}/invites", self.id), options).await
  }

  /// Follows an announcement channel to send messages to the target channel
  pub async fn follow<T: ToString>(&self, rest: &Rest, target_channel_id: T) -> Result<FollowedChannel, RestError> {
    let body = json!({ "webhook_channel_id": target_channel_id.to_string() });
    rest.post(format!("channels/{}/followers", self.id), body).await
  }

  /// Trigger a typing indicator in the channel
  pub async fn trigger_typing(&self, rest: &Rest) -> Result<(), RestError> {
    rest.post(format!("channels/{}/typing", self.id), Value::Null).await
  }

  /// Get all pinned messages in the channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = input.channel.unwrap();
  /// let pinned_messages = channel.get_pinned_messages(&input.rest).await?;
  /// let ids = pinned_messages.into_iter().map(|m| m.id).collect::<Vec<String>>().join(", ");
  /// res.send_message(ids).await?;
  /// # }
  /// ```
  pub async fn get_pinned_messages(&self, rest: &Rest) -> Result<Vec<Message>, RestError> {
    rest.get(format!("channels/{}/pins", self.id)).await
  }

  /// Pin a message to the channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// channel.pin_message(&input.rest, "1130579253067534356").await?;
  /// # }
  /// ```
  pub async fn pin_message<T: ToString>(&self, rest: &Rest, message_id: T) -> Result<(), RestError> {
    rest.put(format!("channels/{}/pins/{}", self.id, message_id.to_string()), Value::Null).await
  }

  /// Unpin a message from the channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Channel;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = Channel::fetch(&input.rest, "613430047285706767").await?;
  /// channel.unpin_message(&input.rest, "1130579253067534356").await?;
  /// # }
  /// ```
  pub async fn unpin_message<T: ToString>(&self, rest: &Rest, message_id: T) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/pins/{}", self.id, message_id.to_string())).await
  }

  /// Starts a thread, forum post or media post in the channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::{ChannelType, ThreadCreateOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = input.channel.unwrap();
  /// // In a regular channel
  /// let options = ThreadCreateOptions::new("A thread")
  ///   .set_thread_type(ChannelType::GUILD_PUBLIC_THREAD);
  /// channel.start_thread(&input.rest, options).await?;
  /// // In a forum or media channel
  /// let options2 = ThreadCreateOptions::new("A post")
  ///   .set_message("Hello!");
  /// channel.start_thread(&input.rest, options2).await?;
  /// # }
  /// ```
  pub async fn start_thread(&self, rest: &Rest, mut options: ThreadCreateOptions) -> Result<Channel, RestError> {
    let path = format!("channels/{}/threads", self.id);

    if let Some(files) = options.message.as_mut().and_then(|m| m.files.take()) {
      rest.post_files(path, options, files).await
    } else {
      rest.post(path, options).await
    }
  }

  /// Adds the bot user to the thread
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let thread = input.args.get("thread").unwrap().as_channel().unwrap();
  /// thread.join_thread(&input.rest).await?;
  /// # }
  /// ```
  pub async fn join_thread(&self, rest: &Rest) -> Result<(), RestError> {
    self.add_thread_member(rest, "@me").await
  }

  /// Adds another member to a thread
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::{ThreadCreateOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = input.channel.unwrap();
  /// let thread = channel.start_thread(&input.rest, ThreadCreateOptions::new("A thread")).await?;
  /// thread.add_thread_member(&input.rest, input.user.id).await?;
  /// # }
  /// ```
  pub async fn add_thread_member<T: ToString>(&self, rest: &Rest, user_id: T) -> Result<(), RestError> {
    rest.put(format!("channels/{}/thread-members/{}", self.id, user_id.to_string()), Value::Null).await
  }

  /// Removes the bot user from the thread
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::{ThreadCreateOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let channel = input.channel.unwrap();
  /// let thread = channel.start_thread(&input.rest, ThreadCreateOptions::new("A thread")).await?;
  /// thread.leave_thread(&input.rest).await?;
  /// # }
  /// ```
  pub async fn leave_thread(&self, rest: &Rest) -> Result<(), RestError> {
    self.remove_thread_member(rest, "@me").await
  }

  /// Removes another member from the thread
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let thread = input.args.get("thread").unwrap().as_channel().unwrap();
  /// let user = input.args.get("user").unwrap().as_user().unwrap();
  /// thread.remove_thread_member(&input.rest, &user.id).await?;
  /// # }
  /// ```
  pub async fn remove_thread_member<T: ToString>(&self, rest: &Rest, user_id: T) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/thread-members/{}", self.id, user_id.to_string())).await
  }

  /// Gets a thread member object for the specified user
  pub async fn get_thread_member<T: ToString>(&self, rest: &Rest, user_id: T, options: ThreadMemberOptions) -> Result<ThreadMember, RestError> {
    rest.get_query(format!("channels/{}/thread-members/{}", self.id, user_id.to_string()), options).await
  }

  /// Lists thread members. Results will be paginated if `with_member` is set to true
  pub async fn list_thread_members(&self, rest: &Rest, options: ThreadMemberOptions) -> Result<Vec<ThreadMember>, RestError> {
    rest.get_query(format!("channels/{}/thread-members", self.id), options).await
  }

  /// Gets archived threads in the channel that are public
  pub async fn list_public_archived_threads(&self, rest: &Rest, options: ThreadListOptions) -> Result<ThreadListResponse, RestError> {
    rest.get_query(format!("channels/{}/threads/archived/public", self.id), options).await
  }

  /// Gets archived threads in the channel that are private
  pub async fn list_private_archived_threads(&self, rest: &Rest, options: ThreadListOptions) -> Result<ThreadListResponse, RestError> {
    rest.get_query(format!("channels/{}/threads/archived/private", self.id), options).await
  }

  /// Gets archived threads in the channel that are private and the user has joined
  pub async fn list_joined_private_archived_threads(&self, rest: &Rest, options: ThreadListOptions) -> Result<ThreadListResponse, RestError> {
    rest.get_query(format!("channels/{}/users/@me/threads/archived/private", self.id), options).await
  }
}

impl ChannelModifyOptions {
  /// Creates a new empty ChannelModifyOptions
  pub fn new() -> Self {
    Self {
      name: None,
      icon: None,
      channel_type: None,
      position: None,
      topic: None,
      nsfw: None,
      rate_limit_per_user: None,
      bitrate: None,
      user_limit: None,
      permission_overwrites: None,
      parent_id: None,
      rtc_region: None,
      video_quality_mode: None,
      default_auto_archive_duration: None,
      flags: None,
      available_tags: None,
      default_reaction_emoji: None,
      default_thread_rate_limit_per_user: None,
      default_sort_order: None,
      default_forum_layout: None,
      archived: None,
      auto_archive_duration: None,
      locked: None,
      invitable: None,
      applied_tags: None,
    }
  }

  /// Sets the name
  pub fn set_name<T: ToString>(mut self, name: T) -> Self {
    self.name = Some(name.to_string());
    self
  }

  /// Sets the icon
  pub fn set_icon<T: ToString>(mut self, icon: T) -> Self {
    self.icon = Some(icon.to_string());
    self
  }

  /// Sets the channel type
  pub fn set_type(mut self, channel_type: ChannelType) -> Self {
    self.channel_type = Some(channel_type);
    self
  }

  /// Sets the position
  pub fn set_position(mut self, position: i64) -> Self {
    self.position = Some(position);
    self
  }

  /// Sets the topic
  pub fn set_topic<T: ToString>(mut self, topic: T) -> Self {
    self.topic = Some(topic.to_string());
    self
  }

  /// Sets nsfw
  pub fn set_nsfw(mut self, nsfw: bool) -> Self {
    self.nsfw = Some(nsfw);
    self
  }

  /// Sets the rate limit per user
  pub fn set_rate_limit_per_user(mut self, ratelimit: i64) -> Self {
    self.rate_limit_per_user = Some(ratelimit);
    self
  }

  /// Sets the bitrate
  pub fn set_bitrate(mut self, bitrate: i64) -> Self {
    self.bitrate = Some(bitrate);
    self
  }

  /// Sets the user limit
  pub fn set_user_limit(mut self, limit: i64) -> Self {
    self.user_limit = Some(limit);
    self
  }

  /// Adds a permission overwrite
  pub fn add_permission_overwrite(mut self, overwrite: PermissionOverwrite) -> Self {
    let mut overwrites = self.permission_overwrites.unwrap_or_default();
    overwrites.push(overwrite);
    self.permission_overwrites = Some(overwrites);
    self
  }

  /// Sets the parent id
  pub fn set_parent_id<T: ToString>(mut self, id: T) -> Self {
    self.parent_id = Some(id.to_string());
    self
  }

  /// Sets the RTC region
  pub fn set_rtc_region(mut self, region: Option<String>) -> Self {
    self.rtc_region = Some(region);
    self
  }

  /// Sets the video quality mode
  pub fn set_video_quality_mode(mut self, mode: VideoQualityMode) -> Self {
    self.video_quality_mode = Some(mode);
    self
  }

  /// Sets the default auto archive duration
  pub fn set_default_auto_archive_duration(mut self, duration: i64) -> Self {
    self.default_auto_archive_duration = Some(duration);
    self
  }

  /// Sets flags
  pub fn set_flags(mut self, flags: ChannelFlags) -> Self {
    self.flags = Some(flags);
    self
  }

  /// Sets available tags
  pub fn set_available_tags(mut self, tags: Vec<ForumTag>) -> Self {
    self.available_tags = Some(tags);
    self
  }

  /// Sets the default reaction emoji
  pub fn set_default_reaction_emoji(mut self, emoji: DefaultReaction) -> Self {
    self.default_reaction_emoji = Some(emoji);
    self
  }

  /// Sets the default thread rate limit per user
  pub fn set_default_thread_rate_limit_per_user(mut self, ratelimit: i64) -> Self {
    self.default_thread_rate_limit_per_user = Some(ratelimit);
    self
  }

  /// Sets the default sort order
  pub fn set_default_sort_order(mut self, sort_order: SortOrderType) -> Self {
    self.default_sort_order = Some(sort_order);
    self
  }

  /// Sets the default forum layout
  pub fn set_default_forum_layout(mut self, forum_layout: ForumLayoutType) -> Self {
    self.default_forum_layout = Some(forum_layout);
    self
  }

  /// Sets archived
  pub fn set_archived(mut self, archived: bool) -> Self {
    self.archived = Some(archived);
    self
  }

  /// Sets the auto archive duration
  pub fn set_auto_archive_duration(mut self, duration: i64) -> Self {
    self.auto_archive_duration = Some(duration);
    self
  }

  /// Sets locked
  pub fn set_locked(mut self, locked: bool) -> Self {
    self.locked = Some(locked);
    self
  }

  /// Sets invitable
  pub fn set_invitable(mut self, invitable: bool) -> Self {
    self.invitable = Some(invitable);
    self
  }

  /// Sets applied tags
  pub fn set_applied_tags(mut self, tags: Vec<Snowflake>) -> Self {
    self.applied_tags = Some(tags);
    self
  }
}

impl ThreadCreateOptions {
  /// Creates a new ThreadCreateOptions with a name
  pub fn new<T: ToString>(name: T) -> Self {
    Self {
      name: name.to_string(),
      auto_archive_duration: None,
      thread_type: None,
      invitable: None,
      rate_limit_per_user: None,
      message: None,
      applied_tags: None,
    }
  }

  /// Sets the auto archive duration
  pub fn set_auto_archive_duration(mut self, duration: i64) -> Self {
    self.auto_archive_duration = Some(duration);
    self
  }

  /// Sets the thread type
  pub fn set_thread_type(mut self, thread_type: ChannelType) -> Self {
    self.thread_type = Some(thread_type);
    self
  }

  /// Sets invitable
  pub fn set_invitable(mut self, invitable: bool) -> Self {
    self.invitable = Some(invitable);
    self
  }

  /// Sets the rate limit per user
  pub fn set_rate_limit_per_user(mut self, ratelimit: i64) -> Self {
    self.rate_limit_per_user = Some(ratelimit);
    self
  }

  /// Sets the message
  pub fn set_message<T: Into<MessageResponse>>(mut self, message: T) -> Self {
    self.message = Some(message.into());
    self
  }

  /// Sets applied tags
  pub fn set_applied_tags(mut self, tags: Vec<Snowflake>) -> Self {
    self.applied_tags = Some(tags);
    self
  }
}

impl ThreadMemberOptions {
  /// Creates a new ThreadMemberOptions
  pub fn new() -> Self {
    Self {
      with_member: None,
      after: None,
      limit: None,
    }
  }

  /// Sets with member
  pub fn set_with_member(mut self, with_member: bool) -> Self {
    self.with_member = Some(with_member);
    self
  }

  /// Sets after
  pub fn set_after(mut self, after: Snowflake) -> Self {
    self.after = Some(after);
    self
  }

  /// Sets the limit
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }
}

impl ThreadListOptions {
  /// Creates a new ThreadListOptions
  pub fn new() -> Self {
    Self {
      before: None,
      limit: None,
    }
  }

  /// Sets before
  pub fn set_before(mut self, before: DateTime<Utc>) -> Self {
    self.before = Some(before);
    self
  }

  /// Sets the limit
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }
}

impl TryFrom<u8> for ChannelType {
  type Error = serde_json::Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    serde_json::from_value(value.into())
  }
}

impl<'de> Deserialize<'de> for ChannelFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}

impl Serialize for ChannelFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}

impl Attachments for ThreadCreateOptions {
  fn take_attachments(&mut self) -> Vec<Attachment> {
    self.message.as_mut().and_then(|m| m.attachments.take()).unwrap_or_default()
  }

  fn set_attachments(&mut self, attachments: Vec<Attachment>) -> &mut Self {
    self.message.as_mut().and_then(|m| m.attachments.replace(attachments));
    self
  }
}
