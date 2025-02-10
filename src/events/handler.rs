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

use super::{EventHandler, responder::{EventResponder, EventResponseError}};
use crate::structs::events::{EventType, EventBody, EventData};
use crate::rest::Rest;

/// Metadata values passed as inputs for your event handler and methods for responding to the event.
/// The actual event data is included as the second argument.
pub struct Event {
  /// [Event type](EventType)
  pub event_type: EventType,
  /// Timestamp of when the event occurred in [ISO8601 format](https://discord.com/developers/docs/reference#iso8601-datetime)
  pub timestamp: DateTime<Utc>,
  /// Handler for Discord API calls
  pub rest: Rest,
  responder: Option<EventResponder>,
}

impl Event {
  /// Acknowledge an event.\
  /// If you do not acknowledge within 3 seconds (perhaps due to an error), Discord will keep repeating the event
  /// with exponential backoff for up to 10 minutes.
  /// If you fail to acknowledge too often Discord will stop sending events and notify you via email.
  pub async fn ack(&self) -> Result<(), EventResponseError> {
    self.responder.as_ref().unwrap().ack().await
  }
}

pub(crate) struct EventHandlers {
  pub(crate) events: HashMap<EventType, Arc<Mutex<EventHandler>>>
}

impl EventHandlers {
  pub fn new() -> Self {
    Self {
      events: HashMap::new(),
    }
  }

  pub fn add(&mut self, event_handler: EventHandler) {
    self.events.insert(event_handler.event_type.clone(), Arc::new(Mutex::new(event_handler)));
  }

  pub async fn rocket_bridge(self: &Arc<Self>, mut receiver: mpsc::UnboundedReceiver::<RocketEvent>) {
    while let Some(event) = receiver.recv().await {
      let event_handlers = self.clone();
      spawn(async move {
        let RocketEvent(event_body, bot_token, handler_send) = event;

        let value = event_handlers.handle_event(event_body, bot_token).await;
        handler_send.send(value).unwrap();
      });
    }
  }

  async fn spawn_event_handler(&self, event_handler: Arc<Mutex<EventHandler>>, mut event: Event, data: EventData) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel::<()>();
    let responder = EventResponder {
      tx,
    };
    event.responder = Some(responder);

    spawn(async move {
      let fut = event_handler.lock().unwrap().func.call(event, data);
      if let Err(err) = fut.await {
        eprintln!("Error returned from event handler: {:?}", err);
      }
    });

    rx.recv().await.context("Event handler finished without responding")?;
    rx.close();

    Ok(())
  }

  pub async fn handle_event(&self, event_body: EventBody, bot_token: Option<String>) -> anyhow::Result<()> {
    let event_handler = self.events.get(&event_body.event_type).with_context(|| format!("Received event ({:?}) has no registered event handler", event_body.event_type))?;
    let task_event_handler = event_handler.clone();

    let event = Event {
      event_type: event_body.event_type,
      timestamp: event_body.timestamp,
      rest: Rest::with_optional_token(bot_token),
      responder: None,
    };

    let data = event_body.data.context("Event has no data")?;

    self.spawn_event_handler(task_event_handler, event, data).await?;
    Ok(())
  }
}

#[derive(Debug)]
pub(crate) struct RocketEvent(pub EventBody, pub Option<String>, pub oneshot::Sender::<anyhow::Result<()>>);
