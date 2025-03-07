// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use serde::{de::{Deserializer, Error}, Deserialize};
use serde_json::Value;
use crate::structs::Snowflake;

pub fn snowflake_that_is_usually_a_string_but_sometimes_an_int_for_no_reason<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Snowflake>, D::Error> {
  match serde_json::Value::deserialize(d)? {
    Value::String(s) => Ok(Some(s)),
    Value::Number(i) => Ok(Some(i.to_string())),
    Value::Null => Ok(None),
    _ => Err(D::Error::custom("Expected string or number"))
  }
}
