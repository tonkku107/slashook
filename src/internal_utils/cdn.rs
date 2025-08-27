// Copyright 2025 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub fn pick_format(hash: &str, static_format: String, animated_format: Option<String>) -> (String, bool) {
  let Some(animated_format) = animated_format else {
    return (static_format, false);
  };

  if hash.starts_with("a_") {
    return (animated_format, true)
  }

  (static_format, false)
}
