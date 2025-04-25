// Copyright 2024 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord users

use serde::{Deserialize, de::Deserializer};
use serde::{Serialize, ser::Serializer};
use serde_json::json;
use serde_repr::{Serialize_repr, Deserialize_repr};
use bitflags::bitflags;

use crate::rest::{Rest, RestError};
use super::{
  channels::Channel,
  guilds::{Guild, GuildMember},
  Snowflake,
};

/// Discord User Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
  /// The user's id
  pub id: Snowflake,
  /// The user's username, not unique across the platform
  pub username: String,
  /// The user's 4-digit discord-tag
  pub discriminator: String,
  /// The user's display name, if it is set. For bots, this is the application name
  pub global_name: Option<String>,
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
  /// Data for the user's avatar decoration
  pub avatar_decoration_data: Option<AvatarDecorationData>,
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
  UNKNOWN,
}

/// Discord Avatar Decoration Data Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvatarDecorationData {
  /// The [avatar decoration hash](https://discord.com/developers/docs/reference#image-formatting)
  pub asset: String,
  /// ID of the avatar decoration's SKU
  pub sku_id: Snowflake,
}

/// Options for modifying the user with [`modify_current_user`](User::modify_current_user)
#[derive(Serialize, Default, Clone, Debug)]
pub struct ModifyUserOptions {
  /// User's username
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,
  /// If passed, modifies the user's avatar. Contains [base64 image data URL](https://discord.com/developers/docs/reference#image-data)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub avatar: Option<Option<String>>,
  /// If passed, modifies the user's banner. Contains [base64 image data URL](https://discord.com/developers/docs/reference#image-data)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub banner: Option<Option<String>>,
}

/// Options for listing user guilds with [`get_current_user_guilds`](User::get_current_user_guilds)
#[derive(Serialize, Default, Clone, Debug)]
pub struct GetUserGuildsOptions {
  /// Get guilds before this guild ID
  pub before: Option<Snowflake>,
  /// Get guilds after this guild ID
  pub after: Option<Snowflake>,
  /// Max number of guilds to return (1-200). Default 200
  pub limit: Option<i64>,
  /// Include approximate member and presence counts in response
  pub with_counts: Option<bool>,
}

impl User {
  /// Get user's custom avatar url. `None` if the user has no custom avatar
  pub fn avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> Option<String> {
    self.avatar.as_ref().map(|a| format!("https://cdn.discordapp.com/avatars/{}/{}.{}?size={}", self.id, a, format.to_string(), size.to_string()))
  }

  /// Get the url for the user's default avatar
  pub fn default_avatar_url(&self) -> String {
    let index = if self.discriminator == "0" {
      let id = self.id.parse::<u64>().unwrap_or_default();
      ((id >> 22) % 6) as u8
    } else {
      let discrim = self.discriminator.parse::<u16>().unwrap_or_default();
      (discrim % 5) as u8
    };

    format!("https://cdn.discordapp.com/embed/avatars/{}.png", index)
  }

  /// Get the url for the user avatar that would be displayed in app, falling back to the default avatar if the user doesn't have one
  pub fn display_avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> String {
    self.avatar_url(format, size).unwrap_or_else(|| self.default_avatar_url())
  }

  /// Get the url for the user avatar that would be displayed in app, taking into account the per-server profile. Workaround for [`GuildMember`] that don't have `user` set.
  pub fn display_avatar_url_with_member<T: ToString, U: ToString, V: ToString>(&self, format: T, size: U, guild_id: V, member: GuildMember) -> String {
    member.avatar_url(guild_id, &self.id, format.to_string(), size.to_string())
      .unwrap_or_else(|| self.display_avatar_url(format, size))
  }

  /// Get user's banner url. `None` if the user has no banner
  pub fn banner_url<T: ToString, U: ToString>(&self, format: T, size: U) -> Option<String> {
    self.banner.as_ref().map(|b| format!("https://cdn.discordapp.com/banners/{}/{}.{}?size={}", self.id, b, format.to_string(), size.to_string()))
  }

  /// Get the url for the user banner that would be displayed in app, taking into account the per-server profile. `None` if the user has no banner. Workaround for [`GuildMember`] that don't have `user` set.
  pub fn display_banner_url_with_member<T: ToString, U: ToString, V: ToString>(&self, format: T, size: U, guild_id: V, member: GuildMember) -> Option<String> {
    member.banner_url(guild_id, &self.id, format.to_string(), size.to_string())
      .or_else(|| self.banner_url(format, size))
  }

  /// Returns a string representing a user mention
  pub fn mention(&self) -> String {
    format!("<@{}>", self.id)
  }

  /// Fetch a user with a user ID
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::users::User;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let user = User::fetch(&input.rest, input.user.id).await?;
  /// # }
  /// ```
  pub async fn fetch<T: ToString>(rest: &Rest, user_id: T) -> Result<Self, RestError> {
    rest.get(format!("users/{}", user_id.to_string())).await
  }

  /// Gets the User object of the bot
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::users::User;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let user = User::get_current_user(&input.rest).await?;
  /// # }
  /// ```
  pub async fn get_current_user(rest: &Rest) -> Result<Self, RestError> {
    Self::fetch(rest, "@me").await
  }

  /// Modifies the bot's user
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::structs::utils::File;
  /// # use slashook::tokio::fs::File as TokioFile;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::users::{User, ModifyUserOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let tokio_file = TokioFile::open("cat.png").await?;
  /// let file = File::from_file("cat.png", tokio_file).await?;
  /// let options = ModifyUserOptions::new()
  ///   .set_username("Catbot")
  ///   .set_avatar(file);
  /// let user = User::modify_current_user(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn modify_current_user(rest: &Rest, options: ModifyUserOptions) -> Result<Self, RestError> {
    rest.patch(String::from("users/@me"), options).await
  }

  /// Gets the guilds the bot user is in
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::users::{User, GetUserGuildsOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let options = GetUserGuildsOptions::new().set_with_counts(true);
  /// let guilds = User::get_current_user_guilds(&input.rest, options).await?;
  /// # }
  /// ```
  pub async fn get_current_user_guilds(rest: &Rest, options: GetUserGuildsOptions) -> Result<Vec<Guild>, RestError> {
    rest.get_query(String::from("users/@me/guilds"), options).await
  }

  /// Leaves a guild with the bot user
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::users::User;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// User::leave_guild(&input.rest, input.guild_id.unwrap()).await?;
  /// # }
  /// ```
  pub async fn leave_guild<T: ToString>(rest: &Rest, guild_id: T) -> Result<(), RestError> {
    rest.delete(format!("users/@me/guilds/{}", guild_id.to_string())).await
  }

  /// Creates a DM with the user
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let dm = input.user.create_dm(&input.rest).await?;
  /// dm.create_message(&input.rest, "Hello!").await?;
  /// # }
  /// ```
  pub async fn create_dm(&self, rest: &Rest) -> Result<Channel, RestError> {
    rest.post(String::from("users/@me/channels"), json!({ "recipient_id": self.id })).await
  }
}

impl ModifyUserOptions {
  /// Creates a new empty `ModifyUserOptions`
  pub fn new() -> Self {
    Self {
      username: None,
      avatar: None,
      banner: None,
    }
  }

  /// Sets the username
  pub fn set_username<T: ToString>(mut self, username: T) -> Self {
    self.username = Some(username.to_string());
    self
  }

  /// Sets the avatar\
  /// The `avatar_data` can be a [`File`](super::utils::File)
  pub fn set_avatar<T: ToString>(mut self, avatar_data: T) -> Self {
    self.avatar = Some(Some(avatar_data.to_string()));
    self
  }

  /// Unsets the avatar
  pub fn unset_avatar(mut self) -> Self {
    self.avatar = Some(None);
    self
  }

  /// Sets the banner
  /// The `banner_data` can be a [`File`](super::utils::File)
  pub fn set_banner<T: ToString>(mut self, banner_data: T) -> Self {
    self.banner = Some(Some(banner_data.to_string()));
    self
  }

  /// Unsets the banner
  pub fn unset_banner(mut self) -> Self {
    self.banner = Some(None);
    self
  }
}

impl GetUserGuildsOptions {
  /// Creates a new empty `GetUserGuildsOptions`
  pub fn new() -> Self {
    Self {
      before: None,
      after: None,
      limit: None,
      with_counts: None,
    }
  }

  /// Sets the guild ID to search before.
  /// Also removes `after` if set.
  pub fn set_before<T: ToString>(mut self, before: T) -> Self {
    self.before = Some(before.to_string());
    self.after = None;
    self
  }

  /// Sets the guild ID to search after.
  /// Also removes `before` if set.
  pub fn set_after<T: ToString>(mut self, after: T) -> Self {
    self.after = Some(after.to_string());
    self.before = None;
    self
  }

  /// Sets the limit for the amount of guilds to fetch
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }

  /// Sets whether approximate user and presence counts should be included
  pub fn set_with_counts(mut self, with_counts: bool) -> Self {
    self.with_counts = Some(with_counts);
    self
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
