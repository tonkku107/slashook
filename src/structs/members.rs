// Copyright 2026 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord guild members

use serde::{Deserialize, de::Deserializer};
use chrono::{DateTime, Utc};
use bitflags::bitflags;

use super::{
  Permissions,
  users::{User, AvatarDecorationData},
  Snowflake,
};
use crate::internal_utils::cdn::pick_format;

/// Discord Guild Member Object
#[derive(Deserialize, Clone, Debug)]
pub struct GuildMember {
  /// The user this guild member represents
  pub user: Option<User>,
  /// This users guild nickname
  pub nick: Option<String>,
  /// The member's [guild avatar hash](https://discord.com/developers/docs/reference#image-formatting)
  pub avatar: Option<String>,
  /// The member's [guild banner hash](https://discord.com/developers/docs/reference#image-formatting)
  pub banner: Option<String>,
  /// Array of [role](super::guilds::Role) object ids
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
  pub communication_disabled_until: Option<DateTime<Utc>>,
  /// Data for the member's guild avatar decoration
  pub avatar_decoration_data: Option<AvatarDecorationData>,
}

bitflags! {
  /// Discord Guild Member Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct GuildMemberFlags: u32 {
    /// Member has left and rejoined the guild
    const DID_REJOIN = 1 << 0;
    /// Member has completed onboarding
    const COMPLETED_ONBOARDING = 1 << 1;
    /// Member is exempt from guild verification requirements
    const BYPASSES_VERIFICATION = 1 << 2;
    /// Member has started onboarding
    const STARTED_ONBOARDING = 1 << 3;
    /// Member is a guest and can only access the voice channel they were invited to
    const IS_GUEST = 1 << 4;
    /// Member has started Server Guide new member actions
    const STARTED_HOME_ACTIONS = 1 << 5;
    /// Member has completed Server Guide new member actions
    const COMPLETED_HOME_ACTIONS = 1 << 6;
    /// Member's username, display name, or nickname is blocked by AutoMod
    const AUTOMOD_QUARANTINED_USERNAME = 1 << 7;
    /// Member has dismissed the DM settings upsell
    const DM_SETTINGS_UPSELL_ACKNOWLEDGED = 1 << 8;
  }
}

impl GuildMember {
  /// Get the url for the per-server member avatar. `None` if the member has no server-specific avatar
  pub fn avatar_url<T: ToString, U: ToString, V: ToString, W: ToString, X: ToString>(&self, guild_id: T, user_id: U, static_format: V, animated_format: Option<W>, size: X) -> Option<String> {
    self.avatar.as_deref().map(|a| {
      let (format, animated) = pick_format(a, static_format.to_string(), animated_format.map(|f| f.to_string()));
      format!("https://cdn.discordapp.com/guilds/{}/users/{}/avatars/{}.{}?size={}&animated={}", guild_id.to_string(), user_id.to_string(), a, format, size.to_string(), animated)
    })
  }

  /// Get the url for the per-server member banner. `None` if the member has no server-specific banner
  pub fn banner_url<T: ToString, U: ToString, V: ToString, W: ToString, X: ToString>(&self, guild_id: T, user_id: U, static_format: V, animated_format: Option<W>, size: X) -> Option<String> {
    self.banner.as_deref().map(|b| {
      let (format, animated) = pick_format(b, static_format.to_string(), animated_format.map(|f| f.to_string()));
      format!("https://cdn.discordapp.com/guilds/{}/users/{}/banners/{}.{}?size={}&animated={}", guild_id.to_string(), user_id.to_string(), b, format, size.to_string(), animated)
    })
  }

  /// Get the url for the member avatar that would be displayed in app.\
  /// **NOTE:** Will return `None` if `user` in the `GuildMember` is `None`. Use [`User::display_avatar_url_with_member`] if you want to make sure you get the correct avatar.
  pub fn display_avatar_url<T: ToString, U: ToString, V: ToString, W: ToString>(&self, guild_id: T, static_format: U, animated_format: Option<V>, size: W) -> Option<String> {
    let Some(user) = &self.user else {
      return None;
    };

    self.avatar_url(guild_id, &user.id, static_format.to_string(), animated_format.as_ref().map(|f| f.to_string()), size.to_string())
      .or_else(|| Some(user.display_avatar_url(static_format, animated_format, size)))
  }

  /// Get the url for the member banner that would be displayed in app. `None` if no banner is set.\
  /// **NOTE:** Will return `None` if `user` in the `GuildMember` is `None`. Use [`User::display_banner_url_with_member`] if you want to make sure you get the correct banner.
  pub fn display_banner_url<T: ToString, U: ToString, V: ToString, W: ToString>(&self, guild_id: T, static_format: U, animated_format: Option<V>, size: W) -> Option<String> {
    let Some(user) = &self.user else {
      return None;
    };

    self.banner_url(guild_id, &user.id, static_format.to_string(), animated_format.as_ref().map(|f| f.to_string()), size.to_string())
      .or_else(|| user.banner_url(static_format, animated_format, size))
  }
}

impl<'de> Deserialize<'de> for GuildMemberFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}
