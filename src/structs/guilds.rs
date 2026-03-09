// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord guilds

use serde::{Deserialize, Serialize, ser::Serializer, de::Deserializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use chrono::{DateTime, Utc};
use bitflags::bitflags;

use super::{
  channels::{Channel, ChannelCreateOptions, ThreadMember},
  Emoji,
  members::GuildMember,
  Permissions,
  roles::Role,
  stickers::Sticker,
  users::User,
  Snowflake,
};
use crate::rest::{Rest, RestError};

/// Discord Guild Object
#[derive(Deserialize, Clone, Debug)]
pub struct Guild {
  /// Guild id
  pub id: Snowflake,
  /// Guild name (2-100 characters, excluding trailing and leading whitespace)
  pub name: String,
  /// [Icon hash](https://discord.com/developers/docs/reference#image-formatting)
  pub icon: Option<String>,
  /// [Icon hash](https://discord.com/developers/docs/reference#image-formatting), returned when in the template object
  pub icon_hash: Option<String>,
  /// [Splash hash](https://discord.com/developers/docs/reference#image-formatting)
  pub splash: Option<String>,
  /// [Discovery splash hash](https://discord.com/developers/docs/reference#image-formatting); only present for guilds with the "DISCOVERABLE" feature
  pub discovery_splash: Option<String>,
  /// True if [the user](https://discord.com/developers/docs/resources/user#get-current-user-guilds) is the owner of the guild
  pub owner: Option<bool>,
  /// Id of owner
  pub owner_id: Option<Snowflake>,
  /// Total permissions for [the user](https://discord.com/developers/docs/resources/user#get-current-user-guilds) in the guild (excludes overwrites and [implicit permissions](https://docs.discord.com/developers/topics/permissions#implicit-permissions))
  pub permissions: Option<Permissions>,
  /// Id of afk channel
  pub afk_channel_id: Option<Snowflake>,
  /// Afk timeout in seconds
  pub afk_timeout: Option<i64>,
  /// True if the server widget is enabled
  pub widget_enabled: Option<bool>,
  /// The channel id that the widget will generate an invite to, or None if set to no invite
  pub widget_channel_id: Option<Snowflake>,
  /// [Verification level](VerificationLevel) required for the guild
  pub verification_level: Option<VerificationLevel>,
  /// Default [message notifications level](MessageNotificationLevel)
  pub default_message_notifications: Option<MessageNotificationLevel>,
  /// [Explicit content filter level](ExplicitContentFilterLevel)
  pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
  /// Roles in the guild
  pub roles: Option<Vec<Role>>,
  /// Custom guild emojis
  pub emojis: Option<Vec<Emoji>>,
  /// Enabled guild features
  pub features: Vec<String>,
  /// Required [MFA level](MFALevel) for the guild
  pub mfa_level: Option<MFALevel>,
  /// Application id of the guild creator if it is bot-created
  pub application_id: Option<Snowflake>,
  /// The id of the channel where guild notices such as welcome messages and boost events are posted
  pub system_channel_id: Option<Snowflake>,
  /// [System channel flags](SystemChannelFlags)
  pub system_channel_flags: Option<SystemChannelFlags>,
  /// The id of the channel where Community guilds can display rules and/or guidelines
  pub rules_channel_id: Option<Snowflake>,
  /// The maximum number of presences for the guild (`None` is always returned, apart from the largest of guilds)
  pub max_presences: Option<i64>,
  /// The maximum number of members for the guild
  pub max_members: Option<i64>,
  /// The vanity url code for the guild
  pub vanity_url_code: Option<String>,
  /// The description of a guild
  pub description: Option<String>,
  /// [Banner hash](https://discord.com/developers/docs/reference#image-formatting)
  pub banner: Option<String>,
  /// [Premium tier](PremiumTier) (Server Boost level)
  pub premium_tier: Option<PremiumTier>,
  /// The number of boosts this guild currently has
  pub premium_subscription_count: Option<i64>,
  /// The preferred locale of a Community guild; used in server discovery and notices from Discord, and sent in interactions; defaults to "en-US"
  pub preferred_locale: Option<String>,
  /// The id of the channel where admins and moderators of Community guilds receive notices from Discord
  pub public_updates_channel_id: Option<Snowflake>,
  /// The maximum amount of users in a video channel
  pub max_video_channel_users: Option<i64>,
  /// The maximum amount of users in a stage video channel
  pub max_stage_video_channel_users: Option<i64>,
  /// Approximate number of members in this guild, returned from the `GET /guilds/<id>` and `/users/@me/guilds` endpoints when `with_counts` is `true`
  pub approximate_member_count: Option<i64>,
  /// Approximate number of non-offline members in this guild, returned from the `GET /guilds/<id>` and `/users/@me/guilds` endpoints when `with_counts` is `true`
  pub approximate_presence_count: Option<i64>,
  /// The welcome screen of a Community guild, shown to new members, returned in an [Invite](super::invites::Invite)'s guild object
  pub welcome_screen: Option<WelcomeScreen>,
  /// [Guild age-restriction level](NSFWLevel)
  pub nsfw_level: Option<NSFWLevel>,
  /// Custom guild stickers
  pub stickers: Option<Vec<Sticker>>,
  /// Whether the guild has the boost progress bar enabled
  pub premium_progress_bar_enabled: Option<bool>,
  /// The id of the channel where admins and moderators of Community guilds receive safety alerts from Discord
  pub safety_alerts_channel_id: Option<Snowflake>,
  /// The incidents data for this guild
  pub incidents_data: Option<GuildIncidentsData>,
}

/// Discord Guild Preview Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildPreview {
  /// Guild id
  pub id: Snowflake,
  /// Guild name (2-100 characters)
  pub name: String,
  /// [Icon hash](https://discord.com/developers/docs/reference#image-formatting)
  pub icon: Option<String>,
  /// [Splash hash](https://discord.com/developers/docs/reference#image-formatting)
  pub splash: Option<String>,
  /// [Discovery splash hash](https://discord.com/developers/docs/reference#image-formatting)
  pub discovery_splash: Option<String>,
  /// Custom guild emojis
  pub emojis: Vec<Emoji>,
  /// Enabled guild features
  pub features: Vec<String>,
  /// Approximate number of members in this guild
  pub approximate_member_count: i64,
  /// Approximate number of online members in this guild
  pub approximate_presence_count: i64,
  /// The description for the guild
  pub description: Option<String>,
  /// Custom guild stickers
  pub stickers: Vec<Sticker>,
}

/// Discord Verification Levels
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum VerificationLevel {
  /// Unrestricted
  NONE = 0,
  /// Must have verified email on account
  LOW = 1,
  /// Must be registered on Discord for longer than 5 minutes
  MEDIUM = 2,
  /// Must be a member of the server for longer than 10 minutes
  HIGH = 3,
  /// Must have a verified phone number
  VERY_HIGH = 4,
  /// Verification level that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord Message Notifications Level
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MessageNotificationLevel {
  /// Members will receive notifications for all messages by default
  ALL_MESSAGES = 0,
  /// Members will receive notifications only for messages that @mention them by default
  ONLY_MENTIONS = 1,
  /// Message notifications level that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord Explicit Content Filter Level
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ExplicitContentFilterLevel {
  /// Media content will not be scanned
  DISABLED = 0,
  /// Media content sent by members without roles will be scanned
  MEMBERS_WITHOUT_ROLES = 1,
  /// Media content sent by all members will be scanned
  ALL_MEMBERS = 2,
  /// Explicit content filter level that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord MFA Level
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MFALevel {
  /// Guild has no MFA/2FA requirement for moderation actions
  NONE = 0,
  /// Guild has a 2FA requirement for moderation actions
  ELEVATED = 1,
  /// MFA level that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

bitflags! {
  /// Bitflags for Discord System Channel Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct SystemChannelFlags: u32 {
    /// Suppress member join notifications
    const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
    /// Suppress server boost notifications
    const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
    /// Suppress server setup tips
    const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
    /// Hide member join sticker reply buttons
    const SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;
    /// Suppress role subscription purchase and renewal notifications
    const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4;
    /// Hide role subscription sticker reply buttons
    const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5;
  }
}

/// Discord Premium Tier
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PremiumTier {
  /// Guild has not unlocked any Server Boost perks
  NONE = 0,
  /// Guild has unlocked Server Boost level 1 perks
  TIER_1 = 1,
  /// Guild has unlocked Server Boost level 2 perks
  TIER_2 = 2,
  /// Guild has unlocked Server Boost level 3 perks
  TIER_3 = 3,
  /// Premium tier that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord Welcome Screen Object
#[derive(Deserialize, Clone, Debug)]
pub struct WelcomeScreen {
  /// The server description shown in the welcome screen
  pub description: Option<String>,
  /// The channels shown in the welcome screen, up to 5
  pub welcome_channels: Vec<WelcomeScreenChannel>,
}

/// Discord Welcome Screen Channel Object
#[derive(Deserialize, Clone, Debug)]
pub struct WelcomeScreenChannel {
  /// The channel's id
  pub channel_id: Snowflake,
  /// The description shown for the channel
  pub description: String,
  /// The [emoji id](https://discord.com/developers/docs/reference#image-formatting), if the emoji is custom
  pub emoji_id: Option<String>,
  /// The emoji name if custom, the unicode character if standard, or `None` if no emoji is set
  pub emoji_name: Option<String>,
}

/// Discord Guild NSFW Level
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum NSFWLevel {
  /// Default
  DEFAULT = 0,
  /// Explicit
  EXPLICIT = 1,
  /// Safe
  SAFE = 2,
  /// Age Restricted
  AGE_RESTRICTED = 3,
  /// NSFW level that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN
}

/// Discord Incidents Data Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildIncidentsData {
  /// When invites get enabled again
  pub invites_disabled_until: Option<DateTime<Utc>>,
  /// When direct messages get enabled again
  pub dms_disabled_until: Option<DateTime<Utc>>,
  /// When the dm spam was detected
  pub dm_spam_detected_at: Option<DateTime<Utc>>,
  /// When the raid was detected
  pub raid_detected_at: Option<DateTime<Utc>>,
}

/// Discord Guild Scheduled Event Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildScheduledEvent {
  /// The id of the scheduled event
  pub id: Snowflake,
  /// The guild id which the scheduled event belongs to
  pub guild_id: Snowflake,
  /// The channel id in which the scheduled event will be hosted, or `None` if scheduled entity type is `EXTERNAL`
  pub channel_id: Option<Snowflake>,
  /// The id of the user that created the scheduled event
  pub creator_id: Option<Snowflake>,
  /// The name of the scheduled event (1-100 characters)
  pub name: String,
  /// The description of the scheduled event (1-1000 characters)
  pub description: Option<String>,
  /// The time the scheduled event will start
  pub scheduled_start_time: DateTime<Utc>,
  /// The time the scheduled event will end, required if entity_type is `EXTERNAL`
  pub scheduled_end_time: Option<DateTime<Utc>>,
  /// The privacy level of the scheduled event
  pub privacy_level: PrivacyLevel,
  /// The status of the scheduled event
  pub status: EventStatus,
  /// The type of the scheduled event
  pub entity_type: EntityType,
  /// The id of an entity associated with a guild scheduled event
  pub entity_id: Option<Snowflake>,
  /// Additional metadata for the guild scheduled event
  pub entity_metadata: Option<EntityMetadata>,
  /// The user that created the scheduled event
  pub creator: Option<User>,
  /// The number of users subscribed to the scheduled event
  pub user_count: Option<i64>,
  /// The [cover image hash](https://discord.com/developers/docs/reference#image-formatting) of the scheduled event
  pub image: Option<String>,
  /// The definition for how often this event should recur
  pub recurrence_rule: Option<EventRecurrenceRule>,
}

/// Discord Guild Scheduled Event Privacy Level
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PrivacyLevel {
  /// The scheduled event is only accessible to guild members
  GUILD_ONLY = 2,
  /// Privacy level that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN
}

/// Discord Guild Scheduled Event Status
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EventStatus {
  /// Scheduled
  SCHEDULED = 1,
  /// Active
  ACTIVE = 2,
  /// Completed
  COMPLETED = 3,
  /// Canceled
  CANCELED = 4,
  /// Status that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN
}

/// Discord Guild Scheduled Event Entity Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EntityType {
  /// Stage instance
  STAGE_INSTANCE = 1,
  /// Voice
  VOICE = 2,
  /// External
  EXTERNAL = 3,
  /// Entity type that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN
}

/// Discord Guild Scheduled Event Entity Metadata
#[derive(Deserialize, Clone, Debug)]
pub struct EntityMetadata {
  /// Location of the event (1-100 characters)
  pub location: Option<String>,
}

/// Discord Guild Scheduled Event Recurrence Rule Object
#[derive(Deserialize, Clone, Debug)]
pub struct EventRecurrenceRule {
  /// Starting time of the recurrence interval
  pub start: DateTime<Utc>,
  /// Ending time of the recurrence interval
  pub end: Option<DateTime<Utc>>,
  /// How often the event occurs
  pub frequency: EventRecurrenceRuleFrequency,
  /// The spacing between the events, defined by `frequency`. For example, `frequency` of `WEEKLY` and an `interval` of `2` would be "every-other week"
  pub interval: i64,
  /// Set of specific days within a week for the event to recur on
  pub by_weekday: Option<Vec<EventRecurrenceRuleWeekday>>,
  /// List of specific days within a specific week (1-5) to recur on
  pub by_n_weekday: Option<Vec<EventRecurrenceRuleNWeekday>>,
  /// Set of specific months to recur on
  pub by_month: Option<Vec<EventRecurrenceRuleMonth>>,
  /// Set of specific dates within a month to recur on
  pub by_month_day: Option<Vec<i64>>,
  /// Set of days within a year to recur on (1-364)
  pub by_year_day: Option<Vec<i64>>,
  /// The total amount of times that the event is allowed to recur before stopping
  pub count: Option<i64>,
}

/// Discord Guild Scheduled Event Recurrence Rule - Frequency
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EventRecurrenceRuleFrequency {
  /// Yearly
  YEARLY = 0,
  /// Monthly
  MONTHLY = 1,
  /// Weekly
  WEEKLY = 2,
  /// Daily
  DAILY = 3,
  /// Frequency not implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord Guild Scheduled Event Recurrence Rule - Weekday
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EventRecurrenceRuleWeekday {
  /// Monday
  MONDAY = 0,
  /// Tuesday
  TUESDAY = 1,
  /// Wednesday
  WEDNESDAY = 2,
  /// Thursday
  THURSDAY = 3,
  /// Friday
  FRIDAY = 4,
  /// Saturday
  SATURDAY = 5,
  /// Sunday
  SUNDAY = 6,
  /// If for some reason humanity decides to create a new weekday
  #[serde(other)]
  UNKNOWN,
}

/// Discord Guild Scheduled Event Recurrence Rule - N_Weekday
#[derive(Deserialize, Clone, Debug)]
pub struct EventRecurrenceRuleNWeekday {
  /// The week to reoccur on. 1 - 5
  pub n: i64,
  /// The day within the week to reoccur on
  pub day: EventRecurrenceRuleWeekday,
}

/// Discord Guild Scheduled Event Recurrence Rule - Month
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EventRecurrenceRuleMonth {
  /// January
  JANUARY = 1,
  /// February
  FEBRUARY = 2,
  /// March
  MARCH = 3,
  /// April
  APRIL = 4,
  /// May
  MAY = 5,
  /// June
  JUNE = 6,
  /// July
  JULY = 7,
  /// August
  AUGUST = 8,
  /// September
  SEPTEMBER = 9,
  /// October
  OCTOBER = 10,
  /// November
  NOVEMBER = 11,
  /// December
  DECEMBER = 12,
  /// If for some reason humanity decides to create a new month
  #[serde(other)]
  UNKNOWN,
}

/// Options for fetching a guild
#[derive(Serialize, Default, Clone, Debug)]
pub struct GuildFetchOptions {
  /// when `true`, will return approximate member and presence counts for the guild
  pub with_counts: Option<bool>,
}

/// Parameters for modifying a guild with [`Guild::modify`]
#[derive(Serialize, Default, Clone, Debug)]
pub struct GuildModifyOptions {
  /// Guild name
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// [Verification level](VerificationLevel)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verification_level: Option<VerificationLevel>,
  /// Default [message notification level](MessageNotificationLevel)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_message_notifications: Option<MessageNotificationLevel>,
  /// [Explicit content filter level](ExplicitContentFilterLevel)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
  /// Id for afk channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub afk_channel_id: Option<Option<Snowflake>>,
  /// afk timeout in seconds, can be set to: 60, 300, 900, 1800, 3600
  #[serde(skip_serializing_if = "Option::is_none")]
  pub afk_timeout: Option<i64>,
  /// base64 1024x1024 png/jpeg/gif image for the guild icon (can be animated gif when the server has the `ANIMATED_ICON` feature)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon: Option<Option<String>>,
  /// base64 16:9 png/jpeg image for the guild splash (when the server has the `INVITE_SPLASH` feature)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub splash: Option<Option<String>>,
  /// base64 16:9 png/jpeg image for the guild discovery splash (when the server has the `DISCOVERABLE` feature)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub discovery_splash: Option<Option<String>>,
  /// base64 16:9 png/jpeg image for the guild banner (when the server has the `BANNER` feature; can be animated gif when the server has the `ANIMATED_BANNER` feature)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub banner: Option<Option<String>>,
  /// The id of the channel where guild notices such as welcome messages and boost events are posted
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_channel_id: Option<Option<Snowflake>>,
  /// [System channel flags](SystemChannelFlags)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_channel_flags: Option<SystemChannelFlags>,
  /// The id of the channel where Community guilds display rules and/or guidelines
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rules_channel_id: Option<Option<Snowflake>>,
  /// The id of the channel where admins and moderators of Community guilds receive notices from Discord
  #[serde(skip_serializing_if = "Option::is_none")]
  pub public_updates_channel_id: Option<Option<Snowflake>>,
  /// The preferred [locale](https://docs.discord.com/developers/reference#locales) of a Community guild used in server discovery and notices from Discord; defaults to “en-US”
  #[serde(skip_serializing_if = "Option::is_none")]
  pub preferred_locale: Option<Option<String>>,
  /// Enabled guild features
  #[serde(skip_serializing_if = "Option::is_none")]
  pub features: Option<Vec<String>>,
  /// The description for the guild
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<Option<String>>,
  /// Whether the guild’s boost progress bar should be enabled
  #[serde(skip_serializing_if = "Option::is_none")]
  pub premium_progress_bar_enabled: Option<bool>,
  /// The id of the channel where admins and moderators of Community guilds receive safety alerts from Discord
  #[serde(skip_serializing_if = "Option::is_none")]
  pub safety_alerts_channel_id: Option<Option<Snowflake>>,
}

/// Options for modifying channel positions with [`modify_channel_positions`](Guild::modify_channel_positions)
#[derive(Serialize, Clone, Debug)]
pub struct GuildChannelModifyPositionOptions {
  /// Channel id
  pub id: Snowflake,
  /// Sorting position of the channel (channels with the same position are sorted by id)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub position: Option<i64>,
  /// Syncs the permission overwrites with the new parent, if moving to a new category
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lock_permissions: Option<bool>,
  /// The new parent ID for the channel that is moved
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<Snowflake>,
}

/// Response from fetching active threads
#[derive(Deserialize, Clone, Debug)]
pub struct GuildListThreadsResponse {
  /// The active threads
  pub threads: Vec<Channel>,
  /// A thread member object for each returned thread the current user has joined
  pub members: Vec<ThreadMember>,
}

/// Options for listing guild members with [`list_members`](Guild::list_members)
#[derive(Serialize, Clone, Debug)]
pub struct GuildMemberListOptions {
  /// max number of members to return (1-1000); default 1
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<i64>,
  /// The highest user id in the previous page
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<Snowflake>,
}

/// Options for adding a guild member with [`add_member`](Guild::add_member)
#[derive(Serialize, Clone, Debug)]
pub struct GuildMemberAddOptions {
  /// An oauth2 access token granted with the `guilds.join` to the bot’s application for the user you want to add to the guild
  pub access_token: String,
  /// Value to set user’s nickname to
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nick: Option<String>,
  /// Array of role ids the member is assigned
  #[serde(skip_serializing_if = "Option::is_none")]
  pub roles: Option<Vec<Snowflake>>,
  /// Whether the user is muted in voice channels
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mute: Option<bool>,
  /// Whether the user is deafened in voice channels
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deaf: Option<bool>,
}

impl Guild {
  /// Fetch a guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let options = GuildFetchOptions::new().set_with_counts(true);
  /// let guild = Guild::fetch(&input.rest, "613425648685547541", options).await?;
  /// # }
  /// ```
  pub async fn fetch<T: ToString>(rest: &Rest, guild_id: T, options: GuildFetchOptions) -> Result<Self, RestError> {
    rest.get_query(format!("guilds/{}", guild_id.to_string()), options).await
  }

  /// Get a preview for a guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::Guild;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let preview = Guild::get_preview(&input.rest, "613425648685547541").await?;
  /// # }
  /// ```
  pub async fn get_preview<T: ToString>(rest: &Rest, guild_id: T) -> Result<GuildPreview, RestError> {
    rest.get(format!("guilds/{}/preview", guild_id.to_string())).await
  }

  /// Modify a guild's settings
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, GuildModifyOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = GuildModifyOptions::new().set_name("Cool server");
  /// let modified_guild = guild.modify(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn modify(&self, rest: &Rest, options: GuildModifyOptions) -> Result<Self, RestError> {
    rest.patch(format!("guilds/{}", self.id), options).await
  }

  /// Get the channels in the guild. Does not include threads
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let channels = guild.get_channels(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_channels(&self, rest: &Rest) -> Result<Vec<Channel>, RestError> {
    rest.get(format!("guilds/{}/channels", self.id)).await
  }

  /// Create a new channel in this guild\
  /// See also [`Channel::create`]
  pub async fn create_channel(&self, rest: &Rest, options: ChannelCreateOptions) -> Result<Channel, RestError> {
    Channel::create(rest, &self.id, options).await
  }

  /// Modify the positions of channels
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, GuildChannelModifyPositionOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = GuildChannelModifyPositionOptions::new("1130595287078015027")
  ///   .set_position(1)
  ///   .set_parent_id("696891020146638868");
  /// let modified_guild = guild.modify_channel_positions(&input.rest, vec![options]).await?;
  /// # }
  /// ```
  pub async fn modify_channel_positions(&self, rest: &Rest, options: Vec<GuildChannelModifyPositionOptions>) -> Result<(), RestError> {
    rest.patch(format!("guilds/{}/channels", self.id), options).await
  }

  /// List active threads in guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let threads = guild.list_active_threads(&input.rest).await?;
  /// # }
  /// ```
  pub async fn list_active_threads(&self, rest: &Rest) -> Result<GuildListThreadsResponse, RestError> {
    rest.get(format!("guilds/{}/threads/active", self.id)).await
  }

  /// Get a member in the guild\
  /// See also [`GuildMember::fetch`]
  pub async fn get_member<T: ToString>(&self, rest: &Rest, user_id: T) -> Result<GuildMember, RestError> {
    GuildMember::fetch(rest, &self.id, user_id).await
  }

  /// List members in the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, GuildMemberListOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = GuildMemberListOptions::new().set_limit(50);
  /// let members = guild.list_members(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn list_members(&self, rest: &Rest, options: GuildMemberListOptions) -> Result<Vec<GuildMember>, RestError> {
    rest.get_query(format!("guilds/{}/members", self.id), options).await
  }

  /// Add a member to the guild. Requires prior oauth2 authorization
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, GuildMemberAddOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// # let access_token = String::from("somewhere_prior");
  /// let options = GuildMemberAddOptions::new(access_token).set_nick("Noob");
  /// let member = guild.add_member(&input.rest, "933795693162799156", options).await?;
  /// # }
  /// ```
  pub async fn add_member<T: ToString>(&self, rest: &Rest, user_id: T, options: GuildMemberAddOptions) -> Result<GuildMember, RestError> {
    rest.put(format!("guilds/{}/members/{}", self.id, user_id.to_string()), options).await
  }
}

impl GuildFetchOptions {
  /// Creates a new empty `GuildFetchOptions`
  pub fn new() -> Self {
    Self {
      with_counts: None
    }
  }

  /// Choose whether you want approximate member and presence counts included
  pub fn set_with_counts(mut self, with_counts: bool) -> Self {
    self.with_counts = Some(with_counts);
    self
  }
}

impl GuildModifyOptions {
  /// Creates a new empty `GuildModifyOptions`
  pub fn new() -> Self {
    Self {
      name: None,
      verification_level: None,
      default_message_notifications: None,
      explicit_content_filter: None,
      afk_channel_id: None,
      afk_timeout: None,
      icon: None,
      splash: None,
      discovery_splash: None,
      banner: None,
      system_channel_id: None,
      system_channel_flags: None,
      rules_channel_id: None,
      public_updates_channel_id: None,
      preferred_locale: None,
      features: None,
      description: None,
      premium_progress_bar_enabled: None,
      safety_alerts_channel_id: None,
    }
  }

  /// Set the name
  pub fn set_name<T: ToString>(mut self, name: T) -> Self {
    self.name = Some(name.to_string());
    self
  }

  /// Set the verification level
  pub fn set_verification_level(mut self, verification_level: VerificationLevel) -> Self {
    self.verification_level = Some(verification_level);
    self
  }

  /// Set the default message notification level
  pub fn set_default_message_notifications(mut self, default_message_notifications: MessageNotificationLevel) -> Self {
    self.default_message_notifications = Some(default_message_notifications);
    self
  }

  /// Set the explicit content filter
  pub fn set_explicit_content_filter(mut self, explicit_content_filter: ExplicitContentFilterLevel) -> Self {
    self.explicit_content_filter = Some(explicit_content_filter);
    self
  }

  /// Set the afk channel ID
  pub fn set_afk_channel_id<T: ToString>(mut self, afk_channel_id: Option<T>) -> Self {
    self.afk_channel_id = Some(afk_channel_id.map(|t| t.to_string()));
    self
  }

  /// Set the afk timeout
  pub fn set_afk_timeout(mut self, afk_timeout: i64) -> Self {
    self.afk_timeout = Some(afk_timeout);
    self
  }

  /// Set the icon
  pub fn set_icon<T: ToString>(mut self, icon: Option<T>) -> Self {
    self.icon = Some(icon.map(|t| t.to_string()));
    self
  }

  /// Set the splash
  pub fn set_splash<T: ToString>(mut self, splash: Option<T>) -> Self {
    self.splash = Some(splash.map(|t| t.to_string()));
    self
  }

  /// Set the discovery splash
  pub fn set_discovery_splash<T: ToString>(mut self, discovery_splash: Option<T>) -> Self {
    self.discovery_splash = Some(discovery_splash.map(|t| t.to_string()));
    self
  }

  /// Set the banner
  pub fn set_banner<T: ToString>(mut self, banner: Option<T>) -> Self {
    self.banner = Some(banner.map(|t| t.to_string()));
    self
  }

  /// Set the system channel ID
  pub fn set_system_channel_id<T: ToString>(mut self, system_channel_id: Option<T>) -> Self {
    self.system_channel_id = Some(system_channel_id.map(|t| t.to_string()));
    self
  }

  /// Set the system channel flags
  pub fn set_system_channel_flags(mut self, flags: SystemChannelFlags) -> Self {
    self.system_channel_flags = Some(flags);
    self
  }

  /// Set the rules channel ID
  pub fn set_rules_channel_id<T: ToString>(mut self, rules_channel_id: Option<T>) -> Self {
    self.rules_channel_id = Some(rules_channel_id.map(|t| t.to_string()));
    self
  }

  /// Set the public updates channel ID
  pub fn set_public_updates_channel_id<T: ToString>(mut self, public_updates_channel_id: Option<T>) -> Self {
    self.public_updates_channel_id = Some(public_updates_channel_id.map(|t| t.to_string()));
    self
  }

  /// Set the preferred locale
  pub fn set_preferred_locale<T: ToString>(mut self, preferred_locale: Option<T>) -> Self {
    self.preferred_locale = Some(preferred_locale.map(|t| t.to_string()));
    self
  }

  /// Set the features
  pub fn set_features(mut self, features: Vec<String>) -> Self {
    self.features = Some(features);
    self
  }

  /// Set the description
  pub fn set_description<T: ToString>(mut self, description: Option<T>) -> Self {
    self.description = Some(description.map(|t| t.to_string()));
    self
  }

  /// Set whether the premium progress bar is enabled
  pub fn set_premium_progress_bar_enabled(mut self, enabled: bool) -> Self {
    self.premium_progress_bar_enabled = Some(enabled);
    self
  }

  /// Set the safety alerts channel ID
  pub fn set_safety_alerts_channel_id<T: ToString>(mut self, safety_alerts_channel_id: Option<T>) -> Self {
    self.safety_alerts_channel_id = Some(safety_alerts_channel_id.map(|t| t.to_string()));
    self
  }
}

impl GuildChannelModifyPositionOptions {
  /// Creates a new `GuildChannelModifyPositionOptions` with a channel id
  pub fn new<T: ToString>(id: T) -> Self {
    Self {
      id: id.to_string(),
      position: None,
      lock_permissions: None,
      parent_id: None
    }
  }

  /// Set the position
  pub fn set_position(mut self, position: i64) -> Self {
    self.position = Some(position);
    self
  }

  /// Set whether permissions are locked
  pub fn set_lock_permissions(mut self, lock: bool) -> Self {
    self.lock_permissions = Some(lock);
    self
  }

  /// Set the parent ID
  pub fn set_parent_id<T: ToString>(mut self, parent_id: T) -> Self {
    self.parent_id = Some(parent_id.to_string());
    self
  }
}

impl GuildMemberListOptions {
  /// Creates a new empty `GuildMemberListOptions`
  pub fn new() -> Self {
    Self {
      limit: None,
      after: None
    }
  }

  /// Set the limit
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }

  /// Set the after
  pub fn set_after(mut self, after: Snowflake) -> Self {
    self.after = Some(after);
    self
  }
}

impl GuildMemberAddOptions {
  /// Creates a new `GuildMemberAddOptions` with an `access_token`
  pub fn new(access_token: String) -> Self {
    Self {
      access_token,
      nick: None,
      roles: None,
      mute: None,
      deaf: None,
    }
  }

  /// Set the nickname
  pub fn set_nick<T: ToString>(mut self, nick: T) -> Self {
    self.nick = Some(nick.to_string());
    self
  }

  /// Add a role
  pub fn add_role<T: ToString>(mut self, role_id: T) -> Self {
    let mut roles = self.roles.unwrap_or_default();
    roles.push(role_id.to_string());
    self.roles = Some(roles);
    self
  }

  /// Set mute
  pub fn set_mute(mut self, mute: bool) -> Self {
    self.mute = Some(mute);
    self
  }

  /// Set deaf
  pub fn set_deaf(mut self, deaf: bool) -> Self {
    self.deaf = Some(deaf);
    self
  }
}

impl Default for GuildMemberListOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl<'de> Deserialize<'de> for SystemChannelFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}

impl Serialize for SystemChannelFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}
