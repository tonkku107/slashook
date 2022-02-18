// Copyright 2022 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use rocket::{
  http::Status,
  request::{Outcome, Request, FromRequest}
};

pub struct SignatureHeaders<'r> {
  pub signature: &'r[u8],
  pub timestamp: &'r[u8]
}

#[derive(Debug)]
pub enum SignatureHeaderError {
  MissingSignature,
  MissingTimestamp
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SignatureHeaders<'r> {
  type Error = SignatureHeaderError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let sig = request.headers().get_one("X-Signature-Ed25519");
    if sig.is_none() {
      return Outcome::Failure((Status::Unauthorized, SignatureHeaderError::MissingSignature))
    }

    let ts = request.headers().get_one("X-Signature-Timestamp");
    if ts.is_none() {
      return Outcome::Failure((Status::Unauthorized, SignatureHeaderError::MissingTimestamp))
    }

    Outcome::Success(SignatureHeaders{ signature: sig.unwrap().as_bytes(), timestamp: ts.unwrap().as_bytes() })
  }
}
