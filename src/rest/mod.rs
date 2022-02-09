// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Discord rest api handling

/// Discord API base URL
pub const API_URL: &str = "https://discord.com/api/v9";

use serde::{Serialize, de::DeserializeOwned};
use crate::structs::utils::File;
use reqwest::{
  StatusCode,
  Response
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
  }
}

/// Handler for Discord API calls
#[derive(Clone, Default)]
pub struct Rest {
  token: Option<String>,
}

async fn handle_response<T: DeserializeOwned>(res: Response) -> Result<T, RestError> {
  let status = res.status();
  if status.is_client_error() || status.is_server_error() {
    let body = res.text().await?;
    return Err(RestError::RequestFailed{ status, body });
  }
  let body = res.json::<T>().await?;
  Ok(body)
}

fn handle_multipart<U: Serialize>(json_data: U, files: Vec<File>) -> Result<reqwest::multipart::Form, RestError> {
  let mut form_data = reqwest::multipart::Form::new()
    .text("payload_json", serde_json::to_string(&json_data)?);
  for file in files.into_iter() {
    let filename = file.filename.clone();
    let part = reqwest::multipart::Part::bytes(file.data).file_name(file.filename);
    form_data = form_data.part(filename, part);
  }
  Ok(form_data)
}

impl Rest {
  /// Creates a new Rest handler without a token
  pub fn new() -> Self {
    Self { token: None }
  }

  /// Creates a new Rest handler with a token
  pub fn with_token(token: String) -> Self {
    Self { token: Some(token) }
  }

  /// Creates a new Rest handler with or without a token
  pub fn with_optional_token(token: Option<String>) -> Self {
    Self { token }
  }

  /// Make a get request
  pub async fn get<T: DeserializeOwned>(&self, path: String) -> Result<T, RestError> {
    let mut req = reqwest::Client::new()
      .get(format!("{}/{}", API_URL, path));
    if let Some(token) = &self.token {
      req = req.header("Authorization", format!("Bot {}", token));
    }
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a get request with query parameters
  pub async fn get_query<T: DeserializeOwned, U: Serialize>(&self, path: String, query: U) -> Result<T, RestError> {
    let mut req = reqwest::Client::new()
      .get(format!("{}/{}", API_URL, path))
      .query(&query);
    if let Some(token) = &self.token {
      req = req.header("Authorization", format!("Bot {}", token));
    }
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a post request
  pub async fn post<T: DeserializeOwned, U: Serialize>(&self, path: String, data: U) -> Result<T, RestError> {
    let mut req = reqwest::Client::new()
      .post(format!("{}/{}", API_URL, path))
      .json(&data);
    if let Some(token) = &self.token {
      req = req.header("Authorization", format!("Bot {}", token));
    }
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a post request including files
  pub async fn post_files<T: DeserializeOwned, U: Serialize>(&self, path: String, json_data: U, files: Vec<File>) -> Result<T, RestError> {
    let form_data = handle_multipart(json_data, files)?;
    let mut req = reqwest::Client::new()
      .post(format!("{}/{}", API_URL, path))
      .multipart(form_data);
    if let Some(token) = &self.token {
      req = req.header("Authorization", format!("Bot {}", token));
    }
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a patch request
  pub async fn patch<T: DeserializeOwned, U: Serialize>(&self, path: String, data: U) -> Result<T, RestError> {
    let mut req = reqwest::Client::new()
      .patch(format!("{}/{}", API_URL, path))
      .json(&data);
    if let Some(token) = &self.token {
      req = req.header("Authorization", format!("Bot {}", token));
    }
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a patch request including files
  pub async fn patch_files<T: DeserializeOwned, U: Serialize>(&self, path: String, json_data: U, files: Vec<File>) -> Result<T, RestError> {
    let form_data = handle_multipart(json_data, files)?;
    let mut req = reqwest::Client::new()
      .patch(format!("{}/{}", API_URL, path))
      .multipart(form_data);
    if let Some(token) = &self.token {
      req = req.header("Authorization", format!("Bot {}", token));
    }
    let res = req.send().await?;
    handle_response(res).await
  }

  /// Make a delete request
  pub async fn delete(&self, path: String) -> Result<(), RestError> {
    let mut req = reqwest::Client::new()
      .delete(format!("{}/{}", API_URL, path));
    if let Some(token) = &self.token {
      req = req.header("Authorization", format!("Bot {}", token));
    }
    let res = req.send().await?;

    let status = res.status();
    if status.is_client_error() || status.is_server_error() {
      let body = res.text().await?;
      return Err(RestError::RequestFailed{ status, body });
    }
    Ok(())
  }
}

impl std::fmt::Debug for Rest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Rest").finish()
  }
}
