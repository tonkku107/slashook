// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord events

use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{de, Deserialize};
use serde_json::Value;
use serde_repr::Deserialize_repr;
use super::{
  guilds::Guild,
  interactions::IntegrationType,
  monetization::Entitlement,
  users::User,
  Snowflake
};

/// Discord Event Payload Object
#[doc(hidden)]
#[derive(Deserialize, Clone, Debug)]
pub struct EventPayload {
  /// Version scheme for the webhook event. Currently always `1`
  pub version: u8,
  /// ID of your app
  pub application_id: Snowflake,
  /// Type of webhook, either `0` for PING or `1` for webhook events
  #[serde(rename = "type")]
  pub webhook_type: EventWebhookType,
  /// Event data payload
  pub event: Option<EventBody>,
}

/// Discord Event Webhook Types
#[doc(hidden)]
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EventWebhookType {
  /// PING event sent to verify your Webhook Event URL is active
  PING = 0,
  /// Webhook event (details for event in [event body](EventBody) object)
  EVENT = 1,
  /// A webhook event type that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord Event Body Object
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct EventBody {
  /// [Event type](EventType)
  pub event_type: EventType,
  /// Timestamp of when the event occurred in [ISO8601 format](https://discord.com/developers/docs/reference#iso8601-datetime)
  pub timestamp: DateTime<Utc>,
  /// Data for the event. The shape depends on the [event type](EventType)
  pub data: Option<EventData>,
}

/// Discord Event Types
#[derive(Deserialize, Eq, Hash, PartialEq, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum EventType {
  /// Sent when an app was authorized by a user to a server or their account
  APPLICATION_AUTHORIZED,
  /// Entitlement was created
  ENTITLEMENT_CREATE,
  /// User was added to a Quest (currently unavailable)
  QUEST_USER_ENROLLMENT,
  /// An event type that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord Event Data
#[derive(Clone, Debug)]
pub enum EventData {
  /// Sent when an app was authorized by a user to a server or their account
  ApplicationAuthorized(Box<ApplicationAuthorizedEventData>),
  /// Entitlement was created
  EntitlementCreate(Entitlement),
  /// User was added to a Quest (currently unavailable)
  QuestUserEnrollment(Value),
  /// An event type that hasn't been implemented yet
  Unknown(Value),
}

/// Discord Application Authorized Event Data Object
#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationAuthorizedEventData {
  /// [Installation context](IntegrationType) for the authorization. Either guild ([`GUILD_INSTALL`](IntegrationType::GUILD_INSTALL)) if installed to a server or user ([`USER_INSTALL`](IntegrationType::USER_INSTALL)) if installed to a user's account
  pub integration_type: IntegrationType,
  /// User who authorized the app
  pub user: User,
  /// List of [scopes](https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes) the user authorized
  pub scopes: Vec<String>,
  /// Server which app was authorized for (when integration type is [`GUILD_INSTALL`](IntegrationType::GUILD_INSTALL))
  pub guild: Option<Guild>,
}

impl<'de> Deserialize<'de> for EventBody {
  fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let mut value = Value::deserialize(d)?;

    let event_type_value = value.get("type").ok_or_else(|| de::Error::custom("Expected a field \"type\""))?;
    let event_type = EventType::deserialize(event_type_value).map_err(de::Error::custom)?;

    let timestamp_str = value.get("timestamp").and_then(Value::as_str).ok_or_else(|| de::Error::custom("Expected a field \"timestamp\" of type str"))?;
    // Event timestamps are for some reason missing the timezone designator which is why NaiveDateTime is required for parsing
    let timestamp = NaiveDateTime::from_str(timestamp_str).map_err(|e| de::Error::custom(format!("Timestamp parsing failed: {:?}", e)))?.and_utc();

    let mut event_body = EventBody {
      event_type,
      timestamp,
      data: None,
    };

    if let Some(raw_data) = value.get_mut("data") {
      let event_data = match event_body.event_type {
        EventType::APPLICATION_AUTHORIZED => EventData::ApplicationAuthorized(Box::new(ApplicationAuthorizedEventData::deserialize(&*raw_data).map_err(de::Error::custom)?)),
        EventType::ENTITLEMENT_CREATE => EventData::EntitlementCreate(Entitlement::deserialize(&*raw_data).map_err(de::Error::custom)?),
        EventType::QUEST_USER_ENROLLMENT => EventData::QuestUserEnrollment(raw_data.take()),
        EventType::UNKNOWN => EventData::Unknown(raw_data.take()),
      };

      event_body.data = Some(event_data);
    }

    Ok(event_body)
  }
}
