// Copyright 2023 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord monetization

use serde::{Deserialize, de::Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use chrono::{DateTime, Utc};
use bitflags::bitflags;
use super::Snowflake;
use crate::rest::{Rest, RestError};

/// Discord SKU Object
#[derive(Deserialize, Clone, Debug)]
pub struct SKU {
  /// ID of SKU
  pub id: Snowflake,
  /// [Type of SKU](SKUType)
  #[serde(rename = "type")]
  pub sku_type: SKUType,
  /// ID of the parent application
  pub application_id: Snowflake,
  /// Customer-facing name of your premium offering
  pub name: String,
  /// System-generated URL slug based on the SKU's name
  pub slug: String,
  /// [SKU flags](SKUFlags) combined as a [bitfield](https://en.wikipedia.org/wiki/Bit_field)
  pub flags: SKUFlags,
}

/// Discord SKU Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum SKUType {
  ///	Represents a recurring subscription
  SUBSCRIPTION = 5,
  /// System-generated group for each SUBSCRIPTION SKU created
  SUBSCRIPTION_GROUP = 6,
  /// An SKU type that hasn't been implemented yet
  UNKNOWN
}

bitflags! {
  /// Bitflags for Discord Message Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct SKUFlags: u32 {
    /// SKU is available for purchase
    const AVAILABLE = 1 << 2;
    /// Recurring SKU that can be purchased by a user and applied to a single server. Grants access to every user in that server.
    const GUILD_SUBSCRIPTION = 1 << 7;
    /// Recurring SKU purchased by a user for themselves. Grants access to the purchasing user in every server.
    const USER_SUBSCRIPTION = 1 << 8;
  }
}

/// Discord Entitlement Object
#[derive(Deserialize, Clone, Debug)]
pub struct Entitlement {
  /// ID of the entitlement
  pub id: Snowflake,
  /// ID of the SKU
  pub sku_id: Snowflake,
  /// ID of the parent application
  pub application_id: Snowflake,
  /// ID of the user that is granted access to the entitlement's sku
  pub user_id: Option<Snowflake>,
  /// [Type of entitlement](EntitlementType)
  #[serde(rename = "type")]
  pub entitlement_type: EntitlementType,
  /// Entitlement was deleted
  pub deleted: bool,
  /// Start date at which the entitlement is valid. Not present when using test entitlements.
  pub starts_at: Option<DateTime<Utc>>,
  /// Date at which the entitlement is no longer valid. Not present when using test entitlements.
  pub ends_at: Option<DateTime<Utc>>,
  /// ID of the guild that is granted access to the entitlement's sku
  pub guild_id: Option<Snowflake>,
}

/// Discord Entitlement Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum EntitlementType {
  /// Entitlement was purchased as an app subscription
  APPLICATION_SUBSCRIPTION = 8,
  /// An entitlement type that hasn't been implemented yet
  UNKNOWN
}

/// Options for fetching entitlements
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListEntitlementsOptions {
  /// User ID to look up entitlements for
  pub user_id: Option<Snowflake>,
  /// Optional comma-delimited list of SKU IDs to check entitlements for
  pub sku_ids: Option<String>,
  /// Retrieve entitlements before this entitlement ID
  pub before: Option<Snowflake>,
  /// Retrieve entitlements after this entitlement ID
  pub after: Option<Snowflake>,
  /// Number of entitlements to return, 1-100, default 100
  pub limit: Option<i64>,
  /// Guild ID to look up entitlements for
  pub guild_id: Option<Snowflake>,
  /// Whether or not ended entitlements should be omitted
  pub exclude_ended: Option<bool>,
}

/// Options for creating test entitlements
#[derive(Serialize, Clone, Debug)]
pub struct TestEntitlementOptions {
  /// ID of the SKU to grant the entitlement to
  pub sku_id: Snowflake,
  /// ID of the guild or user to grant the entitlement to
  pub owner_id: Snowflake,
  /// guild subscription or user subscription
  pub owner_type: EntitlementOwnerType,
}

/// Discord Entitlement Owner Types
#[derive(Serialize_repr, Clone, Debug)]
#[repr(u8)]
pub enum EntitlementOwnerType {
  /// For a guild subscription
  Guild = 1,
  /// For a user subscription
  User = 2,
}

impl SKU {
  /// Lists all SKUs
  pub async fn list_skus<T: ToString>(rest: &Rest, application_id: T) -> Result<Vec<SKU>, RestError> {
    rest.get(format!("applications/{}/skus", application_id.to_string())).await
  }
}

impl Entitlement {
  /// Lists all entitlements
  pub async fn list_entitlements<T: ToString>(rest: &Rest, application_id: T, options: ListEntitlementsOptions) -> Result<Vec<Entitlement>, RestError> {
    rest.get_query(format!("applications/{}/entitlements", application_id.to_string()), options).await
  }

  /// Creates a test entitlement
  pub async fn create_test_entitlement<T: ToString>(rest: &Rest, application_id: T, options: TestEntitlementOptions) -> Result<Entitlement, RestError> {
    rest.post(format!("applications/{}/entitlements", application_id.to_string()), options).await
  }

  /// Deletes a test entitlement
  pub async fn delete_test_entitlement<T: ToString>(&self, rest: &Rest, application_id: T) -> Result<(), RestError> {
    rest.delete(format!("applications/{}/entitlements/{}", application_id.to_string(), self.id)).await
  }
}

impl<'de> Deserialize<'de> for SKUFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}
