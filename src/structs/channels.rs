// Copyright 2022 slashook Developers
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
  applications::Application,
  components::Component,
  embeds::Embed,
  emojis::Emoji,
  guilds::GuildMember,
  interactions::InteractionType,
  invites::{Invite, CreateInviteOptions},
  permissions::Permissions,
  stickers::StickerItem,
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
  /// Default duration for newly created threads, in minutes, to automatically archive the thread after recent activity, can be set to: 60, 1440, 4320, 10080
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
  /// A channel that [users can follow and crosspost into their own server](https://support.discord.com/hc/en-us/articles/360032008192)
  GUILD_NEWS = 5,
  /// A channel in which game developers can [sell their game on Discord](https://discord.com/developers/docs/game-and-server-management/special-channels)
  GUILD_STORE = 6,
  /// A temporary sub-channel within a GUILD_NEWS channel
  GUILD_NEWS_THREAD = 10,
  /// A temporary sub-channel within a GUILD_TEXT channel
  GUILD_PUBLIC_THREAD = 11,
  /// A temporary sub-channel within a GUILD_TEXT channel that is only viewable by those invited and those with the MANAGE_THREADS permission
  GUILD_PRIVATE_THREAD = 12,
  /// A voice channel for [hosting events with an audience](https://support.discord.com/hc/en-us/articles/1500005513722)
  GUILD_STAGE_VOICE = 13,
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
  /// Duration in minutes to automatically archive the thread after recent activity, can be set to: 60, 1440, 4320, 10080
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
  /// The time the current user last joined the thread
  pub join_timestamp: DateTime<Utc>,
  /// Any user-thread settings, currently only used for notifications
  pub flags: i64
}

bitflags! {
  /// Bitflags for Discord Channel Flags
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

/// Discord Followed Channel Object
#[derive(Deserialize, Clone, Debug)]
pub struct FollowedChannel {
  /// Source channel id
  pub channel_id: Snowflake,
  /// Created target webhook id
  pub webhook_id: Snowflake,
}

/// Discord Message Object
#[derive(Deserialize, Clone, Debug)]
pub struct Message {
  /// Id of the message
  pub id: Snowflake,
  /// Id of the channel the message was sent in
  pub channel_id: Snowflake,
  /// Id of the guild the message was sent in
  pub guild_id: Option<Snowflake>,
  /// The author of this message (not guaranteed to be a valid user)
  pub author: User,
  /// Member properties for this message's author
  pub member: Option<GuildMember>,
  /// Contents of the message
  pub content: String,
  /// When this message was sent
  pub timestamp: DateTime<Utc>,
  /// When this message was edited (or None if never)
  pub edited_timestamp: Option<DateTime<Utc>>,
  /// Whether this was a TTS message
  pub tts: bool,
  /// Whether this message mentions everyone
  pub mention_everyone: bool,
  /// Users specifically mentioned in the message
  pub mentions: Vec<User>,
  /// Roles specifically mentioned in this message
  pub mention_roles: Vec<Snowflake>,
  /// Channels specifically mentioned in this message
  pub mention_channels: Option<Vec<ChannelMention>>,
  /// Any attached files
  pub attachments: Vec<Attachment>,
  /// Any embedded content
  pub embeds: Vec<Embed>,
  /// Reactions to the message
  pub reactions: Option<Vec<Reaction>>,
  /// Used for validating a message was sent
  pub nonce: Option<String>,
  /// Whether this message is pinned
  pub pinned: bool,
  /// If the message is generated by a webhook, this is the webhook's id
  pub webhook_id: Option<Snowflake>,
  /// [type of message](MessageType)
  #[serde(rename = "type")]
  pub message_type: MessageType,
  /// Sent with Rich Presence-related chat embeds
  pub activity: Option<MessageActivity>,
  /// Sent with Rich Presence-related chat embeds
  pub application: Option<Application>,
  /// If the message is a response to an [Interaction](https://discord.com/developers/docs/interactions/receiving-and-responding), this is the id of the interaction's application
  pub application_id: Option<Snowflake>,
  /// Data showing the source of a crosspost, channel follow add, pin, or reply message
  pub message_reference: Option<MessageReference>,
  /// [Message flags](MessageFlags) combined as a [bitfield](https://en.wikipedia.org/wiki/Bit_field)
  pub flags: Option<MessageFlags>,
  /// The message associated with the message_reference
  pub referenced_message: Option<Box<Message>>,
  /// Sent if the message is a response to an [Interaction](https://discord.com/developers/docs/interactions/receiving-and-responding)
  pub interaction: Option<MessageInteraction>,
  /// The thread that was started from this message, includes [thread member](ThreadMember) object
  pub thread: Option<Channel>,
  /// Sent if the message contains components like buttons, action rows, or other interactive components
  pub components: Option<Vec<Component>>,
  /// Sent if the message contains stickers
  pub sticker_items: Option<Vec<StickerItem>>
}

/// Discord Channel Mention Object
#[derive(Deserialize, Clone, Debug)]
pub struct ChannelMention {
  /// Id of the channel
  pub id: Snowflake,
  /// Id of the guild containing the channel
  pub guild_id: Snowflake,
  /// The [type of channel](ChannelType)
  #[serde(rename = "type")]
  pub channel_type: ChannelType,
  /// The name of the channel
  pub name: String
}

/// Discord Attachment Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attachment {
  /// Attachment id
  pub id: Snowflake,
  /// Name of file attached
  pub filename: String,
  /// Description for the file
  pub description: Option<String>,
  /// The attachment's [media type](https://en.wikipedia.org/wiki/Media_type)
  pub content_type: Option<String>,
  /// Size of file in bytes
  pub size: i64,
  /// Source url of file
  pub url: String,
  /// A proxied url of file
  pub proxy_url: String,
  /// Height of file (if image)
  pub height: Option<i64>,
  /// Width of file (if image)
  pub width: Option<i64>,
  /// Whether this attachment is ephemeral
  pub ephemeral: Option<bool>
}

/// Discord Reaction Object
#[derive(Deserialize, Clone, Debug)]
pub struct Reaction {
  /// Times this emoji has been used to react
  pub count: i64,
  /// Whether the current user reacted using this emoji
  pub me: bool,
  /// Emoji information
  pub emoji: Emoji
}

/// Discord Message Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MessageType {
  /// Normal message
  DEFAULT = 0,
  /// Member added to a group chat
  RECIPIENT_ADD = 1,
  /// Member removed from a group chat
  RECIPIENT_REMOVE = 2,
  /// Call notification
  CALL = 3,
  /// Channel name changed
  CHANNEL_NAME_CHANGE = 4,
  /// Channel icon changed
  CHANNEL_ICON_CHANGE = 5,
  /// A message was pinned
  CHANNEL_PINNED_MESSAGE = 6,
  /// A new member joined a server
  GUILD_MEMBER_JOIN = 7,
  /// A member boosted the server
  USER_PREMIUM_GUILD_SUBSCRIPTION = 8,
  /// A member boosted the server and reached tier 1
  USER_PREMIUM_GUILD_SUBSCRIPTION_TIER_1 = 9,
  /// A member boosted the server and reached tier 2
  USER_PREMIUM_GUILD_SUBSCRIPTION_TIER_2 = 10,
  /// A member boosted the server and reached tier 3
  USER_PREMIUM_GUILD_SUBSCRIPTION_TIER_3 = 11,
  /// A news channel followed
  CHANNEL_FOLLOW_ADD = 12,
  /// Server is no longer qualified for discovery
  GUILD_DISCOVERY_DISQUALIFIED = 14,
  /// Server is qualified for discovery again
  GUILD_DISCOVERY_REQUALIFIED = 15,
  /// First warning about losing discovery eligibility
  GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING = 16,
  /// Final warning about losing discovery eligibility
  GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING = 17,
  /// A new thread was created
  THREAD_CREATED = 18,
  /// Message is a reply to another message
  REPLY = 19,
  /// Message was sent from a chat input command
  CHAT_INPUT_COMMAND = 20,
  /// Message is the first message that started the thread
  THREAD_STARTER_MESSAGE = 21,
  /// Server setup tips
  GUILD_INVITE_REMINDER = 22,
  /// Message was sent from a context menu command
  CONTEXT_MENU_COMMAND = 23,
  /// A message type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Message Activity Object
#[derive(Deserialize, Clone, Debug)]
pub struct MessageActivity {
  /// [Type of message activity](MessageActivityType)
  #[serde(rename = "type")]
  pub activity_type: MessageActivityType,
  /// party_id from a [Rich Presence event](https://discord.com/developers/docs/rich-presence/how-to#updating-presence-update-presence-payload-fields)
  pub party_id: Option<String>
}

/// Discord Message Activity Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MessageActivityType {
  /// Invite to join activity
  JOIN = 1,
  /// Invite to spectate activity
  SPECTATE = 2,
  /// Invite to listen along
  LISTEN = 3,
  /// Invite to request to join
  JOIN_REQUEST = 5,
  /// Message activity type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Message Reference Object
#[derive(Deserialize, Clone, Debug)]
pub struct MessageReference {
  /// Id of the originating message
  pub message_id: Option<Snowflake>,
  /// Id of the originating message's channel
  pub channel_id: Option<Snowflake>,
  /// Id of the originating message's guild
  pub guild_id: Option<Snowflake>,
  /// When sending, whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message, default true
  pub fail_if_not_exists: Option<bool>
}

bitflags! {
  /// Bitflags for Discord Message Flags
  pub struct MessageFlags: u32 {
    /// This message has been published to subscribed channels (via Channel Following)
    const CROSSPOSTED = 1 << 0;
    /// This message originated from a message in another channel (via Channel Following)
    const IS_CROSSPOST = 1 << 1;
    /// Do not include any embeds when serializing this message
    const SUPPRESS_EMBEDS = 1 << 2;
    /// The source message for this crosspost has been deleted (via Channel Following)
    const SOURCE_MESSAGE_DELETED = 1 << 3;
    /// This message came from the urgent message system
    const URGENT = 1 << 4;
    /// This message has an associated thread, with the same id as the message
    const HAS_THREAD = 1 << 5;
    /// This message is only visible to the user who invoked the Interaction
    const EPHEMERAL = 1 << 6;
    /// This message is an Interaction Response and the bot is "thinking"
    const LOADING = 1 << 7;
  }
}

/// Discord Message Interaction Object
#[derive(Deserialize, Clone, Debug)]
pub struct MessageInteraction {
  /// Id of the interaction
  pub id: Snowflake,
  /// The type of interaction
  #[serde(rename = "type")]
  pub interaction_type: Option<InteractionType>,
  /// The name of the [application command](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure)
  pub name: String,
  /// The user who invoked the interaction
  pub user: User,
  /// The member who invoked the interaction in the guild
  pub member: Option<GuildMember>
}

/// Discord Allowed Mentions Object
#[derive(Serialize, Clone, Debug)]
pub struct AllowedMentions {
  /// An array of [allowed mention types](AllowedMentionType) to parse from the content.
  pub parse: Option<Vec<AllowedMentionType>>,
  /// Array of role_ids to mention (Max size of 100)
  pub roles: Option<Vec<Snowflake>>,
  /// Array of user_ids to mention (Max size of 100)
  pub users: Option<Vec<Snowflake>>,
  /// For replies, whether to mention the author of the message being replied to (default false)
  pub replied_user: Option<bool>
}

/// Discord Allowed Mention Types
#[derive(Serialize, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum AllowedMentionType {
  /// Allowed to mention roles
  roles,
  /// Allowed to mention users
  users,
  /// Allowed to mention @everyone and @here
  everyone
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

/// Options for fetching multiple messages with [fetch_many](Message::fetch_many).
/// Only one of `around`, `before`, or `after` can be passed at once.
#[derive(Serialize, Default, Clone, Debug)]
pub struct MessageFetchOptions {
  /// Get messages around this message ID
  pub around: Option<Snowflake>,
  /// Get messages before this message ID
  pub before: Option<Snowflake>,
  /// Get messages after this message ID
  pub after: Option<Snowflake>,
  /// Max number of messages to return (1-100). Defaults to 50.
  pub limit: Option<i64>,
}

/// Options for fetching reactions with [get_reactions](Message::get_reactions).
#[derive(Serialize, Default, Clone, Debug)]
pub struct ReactionFetchOptions {
  /// Get users after this user ID
  pub after: Option<Snowflake>,
  /// Max number of users to return (1-100) Defaults to 25.
  pub limit: Option<i64>,
}

impl Channel {
  /// Fetch a channel with a channel ID
  /// ```no_run
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
  /// ```no_run
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
  /// ```no_run
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
  /// ```no_run
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
  /// ```no_run
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
  /// ```no_run
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
  /// ```no_run
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
}

impl TryFrom<u8> for ChannelType {
  type Error = serde_json::Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    serde_json::from_value(value.into())
  }
}

impl Message {
  /// Fetch a single message with a channel and message ID
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let message = Message::fetch(&input.rest, "613430047285706767", "916413462467465246").await?;
  /// # }
  /// ```
  pub async fn fetch<T: ToString, U: ToString>(rest: &Rest, channel_id: T, message_id: U) -> Result<Self, RestError> {
    rest.get(format!("channels/{}/messages/{}", channel_id.to_string(), message_id.to_string())).await
  }

  /// Fetch multiple messages with a channel ID and options
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::{Message, MessageFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let options = MessageFetchOptions::new().set_before("940762083820175440").set_limit(5);
  /// let messages = Message::fetch_many(&input.rest, "697138785317814292", options).await?;
  /// # }
  /// ```
  pub async fn fetch_many<T: ToString>(rest: &Rest, channel_id: T, options: MessageFetchOptions) -> Result<Vec<Self>, RestError> {
    rest.get_query(format!("channels/{}/messages", channel_id.to_string()), options).await
  }

  /// Send a new message to a channel
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581372137963522", "Hello!").await?;
  /// # }
  /// ```
  pub async fn create<T: ToString, U: Into<MessageResponse>>(rest: &Rest, channel_id: T, message: U) -> Result<Self, RestError> {
    let mut message = message.into();
    let files = message.files;
    message.files = None;
    let path = format!("channels/{}/messages", channel_id.to_string());
    if let Some(files) = files {
      Ok(rest.post_files(path, message, files).await?)
    } else {
      Ok(rest.post(path, message).await?)
    }
  }

  /// Edit a message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581372137963522", "Hello!").await?;
  /// let edited_msg = msg.edit(&input.rest, "Bye!").await?;
  /// # }
  /// ```
  pub async fn edit<T: Into<MessageResponse>>(&self, rest: &Rest, message: T) -> Result<Message, RestError> {
    let mut message = message.into();
    let files = message.files;
    message.files = None;
    let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
    if let Some(files) = files {
      Ok(rest.patch_files(path, message, files).await?)
    } else {
      Ok(rest.patch(path, message).await?)
    }
  }

  /// Delete a message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581372137963522", "Hello!").await?;
  /// msg.delete(&input.rest).await?;
  /// # }
  /// ```
  pub async fn delete(&self, rest: &Rest) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/messages/{}", self.channel_id, self.id)).await
  }

  /// Publish a message that was posted in an [Announcement channel](ChannelType::GUILD_NEWS)
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::channels::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581598878105605", "Hello!").await?;
  /// msg.crosspost(&input.rest).await?;
  /// # }
  /// ```
  pub async fn crosspost(&self, rest: &Rest) -> Result<Message, RestError> {
    rest.post(format!("channels/{}/messages/{}/crosspost", self.channel_id, self.id), Value::Null).await
  }

  /// Add a reaction to a message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{channels::Message, Emoji};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581598878105605", "Hello!").await?;
  /// msg.create_reaction(&input.rest, &Emoji::new_standard_emoji("👋")).await?;
  /// # }
  /// ```
  pub async fn create_reaction(&self, rest: &Rest, emoji: &Emoji) -> Result<(), RestError> {
    rest.put(format!("channels/{}/messages/{}/reactions/{}/@me", &self.channel_id, &self.id, emoji.to_url_format()), Value::Null).await
  }

  /// Remove the bot's reaction to a message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{channels::Message, Emoji};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581598878105605", "Hello!").await?;
  /// let emoji = Emoji::new_standard_emoji("👋");
  /// msg.create_reaction(&input.rest, &emoji).await?;
  /// msg.delete_reaction(&input.rest, &emoji).await?;
  /// # }
  /// ```
  pub async fn delete_reaction(&self, rest: &Rest, emoji: &Emoji) -> Result<(), RestError> {
    self.delete_user_reaction(rest, emoji, "@me").await
  }

  /// Remove someone else's reaction to a message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{Emoji, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.delete_user_reaction(&input.rest, &Emoji::new_standard_emoji("👋"), input.user.id).await?;
  /// # }
  /// ```
  pub async fn delete_user_reaction<T: ToString>(&self, rest: &Rest, emoji: &Emoji, user_id: T) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/messages/{}/reactions/{}/{}", &self.channel_id, &self.id, emoji.to_url_format(), user_id.to_string())).await
  }

  /// Get the users who reacted to a message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{Emoji, channels::ReactionFetchOptions, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// let options = ReactionFetchOptions::new().set_limit(5);
  /// let reactions = msg.get_reactions(&input.rest, &Emoji::new_standard_emoji("👋"), options).await?;
  /// println!("{:?}", reactions);
  /// # }
  /// ```
  pub async fn get_reactions(&self, rest: &Rest, emoji: &Emoji, options: ReactionFetchOptions) -> Result<Vec<User>, RestError> {
    rest.get_query(format!("channels/{}/messages/{}/reactions/{}", &self.channel_id, &self.id, emoji.to_url_format()), options).await
  }

  /// Delete all reactions from a message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{Emoji, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.delete_all_reactions(&input.rest).await?;
  /// # }
  /// ```
  pub async fn delete_all_reactions(&self, rest: &Rest) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/messages/{}/reactions", &self.channel_id, &self.id)).await
  }

  /// Delete all reactions for a single emoji from the message
  /// ```no_run
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{Emoji, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.delete_all_reactions_for_emoji(&input.rest, &Emoji::new_standard_emoji("👋")).await?;
  /// # }
  /// ```
  pub async fn delete_all_reactions_for_emoji(&self, rest: &Rest, emoji: &Emoji) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/messages/{}/reactions/{}", &self.channel_id, &self.id, emoji.to_url_format())).await
  }
}

impl Attachment {
  /// Creates an attachment object that can be used to tell discord to keep the attachment when editing.
  pub fn keep_with_id<T: ToString>(id: T) -> Self {
    Self {
      id: id.to_string(),
      filename: String::from(""),
      description: None,
      content_type: None,
      size: 0,
      url: String::from(""),
      proxy_url: String::from(""),
      height: None,
      width: None,
      ephemeral: None
    }
  }

  pub(crate) fn with_description<T: ToString, U: ToString>(id: T, description: U) -> Self {
    Self {
      id: id.to_string(),
      filename: String::from(""),
      description: Some(description.to_string()),
      content_type: None,
      size: 0,
      url: String::from(""),
      proxy_url: String::from(""),
      height: None,
      width: None,
      ephemeral: None
    }
  }
}

impl AllowedMentions {
  /// Create a new allowed mentions object. By default doesn't allow any mentions
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::channels::{AllowedMentions, AllowedMentionType};
  /// let allowed_mentions = AllowedMentions::new();
  /// let response = MessageResponse::from("<@1234> @everyone <@&1235> No pings.")
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn new() -> Self {
    Self {
      parse: Some(Vec::new()),
      roles: None,
      users: None,
      replied_user: None
    }
  }

  /// Add an allowed mention type to parse. You cannot use the user or role type at the same time as an array of allowed ids for that type
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::channels::{AllowedMentions, AllowedMentionType};
  /// let allowed_mentions = AllowedMentions::new().add_parse(AllowedMentionType::users);
  /// let response = MessageResponse::from("<@1234> Get pinged. Not @everyone or <@&1235> tho.")
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn add_parse(mut self, parsed: AllowedMentionType) -> Self {
    let mut parse = self.parse.unwrap_or_default();
    parse.push(parsed);
    self.parse = Some(parse);
    self
  }

  /// Add a user ID to the list of allowed user mentions
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::channels::{AllowedMentions, AllowedMentionType};
  /// # let user_id = String::from("1234");
  /// let message = format!("<@{}> Get pinged.", user_id);
  /// let allowed_mentions = AllowedMentions::new().add_user(user_id);
  /// let response = MessageResponse::from(message)
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn add_user(mut self, user: Snowflake) -> Self {
    let mut users = self.users.unwrap_or_default();
    users.push(user);
    self.users = Some(users);
    self
  }

  /// Add a role ID to the list of allowed role mentions
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::channels::{AllowedMentions, AllowedMentionType};
  /// # let role_id = String::from("1235");
  /// let message = format!("<@&{}> Get pinged.", role_id);
  /// let allowed_mentions = AllowedMentions::new().add_role(role_id);
  /// let response = MessageResponse::from(message)
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn add_role(mut self, role: Snowflake) -> Self {
    let mut roles = self.roles.unwrap_or_default();
    roles.push(role);
    self.roles = Some(roles);
    self
  }

  /// Set if the replied user should be mentioned or not
  pub fn set_replied_user(mut self, b: bool) -> Self {
    self.replied_user = Some(b);
    self
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

impl MessageFetchOptions {
  /// Creates a new empty MessageFetchOptions
  pub fn new() -> Self {
    Self {
      around: None,
      before: None,
      after: None,
      limit: None,
    }
  }

  /// Sets the message ID to search around.
  /// Also removes `before` and `after` if set.
  pub fn set_around<T: ToString>(mut self, around: T) -> Self {
    self.around = Some(around.to_string());
    self.before = None;
    self.after = None;
    self
  }

  /// Sets the message ID to search before.
  /// Also removes `around` and `after` if set.
  pub fn set_before<T: ToString>(mut self, before: T) -> Self {
    self.around = None;
    self.before = Some(before.to_string());
    self.after = None;
    self
  }

  /// Sets the message ID to search after.
  /// Also removes `around` and `before` if set.
  pub fn set_after<T: ToString>(mut self, after: T) -> Self {
    self.around = None;
    self.before = None;
    self.after = Some(after.to_string());
    self
  }

  /// Sets the limit for the amount of messages to fetch
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }
}

impl ReactionFetchOptions {
  /// Creates a new empty ReactionFetchOptions
  pub fn new() -> Self {
    Self {
      after: None,
      limit: None,
    }
  }

  /// Sets the user ID to search after.
  pub fn set_after<T: ToString>(mut self, after: T) -> Self {
    self.after = Some(after.to_string());
    self
  }

  /// Sets the limit for the amount of reactions to fetch
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }
}

impl Default for AllowedMentions {
  fn default() -> Self {
    Self::new()
  }
}

impl<'de> Deserialize<'de> for ChannelFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_truncate(bits))
  }
}

impl Serialize for ChannelFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}

impl<'de> Deserialize<'de> for MessageFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_truncate(bits))
  }
}

impl Serialize for MessageFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}
