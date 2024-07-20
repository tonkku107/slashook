// Copyright 2024 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord users

use serde::{Deserialize, de::Deserializer};
use serde::{Serialize, ser::Serializer};
use serde_repr::{Serialize_repr, Deserialize_repr};
use super::Snowflake;
use bitflags::bitflags;

/// Discord User Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
  /// The user's id
  pub id: Snowflake,
  /// The user's username, not unique across the platform
  pub username: String,
  /// The user's 4-digit discord-tag
  pub discriminator: String,
  /// The user's [avatar hash](https://discord.com/developers/docs/reference#image-formatting)
  pub avatar: Option<String>,
  /// Whether the user belongs to an OAuth2 application
  pub bot: Option<bool>,
  /// Whether the user is an Official Discord System user (part of the urgent message system)
  pub system: Option<bool>,
  /// Whether the user has two factor enabled on their account
  pub mfa_enabled: Option<bool>,
  /// The user's [banner hash](https://discord.com/developers/docs/reference#image-formatting)
  pub banner: Option<String>,
  /// The user's banner color encoded as an integer representation of hexadecimal color code
  pub accent_color: Option<i64>,
  /// The user's chosen language option
  pub locale: Option<String>,
  /// Whether the email on this account has been verified
  pub verified: Option<bool>,
  /// The user's email
  pub email: Option<String>,
  /// The flags on a user's account
  pub flags: Option<UserFlags>,
  /// The [type of Nitro subscription](PremiumType) on a user's account
  pub premium_type: Option<PremiumType>,
  /// The public flags on a user's account
  pub public_flags: Option<UserFlags>,
}

bitflags! {
  /// Bitflags for Discord User Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct UserFlags: u32 {
    /// Discord Employee
    const STAFF = 1 << 0;
    /// Partnered Server Owner
    const PARTNER = 1 << 1;
    /// HypeSquad Events Coordinator
    const HYPESQUAD = 1 << 2;
    /// Bug Hunter Level 1
    const BUG_HUNTER_LEVEL_1 = 1 << 3;
    /// House Bravery Member
    const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
    /// House Brilliance Member
    const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
    /// House Balance Member
    const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
    /// Early Nitro Supporter
    const PREMIUM_EARLY_SUPPORTER = 1 << 9;
    /// User is a [team](https://discord.com/developers/docs/topics/teams)
    const TEAM_PSEUDO_USER = 1 << 10;
    /// Bug Hunter Level 2
    const BUG_HUNTER_LEVEL_2 = 1 << 14;
    /// Verified Bot
    const VERIFIED_BOT = 1 << 16;
    /// Early Verified Bot Developer
    const VERIFIED_DEVELOPER = 1 << 17;
    /// Discord Certified Moderator
    const CERTIFIED_MODERATOR = 1 << 18;
    /// Bot uses only [HTTP interactions](https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction) and is shown in the online member list
    const BOT_HTTP_INTERACTIONS = 1 << 19;
    /// User is an [Active Developer](https://support-dev.discord.com/hc/articles/10113997751447)
    const ACTIVE_DEVELOPER = 1 << 22;
  }
}

/// Discord Premium Types
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PremiumType {
  /// User has no nitro
  NONE = 0,
  /// User has Nitro Classic
  NITRO_CLASSIC = 1,
  /// User has Nitro
  NITRO = 2,
  /// User has Nitro Basic
  NITRO_BASIC = 3,
  /// A premium type that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN
}

impl User {
  /// Get an avatar url for the user. None if the user has no custom avatar
  pub fn avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> Option<String> {
    self.avatar.as_ref().map(|a| format!("https://cdn.discordapp.com/avatars/{}/{}.{}?size={}", self.id, a, format.to_string(), size.to_string()))
  }

  /// Returns a string representing a user mention
  pub fn mention(&self) -> String {
    format!("<@{}>", self.id)
  }
}

impl<'de> Deserialize<'de> for UserFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}

impl Serialize for UserFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}
