// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord applications

use serde::{Deserialize, de::Deserializer};
use serde_repr::Deserialize_repr;
use super::{
  events::EventType,
  guilds::Guild,
  users::User,
  Permissions,
  Snowflake,
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
  pub bot_public: Option<bool>,
  /// When true the app's bot will only join upon completion of the full oauth2 code grant flow
  pub bot_require_code_grant: Option<bool>,
  /// Partial user object for the bot user associated with the app
  pub bot: Option<User>,
  /// The url of the app's terms of service
  pub terms_of_service_url: Option<String>,
  /// The url of the app's privacy policy
  pub privacy_policy_url: Option<String>,
  /// Partial user object containing info on the owner of the application
  pub owner: Option<User>,
  /// The hex encoded key for verification in interactions and the GameSDK's [GetTicket](https://discord.com/developers/docs/game-sdk/applications#getticket)
  pub verify_key: Option<String>,
  /// If the application belongs to a team, this will be the list of the members of that team
  pub team: Option<Team>,
  /// If this application is a game sold on Discord, this field will be the guild to which it has been linked
  pub guild_id: Option<Snowflake>,
  /// Partial object of the associated guild
  pub guild: Option<Guild>,
  /// If this application is a game sold on Discord, this field will be the id of the "Game SKU" that is created, if exists
  pub primary_sku_id: Option<Snowflake>,
  /// If this application is a game sold on Discord, this field will be the URL slug that links to the store page
  pub slug: Option<String>,
  /// The application's default rich presence invite [cover image hash](https://discord.com/developers/docs/reference#image-formatting)
  pub cover_image: Option<String>,
  /// The application's public [flags](ApplicationFlags)
  pub flags: Option<ApplicationFlags>,
  /// Approximate count of guilds the app has been added to
  pub approximate_guild_count: Option<i64>,
  /// Approximate count of users that have installed the app
  pub approximate_user_install_count: Option<i64>,
  /// Array of redirect URIs for the app
  pub redirect_uris: Option<Vec<String>>,
  /// [Interactions endpoint URL](https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction) for the app
  pub interactions_endpoint_url: Option<String>,
  /// Role connection verification URL for the app
  pub role_connections_verification_url: Option<String>,
  /// [Event webhooks URL](https://discord.com/developers/docs/events/webhook-events#preparing-for-events) for the app to receive webhook events
  pub event_webhooks_url: Option<String>,
  /// If [webhook events](https://discord.com/developers/docs/events/webhook-events) are enabled for the app.
  pub event_webhooks_status: Option<ApplicationEventWebhookStatus>,
  /// List of [Webhook event types](EventType) the app subscribes to
  pub event_webhooks_types: Option<Vec<EventType>>,
  /// List of tags describing the content and functionality of the app. Max of 5 tags.
  pub tags: Option<Vec<String>>,
  /// Settings for the application's default in-app authorization link, if enabled
  pub install_params: Option<InstallParams>,
  /// Default scopes and permissions for each supported installation context. Value for each key is an [integration type configuration object](ApplicationIntegrationTypesConfigValue)
  pub integration_types_config: Option<ApplicationIntegrationTypesConfig>,
  /// The application's default custom authorization link, if enabled
  pub custom_install_url: Option<String>,
}

bitflags! {
  /// Bitflags for Discord Application Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct ApplicationFlags: u32 {
    /// Indicates if an app uses the [Auto Moderation API](https://discord.com/developers/docs/resources/auto-moderation)
    const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
    /// Intent required for bots in **100 or more servers** to receive `presence_update` events
    const GATEWAY_PRESENCE = 1 << 12;
    /// Intent required for bots in under 100 servers to receive `presence_update` events, found in Bot Settings
    const GATEWAY_PRESENCE_LIMITED = 1 << 13;
    /// Intent required for bots in **100 or more servers** to receive member-related events like `guild_member_add`. See list of member-related events [under `GUILD_MEMBERS`](https://discord.com/developers/docs/topics/gateway#list-of-intents)
    const GATEWAY_GUILD_MEMBERS = 1 << 14;
    /// Intent required for bots in under 100 servers to receive member-related events like `guild_member_add`, found in Bot Settings. See list of member-related events [under `GUILD_MEMBERS`](https://discord.com/developers/docs/topics/gateway#list-of-intents)
    const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
    /// Indicates unusual growth of an app that prevents verification
    const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
    /// Indicates if an app is embedded within the Discord client (currently unavailable publicly)
    const EMBEDDED = 1 << 17;
    /// Intent required for bots in **100 or more servers** to receive [message content](https://support-dev.discord.com/hc/en-us/articles/4404772028055)
    const GATEWAY_MESSAGE_CONTENT = 1 << 18;
    /// Intent required for bots in under 100 servers to receive [message content](https://support-dev.discord.com/hc/en-us/articles/4404772028055), found in Bot Settings
    const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
    /// Indicates if an app has registered global [application commands](super::interactions::ApplicationCommand)
    const APPLICATION_COMMAND_BADGE = 1 << 23;
  }
}

/// Discord Application Event Webhook Status Enum
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ApplicationEventWebhookStatus {
  /// Webhook events are disabled by developer
  DISABLED = 1,
  /// Webhook events are enabled by developer
  ENABLED = 2,
  /// Webhook events are disabled by Discord, usually due to inactivity
  DISABLED_BY_DISCORD = 3,
  /// Application Event Webhook Status that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord Integration Types Config Object
#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationIntegrationTypesConfig {
  /// Configuration for [`GUILD_INSTALL`](super::interactions::IntegrationType::GUILD_INSTALL) integrations
  #[serde(rename = "0")]
  pub guild_install: Option<ApplicationIntegrationTypesConfigValue>,
  /// Configuration for [`USER_INSTALL`](super::interactions::IntegrationType::USER_INSTALL) integrations
  #[serde(rename = "1")]
  pub user_install: Option<ApplicationIntegrationTypesConfigValue>,
}

/// Discord Integration Types Config Value Object
#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationIntegrationTypesConfigValue {
  /// Install params for each installation context's default in-app authorization link
  pub oauth2_install_params: InstallParams,
}

/// Discord Install Params Object
#[derive(Deserialize, Clone, Debug)]
pub struct InstallParams {
  /// The [scopes](https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes) to add the application to the server with
  pub scopes: Vec<String>,
  /// The [permissions](Permissions) to request for the bot role
  pub permissions: Permissions,
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
  pub owner_user_id: Snowflake,
}

/// Discord Team Members Object
#[derive(Deserialize, Clone, Debug)]
pub struct TeamMember {
  /// The user's [membership state](TeamMembershipState) on the team
  pub membership_state: TeamMembershipState,
  /// The id of the parent team of which they are a member
  pub team_id: Snowflake,
  /// The avatar, discriminator, id and username of the user
  pub user: User,
  /// [Role](TeamMemberRole) of the team member
  pub role: TeamMemberRole,
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
  #[serde(other)]
  UNKNOWN,
}

/// Discord Team Member Role Types
#[derive(Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
#[allow(non_camel_case_types)]
#[serde(rename_all = "snake_case")]
pub enum TeamMemberRole {
  /// Admins have similar access as owners, except they cannot take destructive actions on the team or team-owned apps.
  ADMIN,
  /// Developers can access information about team-owned apps, like the client secret or public key. They can also take limited actions on team-owned apps, like configuring interaction endpoints or resetting the bot token. Members with the Developer role cannot manage the team or its members, or take destructive actions on team-owned apps.
  DEVELOPER,
  /// Read-only members can access information about a team and any team-owned apps. Some examples include getting the IDs of applications and exporting payout records. Members can also invite bots associated with team-owned apps that are marked private.
  READ_ONLY,
  /// A team member role that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

impl<'de> Deserialize<'de> for ApplicationFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}
