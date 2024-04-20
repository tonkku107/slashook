// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs used in creating commands

pub(crate) mod responder;
pub(crate) mod handler;

use std::{
  marker::Send,
  future::Future,
  collections::HashMap,
};
use rocket::futures::future::BoxFuture;

pub use responder::{MessageResponse, CommandResponder, Modal, InteractionResponseError};
pub use handler::CommandInput;
use crate::structs::{
  interactions::{ApplicationCommand, ApplicationCommandType, ApplicationCommandOption, InteractionOptionType, IntegrationType, InteractionContextType},
  Permissions
};

/// The `Result` types expected from a command function
///
/// Since all the responses are sent via methods on [CommandResponder], we don't expect anything special on success.
/// Error can be anything that implements Error (boxed) which is useful for using `?` to handle errors.
pub type CmdResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// A trait for Command functions
///
/// A trait that allows requiring an `async fn(CommandInput, CommandResponder) -> CmdResult` in the [Command] struct.\
/// The function must also be `Send` as they can be transferred between threads
pub trait AsyncCmdFn: Send {
  /// A method that calls the function
  fn call(&self, input: CommandInput, responder: CommandResponder) -> BoxFuture<'static, CmdResult>;
}
impl<T, F> AsyncCmdFn for T
where
  T: Fn(CommandInput, CommandResponder) -> F + Send,
  F: Future<Output = CmdResult> + Send + 'static,
{
  fn call(&self, input: CommandInput, responder: CommandResponder) -> BoxFuture<'static, CmdResult> {
    Box::pin(self(input, responder))
  }
}

/// A struct representing a command that can be executed
///
/// **NOTE: This struct is usually constructed with the help of the [command attribute macro](macro@crate::command)**
pub struct Command {
  /// A handler function for the command
  pub func: Box<dyn AsyncCmdFn>,
  /// Ignore the command when syncing commands
  pub ignore: bool,
  /// [Name of command](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-naming), 1-32 characters
  pub name: String,
  /// Localization dictionary for `name` field. Values follow the same restrictions as `name`
  pub name_localizations: Option<HashMap<String, String>>,
  /// [Type of command](ApplicationCommandType), defaults to `CHAT_INPUT`
  pub command_type: Option<ApplicationCommandType>,
  /// Description for `CHAT_INPUT` commands, 1-100 characters. Empty string for `USER` and `MESSAGE` commands
  pub description: OptionalString,
  /// Localization dictionary for `description` field. Values follow the same restrictions as `description`
  pub description_localizations: Option<HashMap<String, String>>,
  /// Parameters for the command, max of 25
  pub options: Option<Vec<ApplicationCommandOption>>,
  /// Set of [permissions](Permissions) represented as a bit set
  pub default_member_permissions: Option<Permissions>,
  /// Indicates whether the command is age-restricted, defaults to `false`
  pub nsfw: Option<bool>,
  /// [Installation context(s)](https://discord.com/developers/docs/resources/application#installation-context) where the command is available, only for globally-scoped commands. Defaults to `GUILD_INSTALL` (`0`)
  pub integration_types: Option<Vec<IntegrationType>>,
  /// [Interaction context(s)](InteractionContextType) where the command can be used, only for globally-scoped commands. By default, all interaction context types included for new commands.
  pub contexts: Option<Vec<InteractionContextType>>,
  /// Subcommand groups for the command
  pub subcommand_groups: Option<Vec<SubcommandGroup>>,
  /// Subcommands for the command
  pub subcommands: Option<Vec<Subcommand>>,
}

/// Struct representing subcommand groups
#[derive(Default, Clone, Debug)]
pub struct SubcommandGroup {
  /// [Name of subcommand group](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-naming), 1-32 characters
  pub name: String,
  /// Localization dictionary for the `name` field. Values follow the same restrictions as `name`
  pub name_localizations: Option<HashMap<String, String>>,
  /// Description for the subcommand group
  pub description: String,
  /// Localization dictionary for the `description` field. Values follow the same restrictions as `description`
  pub description_localizations: Option<HashMap<String, String>>,
  /// Subcommands in the group
  pub subcommands: Vec<Subcommand>,
}

/// Struct representing subcommands
#[derive(Default, Clone, Debug)]
pub struct Subcommand {
  /// [Name of subcommand](https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-naming), 1-32 characters
  pub name: String,
  /// Localization dictionary for the `name` field. Values follow the same restrictions as `name`
  pub name_localizations: Option<HashMap<String, String>>,
  /// Description for the subcommand
  pub description: String,
  /// Localization dictionary for the `description` field. Values follow the same restrictions as `description`
  pub description_localizations: Option<HashMap<String, String>>,
  /// Parameters for the command, max of 25
  pub options: Vec<ApplicationCommandOption>,
}

/// Wrapper struct for an `Option<String>` so extra traits can be implemented on it
#[derive(Clone, Debug)]
pub struct OptionalString(Option<String>);

impl<T: Into<String>> From<T> for OptionalString {
  fn from(value: T) -> Self {
    Self(Some(value.into()))
  }
}

async fn dummy (_: CommandInput, _: CommandResponder) -> CmdResult { Ok(()) }
impl Default for Command {
  fn default() -> Self {
    Self {
      func: Box::new(dummy),
      ignore: false,
      name: String::new(),
      name_localizations: None,
      command_type: None,
      description: OptionalString(None),
      description_localizations: None,
      options: None,
      default_member_permissions: None,
      nsfw: None,
      integration_types: None,
      contexts: None,
      subcommand_groups: None,
      subcommands: None
    }
  }
}

impl Clone for Command {
  fn clone(&self) -> Self {
    Self {
      func: Box::new(dummy),
      ignore: self.ignore,
      name: self.name.clone(),
      name_localizations: self.name_localizations.clone(),
      command_type: self.command_type.clone(),
      description: self.description.clone(),
      description_localizations: self.description_localizations.clone(),
      options: self.options.clone(),
      default_member_permissions: self.default_member_permissions,
      nsfw: self.nsfw,
      integration_types: self.integration_types.clone(),
      contexts: self.contexts.clone(),
      subcommand_groups: self.subcommand_groups.clone(),
      subcommands: self.subcommands.clone(),
    }
  }
}

impl TryFrom<Command> for ApplicationCommand {
  type Error = anyhow::Error;

  fn try_from(value: Command) -> anyhow::Result<Self> {
    if value.options.is_some() && (value.subcommands.is_some() || value.subcommand_groups.is_some()) {
      anyhow::bail!("You cannot have options on the base command when using subcommands or subcommand groups");
    }

    let mut options = value.options;
    if let Some(scgs) = value.subcommand_groups {
      options = Some(scgs.into_iter().map(|scg| scg.into()).collect());
    }
    if let Some(scs) = value.subcommands {
      let mut opts = options.unwrap_or_default();
      opts.extend(scs.into_iter().map(|sc| sc.into()));
      options = Some(opts);
    }

    Ok(Self {
      id: None,
      command_type: value.command_type,
      application_id: None,
      guild_id: None,
      name: value.name,
      name_localizations: value.name_localizations,
      description: value.description.0,
      description_localizations: value.description_localizations,
      options,
      default_member_permissions: value.default_member_permissions,
      nsfw: value.nsfw,
      integration_types: value.integration_types,
      contexts: value.contexts,
      version: None
    })
  }
}

impl From<SubcommandGroup> for ApplicationCommandOption {
  fn from(value: SubcommandGroup) -> Self {
    let options = value.subcommands.into_iter().map(|sc| sc.into()).collect();

    Self {
      option_type: InteractionOptionType::SUB_COMMAND_GROUP,
      name: value.name,
      name_localizations: value.name_localizations,
      description: value.description,
      description_localizations: value.description_localizations,
      options: Some(options),
      ..Default::default()
    }
  }
}

impl From<Subcommand> for ApplicationCommandOption {
  fn from(value: Subcommand) -> Self {
    Self {
      option_type: InteractionOptionType::SUB_COMMAND,
      name: value.name,
      name_localizations: value.name_localizations,
      description: value.description,
      description_localizations: value.description_localizations,
      options: Some(value.options),
      ..Default::default()
    }
  }
}
