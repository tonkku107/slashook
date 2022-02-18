// Copyright 2022 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord guilds

use serde::{Deserialize, de::Deserializer};
use super::{Snowflake, users::User, Permissions};
use chrono::{DateTime, Utc};

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
  pub joined_at: String,
  /// When the user started [boosting](https://support.discord.com/hc/en-us/articles/360028038352-Server-Boosting-) the guild
  pub premium_since: Option<String>,
  /// Whether the user is deafened in voice channels
  pub deaf: Option<bool>,
  /// Whether the user is muted in voice channels
  pub mute: Option<bool>,
  /// Whether the user has not yet passed the guild's [Membership Screening](https://discord.com/developers/docs/resources/guild#membership-screening-object) requirements
  pub pending: Option<bool>,
  /// Total permissions of the member in the channel, including overwrites, returned when in the interaction object
  pub permissions: Option<Permissions>,
  /// When the user's [timeout](https://support.discord.com/hc/en-us/articles/4413305239191-Time-Out-FAQ) will expire and the user will be able to communicate in the guild again, None or a time in the past if the user is not timed out
  pub communication_disabled_until: Option<DateTime<Utc>>
}

/// Discord Role Object
#[derive(Deserialize, Clone, Debug)]
pub struct Role {
  /// Role id
  pub id: Snowflake,
  /// Role name
  pub name: String,
  /// Integer representation of hexadecimal color code
  pub color: i64,
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
  /// Whether this is the guild's premium subscriber role
  #[serde(default, deserialize_with = "exists")]
  pub premium_subscriber: bool
}

fn exists<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
  serde_json::Value::deserialize(d)?;
  Ok(true)
}
