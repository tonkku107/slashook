// Copyright 2022 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate ring;
extern crate hex;
mod signature_headers;
mod multipart;

use super::{Config, commands::handler::RocketCommand};
use super::structs::interactions::{Interaction, InteractionType, InteractionCallback, InteractionCallbackType};
use signature_headers::SignatureHeaders;
use rocket::{
  http::Status,
  request::Request,
  response::{self, Response, Responder, content},
  State,
  tokio::sync::{mpsc, oneshot}
};
use serde_json::{Value, json};
use ring::signature;

enum Res {
  Raw {
    status: Status,
    json: Value,
  },
  Response {
    status: Status,
    data: Box<InteractionCallback>
  }
}

impl<'r> Responder<'r, 'static> for Res {
  fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
    let mut response = Response::build();

    match self {
      Self::Raw{ status, json } => {
        response
          .merge(content::RawJson(json.to_string()).respond_to(req)?)
          .status(status);
      },

      Self::Response{ status, data } => {
        if data.data.as_ref().map_or(false, |d| d.files.is_some()) {
          response.merge(multipart::handle_multipart(*data)?);
        } else {
          let json = serde_json::to_string(&data).map_err(|_| Status::InternalServerError)?;
          response.merge(content::RawJson(json).respond_to(req)?);
        }
        response.status(status);
      }
    }

    response
      .raw_header("User-Agent", crate::USER_AGENT)
      .ok()
  }
}

fn verify_signature(body: &[u8], headers: SignatureHeaders, public_key: &str) -> bool {
  let decoding_pubkey = hex::decode(public_key);
  let decoding_signature = hex::decode(headers.signature);
  if decoding_pubkey.is_err() || decoding_signature.is_err() { return false }

  let decoded_pubkey: &[u8] = &decoding_pubkey.unwrap();
  let decoded_signature: &[u8] = &decoding_signature.unwrap();

  let usable_pubkey = signature::UnparsedPublicKey::new(&signature::ED25519, decoded_pubkey);
  let message: &[u8] = &[headers.timestamp, body].concat();

  usable_pubkey.verify(message, decoded_signature).is_ok()
}

#[post("/", data = "<body>")]
async fn index(body: &[u8], headers: SignatureHeaders<'_>, config: &State<Config>, cmd_sender: &State<mpsc::UnboundedSender::<RocketCommand>>) -> Res {

  if !verify_signature(body, headers, &config.public_key) {
    return Res::Raw{ status: Status::Unauthorized, json: json!({ "error": "Bad signature" })}
  }

  let interaction: Interaction = match serde_json::from_slice(body) {
    Ok(i) => i,
    Err(err) => {
      eprintln!("Received bad request body from Discord. Error: {}", err);
      return Res::Raw{ status: Status::BadRequest, json: json!({ "error": "Bad body" })}
    }
  };

  match interaction.interaction_type {
    InteractionType::PING => {
      let response = InteractionCallback{
        response_type: InteractionCallbackType::PONG,
        data: None
      };
      Res::Raw{ status: Status::Ok, json: json!(response) }
    },

    InteractionType::UNKNOWN => {
      Res::Raw{ status: Status::NotFound, json: json!({ "error": "Unknown interaction type" }) }
    },

    _ => {
      let (handler_send, handler_respond) = oneshot::channel::<anyhow::Result<InteractionCallback>>();
      cmd_sender.send(RocketCommand(interaction, config.bot_token.clone(), handler_send)).expect("Cannot execute handler");
      let response = handler_respond.await.unwrap();

      match response {
        Err(err) => {
          eprintln!("Error when processing command: {:?}", err);
          Res::Raw{ status: Status::InternalServerError, json: json!({ "error": "Handler failed" }) }
        },
        Ok(res) => Res::Response{ status: Status::Ok, data: Box::new(res) }
      }
    }
  }
}

#[catch(404)]
fn not_found() -> Res {
  Res::Raw{ status: Status::NotFound, json: json!({ "error": "Not found" }) }
}

#[catch(default)]
fn default_error() -> Res {
  Res::Raw{ status: Status::InternalServerError, json: json!({ "error": "Unexpected error" }) }
}

pub(crate) async fn start(config: Config, sender: mpsc::UnboundedSender::<RocketCommand>) {
  let figment = rocket::Config::figment()
    .merge(("address", config.ip))
    .merge(("port", config.port))
    .merge(("ident", crate::USER_AGENT))
    .merge(("log_level", rocket::config::LogLevel::Off));

  let result = rocket::custom(figment)
    .mount("/", routes![index])
    .register("/", catchers![not_found, default_error])
    .manage(config)
    .manage(sender)
    .launch()
    .await;

  if let Err(error) = result {
    panic!("Couldn't start web server: {}", error);
  }
}
