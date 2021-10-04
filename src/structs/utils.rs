// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Misc utility structs

use serde::{Serialize, Deserialize};
use crate::tokio::{fs, io::AsyncReadExt};
use std::convert::TryFrom;

/// Represents a color
///
/// This can be constructed from a hex string or u32 using the TryFrom trait.
/// ```
/// # use slashook::structs::utils::Color;
/// # use std::convert::TryFrom;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let color = Color::try_from("#c0ffee")?;
/// assert_eq!(color.0, 0xc0ffee);
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Color(pub u32);

/// Represents a file
#[derive(Clone, Debug)]
pub struct File {
  /// Name of the file
  pub filename: String,
  /// The bytes in the file
  pub data: Vec<u8>
}

impl Color {
  /// Returns a hex color code representation of the color
  /// ```
  /// # use slashook::structs::utils::Color;
  /// let color = Color::from(0xc0ffee);
  /// let hex = color.to_hex();
  /// assert_eq!(hex, "#c0ffee");
  /// ```
  pub fn to_hex(&self) -> String {
    format!("#{:06x}", self.0)
  }
}

impl TryFrom<String> for Color {
  type Error = std::num::ParseIntError;
  fn try_from(s: String) -> Result<Color, Self::Error> {
    let stripped = s.strip_prefix('#');
    let color_code = stripped.unwrap_or(&s);
    let parsed_color = u32::from_str_radix(color_code, 16)?;
    Ok(Color(parsed_color))
  }
}

impl TryFrom<&str> for Color {
  type Error = std::num::ParseIntError;
  fn try_from(s: &str) -> Result<Color, Self::Error> {
    let stripped = s.strip_prefix('#');
    let color_code = stripped.unwrap_or(s);
    let parsed_color = u32::from_str_radix(color_code, 16)?;
    Ok(Color(parsed_color))
  }
}

impl From<u32> for Color {
  fn from(n: u32) -> Color {
    Color(n)
  }
}

impl File {
  /// Create a new file from bytes
  /// ```
  /// # use slashook::structs::utils::File;
  /// let file = File::new("test.txt", "Test file");
  /// ```
  pub fn new<T: ToString, U: Into<Vec<u8>>>(filename: T, data: U) -> Self {
    Self {
      filename: filename.to_string(),
      data: data.into()
    }
  }

  /// Create a new file from a [Tokio File](https://docs.rs/tokio/latest/tokio/fs/struct.File.html)
  /// ```no_run
  /// # use slashook::structs::utils::File;
  /// use slashook::tokio::fs::File as TokioFile;
  /// # #[slashook::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let tokio_file = TokioFile::open("cat.png").await?;
  /// let file = File::from_file("cat.png", tokio_file).await?;
  /// # Ok(())
  /// # }
  /// ```
  pub async fn from_file<T: ToString>(filename: T, mut file: fs::File) -> std::io::Result<Self> {
    let mut data = Vec::new();
    file.read_to_end(&mut data).await?;
    Ok(Self {
      filename: filename.to_string(),
      data
    })
  }
}
