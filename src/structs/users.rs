// Copyright 2021 slashook Developers
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
  pub struct UserFlags: u32 {
    const DISCORD_EMPLOYEE = 1 << 0;
    const PARTNERED_SERVER_OWNER = 1 << 1;
    const HYPESQUAD_EVENTS = 1 << 2;
    const BUG_HUNTER_LEVEL_1 = 1 << 3;
    const HOUSE_BRAVERY = 1 << 6;
    const HOUSE_BRILLIANCE = 1 << 7;
    const HOUSE_BALANCE = 1 << 8;
    const EARLY_SUPPORTER = 1 << 9;
    const TEAM_USER = 1 << 10;
    const BUG_HUNTER_LEVEL_2 = 1 << 14;
    const VERIFIED_BOT = 1 << 16;
    const EARLY_VERIFIED_BOT_DEVELOPER = 1 << 17;
    const DISCORD_CERTIFIED_MODERATOR = 1 << 18;
  }
}

/// Discord Premium Types
#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PremiumType {
  NONE = 0,
  NITRO_CLASSIC = 1,
  NITRO = 2,
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
    Ok(Self::from_bits_truncate(bits))
  }
}

impl Serialize for UserFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}
