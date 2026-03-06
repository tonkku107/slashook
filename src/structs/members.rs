// Copyright 2026 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord guild members

use serde::{Deserialize, Serialize, ser::Serializer, de::Deserializer};
use chrono::{DateTime, Utc};
use bitflags::bitflags;

use super::{
  Permissions,
  users::{User, AvatarDecorationData},
  Snowflake,
};
use crate::rest::{Rest, RestError};
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
    const DM_SETTINGS_UPSELL_ACKNOWLEDGED = 1 << 9;
    /// Member’s guild tag is blocked by AutoMod
    const AUTOMOD_QUARANTINED_GUILD_TAG = 1 << 10;
  }
}

/// Options for modifying a member with [`GuildMember::modify`]
#[derive(Serialize, Clone, Debug)]
pub struct GuildMemberModifyOptions {
  /// Value to set user’s nickname to
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nick: Option<Option<String>>,
  /// Array of role ids the member is assigned
  #[serde(skip_serializing_if = "Option::is_none")]
  pub roles: Option<Option<Vec<Snowflake>>>,
  /// Whether the user is muted in voice channels. Will throw a 400 error if the user is not in a voice channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mute: Option<bool>,
  /// Whether the user is deafened in voice channels. Will throw a 400 error if the user is not in a voice channel
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deaf: Option<bool>,
  /// ID of channel to move user to (if they are connected to voice).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub channel_id: Option<Option<Snowflake>>,
  /// When the user’s [timeout](https://support.discord.com/hc/en-us/articles/4413305239191-Time-Out-FAQ) will expire and the user will be able to communicate in the guild again (up to 28 days in the future), set to null to remove timeout. Will throw a 403 error if the user has the ADMINISTRATOR permission or is the owner of the guild
  #[serde(skip_serializing_if = "Option::is_none")]
  pub communication_disabled_until: Option<Option<DateTime<Utc>>>,
  /// [Guild member flags](GuildMemberFlags)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flags: Option<Option<GuildMemberFlags>>,
  /// [Data URI base64 encoded](https://docs.discord.com/developers/reference#image-data) banner image. Only available when modifying current member
  #[serde(skip_serializing_if = "Option::is_none")]
  pub banner: Option<Option<String>>,
  /// [Data URI base64 encoded](https://docs.discord.com/developers/reference#image-data) avatar image. Only available when modifying current member
  #[serde(skip_serializing_if = "Option::is_none")]
  pub avatar: Option<Option<String>>,
  /// Guild member bio. Only available when modifying current member
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bio: Option<Option<String>>,
}

impl GuildMember {
  /// Fetch a guild member
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::members::{GuildMember};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let member = GuildMember::fetch(&input.rest, "613425648685547541", "933795693162799156").await?;
  /// # }
  /// ```
  pub async fn fetch<T: ToString, U: ToString>(rest: &Rest, guild_id: T, user_id: U) -> Result<Self, RestError> {
    rest.get(format!("guilds/{}/members/{}", guild_id.to_string(), user_id.to_string())).await
  }

  /// Modify a guild member
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::members::{GuildMember, GuildMemberModifyOptions};
  /// # use chrono::offset::Utc;
  /// # use std::time::Duration;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let options = GuildMemberModifyOptions::new()
  ///   .set_nick(None::<String>)
  ///   .set_communication_disabled_until(Some(Utc::now() + Duration::from_hours(1)));
  /// let modified_member = GuildMember::modify(&input.rest, "613425648685547541", "933795693162799156", options, Some("Breaking the rules")).await?;
  /// # }
  /// ```
  pub async fn modify<T: ToString, U: ToString, V: ToString>(rest: &Rest, guild_id: T, user_id: U, options: GuildMemberModifyOptions, reason: Option<V>) -> Result<Self, RestError> {
    if let Some(reason) = reason {
      rest.patch_reason(format!("guilds/{}/members/{}", guild_id.to_string(), user_id.to_string()), options, reason).await
    } else {
      rest.patch(format!("guilds/{}/members/{}", guild_id.to_string(), user_id.to_string()), options).await
    }
  }

  /// Modify the bot's membership
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::members::{GuildMember, GuildMemberModifyOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let options = GuildMemberModifyOptions::new()
  ///   .set_nick(Some("Cool bot"));
  /// let modified_member = GuildMember::modify_current_member(&input.rest, "613425648685547541", options, None::<String>).await?;
  /// # }
  /// ```
  pub async fn modify_current_member<T: ToString, U: ToString>(rest: &Rest, guild_id: T, options: GuildMemberModifyOptions, reason: Option<U>) -> Result<Self, RestError> {
    Self::modify(rest, guild_id, "@me", options, reason).await
  }

  /// Kick a member
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::members::{GuildMember, GuildMemberModifyOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// GuildMember::kick(&input.rest, "613425648685547541", "933795693162799156", Some("Breaking the rules")).await?;
  /// # }
  /// ```
  pub async fn kick<T: ToString, U: ToString, V: ToString>(rest: &Rest, guild_id: T, user_id: U, reason: Option<V>) -> Result<(), RestError> {
    if let Some(reason) = reason {
      rest.delete_reason(format!("guilds/{}/members/{}", guild_id.to_string(), user_id.to_string()), reason).await
    } else {
      rest.delete(format!("guilds/{}/members/{}", guild_id.to_string(), user_id.to_string())).await
    }
  }

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

impl GuildMemberModifyOptions {
  /// Creates a new empty `GuildMemberModifyOptions`
  pub fn new() -> Self {
    Self {
      nick: None,
      roles: None,
      mute: None,
      deaf: None,
      channel_id: None,
      communication_disabled_until: None,
      flags: None,
      banner: None,
      avatar: None,
      bio: None,
    }
  }

  /// Set the nickname
  pub fn set_nick<T: ToString>(mut self, nick: Option<T>) -> Self {
    self.nick = Some(nick.map(|t| t.to_string()));
    self
  }

  /// Set roles
  pub fn set_roles(mut self, roles: Option<Vec<Snowflake>>) -> Self {
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

  /// Set channel ID
  pub fn set_channel_id<T: ToString>(mut self, channel_id: Option<T>) -> Self {
    self.channel_id = Some(channel_id.map(|t| t.to_string()));
    self
  }

  /// Set communication disabled until
  pub fn set_communication_disabled_until(mut self, communication_disabled_until: Option<DateTime<Utc>>) -> Self {
    self.communication_disabled_until = Some(communication_disabled_until);
    self
  }

  /// Set flags
  pub fn set_flags(mut self, flags: Option<GuildMemberFlags>) -> Self {
    self.flags = Some(flags);
    self
  }

  /// Set the banner
  pub fn set_banner<T: ToString>(mut self, banner: Option<T>) -> Self {
    self.banner = Some(banner.map(|t| t.to_string()));
    self
  }

  /// Set the avatar
  pub fn set_avatar<T: ToString>(mut self, avatar: Option<T>) -> Self {
    self.avatar = Some(avatar.map(|t| t.to_string()));
    self
  }

  /// Set the bio
  pub fn set_bio<T: ToString>(mut self, bio: Option<T>) -> Self {
    self.bio = Some(bio.map(|t| t.to_string()));
    self
  }
}

impl Default for GuildMemberModifyOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl<'de> Deserialize<'de> for GuildMemberFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}

impl Serialize for GuildMemberFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}
