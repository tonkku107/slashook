// Copyright 2022 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord guilds

use serde::{Deserialize, de::Deserializer};
use serde_repr::Deserialize_repr;
use super::{
  Snowflake,
  Emoji,
  Permissions,
  stickers::Sticker,
  users::User, utils::Color
};
use chrono::{DateTime, Utc};
use bitflags::bitflags;

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
  /// Total permissions for [the user](https://discord.com/developers/docs/resources/user#get-current-user-guilds) in the guild (excludes overwrites)
  pub permissions: Option<Permissions>,
  /// Id of afk channel
  pub afk_channel_id: Option<Snowflake>,
  /// Afk timeout in seconds, can be set to: 60, 300, 900, 1800, 3600
  pub afk_timeout: Option<i64>,
  /// True if the server widget is enabled
  pub widget_enabled: Option<bool>,
  /// The channel id that the widget will generate an invite to, or None if set to no invite
  pub widget_channel_id: Option<Snowflake>,
  /// [Verification level](VerificationLevel) required for the guild
  pub verification_level: VerificationLevel,
  /// Default [message notifications level](MessageNotificationsLevel)
  pub default_message_notifications: Option<MessageNotificationsLevel>,
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
  /// Approximate number of members in this guild, returned from the `GET /guilds/<id>` endpoint when `with_counts` is `true`
  pub approximate_member_count: Option<i64>,
  /// Approximate number of non-offline members in this guild, returned from the `GET /guilds/<id>` endpoint when `with_counts` is `true`
  pub approximate_presence_count: Option<i64>,
  /// The welcome screen of a Community guild, shown to new members, returned in an [Invite](super::invites::Invite)'s guild object
  pub welcome_screen: Option<WelcomeScreen>,
  /// [Guild NSFW level](NSFWLevel)
  pub nsfw_level: NSFWLevel,
  /// Custom guild stickers
  pub stickers: Option<Sticker>,
  /// Whether the guild has the boost progress bar enabled
  pub premium_progress_bar_enabled: Option<bool>,
}

/// Discord Verification Levels
#[derive(Deserialize_repr, Clone, Debug)]
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
  UNKNOWN
}

/// Discord Message Notifications Level
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MessageNotificationsLevel {
  /// Members will receive notifications for all messages by default
  ALL_MESSAGES = 0,
  /// Members will receive notifications only for messages that @mention them by default
  ONLY_MENTIONS = 1,
  /// Message notifications level that hasn't been implemented yet
  UNKNOWN
}

/// Discord Explicit Content Filter Level
#[derive(Deserialize_repr, Clone, Debug)]
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
  UNKNOWN
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
  UNKNOWN
}

bitflags! {
  /// Bitflags for Discord System Channel Flags
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
  UNKNOWN
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
  UNKNOWN
}

/// Discord Guild Member Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildMember {
  /// The user this guild member represents
  pub user: Option<User>,
  /// This users guild nickname
  pub nick: Option<String>,
  /// The member's [guild avatar hash](https://discord.com/developers/docs/reference#image-formatting)
  pub avatar: Option<String>,
  /// Array of [role](Role) object ids
  pub roles: Vec<Snowflake>,
  /// When the user joined the guild
  pub joined_at: DateTime<Utc>,
  /// When the user started [boosting](https://support.discord.com/hc/en-us/articles/360028038352-Server-Boosting-) the guild
  pub premium_since: Option<DateTime<Utc>>,
  /// Whether the user is deafened in voice channels
  pub deaf: Option<bool>,
  /// Whether the user is muted in voice channels
  pub mute: Option<bool>,
  /// [Guild member flags](GuildMemberFlags) represented as a bit set, defaults to 0
  pub flags: GuildMemberFlags,
  /// Whether the user has not yet passed the guild's [Membership Screening](https://discord.com/developers/docs/resources/guild#membership-screening-object) requirements
  pub pending: Option<bool>,
  /// Total permissions of the member in the channel, including overwrites, returned when in the interaction object
  pub permissions: Option<Permissions>,
  /// When the user's [timeout](https://support.discord.com/hc/en-us/articles/4413305239191-Time-Out-FAQ) will expire and the user will be able to communicate in the guild again, None or a time in the past if the user is not timed out
  pub communication_disabled_until: Option<DateTime<Utc>>
}

bitflags! {
  /// Discord Guild Member Flags
  pub struct GuildMemberFlags: u32 {
    /// Member has left and rejoined the guild
    const DID_REJOIN = 1 << 0;
    /// Member has completed onboarding
    const COMPLETED_ONBOARDING = 1 << 1;
    /// Member is exempt from guild verification requirements
    const BYPASSES_VERIFICATION = 1 << 2;
    /// Member has started onboarding
    const STARTED_ONBOARDING = 1 << 3;
  }
}

/// Discord Role Object
#[derive(Deserialize, Clone, Debug)]
pub struct Role {
  /// Role id
  pub id: Snowflake,
  /// Role name
  pub name: String,
  /// Integer representation of hexadecimal color code
  pub color: Color,
  /// If this role is pinned in the user listing
  pub hoist: bool,
  /// Role [icon hash](https://discord.com/developers/docs/reference#image-formatting)
  pub icon: Option<String>,
  /// Role unicode emoji
  pub unicode_emoji: Option<String>,
  /// Position of this role
  pub position: i64,
  /// Permission bit set
  pub permissions: Permissions,
  /// Whether this role is managed by an integration
  pub managed: bool,
  /// Whether this role is mentionable
  pub mentionable: bool,
  /// The tags this role has
  pub tags: Option<RoleTags>
}

/// Discord Role Tags Object
#[derive(Deserialize, Clone, Debug)]
pub struct RoleTags {
  /// The id of the bot this role belongs to
  pub bot_id: Option<Snowflake>,
  /// The id of the integration this role belongs to
  pub integration_id: Option<Snowflake>,
  /// Whether this is the guild's Booster role
  #[serde(default, deserialize_with = "exists")]
  pub premium_subscriber: bool,
  /// The id of this role's subscription sku and listing
  pub subscription_listing_id: Option<Snowflake>,
  /// Whether this role is available for purchase
  #[serde(default, deserialize_with = "exists")]
  pub available_for_purchase: bool,
  /// Whether this role is a guild's linked role
  #[serde(default, deserialize_with = "exists")]
  pub guild_connections: bool,
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
}

/// Discord Guild Scheduled Event Privacy Level
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PrivacyLevel {
  /// The scheduled event is only accessible to guild members
  GUILD_ONLY = 2,
  /// Privacy level that hasn't been implemented yet
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
  UNKNOWN
}

/// Discord Guild Scheduled Event Entity Metadata
#[derive(Deserialize, Clone, Debug)]
pub struct EntityMetadata {
  /// Location of the event (1-100 characters)
  pub location: Option<String>,
}

fn exists<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
  serde_json::Value::deserialize(d)?;
  Ok(true)
}

impl<'de> Deserialize<'de> for SystemChannelFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_truncate(bits))
  }
}

impl<'de> Deserialize<'de> for GuildMemberFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_truncate(bits))
  }
}
