// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs used for handling events

use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
use crate::tokio::{spawn, sync::{mpsc, oneshot}};
use anyhow::Context;
use chrono::{DateTime, Utc};

use super::{Event, responder::{EventResponder, EventResponseError}};
use crate::structs::events::{EventType, EventBody, EventData};
use crate::rest::Rest;

/// Metadata values passed as inputs for your event function and methods for responding to the event.
/// The actual event data is included as the second argument.
pub struct EventInput {
  /// [Event type](EventType)
  pub event_type: EventType,
  /// Timestamp of when the event occurred in [ISO8601 format](https://discord.com/developers/docs/reference#iso8601-datetime)
  pub timestamp: DateTime<Utc>,
  /// Handler for Discord API calls
  pub rest: Rest,
  responder: Option<EventResponder>,
}

impl EventInput {
  /// Acknowledge an event.\
  /// If you do not acknowledge within 3 seconds (perhaps due to an error), Discord will keep repeating the event
  /// with exponential backoff for up to 10 minutes.
  /// If you fail to acknowledge too often Discord will stop sending events and notify you via email.
  pub async fn ack(&self) -> Result<(), EventResponseError> {
    self.responder.as_ref().unwrap().ack().await
  }
}

pub(crate) struct EventHandler {
  pub(crate) events: HashMap<EventType, Arc<Mutex<Event>>>
}

impl EventHandler {
  pub fn new() -> Self {
    Self {
      events: HashMap::new(),
    }
  }

  pub fn add(&mut self, event: Event) {
    self.events.insert(event.event_type.clone(), Arc::new(Mutex::new(event)));
  }

  pub async fn rocket_bridge(self: &Arc<Self>, mut receiver: mpsc::UnboundedReceiver::<RocketEvent>) {
    while let Some(event) = receiver.recv().await {
      let event_handler = self.clone();
      spawn(async move {
        let RocketEvent(event_body, bot_token, handler_send) = event;

        let value = event_handler.handle_event(event_body, bot_token).await;
        handler_send.send(value).unwrap();
      });
    }
  }

  async fn spawn_event_handler(&self, event: Arc<Mutex<Event>>, mut event_input: EventInput, data: EventData) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel::<()>();
    let responder = EventResponder {
      tx,
    };
    event_input.responder = Some(responder);

    spawn(async move {
      let fut = event.lock().unwrap().func.call(event_input, data);
      if let Err(err) = fut.await {
        eprintln!("Error returned from event handler: {:?}", err);
      }
    });

    rx.recv().await.context("Event handler finished without responding")?;
    rx.close();

    Ok(())
  }

  pub async fn handle_event(&self, event_body: EventBody, bot_token: Option<String>) -> anyhow::Result<()> {
    let event = self.events.get(&event_body.event_type).with_context(|| format!("Received event ({:?}) has no registered event handler", event_body.event_type))?;
    let task_event = event.clone();

    let event_input = EventInput {
      event_type: event_body.event_type,
      timestamp: event_body.timestamp,
      rest: Rest::with_optional_token(bot_token),
      responder: None,
    };

    let data = event_body.data.context("Event has no data")?;

    self.spawn_event_handler(task_event, event_input, data).await?;
    Ok(())
  }
}

#[derive(Debug)]
pub(crate) struct RocketEvent(pub EventBody, pub Option<String>, pub oneshot::Sender::<anyhow::Result<()>>);
