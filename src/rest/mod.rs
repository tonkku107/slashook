// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub const API_URL: &str = "https://discord.com/api/v9";

extern crate reqwest;
use serde::{Serialize, de::DeserializeOwned};
use crate::structs::utils::File;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct RestError {
  message: String
}

impl RestError {
  pub fn new(msg: String) -> Self {
    Self { message: msg }
  }
}

impl std::fmt::Display for RestError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for RestError {}

async fn handle_response<T: DeserializeOwned>(res: reqwest::Response) -> Result<T> {
  let status = res.status();
  if status.is_client_error() || status.is_server_error() {
    return Err(Box::new(RestError::new(format!("Status {}, Body: {}", status.as_u16(), res.text().await?))))
  }
  let body = res.json::<T>().await?;
  Ok(body)
}

fn handle_multipart<U: Serialize>(json_data: U, files: Vec<File>) -> Result<reqwest::multipart::Form> {
  let mut form_data = reqwest::multipart::Form::new()
    .text("payload_json", serde_json::to_string(&json_data)?);
  for file in files.into_iter() {
    let filename = file.filename.clone();
    let part = reqwest::multipart::Part::bytes(file.data).file_name(file.filename);
    form_data = form_data.part(filename, part);
  }
  Ok(form_data)
}

pub async fn get<T: DeserializeOwned>(path: String) -> Result<T> {
  let client = reqwest::Client::new();
  let res = client.get(format!("{}/{}", API_URL, path))
    .send().await?;
  handle_response(res).await
}

pub async fn post<T: DeserializeOwned, U: Serialize>(path: String, data: U) -> Result<T> {
  let client = reqwest::Client::new();
  let res = client.post(format!("{}/{}", API_URL, path))
    .json(&data).send().await?;
  handle_response(res).await
}

pub async fn post_files<T: DeserializeOwned, U: Serialize>(path: String, json_data: U, files: Vec<File>) -> Result<T> {
  let client = reqwest::Client::new();
  let form_data = handle_multipart(json_data, files)?;
  let res = client.post(format!("{}/{}", API_URL, path))
    .multipart(form_data).send().await?;
  handle_response(res).await
}

pub async fn patch<T: DeserializeOwned, U: Serialize>(path: String, data: U) -> Result<T> {
  let client = reqwest::Client::new();
  let res = client.patch(format!("{}/{}", API_URL, path))
    .json(&data).send().await?;
  handle_response(res).await
}

pub async fn patch_files<T: DeserializeOwned, U: Serialize>(path: String, json_data: U, files: Vec<File>) -> Result<T> {
  let client = reqwest::Client::new();
  let form_data = handle_multipart(json_data, files)?;
  let res = client.patch(format!("{}/{}", API_URL, path))
    .multipart(form_data).send().await?;
  handle_response(res).await
}

pub async fn delete(path: String) -> Result<()> {
  let client = reqwest::Client::new();
  let res = client.delete(format!("{}/{}", API_URL, path))
    .send().await?;
  handle_response(res).await
}
