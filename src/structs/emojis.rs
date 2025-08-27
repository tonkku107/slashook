// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord Emojis

use serde::{Serialize, Deserialize};
use super::{
  Snowflake,
  users::User
};

/// Discord Emoji Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Emoji {
  /// Emoji id
  pub id: Option<Snowflake>,
  /// Emoji name
  pub name: Option<String>,
  /// Roles allowed to use this emoji
  pub roles: Option<Vec<Snowflake>>,
  /// User that created this emoji
  pub user: Option<User>,
  /// Whether this emoji must be wrapped in colons
  pub require_colons: Option<bool>,
  /// Whether this emoji is managed
  pub managed: Option<bool>,
  /// Whether this emoji is animated
  pub animated: Option<bool>,
  /// Whether this emoji can be used, may be false due to loss of Server Boosts
  pub available: Option<bool>
}

impl Emoji {
  /// Creates a new Emoji from an unicode emoji
  /// ```
  /// # use slashook::structs::Emoji;
  /// let emoji = Emoji::new_standard_emoji("👌🏻");
  /// assert_eq!(emoji.name, Some(String::from("👌🏻")));
  /// ```
  pub fn new_standard_emoji<T: ToString>(emoji: T) -> Self {
    Self {
      id: None,
      name: Some(emoji.to_string()),
      roles: None,
      user: None,
      require_colons: None,
      managed: None,
      animated: None,
      available: None
    }
  }

  /// Creates a new Emoji from custom emojis
  /// ```
  /// # use slashook::structs::Emoji;
  /// let emoji = Emoji::new_custom_emoji("356549630474846209", "Thonk", false);
  /// assert_eq!(emoji.mention(), "<:Thonk:356549630474846209>");
  /// ```
  pub fn new_custom_emoji<T: ToString, U: ToString>(id: T, name: U, animated: bool) -> Self {
    Self {
      id: Some(id.to_string()),
      name: Some(name.to_string()),
      roles: None,
      user: None,
      require_colons: None,
      managed: None,
      animated: Some(animated),
      available: None
    }
  }

  /// Returns a string representing an emoji mention. Returns just the unicode emoji if not custom.
  /// ```
  /// # use slashook::structs::Emoji;
  /// let emoji = Emoji::new_custom_emoji("837407035862679573", "fastnod", true);
  /// assert_eq!(emoji.mention(), "<a:fastnod:837407035862679573>");
  /// let normal_emoji = Emoji::new_standard_emoji("👌🏻");
  /// assert_eq!(normal_emoji.mention(), "👌🏻");
  /// ```
  pub fn mention(&self) -> String {
    let fallback = String::new();
    let name = self.name.as_ref().unwrap_or(&fallback);
    match &self.id {
      Some(id) => {
        let animated = if self.animated.unwrap_or(false) { "a" } else { "" };
        format!("<{}:{}:{}>", animated, name, id)
      },
      None => name.to_string()
    }
  }

  pub(crate) fn to_url_format(&self) -> String {
    let fallback = String::new();
    if let Some(id) = &self.id {
      format!("{}:{}", self.name.as_ref().unwrap_or(&fallback), id)
    } else {
      self.name.as_ref().unwrap_or(&fallback).to_string()
    }
  }
}
