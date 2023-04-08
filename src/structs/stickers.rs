// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord stickers

use serde::Deserialize;
use serde_repr::Deserialize_repr;
use super::{
  Snowflake,
  users::User,
};

/// Discord Sticker Object
#[derive(Deserialize, Clone, Debug)]
pub struct Sticker {
  /// [Id of the sticker](https://discord.com/developers/docs/reference#image-formatting)
  pub id: Snowflake,
  /// For standard stickers, id of the pack the sticker is from
  pub pack_id: Option<Snowflake>,
  /// Name of the sticker
  pub name: String,
  /// Description of the sticker
  pub description: Option<String>,
  /// Autocomplete/suggestion tags for the sticker (max 200 characters)
  pub tags: String,
  /// [Type of sticker](StickerType)
  #[serde(rename = "type")]
  pub sticker_type: StickerType,
  /// [Type of sticker format](StickerFormatType)
  pub format_type: StickerFormatType,
  /// Whether this guild sticker can be used, may be false due to loss of Server Boosts
  pub available: Option<bool>,
  /// Id of the guild that owns this sticker
  pub guild_id: Option<Snowflake>,
  /// The user that uploaded the guild sticker
  pub user: Option<User>,
  /// The standard sticker's sort order within its pack
  pub sort_value: Option<i64>,
}

/// Discord Sticker Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum StickerType {
  /// An official sticker in a pack, part of Nitro or in a removed purchasable pack
  STANDARD = 1,
  /// A sticker uploaded to a guild for the guild's members
  GUILD = 2,
  /// Sticker type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Sticker Item Object
#[derive(Deserialize, Clone, Debug)]
pub struct StickerItem {
  /// Id of the sticker
  pub id: Snowflake,
  /// Name of the sticker
  pub name: String,
  /// [Type of sticker format](StickerFormatType)
  pub format_type: StickerFormatType
}

/// Discord Sticker Format Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum StickerFormatType {
  /// .png format
  PNG = 1,
  /// Animated .png format
  APNG = 2,
  /// Lottie .json format
  LOTTIE = 3,
  /// Sticker format type that hasn't been implemented yet
  UNKNOWN
}
