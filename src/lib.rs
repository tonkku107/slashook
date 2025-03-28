// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![warn(clippy::all)]
#![warn(missing_docs)]

//! A webhook-based Discord slash command library
//!
//! This library focuses on the use of a web server to receive commands and events with the interaction system instead of the traditional gateway websocket.
//! Scaling can be performed using any load balancing solution and no guild count based sharding is required.
//!
//! ## Usage
//! First, head over to the [Discord Developer Portal](https://discord.com/developers/applications) and grab your application's public key and optionally a bot token, client id and/or client secret.\
//! Here's a simple example to get you started:
//! ```no_run
//! #[macro_use] extern crate slashook;
//! use slashook::{ Client, Config };
//! use slashook::commands::{ CommandInput, CommandResponder };
//! use slashook::events::{ EventInput, EventType };
//! use slashook::structs::events::ApplicationAuthorizedEventData;
//!
//! #[slashook::main]
//! async fn main() {
//!   let config = Config {
//!     public_key: String::from("your_public_key"),
//!     bot_token: Some(String::from("your.bot.token")),
//!     client_id: Some(String::from("your_client_id")),
//!     ..Default::default()
//!   };
//!
//!   #[command(name = "ping", description = "pong")]
//!   fn ping(input: CommandInput, res: CommandResponder) {
//!     res.send_message("Pong!").await?;
//!   }
//!
//!   #[event(EventType::APPLICATION_AUTHORIZED)]
//!   fn authorized(event: EventInput, data: ApplicationAuthorizedEventData) {
//!     event.ack().await?;
//!     println!("Authorized by {} at {}", data.user.username, event.timestamp);
//!   }
//!
//!   let mut client = Client::new(config);
//!   client.register_command(ping);
//!   client.register_event(authorized);
//!   client.sync_commands().await;
//!   client.start().await;
//! }
//! ```
//! Your bot will now be listening on `http://0.0.0.0:3000/`. See [Config] for IP and port options.\
//! You may now route it through a reverse proxy and set your interaction url with route `/` and event url with route `/events` on the Developer Portal.
//!
//! Take a look at [CommandInput](commands::CommandInput) and [CommandResponder](commands::CommandResponder) for the values and functions you have at your disposal in your command functions.
//! Check out [EventInput](events::EventInput) for data available in every event as well as how to acknowledge them and
//! [EventData](structs::events::EventData) for the available data types for different [EventType](events::EventType)s.

pub(crate) const USER_AGENT: &str = concat!("slashook/", env!("CARGO_PKG_VERSION"));

#[macro_use] extern crate rocket;
mod webhook;
pub mod structs;
pub mod commands;
pub mod events;
pub mod rest;
pub(crate) mod internal_utils;

// Macros
pub use slashook_macros::*;

// Re-exports
pub use rocket::{async_main, tokio};
pub use chrono;

use std::{
  net::{IpAddr, Ipv4Addr},
  sync::Arc
};
use tokio::{sync::mpsc, spawn};

use commands::{Command, handler::{CommandHandler, RocketCommand}};
use events::{Event, handler::{EventHandler, RocketEvent}};
use structs::interactions::ApplicationCommand;
use rest::Rest;

/// Configuration options for the client
#[derive(Clone, Debug)]
pub struct Config {
  /// IP address to bind to
  pub ip: IpAddr,
  /// Port to listen to
  pub port: u16,
  /// Public key provided by Discord for verifying their request signatures
  pub public_key: String,
  /// Client ID provided by Discord, required for syncing commands
  pub client_id: Option<String>,
  /// Client Secret provided by Discord, required for syncing commands without a bot token
  pub client_secret: Option<String>,
  /// Bot token provided by Discord for Bot accounts
  pub bot_token: Option<String>
}

impl Default for Config {
  fn default() -> Self {
    Self {
      ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
      port: 3000,
      public_key: "".to_string(),
      client_id: None,
      client_secret: None,
      bot_token: None,
    }
  }
}

/// The entry point of the library
pub struct Client {
  config: Config,
  command_handler: CommandHandler,
  event_handler: EventHandler,
}

impl Client {
  /// Creates a new client with the configuration provided
  pub fn new(config: Config) -> Self {
    Self {
      config,
      command_handler: CommandHandler::new(),
      event_handler: EventHandler::new(),
    }
  }

  /// Registers a command to the command handler
  ///
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::{Client, Config, commands::{CommandInput, CommandResponder}};
  /// # let config = Config::default();
  /// # let mut client = Client::new(config);
  /// ##[command(name = "command", description = "An example command")]
  /// fn command(_: CommandInput, res: CommandResponder) {
  ///   res.send_message("Response");
  /// }
  /// client.register_command(command);
  /// ```
  pub fn register_command(&mut self, command: Command) -> &mut Self {
    self.command_handler.add(command);
    self
  }

  /// Registers multiple commands at once to the command handler
  ///
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::{Client, Config, commands::{CommandInput, CommandResponder}};
  /// # let config = Config::default();
  /// # let mut client = Client::new(config);
  /// ##[command(name = "command1", description = "An example command")]
  /// fn command1(_: CommandInput, res: CommandResponder) {
  ///   res.send_message("Response");
  /// }
  /// ##[command(name = "command2", description = "An example command")]
  /// fn command2(_: CommandInput, res: CommandResponder) {
  ///   res.send_message("A different response");
  /// }
  /// client.register_commands(vec![command1, command2]);
  /// ```
  pub fn register_commands(&mut self, commands: Vec<Command>) -> &mut Self {
    for command in commands.into_iter() {
      self.command_handler.add(command);
    }
    self
  }

  /// Registers an event to the event handler
  ///
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::{Client, Config, events::{EventType, EventInput}, structs::events::ApplicationAuthorizedEventData};
  /// # let config = Config::default();
  /// # let mut client = Client::new(config);
  /// ##[event(EventType::APPLICATION_AUTHORIZED)]
  /// fn event(event: EventInput, data: ApplicationAuthorizedEventData) {
  ///   event.ack().await?;
  /// }
  /// client.register_event(event);
  /// ```
  pub fn register_event(&mut self, event: Event) -> &mut Self {
    self.event_handler.add(event);
    self
  }

  /// Registers multiple events at once to the event handler
  ///
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::{Client, Config, events::{EventType, EventInput}, structs::{events::ApplicationAuthorizedEventData, monetization::Entitlement}};
  /// # let config = Config::default();
  /// # let mut client = Client::new(config);
  /// ##[event(EventType::APPLICATION_AUTHORIZED)]
  /// fn event1(event: EventInput, data: ApplicationAuthorizedEventData) {
  ///   event.ack().await?;
  /// }
  /// ##[event(EventType::ENTITLEMENT_CREATE)]
  /// fn event2(event: EventInput, data: Entitlement) {
  ///   event.ack().await?;
  /// }
  /// client.register_events(vec![event1, event2]);
  /// ```
  pub fn register_events(&mut self, events: Vec<Event>) -> &mut Self {
    for event in events.into_iter() {
      self.event_handler.add(event);
    }
    self
  }

  async fn create_sync_rest(&self) -> anyhow::Result<Rest> {
    let rest;

    if let Some(bot_token) = &self.config.bot_token {
      rest = Rest::with_token(bot_token.to_string());
    } else {
      if self.config.client_secret.is_none() {
        anyhow::bail!("A client_secret or bot_token is required in the config to sync commands");
      }
      rest = Rest::with_client_credentials(
        self.config.client_id.as_ref().unwrap().to_string(),
        self.config.client_secret.as_ref().unwrap().to_string(),
        vec![String::from("applications.commands.update")]
      ).await?;
    }

    Ok(rest)
  }

  /// Syncs defined commands with Discord
  ///
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::{Client, Config, commands::{CommandInput, CommandResponder}};
  /// # #[slashook::main]
  /// # async fn main() {
  /// # let config = Config::default();
  /// # let mut client = Client::new(config);
  /// ##[command(name = "command", description = "An example command")]
  /// fn command(_: CommandInput, res: CommandResponder) {
  ///   res.send_message("Response");
  /// }
  /// client.register_command(command);
  /// client.sync_commands().await;
  /// # }
  /// ```
  pub async fn sync_commands(&self) -> anyhow::Result<Vec<ApplicationCommand>> {
    if self.config.client_id.is_none() {
      anyhow::bail!("A client_id is required in the config to sync commands");
    }

    let rest = self.create_sync_rest().await?;
    let commands = self.command_handler.convert_commands()?;

    Ok(ApplicationCommand::bulk_overwrite_global_commands(&rest, self.config.client_id.as_ref().unwrap(), commands).await?)
  }

  /// Syncs defined commands with Discord as guild commands
  ///
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::{Client, Config, commands::{CommandInput, CommandResponder}};
  /// # #[slashook::main]
  /// # async fn main() {
  /// # let config = Config::default();
  /// # let mut client = Client::new(config);
  /// ##[command(name = "command", description = "An example command")]
  /// fn command(_: CommandInput, res: CommandResponder) {
  ///   res.send_message("Response");
  /// }
  /// client.register_command(command);
  /// client.sync_guild_commands("613425648685547541").await;
  /// # }
  /// ```
  pub async fn sync_guild_commands<T: ToString>(&self, guild_id: T) -> anyhow::Result<Vec<ApplicationCommand>> {
    if self.config.client_id.is_none() {
      anyhow::bail!("A client_id is required in the config to sync commands");
    }

    let rest = self.create_sync_rest().await?;
    let commands = self.command_handler.convert_commands()?;

    Ok(ApplicationCommand::bulk_overwrite_guild_commands(&rest, self.config.client_id.as_ref().unwrap(), guild_id, commands).await?)
  }

  /// Starts the webhook listener, setting everything into motion
  pub async fn start(self) {
    let (command_sender, command_receiver) = mpsc::unbounded_channel::<RocketCommand>();
    let (event_sender, event_receiver) = mpsc::unbounded_channel::<RocketEvent>();
    let rocket = webhook::start(self.config, command_sender, event_sender);

    let command_handler = Arc::new(self.command_handler);
    spawn(async move {
      command_handler.rocket_bridge(command_receiver).await;
    });

    let event_handler = Arc::new(self.event_handler);
    spawn(async move {
      event_handler.rocket_bridge(event_receiver).await;
    });

    rocket.await;
  }
}
