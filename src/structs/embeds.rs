// Copyright 2022 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord embeds

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, TimeZone};
use std::convert::TryInto;
use super::utils::Color;

/// Discord Embed Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Embed {
  /// Title of embed
  pub title: Option<String>,
  /// Type of embed (always "rich" for webhook embeds)
  #[serde(rename = "type")]
  pub embed_type: Option<String>,
  /// Description of embed
  pub description: Option<String>,
  /// Url of embed
  pub url: Option<String>,
  /// Timestamp of embed content
  pub timestamp: Option<DateTime<Utc>>,
  /// Color code of the embed
  pub color: Option<Color>,
  /// Footer information
  pub footer: Option<EmbedFooter>,
  /// Image information
  pub image: Option<EmbedImage>,
  /// Thumbnail information
  pub thumbnail: Option<EmbedThumbnail>,
  /// Video information
  pub video: Option<EmbedVideo>,
  /// Provider information
  pub provider: Option<EmbedProvider>,
  /// Author information
  pub author: Option<EmbedAuthor>,
  /// Fields information
  pub fields: Option<Vec<EmbedField>>,
}

/// Discord Embed Thumbnail Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedThumbnail {
  /// Source url of thumbnail (only supports http(s) and attachments)
  pub url: String,
  /// A proxied url of the thumbnail
  pub proxy_url: Option<String>,
  /// Height of thumbnail
  pub height: Option<i64>,
  /// Width of thumbnail
  pub width: Option<i64>
}

/// Discord Embed Video Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedVideo {
  /// Source url of video
  pub url: Option<String>,
  /// A proxied url of the video
  pub proxy_url: Option<String>,
  /// Height of video
  pub height: Option<i64>,
  /// Width of video
  pub width: Option<i64>
}

/// Discord Embed Image Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedImage {
  /// Source url of image (only supports http(s) and attachments)
  pub url: String,
  /// A proxied url of the image
  pub proxy_url: Option<String>,
  /// Height of image
  pub height: Option<i64>,
  /// Width of image
  pub width: Option<i64>
}

/// Discord Embed Provider Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedProvider {
  /// Name of provider
  pub name: Option<String>,
  /// Url of provider
  pub url: Option<String>
}

/// Discord Embed Author Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedAuthor {
  /// Name of author
  pub name: String,
  /// Url of author
  pub url: Option<String>,
  /// Url of author icon (only supports http(s) and attachments)
  pub icon_url: Option<String>,
  /// A proxied url of author icon
  pub proxy_icon_url: Option<String>
}

/// Discord Embed Footer Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedFooter {
  /// Footer text
  pub text: String,
  /// Url of footer icon (only supports http(s) and attachments)
  pub icon_url: Option<String>,
  /// A proxied url of footer icon
  pub proxy_icon_url: Option<String>
}

/// Discord Embed Field Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedField {
  /// Name of the field
  pub name: String,
  /// Value of the field
  pub value: String,
  /// Whether or not this field should display inline
  pub inline: Option<bool>
}

impl Embed {
  /// Creates a new embed
  pub fn new() -> Self {
    Self {
      title: None,
      embed_type: Some(String::from("rich")),
      description: None,
      url: None,
      timestamp: None,
      color: None,
      footer: None,
      image: None,
      thumbnail: None,
      video: None,
      provider: None,
      author: None,
      fields: None
    }
  }

  /// Set the title of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new()
  ///   .set_title("My cool title!");
  /// assert_eq!(embed.title, Some(String::from("My cool title!")));
  /// ```
  pub fn set_title<T: ToString>(mut self, title: T) -> Self {
    self.title = Some(title.to_string());
    self
  }

  /// Set the description of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new()
  ///   .set_description("A good description");
  /// assert_eq!(embed.description, Some(String::from("A good description")));
  /// ```
  pub fn set_description<T: ToString>(mut self, descrption: T) -> Self {
    self.description = Some(descrption.to_string());
    self
  }

  /// Set the url of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new()
  ///   .set_url("https://example.com");
  /// assert_eq!(embed.url, Some(String::from("https://example.com")));
  /// ```
  pub fn set_url<T: ToString>(mut self, url: T) -> Self {
    self.url = Some(url.to_string());
    self
  }

  /// Set the timestamp of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// # use slashook::chrono;
  /// let timestamp = chrono::Local::now();
  /// let embed = Embed::new()
  ///   .set_timestamp(timestamp);
  /// ```
  pub fn set_timestamp<Tz: TimeZone>(mut self, timestamp: DateTime<Tz>) -> Self {
    self.timestamp = Some(timestamp.with_timezone(&Utc));
    self
  }

  /// Set the color of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let embed = Embed::new()
  ///   .set_color("#c0ffee")?;
  /// assert_eq!(embed.color.unwrap().0, 0xc0ffee);
  /// # Ok(())
  /// # }
  /// ```
  pub fn set_color<T: TryInto<Color>>(mut self, color: T) -> Result<Self, T::Error> {
    let color = color.try_into()?;
    self.color = Some(color);
    Ok(self)
  }

  /// Set the footer of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new()
  ///   .set_footer("A sneaky footer", None::<String>);
  /// assert_eq!(embed.footer.unwrap().text, String::from("A sneaky footer"));
  /// ```
  pub fn set_footer<T: ToString, U: ToString>(mut self, text: T, icon_url: Option<U>) -> Self {
    self.footer = Some(EmbedFooter{
      text: text.to_string(),
      icon_url: icon_url.map(|i| i.to_string()),
      proxy_icon_url: None
    });
    self
  }

  /// Set the image of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new()
  ///   .set_image("https://canary.discord.com/assets/7c8f476123d28d103efe381543274c25.png");
  /// assert_eq!(embed.image.unwrap().url, String::from("https://canary.discord.com/assets/7c8f476123d28d103efe381543274c25.png"));
  /// ```
  pub fn set_image<T: ToString>(mut self, url: T) -> Self {
    self.image = Some(EmbedImage {
      url: url.to_string(),
      proxy_url: None,
      height: None,
      width: None
    });
    self
  }

  /// Set the thumbnail of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new()
  ///   .set_thumbnail("https://canary.discord.com/assets/7c8f476123d28d103efe381543274c25.png");
  /// assert_eq!(embed.thumbnail.unwrap().url, String::from("https://canary.discord.com/assets/7c8f476123d28d103efe381543274c25.png"));
  /// ```
  pub fn set_thumbnail<T: ToString>(mut self, url: T) -> Self {
    self.thumbnail = Some(EmbedThumbnail {
      url: url.to_string(),
      proxy_url: None,
      height: None,
      width: None
    });
    self
  }

  /// Set the author of the embed
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new()
  ///   .set_author("A Discord user", None::<String>, Some("https://canary.discord.com/assets/7c8f476123d28d103efe381543274c25.png"));
  /// assert_eq!(embed.author.unwrap().name, String::from("A Discord user"));
  /// ```
  pub fn set_author<T: ToString, U: ToString, V: ToString>(mut self, name: T, url: Option<U>, icon_url: Option<V>) -> Self {
    self.author = Some(EmbedAuthor {
      name: name.to_string(),
      url: url.map(|u| u.to_string()),
      icon_url: icon_url.map(|i| i.to_string()),
      proxy_icon_url: None
    });
    self
  }

  /// Add a field to the embed. An embed can have up to 25 fields.
  /// ```
  /// # use slashook::structs::embeds::Embed;
  /// let embed = Embed::new()
  ///   .add_field("Field title", "Field description", false);
  /// assert_eq!(embed.fields.unwrap()[0].name, String::from("Field title"));
  /// ```
  pub fn add_field<T: ToString, U: ToString>(mut self, name: T, value: U, inline: bool) -> Self {
    let field = EmbedField {
      name: name.to_string(),
      value: value.to_string(),
      inline: Some(inline)
    };
    if self.fields.is_none() {
      self.fields = Some(vec![field]);
    } else if let Some(mut fields) = self.fields {
      fields.push(field);
      self.fields = Some(fields);
    }
    self
  }
}

impl Default for Embed {
  fn default() -> Self {
    Self::new()
  }
}
