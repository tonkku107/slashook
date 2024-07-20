// Copyright 2024 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord polls

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use super::{
  Emoji,
  users::User,
};
use chrono::{DateTime, Utc};

/// Discord Poll Object
#[derive(Deserialize, Clone, Debug)]
pub struct Poll {
  /// The question of the poll. Only `text` is supported.
  pub question: PollMedia,
  /// Each of the answers available in the poll.
  pub answers: Vec<PollAnswer>,
  /// The time when the poll ends.
  pub expiry: Option<DateTime<Utc>>,
  /// Whether a user can select multiple answers
  pub allow_multiselect: bool,
  /// The [layout type](PollLayoutType) of the poll
  pub layout_type: PollLayoutType,
  /// The results of the poll
  pub results: Option<PollResults>,
}

/// Discord Poll Media Object
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PollMedia {
  /// The text of the field
  pub text: Option<String>,
  /// The emoji of the field
  pub emoji: Option<Emoji>,
}

/// Discord Poll Answer Object
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PollAnswer {
  /// The ID of the answer
  pub answer_id: Option<i64>,
  /// The data of the answer
  pub poll_media: PollMedia,
}

/// Discord Poll Layout Types
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum PollLayoutType {
  /// The, uhm, default layout type.
  DEFAULT = 1,
  /// Poll Layout Type that hasn't been implemented yet
  #[serde(other)]
  UNKNOWN,
}

/// Discord Poll Results Object
#[derive(Deserialize, Clone, Debug)]
pub struct PollResults {
  /// Whether the votes have been precisely counted
  pub is_finalized: bool,
  /// The counts for each answer
  pub answer_counts: Vec<PollAnswerCount>,
}

/// Discord Poll Answer Count Object
#[derive(Deserialize, Clone, Debug)]
pub struct PollAnswerCount {
  /// The `answer_id`
  pub id: i64,
  /// The number of votes for this answer
  pub count: i64,
  /// Whether the current user voted for this answer
  pub me_voted: bool,
}

/// Discord Poll Create Request Object
#[derive(Serialize, Clone, Debug)]
pub struct PollCreateRequest {
  /// The question of the poll. Only `text` is supported.
  pub question: PollMedia,
  /// Each of the answers available in the poll, up to 10
  pub answers: Vec<PollAnswer>,
  /// Number of hours the poll should be open for, up to 7 days
  pub duration: i64,
  /// Whether a user can select multiple answers
  pub allow_multiselect: bool,
  /// The [layout type](PollLayoutType) of the poll. Defaults to... [DEFAULT](PollLayoutType::DEFAULT)!
  pub layout_type: PollLayoutType,
}

/// Response from [get_poll_voters](super::messages::Message::get_poll_voters).
#[derive(Deserialize, Clone, Debug)]
pub struct PollVoters {
  /// Users who voted for this answer
  pub users: Vec<User>,
}

impl PollCreateRequest {
  /// Creates a new poll with a question. Defaults to default layout, no answers, 24h duration, and no multiselect.
  /// ```
  /// # use slashook::structs::polls::{PollCreateRequest, PollMedia};
  /// let poll = PollCreateRequest::new("Poll question here");
  /// let poll2 = PollCreateRequest::new(PollMedia::new().set_text("Another question here"));
  /// ```
  pub fn new<T: Into<PollMedia>>(question: T) -> Self {
    Self {
      question: question.into(),
      answers: Vec::new(),
      duration: 24,
      allow_multiselect: false,
      layout_type: PollLayoutType::DEFAULT,
    }
  }

  /// Adds an answer
  /// ```
  /// # use slashook::structs::{polls::{PollCreateRequest, PollAnswer}, Emoji};
  /// let poll = PollCreateRequest::new("Is this a good poll?")
  ///   .add_answer(PollAnswer::new().set_text("Yes").set_emoji(Emoji::new_standard_emoji("✅")))
  ///   .add_answer(PollAnswer::from("No").set_emoji(Emoji::new_custom_emoji("567088349484023818", "redtick", false)))
  ///   .add_answer("Maybe");
  /// ```
  pub fn add_answer<T: Into<PollAnswer>>(mut self, answer: T) -> Self {
    self.answers.push(answer.into());
    self
  }

  /// Sets the duration
  /// ```
  /// # use slashook::structs::polls::PollCreateRequest;
  /// let poll = PollCreateRequest::new("Shortest poll")
  ///   .set_duration(1);
  /// ```
  pub fn set_duration(mut self, duration: i64) -> Self {
    self.duration = duration;
    self
  }

  /// Sets if multiple answers are allowed
  /// ```
  /// # use slashook::structs::polls::PollCreateRequest;
  /// let poll = PollCreateRequest::new("This poll allows multiple answers")
  ///   .set_allow_multiselect(true);
  /// ```
  pub fn set_allow_multiselect(mut self, allow_multiselect: bool) -> Self {
    self.allow_multiselect = allow_multiselect;
    self
  }

  /// Sets the layout type
  /// ```
  /// # use slashook::structs::polls::{PollCreateRequest, PollLayoutType};
  /// let poll = PollCreateRequest::new("A poll")
  ///   .set_layout_type(PollLayoutType::DEFAULT);
  /// ```
  pub fn set_layout_type(mut self, layout_type: PollLayoutType) -> Self {
    self.layout_type = layout_type;
    self
  }
}

impl PollMedia {
  /// Creates a new empty PollMedia
  pub fn new() -> Self {
    Self {
      text: None,
      emoji: None,
    }
  }

  /// Sets the text
  /// ```
  /// # use slashook::structs::polls::PollMedia;
  /// let poll_media = PollMedia::new()
  ///   .set_text("Question or answer text");
  /// ```
  pub fn set_text<T: ToString>(mut self, text: T) -> Self {
    self.text = Some(text.to_string());
    self
  }

  /// Sets the emoji
  /// ```
  /// # use slashook::structs::{polls::PollMedia, Emoji};
  /// let poll_media = PollMedia::new()
  ///   .set_emoji(Emoji::new_standard_emoji("✅"));
  /// ```
  pub fn set_emoji(mut self, emoji: Emoji) -> Self {
    self.emoji = Some(emoji);
    self
  }
}

impl PollAnswer {
  /// Creates a new answer
  pub fn new() -> Self {
    Self {
      answer_id: None,
      poll_media: PollMedia::new(),
    }
  }

  /// Sets the poll media text. See [PollMedia::set_text].
  pub fn set_text<T: ToString>(mut self, text: T) -> Self {
    self.poll_media = self.poll_media.set_text(text);
    self
  }

  /// Sets the poll media emoji. See [PollMedia::set_emoji]
  pub fn set_emoji(mut self, emoji: Emoji) -> Self {
    self.poll_media = self.poll_media.set_emoji(emoji);
    self
  }

  /// Overwrites the whole poll media struct. See the other methods for editing the poll media.
  pub fn set_poll_media(mut self, poll_media: PollMedia) -> Self {
    self.poll_media = poll_media;
    self
  }
}

impl Default for PollMedia {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: ToString> From<T> for PollMedia {
  fn from(value: T) -> Self {
    Self::new().set_text(value)
  }
}

impl Default for PollAnswer {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Into<PollMedia>> From<T> for PollAnswer {
  fn from(value: T) -> Self {
    Self {
      answer_id: None,
      poll_media: value.into(),
    }
  }
}
