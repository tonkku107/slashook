// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs used in creating event handlers

pub(crate) mod handler;
pub(crate) mod responder;

use std::future::Future;
use crate::commands::CmdResult;
use rocket::futures::future::BoxFuture;

use crate::structs::events::{EventType, EventData};
pub use handler::Event;
pub use responder::EventResponseError;

/// A trait for Event handler functions
///
/// A trait that allows requiring an `async fn(Event, EventData) -> CmdResult` in the [EventHandler] struct.\
/// The function must also be `Send` as they can be transferred between threads
pub trait AsyncEvntFn: Send {
  /// A method that calls the function
  fn call(&self, event: Event, data: EventData) -> BoxFuture<'static, CmdResult>;
}
impl<T, F> AsyncEvntFn for T
where
  T: Fn(Event, EventData) -> F + Send,
  F: Future<Output = CmdResult> + Send + 'static,
{
  fn call(&self, event: Event, data: EventData) -> BoxFuture<'static, CmdResult> {
    Box::pin(self(event, data))
  }
}

/// A struct representing an event handler that can be executed
///
/// **NOTE: This struct is usually constructed with the help of the [event attribute macro](macro@crate::event)**
pub struct EventHandler {
  /// A handler function for the command
  pub func: Box<dyn AsyncEvntFn>,
  /// [Type of event](EventType)
  pub event_type: EventType,
}
