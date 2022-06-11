// Copyright 2022 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![warn(clippy::all)]
#![warn(missing_docs)]

//! A webhook-based Discord slash command library
//!
//! This library focuses on the use of a web server to receive command events with the interaction system instead of the traditional gateway websocket.
//! Scaling can be performed using any load balancing solution and no guild count based sharding is required.
//!
//! ## Usage
//! First, head over to the [Discord Developer Portal](https://discord.com/developers/applications) and grab your application's public key and optionally a bot token.\
//! Here's a simple example to get you started:
//! ```no_run
//! #[macro_use] extern crate slashook;
//! use slashook::{ Client, Config };
//! use slashook::commands::{ CommandInput, CommandResponder };
//!
//! #[slashook::main]
//! async fn main() {
//!   let config = Config {
//!     public_key: String::from("your_public_key"),
//!     bot_token: Some(String::from("your.bot.token")),
//!     ..Default::default()
//!   };
//!
//!   #[command("ping")]
//!   fn ping(input: CommandInput, res: CommandResponder) {
//!     res.send_message("Pong!").await?;
//!   }
//!
//!   let mut client = Client::new(config);
//!   client.register_command(ping);
//!   client.start().await;
//! }
//! ```
//! Your bot will now be listening on `http://0.0.0.0:3000/`. See [Config] for IP and port options.\
//! You may now route it through a reverse proxy and set your interaction url on the Developer Portal.
//! Be sure to also [register your commands](https://discord.com/developers/docs/interactions/application-commands#registering-a-command).

pub(crate) const USER_AGENT: &str = concat!("slashook/", env!("CARGO_PKG_VERSION"));

#[macro_use] extern crate rocket;
mod webhook;
pub mod structs;
pub mod commands;
pub mod rest;

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
use commands::{CommandHandler, Command, RocketCommand};

/// Configuration options for the client
#[derive(Clone, Debug)]
pub struct Config {
  /// IP address to bind to
  pub ip: IpAddr,
  /// Port to listen to
  pub port: u16,
  /// Public key provided by Discord for verifying their request signatures
  pub public_key: String,
  /// Bot token provided by Discord for Bot accounts
  pub bot_token: Option<String>
}

impl Default for Config {
  fn default() -> Self {
    Self {
      ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
      port: 3000,
      public_key: "".to_string(),
      bot_token: None,
    }
  }
}

/// The entry point of the library
pub struct Client {
  config: Config,
  command_handler: CommandHandler
}

impl Client {
  /// Creates a new client with the configuration provided
  pub fn new(config: Config) -> Self {
    Self {
      config,
      command_handler: CommandHandler::new()
    }
  }

  /// Registers a command to the command handler
  ///
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::{Client, Config, commands::{CommandInput, CommandResponder}};
  /// # let config = Config::default();
  /// # let mut client = Client::new(config);
  /// ##[command("command")]
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
  /// ##[command("command1")]
  /// fn command1(_: CommandInput, res: CommandResponder) {
  ///   res.send_message("Response");
  /// }
  /// ##[command("command2")]
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

  /// Starts the webhook listener, setting everything into motion
  pub async fn start(self) {
    let (sender, receiver) = mpsc::unbounded_channel::<RocketCommand>();
    let rocket = webhook::start(self.config, sender);

    let command_handler = Arc::new(self.command_handler);
    spawn(async move {
      command_handler.rocket_bridge(receiver).await;
    });

    rocket.await;
  }
}
