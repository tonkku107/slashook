// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::tokio::sync::mpsc;

/// Error for when a response failed due to the event having been responded to already.
#[derive(Debug)]
pub struct EventResponseError;
impl std::fmt::Display for EventResponseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Event has already been responded to.")
  }
}
impl std::error::Error for EventResponseError { }

/// Struct with methods for responding to events
#[derive(Debug)]
pub struct EventResponder {
  pub(crate) tx: mpsc::UnboundedSender<()>,
}

impl EventResponder {
  pub async fn ack(&self) -> Result<(), EventResponseError> {
    self.tx.send(()).map_err(|_| EventResponseError)?;
    self.tx.closed().await;
    Ok(())
  }
}
