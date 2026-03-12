// Copyright 2026 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord roles

use serde::{Deserialize, Serialize, de::Deserializer};
use bitflags::bitflags;

use super::{
  Permissions,
  utils::Color,
  Snowflake,
};
use crate::rest::{Rest, RestError};

/// Discord Role Object
#[derive(Deserialize, Clone, Debug)]
pub struct Role {
  /// Role id
  pub id: Snowflake,
  /// Role name
  pub name: String,
  /// The role's colors
  pub colors: RoleColors,
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

/// Discord Role Colors Object
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RoleColors {
  /// The primary color for the role
  pub primary_color: Color,
  /// The secondary color for the role, this will make the role a gradient between the other provided colors. Can only be set if the guild has the feature `ENHANCED_ROLE_COLORS`
  pub secondary_color: Option<Color>,
  /// The tertiary color for the role, this will turn the gradient into a holographic style. Can only be set if the guild has the feature `ENHANCED_ROLE_COLORS`
  pub tertiary_color: Option<Color>,
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

/// Options for creating a role with [`Role::create`]
#[derive(Serialize, Clone, Debug)]
pub struct RoleCreateOptions {
  /// Name of the role, max 100 characters
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// Bitwise value of the enabled/disabled permissions
  #[serde(skip_serializing_if = "Option::is_none")]
  pub permissions: Option<Permissions>,
  /// The role’s colors
  #[serde(skip_serializing_if = "Option::is_none")]
  pub colors: Option<RoleColors>,
  /// Whether the role should be displayed separately in the sidebar
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hoist: Option<bool>,
  /// The role’s icon image (if the guild has the `ROLE_ICONS` feature)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon: Option<String>,
  /// The role’s unicode emoji as a standard emoji (if the guild has the `ROLE_ICONS` feature)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub unicode_emoji: Option<String>,
  /// Whether the role should be mentionable
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mentionable: Option<bool>,
}

/// Options for modifying a role with [`Role::modify`]
#[derive(Serialize, Clone, Debug)]
pub struct RoleModifyOptions {
  /// Name of the role, max 100 characters
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<Option<String>>,
  /// Bitwise value of the enabled/disabled permissions
  #[serde(skip_serializing_if = "Option::is_none")]
  pub permissions: Option<Option<Permissions>>,
  /// The role’s colors
  #[serde(skip_serializing_if = "Option::is_none")]
  pub colors: Option<Option<RoleColors>>,
  /// Whether the role should be displayed separately in the sidebar
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hoist: Option<Option<bool>>,
  /// The role’s icon image (if the guild has the `ROLE_ICONS` feature)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon: Option<Option<String>>,
  /// The role’s unicode emoji as a standard emoji (if the guild has the `ROLE_ICONS` feature)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub unicode_emoji: Option<Option<String>>,
  /// Whether the role should be mentionable
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mentionable: Option<Option<bool>>,
}

fn exists<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
  serde_json::Value::deserialize(d)?;
  Ok(true)
}

impl Role {
  /// Fetch a role
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::roles::{Role};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let role = Role::fetch(&input.rest, "613425648685547541", "936746847437983786").await?;
  /// # }
  /// ```
  pub async fn fetch<T: ToString, U: ToString>(rest: &Rest, guild_id: T, role_id: U) -> Result<Role, RestError> {
    rest.get(format!("guilds/{}/roles/{}", guild_id.to_string(), role_id.to_string())).await
  }

  /// Create a new role
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::roles::{Role, RoleCreateOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let options = RoleCreateOptions::new()
  ///   .set_name("nice role");
  /// let role = Role::create(&input.rest, "613425648685547541", options).await?;
  /// # }
  /// ```
  pub async fn create<T: ToString>(rest: &Rest, guild_id: T, options: RoleCreateOptions) -> Result<Role, RestError> {
    rest.post(format!("guilds/{}/roles", guild_id.to_string()), options).await
  }

  /// Modify the role
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{roles::{Role, RoleModifyOptions, RoleColors}, Permissions};
  /// # use slashook::structs::utils::Color;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let role = Role::fetch(&input.rest, "613425648685547541", "936746847437983786").await?;
  /// let options = RoleModifyOptions::new()
  ///   .set_name(Some("better role"))
  ///   .set_colors(Some(RoleColors::new()
  ///     .set_primary_color(Color::try_from("#c0ffee")?)
  ///   ))
  ///   .set_permissions(Some(Permissions::MANAGE_MESSAGES | Permissions::MANAGE_GUILD));
  /// let modified_role = role.modify(&input.rest, "613425648685547541", options).await?;
  /// # }
  /// ```
  pub async fn modify<T: ToString>(&self, rest: &Rest, guild_id: T, options: RoleModifyOptions) -> Result<Role, RestError> {
    rest.patch(format!("guilds/{}/roles/{}", guild_id.to_string(), self.id), options).await
  }

  /// Delete the role
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::roles::Role;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// # let role = Role::fetch(&input.rest, "613425648685547541", "936746847437983786").await?;
  /// role.delete(&input.rest, "613425648685547541").await?;
  /// # }
  /// ```
  pub async fn delete<T: ToString>(&self, rest: &Rest, guild_id: T) -> Result<(), RestError> {
    rest.delete(format!("guilds/{}/roles/{}", guild_id.to_string(), self.id)).await
  }
}

impl RoleColors {
  /// Creates a new default `RoleColors`
  pub fn new() -> Self {
    Self {
      primary_color: Color(0),
      secondary_color: None,
      tertiary_color: None,
    }
  }

  /// Set the primary color
  pub fn set_primary_color(mut self, color: Color) -> Self {
    self.primary_color = color;
    self
  }

  /// Set the secondary color
  pub fn set_secondary_color(mut self, color: Color) -> Self {
    self.secondary_color = Some(color);
    self
  }

  /// Set the tertiary color
  pub fn set_tertiary_color(mut self, color: Color) -> Self {
    self.tertiary_color = Some(color);
    self
  }
}

impl RoleCreateOptions {
  /// Creates a new empty `RoleCreateOptions`
  pub fn new() -> Self {
    Self {
      name: None,
      permissions: None,
      colors: None,
      hoist: None,
      icon: None,
      unicode_emoji: None,
      mentionable: None,
    }
  }

  /// Set the name
  pub fn set_name<T: ToString>(mut self, name: T) -> Self {
    self.name = Some(name.to_string());
    self
  }

  /// Set the permissions
  pub fn set_permissions(mut self, permissions: Permissions) -> Self {
    self.permissions = Some(permissions);
    self
  }

  /// Set the colors
  pub fn set_colors(mut self, colors: RoleColors) -> Self {
    self.colors = Some(colors);
    self
  }

  /// Set hoist
  pub fn set_hoist(mut self, hoist: bool) -> Self {
    self.hoist = Some(hoist);
    self
  }

  /// Set the icon
  pub fn set_icon<T: ToString>(mut self, icon: T) -> Self {
    self.icon = Some(icon.to_string());
    self
  }

  /// Set the unicode emoji
  pub fn set_unicode_emoji<T: ToString>(mut self, unicode_emoji: T) -> Self {
    self.unicode_emoji = Some(unicode_emoji.to_string());
    self
  }

  /// Set mentionable
  pub fn set_mentionable(mut self, mentionable: bool) -> Self {
    self.mentionable = Some(mentionable);
    self
  }
}

impl RoleModifyOptions {
  /// Creates a new empty `RoleModifyOptions`
  pub fn new() -> Self {
    Self {
      name: None,
      permissions: None,
      colors: None,
      hoist: None,
      icon: None,
      unicode_emoji: None,
      mentionable: None,
    }
  }

  /// Set the name
  pub fn set_name<T: ToString>(mut self, name: Option<T>) -> Self {
    self.name = Some(name.map(|t| t.to_string()));
    self
  }

  /// Set the permissions
  pub fn set_permissions(mut self, permissions: Option<Permissions>) -> Self {
    self.permissions = Some(permissions);
    self
  }

  /// Set the colors
  pub fn set_colors(mut self, colors: Option<RoleColors>) -> Self {
    self.colors = Some(colors);
    self
  }

  /// Set hoist
  pub fn set_hoist(mut self, hoist: Option<bool>) -> Self {
    self.hoist = Some(hoist);
    self
  }

  /// Set the icon
  pub fn set_icon<T: ToString>(mut self, icon: Option<T>) -> Self {
    self.icon = Some(icon.map(|t| t.to_string()));
    self
  }

  /// Set the unicode emoji
  pub fn set_unicode_emoji<T: ToString>(mut self, unicode_emoji: Option<T>) -> Self {
    self.unicode_emoji = Some(unicode_emoji.map(|t| t.to_string()));
    self
  }

  /// Set mentionable
  pub fn set_mentionable(mut self, mentionable: Option<bool>) -> Self {
    self.mentionable = Some(mentionable);
    self
  }
}

impl Default for RoleColors {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for RoleCreateOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for RoleModifyOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl<'de> Deserialize<'de> for RoleFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}

