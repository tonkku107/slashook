// Copyright 2021 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Various structs for working with Discord's objects

// Sub-mods categorized loosely based on Discord's documentation categories.
// Some structs receive their own "category" because they're impl heavy.
// Mods with just one struct will be exported without a category.

pub mod interactions;
pub mod embeds;
pub mod users;
pub mod guilds;
pub mod channels;
mod emojis;
pub use emojis::Emoji;
pub mod applications;
pub mod components;
pub mod utils;
mod permissions;
pub use permissions::Permissions;

// TODO: Useful Snowflake impls?
/// Alias for Discord snowflakes
pub type Snowflake = String;
