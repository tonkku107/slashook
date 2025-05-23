// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Discord rest api handling

/// Discord API base URL
pub const API_URL: &str = "https://discord.com/api/v10";

use std::any::TypeId;
use serde::{Serialize, de::{DeserializeOwned, Error}};
use serde_json::{Value, json};
use crate::structs::{
  messages::Attachment,
  interactions::Attachments,
  utils::File
};
use reqwest::{
  Client,
  ClientBuilder,
  StatusCode,
  Response,
  multipart::{Form, Part},
  header::{HeaderMap, HeaderValue}
};
use thiserror::Error;

/// Type for errors from rest api calls
#[derive(Error, Debug)]
pub enum RestError {
  /// Represents an error that occurred within the reqwest library
  #[error("There was an error performing this request")]
  ReqwestError(#[from] reqwest::Error),
  /// Represents an error that occurred within the serde library
  #[error("Failed to (de)serialize data")]
  SerializationError(#[from] serde_json::Error),
  /// Represents an error for requests with a failed status
  #[error("Request failed with status {status}. Body: {body}")]
  RequestFailed {
    /// Status code of the request
    status: StatusCode,
    /// Body of the request
    body: String
  },
  /// The struct used to make a request is invalid
  #[error("Method cannot be used on this struct: {0}")]
  InvalidStruct(&'static str),
}

/// Handler for Discord API calls
#[derive(Clone, Default)]
pub struct Rest {
  client: Client
}

async fn handle_response<T: DeserializeOwned + 'static>(res: Response) -> Result<T, RestError> {
  let status = res.status();
  if status.is_client_error() || status.is_server_error() {
    let body = res.text().await?;
    return Err(RestError::RequestFailed{ status, body });
  }
  if TypeId::of::<T>() == TypeId::of::<()>() {
    return Ok(serde_json::from_value(Value::Null)?)
  };
  let body = res.json::<T>().await?;
  Ok(body)
}

fn handle_multipart<U: Serialize + Attachments>(mut json_data: U, files: Vec<File>) -> Result<Form, RestError> {
  let mut form_data = Form::new();
  let mut attachments = json_data.take_attachments();

  for (i, file) in files.into_iter().enumerate() {
    attachments.push(Attachment::from_file(i.to_string(), &file));
    let part = Part::bytes(file.data).file_name(file.filename);
    form_data = form_data.part(format!("files[{}]", i), part);
  }

  json_data.set_attachments(attachments);
  form_data = form_data.text("payload_json", serde_json::to_string(&json_data)?);
  Ok(form_data)
}

impl Rest {
  fn base_client_builder() -> ClientBuilder {
    Client::builder()
      .user_agent(crate::USER_AGENT)
  }

  /// Creates a new Rest handler without a token
  pub fn new() -> Self {
    Self::with_optional_token(None)
  }

  /// Creates a new Rest handler with a token
  pub fn with_token(token: String) -> Self {
    Self::with_optional_token(Some(token))
  }

  /// Creates a new Rest handler with or without a token
  pub fn with_optional_token(token: Option<String>) -> Self {
    let mut client = Self::base_client_builder();

    if let Some(mut token) = token {
      if !token.starts_with("Bot") && !token.starts_with("Bearer") {
        token = format!("Bot {}", token);
      }

      let mut headers = HeaderMap::new();
      let mut auth = HeaderValue::from_str(token.as_str()).unwrap();
      auth.set_sensitive(true);
      headers.insert("Authorization", auth);
      client = client.default_headers(headers);
    }

    Self {
      client: client.build().unwrap()
    }
  }

  /// Creates a new Rest handler with an access token from client credentials grant
  pub async fn with_client_credentials(client_id: String, client_secret: String, scopes: Vec<String>) -> Result<Self, RestError> {
    let temp_client = Self::base_client_builder().build()?;

    let req = temp_client.post(format!("{}/oauth2/token", API_URL)).form(&json! ({
      "client_id": client_id,
      "client_secret": client_secret,
      "grant_type": "client_credentials",
      "scope": scopes.join(" ")
    }));
    let res = req.send().await?;
    let body: serde_json::Value = res.json().await?;

    let token = body.get("access_token")
      .ok_or_else(|| serde_json::Error::missing_field("access_token"))?.as_str()
      .ok_or_else(|| serde_json::Error::custom("access_token was not a string"))?;

    Ok(Self::with_token(format!("Bearer {}", token)))
  }

  /// Make a get request
  pub async fn get<T: DeserializeOwned + 'static>(&self, path: String) -> Result<T, RestError> {
    let req = self.client.get(format!("{}/{}", API_URL, path));
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a get request with query parameters
  pub async fn get_query<T: DeserializeOwned + 'static, U: Serialize>(&self, path: String, query: U) -> Result<T, RestError> {
    let req = self.client.get(format!("{}/{}", API_URL, path))
      .query(&query);
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a post request
  pub async fn post<T: DeserializeOwned + 'static, U: Serialize>(&self, path: String, data: U) -> Result<T, RestError> {
    let req = self.client.post(format!("{}/{}", API_URL, path))
      .json(&data);
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a post request including files
  pub async fn post_files<T: DeserializeOwned + 'static, U: Serialize + Attachments>(&self, path: String, json_data: U, files: Vec<File>) -> Result<T, RestError> {
    let form_data = handle_multipart(json_data, files)?;
    let req = self.client.post(format!("{}/{}", API_URL, path))
      .multipart(form_data);
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a patch request
  pub async fn patch<T: DeserializeOwned + 'static, U: Serialize>(&self, path: String, data: U) -> Result<T, RestError> {
    let req = self.client.patch(format!("{}/{}", API_URL, path))
      .json(&data);
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a patch request including files
  pub async fn patch_files<T: DeserializeOwned + 'static, U: Serialize + Attachments>(&self, path: String, json_data: U, files: Vec<File>) -> Result<T, RestError> {
    let form_data = handle_multipart(json_data, files)?;
    let req = self.client.patch(format!("{}/{}", API_URL, path))
      .multipart(form_data);
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a put request
  pub async fn put<T: DeserializeOwned + 'static, U: Serialize>(&self, path: String, data: U) -> Result<T, RestError> {
    let req = self.client.put(format!("{}/{}", API_URL, path))
      .json(&data);
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a delete request
  pub async fn delete<T: DeserializeOwned + 'static>(&self, path: String) -> Result<T, RestError> {
    let req = self.client.delete(format!("{}/{}", API_URL, path));
    let res = req.send().await?;
    handle_response(res).await
  }
}

impl std::fmt::Debug for Rest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Rest").finish()
  }
}
