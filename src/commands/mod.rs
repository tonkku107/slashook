// Copyright 2022 slashook Developers
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
  future::Future
};
use rocket::futures::future::BoxFuture;

pub use responder::{MessageResponse, CommandResponder, Modal, InteractionResponseError};
pub use handler::CommandInput;

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
  /// The name of the command
  pub name: String
}
