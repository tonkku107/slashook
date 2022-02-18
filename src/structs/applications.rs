// Copyright 2022 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord applications

use serde::{Deserialize, de::Deserializer};
use serde_repr::{Deserialize_repr};
use super::{
  Snowflake,
  users::User
};
use bitflags::bitflags;

/// Discord Application Object
#[derive(Deserialize, Clone, Debug)]
pub struct Application {
  /// The id of the app
  pub id: Snowflake,
  /// The name of the app
  pub name: String,
  /// The [icon hash](https://discord.com/developers/docs/reference#image-formatting) of the app
  pub icon: Option<String>,
  /// The description of the app
  pub description: String,
  /// An array of rpc origin urls, if rpc is enabled
  pub rpc_origins: Option<Vec<String>>,
  /// When false only app owner can join the app's bot to guilds
  pub bot_public: bool,
  /// When true the app's bot will only join upon completion of the full oauth2 code grant flow
  pub bot_require_code_grant: bool,
  /// The url of the app's terms of service
  pub terms_of_service_url: Option<String>,
  /// The url of the app's privacy policy
  pub privacy_policy_url: Option<String>,
  /// Partial user object containing info on the owner of the application
  pub owner: Option<User>,
  /// If this application is a game sold on Discord, this field will be the summary field for the store page of its primary sku
  pub summary: String,
  /// The hex encoded key for verification in interactions and the GameSDK's [GetTicket](https://discord.com/developers/docs/game-sdk/applications#getticket)
  pub verify_key: String,
  /// If the application belongs to a team, this will be the list of the members of that team
  pub team: Option<Team>,
  /// If this application is a game sold on Discord, this field will be the guild to which it has been linked
  pub guild_id: Option<Snowflake>,
  /// If this application is a game sold on Discord, this field will be the id of the "Game SKU" that is created, if exists
  pub primary_sku_id: Option<Snowflake>,
  /// If this application is a game sold on Discord, this field will be the URL slug that links to the store page
  pub slug: Option<String>,
  /// The application's default rich presence invite [cover image hash](https://discord.com/developers/docs/reference#image-formatting)
  pub cover_image: Option<String>,
  /// The application's public [flags](https://discord.com/developers/docs/resources/application#application-object-application-flags)
  pub flags: Option<ApplicationFlags>
}

bitflags! {
  /// Bitflags for Discord Application Flags
  pub struct ApplicationFlags: u32 {
    /// Has presence intent
    const GATEWAY_PRESENCE = 1 << 12;
    /// Enabled unverified presence intent
    const GATEWAY_PRESENCE_LIMITED = 1 << 13;
    /// Has server members intent
    const GATEWAY_GUILD_MEMBERS = 1 << 14;
    /// Enabled unverified server members intent
    const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
    /// Increased guild limit for applications pending verification
    const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
    /// Embedded activity
    const EMBEDDED = 1 << 17;
    /// Has message content intent
    const GATEWAY_MESSAGE_CONTENT = 1 << 18;
    /// Enabled unverified message content intent
    const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
  }
}

/// Discord Team Object
#[derive(Deserialize, Clone, Debug)]
pub struct Team {
  /// A hash of the image of the team's icon
  pub icon: Option<String>,
  /// The unique id of the team
  pub id: Snowflake,
  /// The members of the team
  pub members: Vec<TeamMember>,
  /// The name of the team
  pub name: String,
  /// The user id of the current team owner
  pub owner_user_id: Snowflake
}

/// Discord Team Members Object
#[derive(Deserialize, Clone, Debug)]
pub struct TeamMember {
  /// The user's [membership state](TeamMembershipState) on the team
  pub membership_state: TeamMembershipState,
  /// Will always be `["*"]`
  pub permissions: Vec<String>,
  /// The id of the parent team of which they are a member
  pub team_id: Snowflake,
  /// The avatar, discriminator, id and username of the user
  pub user: User
}

/// Discord Team Membership State Enum
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum TeamMembershipState {
  /// Member has been invited
  INVITED = 1,
  /// Member has accepted invitation
  ACCEPTED = 2,
  /// Membership state that hasn't been implemented yet
  UNKNOWN
}

impl<'de> Deserialize<'de> for ApplicationFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_truncate(bits))
  }
}
