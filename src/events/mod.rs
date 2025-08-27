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

pub use crate::structs::events::EventType;
pub use handler::EventInput;
pub use responder::EventResponseError;
use crate::structs::events::EventData;

/// A trait for Event functions
///
/// A trait that allows requiring an `async fn(Event, EventData) -> CmdResult` in the [Event] struct.\
/// The function must also be `Send` as they can be transferred between threads
pub trait AsyncEvntFn: Send {
  /// A method that calls the function
  fn call(&self, event: EventInput, data: EventData) -> BoxFuture<'static, CmdResult>;
}
impl<T, F> AsyncEvntFn for T
where
  T: Fn(EventInput, EventData) -> F + Send,
  F: Future<Output = CmdResult> + Send + 'static,
{
  fn call(&self, event: EventInput, data: EventData) -> BoxFuture<'static, CmdResult> {
    Box::pin(self(event, data))
  }
}

/// A struct representing an event that can be executed
///
/// **NOTE: This struct is usually constructed with the help of the [event attribute macro](macro@crate::event)**
pub struct Event {
  /// A handler function for the event
  pub func: Box<dyn AsyncEvntFn>,
  /// [Type of event](EventType)
  pub event_type: EventType,
}
