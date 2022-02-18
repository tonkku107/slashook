// Copyright 2022 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::{
  io::{Cursor, Error},
  pin::Pin
};
use crate::structs::{
  channels::Attachment,
  interactions::{InteractionCallback, Attachments}
};
use rocket::{
  http::Status,
  response::{self, Response},
  futures::{
    stream::Stream,
    task::{Context, Poll}
  }
};
use common_multipart_rfc7578::client::{
  multipart::{
    Body, Form, BoundaryGenerator
  },
  Error as MultipartError
};
use bytes::BytesMut;
use tokio_util::io::StreamReader;
use reqwest::multipart::Form as ReqwestForm;

pub struct ReqwestBoundary;
impl BoundaryGenerator for ReqwestBoundary {
  fn generate_boundary() -> String {
    ReqwestForm::new().boundary().to_string()
  }
}

// All this because the error type doesn't implement Into<std::io::Error> despite containing it...
pub struct FakeBody<'a>(Body<'a>);
impl<'a> Stream for FakeBody<'a> {
  type Item = Result<BytesMut, Error>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.0).poll_next(cx)
      .map(|o| o.map(|r| r.map_err(|e| {
        match e {
          MultipartError::HeaderWrite(io) => io,
          MultipartError::BoundaryWrite(io) => io,
          MultipartError::ContentRead(io) => io
        }
    })))
  }
}

pub fn handle_multipart(status: Status, mut callback: InteractionCallback) -> response::Result<'static> {
  let mut form = Form::new::<ReqwestBoundary>();

  let mut data = callback.data.unwrap();
  let files = data.files.take().unwrap();
  let mut attachments = data.take_attachments();

  for (i, file) in files.into_iter().enumerate() {
    form.add_reader_file(format!("files[{}]", i), Cursor::new(file.data), file.filename);
    if let Some(description) = file.description {
      attachments.push(Attachment::with_description(i, description));
    }
  }

  data.set_attachments(attachments);
  callback.data = Some(data);
  form.add_text("payload_json", serde_json::to_string(&callback).map_err(|_| Status::InternalServerError)?);
  let content_type = form.content_type();

  let body = FakeBody(form.into());
  let stream = StreamReader::new(body);
  Response::build()
    .raw_header("Content-Type", content_type)
    .streamed_body(stream)
    .status(status)
    .ok()
}
