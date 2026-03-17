// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord guilds

use std::collections::HashMap;
use serde::{Deserialize, Serialize, ser::Serializer, de::Deserializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use chrono::{DateTime, Utc};
use bitflags::bitflags;

use super::{
  channels::{Channel, ChannelCreateOptions, ThreadMember},
  Emoji,
  invites::{Invite, VanityUrlInvite},
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
  /// The welcome screen of a Community guild, shown to new members, returned in an [Invite]'s guild object
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
#[derive(Serialize, Deserialize, Clone, Debug)]
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

/// Discord Guild Widget Settings Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildWidgetSettings {
  /// Whether the widget is enabled
  pub enabled: bool,
  /// The widget channel id
  pub channel_id: Option<Snowflake>,
}

/// Discord Guild Widget Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildWidget {
  /// Guild id
  pub id: Snowflake,
  /// Guild name (2-100 characters)
  pub name: Snowflake,
  /// Instant invite for the guilds specified widget invite channel
  pub instant_invite: Option<String>,
  /// Voice and stage channels which are accessible by @everyone
  pub channels: Vec<GuildWidgetChannel>,
  /// Special widget user objects that includes users presence (Limit 100)
  pub members: Vec<GuildWidgetUser>,
  /// Number of online members in this guild
  pub presence_count: i64,
}

/// Special widget channel object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildWidgetChannel {
  /// The id of this channel
  pub id: Snowflake,
  /// The name of the channel (1-100 characters)
  pub name: Option<String>,
  /// Sorting position of the channel
  pub position: Option<i64>,
}

/// Special widget user object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildWidgetUser {
  /// The user's id (anonymized)
  pub id: Snowflake,
  /// The user's username
  pub username: String,
  /// The user's Discord-tag (anonymized)
  pub discriminator: String,
  /// The user's avatar (anonymized)
  pub avatar: Option<String>,
  /// User's status
  pub status: Option<String>,
  /// User's avatar url
  pub avatar_url: Option<String>,
  /// Whether the user is deafened in voice channels
  pub deaf: Option<bool>,
  /// Whether the user is muted in voice channels
  pub mute: Option<bool>,
  /// Whether the user has self deafened in voice channels
  pub self_deaf: Option<bool>,
  /// Whether the user has self muted in voice channels
  pub self_mute: Option<bool>,
  /// Whether the user is suppressed
  pub suppress: Option<bool>,
  /// Voice channel the user is in
  pub channel_id: Option<Snowflake>,
}

/// Discord Integration Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildIntegration {
  /// Integration id
  pub id: Snowflake,
  /// Integration name
  pub name: String,
  /// Integration type (twitch, youtube, discord, or guild_subscription)
  pub integration_type: String,
  /// Is this integration enabled
  pub enabled: bool,
  /// Is this integration syncing
  pub syncing: Option<bool>,
  /// Id that this integration uses for “subscribers”
  pub role_id: Option<Snowflake>,
  /// Whether emoticons should be synced for this integration (twitch only currently)
  pub enable_emoticons: Option<bool>,
  /// The behavior of expiring subscribers
  pub expire_behavior: Option<GuildIntegrationExpireBehavior>,
  /// The grace period (in days) before expiring subscribers
  pub expire_grace_period: Option<i64>,
  /// User for this integration
  pub user: Option<User>,
  /// Integration account information
  pub account: GuildIntegrationAccount,
  /// When this integration was last synced
  pub synced_at: Option<DateTime<Utc>>,
  /// How many subscribers this integration has
  pub subscriber_count: Option<i64>,
  /// Has this integration been revoked
  pub revoked: Option<bool>,
  /// The bot/OAuth2 application for discord integrations
  pub application: Option<GuildIntegrationApplication>,
  /// The scopes the application has been authorized for
  pub scopes: Option<Vec<String>>,
}

/// Discord Integration Expire Behaviors
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum GuildIntegrationExpireBehavior {
  /// Remove role
  REMOVE_ROLE = 0,
  /// Kick
  KICK = 1,
  /// Unknown expire behavior not implemented yet
  UNKNOWN,
}

/// Discord Integration Account Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildIntegrationAccount {
  /// Id of the account
  pub id: String,
  /// Name of the account
  pub name: String,
}

/// Discord Integration Application Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildIntegrationApplication {
  /// The id of the app
  pub id: String,
  /// The name of the app
  pub name: String,
  /// The [icon hash](https://docs.discord.com/developers/reference#image-formatting) of the app
  pub icon: Option<String>,
  /// The description of the app
  pub description: String,
  /// The bot associated with this application
  pub bot: Option<User>,
}

/// Discord Ban Object
#[derive(Deserialize, Clone, Debug)]
pub struct Ban {
  /// The reason for the ban
  pub reason: Option<String>,
  /// The banned user
  pub user: User,
}

/// Discord Guild Onboarding Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildOnboarding {
  /// ID of the guild this onboarding is part of
  pub guild_id: Snowflake,
  /// Prompts shown during onboarding and in customize community
  pub prompts: Vec<GuildOnboardingPrompt>,
  /// Channel IDs that members get opted into automatically
  pub default_channel_ids: Vec<Snowflake>,
  /// Whether onboarding is enabled in the guild
  pub enabled: bool,
  /// Current mode of onboarding
  pub mode: GuildOnboardingMode,
}

/// Discord Guild Onboarding Prompt Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildOnboardingPrompt {
  /// ID of the prompt
  pub id: Snowflake,
  /// Type of prompt
  #[serde(rename = "type")]
  pub prompt_type: GuildOnboardingPromptType,
  /// Options available within the prompt
  pub options: Vec<GuildOnboardingPromptOption>,
  /// Title of the prompt
  pub title: String,
  /// Indicates whether users are limited to selecting one option for the prompt
  pub single_select: bool,
  /// Indicates whether the prompt is required before a user completes the onboarding flow
  pub required: bool,
  /// Indicates whether the prompt is present in the onboarding flow. If `false`, the prompt will only appear in the Channels & Roles tab
  pub in_onboarding: bool,
}

/// Discord Guild Onboarding Prompt Types
#[derive(Serialize_repr, Deserialize_repr, Default, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum GuildOnboardingPromptType {
  /// Multiple choice
  #[default]
  MULTIPLE_CHOICE = 0,
  /// Dropdown
  DROPDOWN = 1,
  /// Prompt type that hasn't been implemented yet
  UNKNOWN,
}

/// Discord Guild Onboarding Prompt Option Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildOnboardingPromptOption {
  /// ID of the prompt option
  pub id: Snowflake,
  /// IDs for channels a member is added to when the option is selected
  pub channel_ids: Vec<Snowflake>,
  /// IDs for roles assigned to a member when the option is selected
  pub role_ids: Vec<Snowflake>,
  /// Emoji of the option (from the API)
  pub emoji: Option<Emoji>,
  /// Emoji ID of the option (Used when creating)
  pub emoji_id: Option<Snowflake>,
  /// Emoji name of the option (Used when creating)
  pub emoji_name: Option<String>,
  /// Whether the emoji is animated (Used when creating)
  pub emoji_animated: Option<bool>,
  /// Title of the option
  pub title: String,
  /// Description of the option
  pub description: Option<String>,
}

/// Discord Guild Onboarding Modes
#[derive(Serialize_repr, Deserialize_repr, Default, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum GuildOnboardingMode {
  /// Counts only Default Channels towards constraints
  #[default]
  ONBOARDING_DEFAULT = 0,
  /// Counts Default Channels and Questions towards constraints
  ONBOARDING_ADVANCED = 1,
  /// Onboarding mode that hasn't been implemented yet
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

/// Options for modifying role positions with [`modify_role_positions`](Guild::modify_role_positions)
#[derive(Serialize, Clone, Debug)]
pub struct GuildRoleModifyPositionOptions {
  /// Role
  pub id: Snowflake,
  /// Sorting position of the role (roles with the same position are sorted by id)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub position: Option<i64>,
}

/// Options for fetching bans with [`get_bans`](Guild::get_bans)
#[derive(Serialize, Clone, Debug)]
pub struct BanFetchOptions {
  /// Number of users to return (up to maximum 1000); Defaults to 1000
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<i64>,
  /// Consider only users before given user id
  #[serde(skip_serializing_if = "Option::is_none")]
  pub before: Option<Snowflake>,
  /// Consider only users after given user id
  #[serde(skip_serializing_if = "Option::is_none")]
  pub after: Option<Snowflake>,
}

/// Options for bulk banning with [`bulk_ban`](Guild::bulk_ban)
#[derive(Serialize, Clone, Debug)]
pub struct BulkBanOptions {
  /// list of user ids to ban (max 200)
  pub user_ids: Vec<Snowflake>,
  /// Number of seconds to delete messages for, between 0 and 604800 (7 days)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub delete_message_seconds: Option<i64>,
}

/// Response from bulk banning with [`bulk_ban`](Guild::bulk_ban)
#[derive(Deserialize, Clone, Debug)]
pub struct BulkBanResponse {
  /// List of user ids, that were successfully banned
  pub banned_users: Vec<Snowflake>,
  /// List of user ids, that were not banned
  pub failed_users: Vec<Snowflake>,
}

/// Options for checking prune count with [`get_prune_count`](Guild::get_prune_count)
#[derive(Serialize, Clone, Debug)]
pub struct PruneCountOptions {
  /// Number of days to count prune for (1-30); Defaults to 7
  #[serde(skip_serializing_if = "Option::is_none")]
  pub days: Option<i64>,
  /// Role(s) to include
  #[serde(serialize_with = "comma_separated_vec", skip_serializing_if = "Option::is_none")]
  pub include_roles: Option<Vec<Snowflake>>,
}

/// Options for performing a prune with [`prune`](Guild::prune)
#[derive(Serialize, Clone, Debug)]
pub struct PruneOptions {
  /// Number of days to count prune for (1-30); Defaults to 7
  #[serde(skip_serializing_if = "Option::is_none")]
  pub days: Option<i64>,
  /// Whether `pruned` is returned, discouraged for large guilds
  #[serde(skip_serializing_if = "Option::is_none")]
  pub compute_prune_count: Option<bool>,
  /// Role(s) to include
  #[serde(skip_serializing_if = "Option::is_none")]
  pub include_roles: Option<Vec<Snowflake>>
}

/// Response from checking or performing a prune with [`get_prune_count`](Guild::get_prune_count) or [`prune`](Guild::prune)
#[derive(Deserialize, Clone, Debug)]
pub struct PruneResponse {
  /// Number of members that were/would be removed in the prune operation.
  /// Can be `None` if `compute_prune_count` is set to `false` when executing the prune
  pub pruned: Option<i64>,
}

/// Options for modifying the widget with [`modify_widget`](Guild::modify_widget)
#[derive(Serialize, Clone, Debug)]
pub struct GuildWidgetModifyOptions {
  /// Whether the widget is enabled
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  /// The widget channel id
  #[serde(skip_serializing_if = "Option::is_none")]
  pub channel_id: Option<Option<Snowflake>>,
}

/// Options for modifying the welcome screen with [`modify_welcome_screen`](Guild::modify_welcome_screen)
#[derive(Serialize, Clone, Debug)]
pub struct WelcomeScreenModifyOptions {
  /// Whether the welcome screen is enabled
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  /// Channels linked in the welcome screen and their display options
  #[serde(skip_serializing_if = "Option::is_none")]
  pub welcome_channels: Option<Option<Vec<WelcomeScreenChannel>>>,
  /// The server description to show in the welcome screen
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<Option<String>>,
}

/// Options for modifying guild onboarding with [`modify_onboarding`](Guild::modify_onboarding)
#[derive(Serialize, Clone, Debug)]
pub struct GuildOnboardingModifyOptions {
  /// Prompts shown during onboarding and in customize community
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prompts: Option<Vec<GuildOnboardingPrompt>>,
  /// Channel IDs that members get opted into automatically
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_channel_ids: Option<Vec<Snowflake>>,
  /// Whether onboarding is enabled in the guild
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  /// Current mode of onboarding
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mode: Option<GuildOnboardingMode>,
}

/// Options for modifying incident actions with [`modify_incident_actions`](Guild::modify_incident_actions)
#[derive(Serialize, Clone, Debug)]
pub struct GuildIncidentActionsModifyOptions {
  /// When invites will be enabled again
  #[serde(skip_serializing_if = "Option::is_none")]
  pub invites_disabled_until: Option<Option<DateTime<Utc>>>,
  /// When direct messages will be enabled again
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dms_disabled_until: Option<Option<DateTime<Utc>>>,
}

fn comma_separated_vec<S: Serializer>(vec: &Option<Vec<String>>, s: S) -> Result<S::Ok, S::Error> {
  let Some(vec) = vec else {
    return s.serialize_none()
  };
  s.serialize_str(&vec.join(","))
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
  /// guild.modify_channel_positions(&input.rest, vec![options]).await?;
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

  /// Get bans in the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, BanFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = BanFetchOptions::new().set_limit(5);
  /// let bans = guild.get_bans(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn get_bans(&self, rest: &Rest, options: BanFetchOptions) -> Result<Vec<Ban>, RestError> {
    rest.get_query(format!("guilds/{}/bans", self.id), options).await
  }

  /// Get a ban in the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let ban = guild.get_ban(&input.rest, "933795693162799156").await?;
  /// # }
  /// ```
  pub async fn get_ban<T: ToString>(&self, rest: &Rest, user_id: T) -> Result<Ban, RestError> {
    rest.get(format!("guilds/{}/bans/{}", self.id, user_id.to_string())).await
  }

  /// Bulk ban members from the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, BulkBanOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = BulkBanOptions::new()
  ///   .add_user("933795693162799156")
  ///   .add_user("545364944258990091")
  ///   .add_user("520953716610957312")
  ///   .set_delete_message_seconds(86400);
  /// let result = guild.bulk_ban(&input.rest, options, Some("spam")).await?;
  /// # }
  /// ```
  pub async fn bulk_ban<T: ToString>(&self, rest: &Rest, options: BulkBanOptions, reason: Option<T>) -> Result<BulkBanResponse, RestError> {
    let route = format!("guilds/{}/bulk-ban", self.id);

    if let Some(reason) = reason {
      rest.post_reason(route, options, reason).await
    } else {
      rest.post(route, options).await
    }
  }

  /// Get roles in the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let roles = guild.get_roles(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_roles(&self, rest: &Rest) -> Result<Vec<Role>, RestError> {
    rest.get(format!("guilds/{}/roles", self.id)).await
  }

  /// Get a role in the guild.\
  /// See also [`Role::fetch`]
  pub async fn get_role<T: ToString>(&self, rest: &Rest, role_id: T) -> Result<Role, RestError> {
    Role::fetch(rest, &self.id, role_id).await
  }

  /// Get member counts for the roles in the guild. Does not include @everyone
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let counts = guild.get_role_member_counts(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_role_member_counts(&self, rest: &Rest) -> Result<HashMap<Snowflake, i64>, RestError> {
    rest.get(format!("guilds/{}/roles/member-counts", self.id)).await
  }

  /// Modify the positions of roles
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, GuildRoleModifyPositionOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = GuildRoleModifyPositionOptions::new("936746847437983786")
  ///   .set_position(1);
  /// let modified_roles = guild.modify_role_positions(&input.rest, vec![options]).await?;
  /// # }
  /// ```
  pub async fn modify_role_positions(&self, rest: &Rest, options: Vec<GuildRoleModifyPositionOptions>) -> Result<Vec<Role>, RestError> {
    rest.patch(format!("guilds/{}/roles", self.id), options).await
  }

  /// Check how many members would get pruned
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, PruneCountOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = PruneCountOptions::new().set_days(1);
  /// let result = guild.get_prune_count(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn get_prune_count(&self, rest: &Rest, options: PruneCountOptions) -> Result<PruneResponse, RestError> {
    rest.get_query(format!("guilds/{}/prune", self.id), options).await
  }

  /// Prune members from the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, PruneCountOptions, PruneOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = PruneCountOptions::new().set_days(1);
  /// let options2 = PruneOptions::from(options.clone()).set_compute_prune_count(false);
  /// let result = guild.get_prune_count(&input.rest, options).await?;
  /// if (result.pruned.unwrap() < 100) {
  ///   guild.prune(&input.rest, options2, Some("Inactivity")).await?;
  /// }
  /// # }
  /// ```
  pub async fn prune<T: ToString>(&self, rest: &Rest, options: PruneOptions, reason: Option<T>) -> Result<PruneResponse, RestError> {
    let route = format!("guilds/{}/prune", self.id);

    if let Some(reason) = reason {
      rest.post_reason(route, options, reason).await
    } else {
      rest.post(route, options).await
    }
  }

  /// Get invites in the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let invites = guild.get_invites(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_invites(&self, rest: &Rest) -> Result<Vec<Invite>, RestError> {
    rest.get(format!("guilds/{}/invites", self.id)).await
  }

  /// Get integrations in the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let integrations = guild.get_integrations(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_integrations(&self, rest: &Rest) -> Result<Vec<GuildIntegration>, RestError> {
    rest.get(format!("guilds/{}/integrations", self.id)).await
  }

  /// Deletes an integration from the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// # let integrations = guild.get_integrations(&input.rest).await?;
  /// # let integration = integrations.first().unwrap().clone();
  /// guild.delete_integration(&input.rest, integration.id).await?;
  /// # }
  /// ```
  pub async fn delete_integration<T: ToString>(&self, rest: &Rest, integration_id: T) -> Result<(), RestError> {
    rest.delete(format!("guilds/{}/integrations/{}", self.id, integration_id.to_string())).await
  }

  /// Get the widget settings for the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let widget_settings = guild.get_widget_settings(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_widget_settings(&self, rest: &Rest) -> Result<GuildWidgetSettings, RestError> {
    rest.get(format!("guilds/{}/widget", self.id)).await
  }

  /// Modify the widget settings for the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, GuildWidgetModifyOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = GuildWidgetModifyOptions::new()
  ///   .set_enabled(true)
  ///   .set_channel_id(Some("613430047285706767"));
  /// let new_settings = guild.modify_widget(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn modify_widget(&self, rest: &Rest, options: GuildWidgetModifyOptions) -> Result<GuildWidgetSettings, RestError> {
    rest.patch(format!("guilds/{}/widget", self.id), options).await
  }

  /// Get widget data for a guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::Guild;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let widget = Guild::get_widget(&input.rest, "81384788765712384").await?;
  /// # }
  /// ```
  pub async fn get_widget<T: ToString>(rest: &Rest, guild_id: T) -> Result<GuildWidget, RestError> {
    rest.get(format!("guilds/{}/widget.json", guild_id.to_string())).await
  }

  /// Get vanity url metadata
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let vanity_url = guild.get_vanity_url(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_vanity_url(&self, rest: &Rest) -> Result<VanityUrlInvite, RestError> {
    rest.get(format!("guilds/{}/vanity-url", self.id)).await
  }

  /// Get the welcome screen for the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let welcome_screen = guild.get_welcome_screen(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_welcome_screen(&self, rest: &Rest) -> Result<WelcomeScreen, RestError> {
    rest.get(format!("guilds/{}/welcome-screen", self.id)).await
  }

  /// Modify the guild's welcome screen
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, WelcomeScreenChannel, WelcomeScreenModifyOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let channels = vec![WelcomeScreenChannel::new("697138785317814292", "Announcements")
  ///   .set_emoji_name("📡")
  /// ];
  /// let options = WelcomeScreenModifyOptions::new()
  ///   .set_enabled(true)
  ///   .set_welcome_channels(Some(channels))
  ///   .set_description(Some("A fun server"));
  /// let welcome_screen = guild.modify_welcome_screen(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn modify_welcome_screen(&self, rest: &Rest, options: WelcomeScreenModifyOptions) -> Result<WelcomeScreen, RestError> {
    rest.patch(format!("guilds/{}/welcome-screen", self.id), options).await
  }

  /// Get the onboarding for the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let onboarding = guild.get_onboarding(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_onboarding(&self, rest: &Rest) -> Result<GuildOnboarding, RestError> {
    rest.get(format!("guilds/{}/onboarding", self.id)).await
  }

  /// Modify the onboarding for the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::Emoji;
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, GuildOnboardingModifyOptions, GuildOnboardingPrompt, GuildOnboardingPromptOption, GuildOnboardingPromptType, GuildOnboardingMode};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = GuildOnboardingModifyOptions::new()
  ///   .add_prompt(GuildOnboardingPrompt::new()
  ///     .set_title("What do you want to do in this community?")
  ///     .add_option(GuildOnboardingPromptOption::new()
  ///       .set_title("Chat with Friends")
  ///       .set_emoji(Emoji::new_custom_emoji("1070002302032826408", "chat", false))
  ///       .add_channel_id("962007075288916001")
  ///     )
  ///     .add_option(GuildOnboardingPromptOption::new()
  ///       .set_title("Get Gud")
  ///       .set_description("We have excellent teachers!")
  ///       .set_emoji(Emoji::new_standard_emoji("😀"))
  ///       .add_role_id("982014491980083211")
  ///     )
  ///     .set_type(GuildOnboardingPromptType::MULTIPLE_CHOICE)
  ///     .set_required(true)
  ///   )
  ///   .set_default_channel_ids(vec![
  ///     "998678771706110023",
  ///     "998678693058719784",
  ///     "1070008122577518632",
  ///     "998678764340912138",
  ///     "998678704446263309",
  ///     "998678683592171602",
  ///     "998678699715067986"
  ///   ])
  ///   .set_enabled(true)
  ///   .set_mode(GuildOnboardingMode::ONBOARDING_DEFAULT);
  /// let onboarding = guild.modify_onboarding(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn modify_onboarding(&self, rest: &Rest, options: GuildOnboardingModifyOptions) -> Result<GuildOnboarding, RestError> {
    rest.put(format!("guilds/{}/onboarding", self.id), options).await
  }

  /// Modify incident actions for the guild
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::guilds::{Guild, GuildFetchOptions, GuildIncidentActionsModifyOptions};
  /// # use chrono::offset::Utc;
  /// # use std::time::Duration;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let guild = Guild::fetch(&input.rest, input.guild_id.unwrap(), GuildFetchOptions::new()).await?;
  /// let options = GuildIncidentActionsModifyOptions::new()
  ///   .set_invites_disabled_until(Some(Utc::now() + Duration::from_hours(24)))
  ///   .set_dms_disabled_until(None);
  /// let incident_data = guild.modify_incident_actions(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn modify_incident_actions(&self, rest: &Rest, options: GuildIncidentActionsModifyOptions) -> Result<GuildIncidentsData, RestError>  {
    rest.put(format!("guilds/{}/incident-actions", self.id), options).await
  }
}

impl WelcomeScreenChannel {
  /// Creates a new `WelcomeScreenChannel` with channel id and description
  pub fn new<T: ToString, U: ToString>(channel_id: T, description: U) -> Self {
    Self {
      channel_id: channel_id.to_string(),
      description: description.to_string(),
      emoji_id: None,
      emoji_name: None,
    }
  }

  /// Set channel id
  pub fn set_channel_id<T: ToString>(mut self, channel_id: T) -> Self {
    self.channel_id = channel_id.to_string();
    self
  }

  /// Set description
  pub fn set_description<T: ToString>(mut self, description: T) -> Self {
    self.description = description.to_string();
    self
  }

  /// Set emoji id
  pub fn set_emoji_id<T: ToString>(mut self, emoji: T) -> Self {
    self.emoji_id = Some(emoji.to_string());
    self
  }

  /// Set emoji name
  pub fn set_emoji_name<T: ToString>(mut self, emoji: T) -> Self {
    self.emoji_name = Some(emoji.to_string());
    self
  }
}

impl GuildOnboardingPrompt {
  /// Creates a new default `GuildOnboardingPrompt`
  pub fn new() -> Self {
    Self {
      id: Utc::now().timestamp().to_string(),
      prompt_type: GuildOnboardingPromptType::MULTIPLE_CHOICE,
      options: Vec::new(),
      title: String::new(),
      single_select: false,
      required: false,
      in_onboarding: true,
    }
  }

  /// Set the type
  pub fn set_type(mut self, prompt_type: GuildOnboardingPromptType) -> Self {
    self.prompt_type = prompt_type;
    self
  }

  /// Add an option
  pub fn add_option(mut self, option: GuildOnboardingPromptOption) -> Self {
    self.options.push(option);
    self
  }

  /// Set the options
  pub fn set_options(mut self, options: Vec<GuildOnboardingPromptOption>) -> Self {
    self.options = options;
    self
  }

  /// Set the title
  pub fn set_title<T: ToString>(mut self, title: T) -> Self {
    self.title = title.to_string();
    self
  }

  /// Set single select
  pub fn set_single_select(mut self, single_select: bool) -> Self {
    self.single_select = single_select;
    self
  }

  /// Set required
  pub fn set_required(mut self, required: bool) -> Self {
    self.required = required;
    self
  }

  /// Set in onboarding
  pub fn set_in_onboarding(mut self, in_onboarding: bool) -> Self {
    self.in_onboarding = in_onboarding;
    self
  }
}

impl GuildOnboardingPromptOption {
  /// Creates a new default `GuildOnboardingPromptOption`
  pub fn new() -> Self {
    Self {
      id: Utc::now().timestamp().to_string(),
      channel_ids: Vec::new(),
      role_ids: Vec::new(),
      emoji: None,
      emoji_id: None,
      emoji_name: None,
      emoji_animated: None,
      title: String::new(),
      description: None,
    }
  }

  /// Add a channel
  pub fn add_channel_id<T: ToString>(mut self, channel_id: T) -> Self {
    self.channel_ids.push(channel_id.to_string());
    self
  }

  /// Set the channels
  pub fn set_channel_ids(mut self, ids: Vec<Snowflake>) -> Self {
    self.channel_ids = ids;
    self
  }

  /// Add a role
  pub fn add_role_id<T: ToString>(mut self, role_id: T) -> Self {
    self.role_ids.push(role_id.to_string());
    self
  }

  /// Set the roles
  pub fn set_role_ids(mut self, ids: Vec<Snowflake>) -> Self {
    self.role_ids = ids;
    self
  }

  /// Set the emoji
  pub fn set_emoji(mut self, emoji: Emoji) -> Self {
    self.emoji_id = emoji.id.clone();
    self.emoji_name = emoji.name.clone();
    self.emoji_animated = emoji.animated;
    self.emoji = Some(emoji);
    self
  }

  /// Set the title
  pub fn set_title<T: ToString>(mut self, title: T) -> Self {
    self.title = title.to_string();
    self
  }

  /// Set the description
  pub fn set_description<T: ToString>(mut self, description: T) -> Self {
    self.description = Some(description.to_string());
    self
  }
}

impl GuildFetchOptions {
  /// Creates a new empty `GuildFetchOptions`
  pub fn new() -> Self {
    Self {
      with_counts: None,
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
      parent_id: None,
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

impl GuildRoleModifyPositionOptions {
  /// Creates a new `GuildRoleModifyPositionOptions` with a role id
  pub fn new<T: ToString>(id: T) -> Self {
    Self {
      id: id.to_string(),
      position: None,
    }
  }

  /// Set the position
  pub fn set_position(mut self, position: i64) -> Self {
    self.position = Some(position);
    self
  }
}

impl BanFetchOptions {
  /// Creates a new empty `BanFetchOptions`
  pub fn new() -> Self {
    Self {
      limit: None,
      before: None,
      after: None,
    }
  }

  /// Set the limit
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }

  /// Set before
  pub fn set_before<T: ToString>(mut self, before: T) -> Self {
    self.before = Some(before.to_string());
    self
  }

  /// Set after
  pub fn set_after<T: ToString>(mut self, after: T) -> Self {
    self.after = Some(after.to_string());
    self
  }
}

impl BulkBanOptions {
  /// Creates a new empty `BulkBanOptions`
  pub fn new() -> Self {
    Self {
      user_ids: Vec::new(),
      delete_message_seconds: None,
    }
  }

  /// Add a user to be banned
  pub fn add_user<T: ToString>(mut self, user_id: T) -> Self {
    self.user_ids.push(user_id.to_string());
    self
  }

  /// Set all user ids at once
  pub fn set_user_ids(mut self, user_ids: Vec<Snowflake>) -> Self {
    self.user_ids = user_ids;
    self
  }

  /// Set delete message seconds
  pub fn set_delete_message_seconds(mut self, seconds: i64) -> Self {
    self.delete_message_seconds = Some(seconds);
    self
  }
}

impl PruneCountOptions {
  /// Creates a new empty `PruneCountOptions`
  pub fn new() -> Self {
    Self {
      days: None,
      include_roles: None,
    }
  }

  /// Set days
  pub fn set_days(mut self, days: i64) -> Self {
    self.days = Some(days);
    self
  }

  /// Add a role to be included
  pub fn include_role<T: ToString>(mut self, role_id: T) -> Self {
    let mut roles = self.include_roles.unwrap_or_default();
    roles.push(role_id.to_string());
    self.include_roles = Some(roles);
    self
  }

  /// Set the roles to include
  pub fn set_include_roles(mut self, roles: Vec<Snowflake>) -> Self {
    self.include_roles = Some(roles);
    self
  }
}

impl PruneOptions {
  /// Creates a new empty `PruneOptions`
  pub fn new() -> Self {
    Self {
      days: None,
      compute_prune_count: None,
      include_roles: None,
    }
  }

  /// Set days
  pub fn set_days(mut self, days: i64) -> Self {
    self.days = Some(days);
    self
  }

  /// Set whether to compute the prune count
  pub fn set_compute_prune_count(mut self, compute: bool) -> Self {
    self.compute_prune_count = Some(compute);
    self
  }

  /// Add a role to be included
  pub fn include_role<T: ToString>(mut self, role_id: T) -> Self {
    let mut roles = self.include_roles.unwrap_or_default();
    roles.push(role_id.to_string());
    self.include_roles = Some(roles);
    self
  }

  /// Set the roles to include
  pub fn set_include_roles(mut self, roles: Vec<Snowflake>) -> Self {
    self.include_roles = Some(roles);
    self
  }
}

impl From<PruneCountOptions> for PruneOptions {
  fn from(value: PruneCountOptions) -> Self {
    Self {
      days: value.days,
      compute_prune_count: None,
      include_roles: value.include_roles,
    }
  }
}

impl GuildWidgetModifyOptions {
  /// Creates a new empty `GuildWidgetSettingsModifyOptions`
  pub fn new() -> Self {
    Self {
      enabled: None,
      channel_id: None,
    }
  }

  /// Set enabled
  pub fn set_enabled(mut self, enabled: bool) -> Self {
    self.enabled = Some(enabled);
    self
  }

  /// Set channel id
  pub fn set_channel_id<T: ToString>(mut self, channel_id: Option<T>) -> Self {
    self.channel_id = Some(channel_id.map(|t| t.to_string()));
    self
  }
}

impl WelcomeScreenModifyOptions {
  /// Creates a new empty `WelcomeScreenModifyOptions`
  pub fn new() -> Self {
    Self {
      enabled: None,
      welcome_channels: None,
      description: None,
    }
  }

  /// Set enabled
  pub fn set_enabled(mut self, enabled: bool) -> Self {
    self.enabled = Some(enabled);
    self
  }

  /// Set welcome channels
  pub fn set_welcome_channels(mut self, welcome_channels: Option<Vec<WelcomeScreenChannel>>) -> Self {
    self.welcome_channels = Some(welcome_channels);
    self
  }

  /// Set description
  pub fn set_description<T: ToString>(mut self, description: Option<T>) -> Self {
    self.description = Some(description.map(|t| t.to_string()));
    self
  }
}

impl GuildOnboardingModifyOptions {
  /// Creates a new empty `GuildOnboardingModifyOptions`
  pub fn new() -> Self {
    Self {
      prompts: None,
      default_channel_ids: None,
      enabled: None,
      mode: None,
    }
  }

  /// Add a prompt
  pub fn add_prompt(mut self, prompt: GuildOnboardingPrompt) -> Self {
    let mut prompts = self.prompts.unwrap_or_default();
    prompts.push(prompt);
    self.prompts = Some(prompts);
    self
  }

  /// Set the prompts
  pub fn set_prompts(mut self, prompts: Vec<GuildOnboardingPrompt>) -> Self {
    self.prompts = Some(prompts);
    self
  }

  /// Add a default channel
  pub fn add_default_channel_id<T: ToString>(mut self, channel_id: T) -> Self {
    let mut channels = self.default_channel_ids.unwrap_or_default();
    channels.push(channel_id.to_string());
    self.default_channel_ids = Some(channels);
    self
  }

  /// Set the default channels
  pub fn set_default_channel_ids<T: ToString>(mut self, ids: Vec<T>) -> Self {
    self.default_channel_ids = Some(ids.into_iter().map(|id| id.to_string()).collect());
    self
  }

  /// Set enabled
  pub fn set_enabled(mut self, enabled: bool) -> Self {
    self.enabled = Some(enabled);
    self
  }

  /// Set the onboarding mode
  pub fn set_mode(mut self, mode: GuildOnboardingMode) -> Self {
    self.mode = Some(mode);
    self
  }
}

impl GuildIncidentActionsModifyOptions {
  /// Creates a new empty `GuildIncidentActionsModifyOptions`
  pub fn new() -> Self {
    Self {
      invites_disabled_until: None,
      dms_disabled_until: None,
    }
  }

  /// Set invites disabled until
  pub fn set_invites_disabled_until(mut self, invites_disabled_until: Option<DateTime<Utc>>) -> Self {
    self.invites_disabled_until = Some(invites_disabled_until);
    self
  }

  /// Set DMs disabled until
  pub fn set_dms_disabled_until(mut self, dms_disabled_until: Option<DateTime<Utc>>) -> Self {
    self.dms_disabled_until = Some(dms_disabled_until);
    self
  }
}

impl Default for GuildOnboardingPrompt {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for GuildOnboardingPromptOption {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for GuildMemberListOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for BanFetchOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for BulkBanOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for PruneCountOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for PruneOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for GuildWidgetModifyOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for WelcomeScreenModifyOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for GuildOnboardingModifyOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for GuildIncidentActionsModifyOptions {
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
