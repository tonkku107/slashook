// Copyright 2026 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord roles

use serde::{Deserialize, de::Deserializer};
use bitflags::bitflags;

use super::{
  Permissions,
  utils::Color,
  Snowflake,
};

/// Discord Role Object
#[derive(Deserialize, Clone, Debug)]
pub struct Role {
  /// Role id
  pub id: Snowflake,
  /// Role name
  pub name: String,
  /// Role color
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
  pub tags: Option<RoleTags>,
  /// [Role flags](RoleFlags) combined as a [bitfield](https://en.wikipedia.org/wiki/Bit_field)
  pub flags: RoleFlags,
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

bitflags! {
  /// Discord Role Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct RoleFlags: u32 {
    /// Role can be selected by members in an [onboarding](https://discord.com/developers/docs/resources/guild#guild-onboarding-object) prompt
    const IN_PROMPT = 1 << 0;
  }
}

fn exists<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
  serde_json::Value::deserialize(d)?;
  Ok(true)
}

impl<'de> Deserialize<'de> for RoleFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}
