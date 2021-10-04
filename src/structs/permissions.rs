// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde::de::{self, Deserialize, Deserializer};
use bitflags::bitflags;

bitflags! {
  /// Bitflags for Discord permissions
  ///
  /// ```
  /// # use slashook::structs::Permissions;
  /// let permissions = Permissions::SEND_MESSAGES | Permissions::CONNECT;
  /// assert_eq!(permissions.contains(Permissions::SEND_MESSAGES), true);
  /// assert_eq!(permissions.contains(Permissions::MANAGE_MESSAGES), false);
  /// ```
  pub struct Permissions: u64 {
    const CREATE_INSTANT_INVITE = 1 << 0;
    const KICK_MEMBERS = 1 << 1;
    const BAN_MEMBERS = 1 << 2;
    const ADMINISTRATOR = 1 << 3;
    const MANAGE_CHANNELS = 1 << 4;
    const MANAGE_GUILD = 1 << 5;
    const ADD_REACTIONS = 1 << 6;
    const VIEW_AUDIT_LOG = 1 << 7;
    const PRIORITY_SPEAKER = 1 << 8;
    const STREAM = 1 << 9;
    const VIEW_CHANNEL = 1 << 10;
    const SEND_MESSAGES = 1 << 11;
    const SEND_TTS_MESSAGES = 1 << 12;
    const MANAGE_MESSAGES = 1 << 13;
    const EMBED_LINKS = 1 << 14;
    const ATTACH_FILES = 1 << 15;
    const READ_MESSAGE_HISTORY = 1 << 16;
    const MENTION_EVERYONE = 1 << 17;
    const USE_EXTERNAL_EMOJIS = 1 << 18;
    const VIEW_GUILD_INSIGHTS = 1 << 19;
    const CONNECT = 1 << 20;
    const SPEAK = 1 << 21;
    const MUTE_MEMBERS = 1 << 22;
    const DEAFEN_MEMBERS = 1 << 23;
    const MOVE_MEMBERS = 1 << 24;
    const USE_VAD = 1 << 25;
    const CHANGE_NICKNAME = 1 << 26;
    const MANAGE_NICKNAMES = 1 << 27;
    const MANAGE_ROLES = 1 << 28;
    const MANAGE_WEBHOOKS = 1 << 29;
    const MANAGE_EMOJIS_AND_STICKERS = 1 << 30;
    const USE_APPLICATION_COMMANDS = 1 << 31;
    const REQUEST_TO_SPEAK = 1 << 32;
    const MANAGE_THREADS = 1 << 34;
    const USE_PUBLIC_THREADS = 1 << 35;
    const USE_PRIVATE_THREADS = 1 << 36;
    const USE_EXTERNAL_STICKERS = 1 << 37;
  }
}

impl<'de> Deserialize<'de> for Permissions {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let string = String::deserialize(d)?;
    let bits: u64 = string.parse().map_err(de::Error::custom)?;
    Ok(Self::from_bits_truncate(bits))
  }
}
