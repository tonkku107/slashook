// Copyright 2024 slashook Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Structs related to Discord messages

use serde::{Deserialize, de::Deserializer};
use serde::{Serialize, ser::Serializer};
use serde_repr::Deserialize_repr;
use serde_json::Value;
use super::{
  Snowflake,
  applications::Application,
  channels::{Channel, ChannelType, ThreadCreateOptions},
  components::Component,
  embeds::Embed,
  emojis::Emoji,
  guilds::GuildMember,
  interactions::{IntegrationOwners, InteractionType, InteractionDataResolved},
  polls::{Poll, PollVoters},
  stickers::StickerItem,
  users::User,
  utils::File,
};
use crate::{
  rest::{Rest, RestError},
  commands::MessageResponse
};
use chrono::{DateTime, Utc};
use bitflags::bitflags;

/// Discord Message Object
#[derive(Deserialize, Clone, Debug)]
pub struct Message {
  /// Id of the message
  pub id: Snowflake,
  /// Id of the channel the message was sent in
  pub channel_id: Snowflake,
  /// Id of the guild the message was sent in
  pub guild_id: Option<Snowflake>,
  /// The author of this message (not guaranteed to be a valid user)
  pub author: User,
  /// Member properties for this message's author
  pub member: Option<GuildMember>,
  /// Contents of the message
  pub content: String,
  /// When this message was sent
  pub timestamp: DateTime<Utc>,
  /// When this message was edited (or None if never)
  pub edited_timestamp: Option<DateTime<Utc>>,
  /// Whether this was a TTS message
  pub tts: bool,
  /// Whether this message mentions everyone
  pub mention_everyone: bool,
  /// Users specifically mentioned in the message
  pub mentions: Vec<User>,
  /// Roles specifically mentioned in this message
  pub mention_roles: Vec<Snowflake>,
  /// Channels specifically mentioned in this message
  pub mention_channels: Option<Vec<ChannelMention>>,
  /// Any attached files
  pub attachments: Vec<Attachment>,
  /// Any embedded content
  pub embeds: Vec<Embed>,
  /// Reactions to the message
  pub reactions: Option<Vec<Reaction>>,
  /// Used for validating a message was sent
  pub nonce: Option<String>,
  /// Whether this message is pinned
  pub pinned: bool,
  /// If the message is generated by a webhook, this is the webhook's id
  pub webhook_id: Option<Snowflake>,
  /// [type of message](MessageType)
  #[serde(rename = "type")]
  pub message_type: MessageType,
  /// Sent with Rich Presence-related chat embeds
  pub activity: Option<MessageActivity>,
  /// Sent with Rich Presence-related chat embeds
  pub application: Option<Application>,
  /// If the message is a response to an [Interaction](https://discord.com/developers/docs/interactions/receiving-and-responding), this is the id of the interaction's application
  pub application_id: Option<Snowflake>,
  /// Data showing the source of a crosspost, channel follow add, pin, or reply message
  pub message_reference: Option<MessageReference>,
  /// [Message flags](MessageFlags) combined as a [bitfield](https://en.wikipedia.org/wiki/Bit_field)
  pub flags: Option<MessageFlags>,
  /// The message associated with the message_reference
  pub referenced_message: Option<Box<Message>>,
  /// Sent if the message is sent as a result of an interaction
  pub interaction_metadata: Option<MessageInteractionMetadata>,
  /// The thread that was started from this message, includes [thread member](super::channels::ThreadMember) object
  pub thread: Option<Channel>,
  /// Sent if the message contains components like buttons, action rows, or other interactive components
  pub components: Option<Vec<Component>>,
  /// Sent if the message contains stickers
  pub sticker_items: Option<Vec<StickerItem>>,
  /// A generally increasing integer (there may be gaps or duplicates) that represents the approximate position of the message in a thread, it can be used to estimate the relative position of the message in a thread in company with `total_message_sent` on parent thread
  pub position: Option<i64>,
  /// Data of the role subscription purchase or renewal that prompted this ROLE_SUBSCRIPTION_PURCHASE message
  pub role_subscription_data: Option<RoleSubscriptionData>,
  /// Data for users, members, channels, and roles in the message's [auto-populated select menus](crate::structs::components::SelectMenu)
  pub resolved: Option<InteractionDataResolved>,
  /// A poll!
  pub poll: Option<Poll>,
}

/// Discord Channel Mention Object
#[derive(Deserialize, Clone, Debug)]
pub struct ChannelMention {
  /// Id of the channel
  pub id: Snowflake,
  /// Id of the guild containing the channel
  pub guild_id: Snowflake,
  /// The [type of channel](ChannelType)
  #[serde(rename = "type")]
  pub channel_type: ChannelType,
  /// The name of the channel
  pub name: String
}

/// Discord Attachment Object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attachment {
  /// Attachment id
  pub id: Snowflake,
  /// Name of file attached
  pub filename: String,
  /// Description for the file
  pub description: Option<String>,
  /// The attachment's [media type](https://en.wikipedia.org/wiki/Media_type)
  pub content_type: Option<String>,
  /// Size of file in bytes
  pub size: i64,
  /// Source url of file
  pub url: String,
  /// A proxied url of file
  pub proxy_url: String,
  /// Height of file (if image)
  pub height: Option<i64>,
  /// Width of file (if image)
  pub width: Option<i64>,
  /// Whether this attachment is ephemeral
  pub ephemeral: Option<bool>,
  /// The duration of the audio file (currently for voice messages)
  pub duration_secs: Option<f64>,
  /// Base64 encoded bytearray representing a sampled waveform (currently for voice messages)
  pub waveform: Option<String>,
  /// [Attachment flags](AttachmentFlags) combined as a bitfield
  pub flags: Option<AttachmentFlags>
}

bitflags! {
  /// Bitflags for Discord Attachment Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct AttachmentFlags: u32 {
    /// This attachment has been edited using the remix feature on mobile
    const IS_REMIX = 1 << 2;
  }
}

/// Discord Reaction Object
#[derive(Deserialize, Clone, Debug)]
pub struct Reaction {
  /// Times this emoji has been used to react
  pub count: i64,
  /// Whether the current user reacted using this emoji
  pub me: bool,
  /// Emoji information
  pub emoji: Emoji
}

/// Discord Message Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MessageType {
  /// Normal message
  DEFAULT = 0,
  /// Member added to a group chat
  RECIPIENT_ADD = 1,
  /// Member removed from a group chat
  RECIPIENT_REMOVE = 2,
  /// Call notification
  CALL = 3,
  /// Channel name changed
  CHANNEL_NAME_CHANGE = 4,
  /// Channel icon changed
  CHANNEL_ICON_CHANGE = 5,
  /// A message was pinned
  CHANNEL_PINNED_MESSAGE = 6,
  /// A new member joined a server
  USER_JOIN = 7,
  /// A member boosted the server
  GUILD_BOOST = 8,
  /// A member boosted the server and reached tier 1
  GUILD_BOOST_TIER_1 = 9,
  /// A member boosted the server and reached tier 2
  GUILD_BOOST_TIER_2 = 10,
  /// A member boosted the server and reached tier 3
  GUILD_BOOST_TIER_3 = 11,
  /// A news channel followed
  CHANNEL_FOLLOW_ADD = 12,
  /// Server is no longer qualified for discovery
  GUILD_DISCOVERY_DISQUALIFIED = 14,
  /// Server is qualified for discovery again
  GUILD_DISCOVERY_REQUALIFIED = 15,
  /// First warning about losing discovery eligibility
  GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING = 16,
  /// Final warning about losing discovery eligibility
  GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING = 17,
  /// A new thread was created
  THREAD_CREATED = 18,
  /// Message is a reply to another message
  REPLY = 19,
  /// Message was sent from a chat input command
  CHAT_INPUT_COMMAND = 20,
  /// Message is the first message that started the thread
  THREAD_STARTER_MESSAGE = 21,
  /// Server setup tips
  GUILD_INVITE_REMINDER = 22,
  /// Message was sent from a context menu command
  CONTEXT_MENU_COMMAND = 23,
  /// AutoMod alert message
  AUTO_MODERATION_ACTION = 24,
  /// User purchased a server subscription
  ROLE_SUBSCRIPTION_PURCHASE = 25,
  /// Interaction premium upsell
  INTERACTION_PREMIUM_UPSELL = 26,
  /// A stage event was started
  STAGE_START = 27,
  /// A stage event was ended
  STAGE_END = 28,
  /// Someone became a speaker
  STAGE_SPEAKER = 29,
  /// Stage topic was changed
  STAGE_TOPIC = 31,
  /// Application premium subscription
  GUILD_APPLICATION_PREMIUM_SUBSCRIPTION = 32,
  /// A message type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Message Activity Object
#[derive(Deserialize, Clone, Debug)]
pub struct MessageActivity {
  /// [Type of message activity](MessageActivityType)
  #[serde(rename = "type")]
  pub activity_type: MessageActivityType,
  /// party_id from a [Rich Presence event](https://discord.com/developers/docs/rich-presence/how-to#updating-presence-update-presence-payload-fields)
  pub party_id: Option<String>
}

/// Discord Message Activity Types
#[derive(Deserialize_repr, Clone, Debug)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MessageActivityType {
  /// Invite to join activity
  JOIN = 1,
  /// Invite to spectate activity
  SPECTATE = 2,
  /// Invite to listen along
  LISTEN = 3,
  /// Invite to request to join
  JOIN_REQUEST = 5,
  /// Message activity type that hasn't been implemented yet
  UNKNOWN
}

/// Discord Message Reference Object
#[derive(Deserialize, Clone, Debug)]
pub struct MessageReference {
  /// Id of the originating message
  pub message_id: Option<Snowflake>,
  /// Id of the originating message's channel
  pub channel_id: Option<Snowflake>,
  /// Id of the originating message's guild
  pub guild_id: Option<Snowflake>,
  /// When sending, whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message, default true
  pub fail_if_not_exists: Option<bool>
}

bitflags! {
  /// Bitflags for Discord Message Flags
  #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
  pub struct MessageFlags: u32 {
    /// This message has been published to subscribed channels (via Channel Following)
    const CROSSPOSTED = 1 << 0;
    /// This message originated from a message in another channel (via Channel Following)
    const IS_CROSSPOST = 1 << 1;
    /// Do not include any embeds when serializing this message
    const SUPPRESS_EMBEDS = 1 << 2;
    /// The source message for this crosspost has been deleted (via Channel Following)
    const SOURCE_MESSAGE_DELETED = 1 << 3;
    /// This message came from the urgent message system
    const URGENT = 1 << 4;
    /// This message has an associated thread, with the same id as the message
    const HAS_THREAD = 1 << 5;
    /// This message is only visible to the user who invoked the Interaction
    const EPHEMERAL = 1 << 6;
    /// This message is an Interaction Response and the bot is "thinking"
    const LOADING = 1 << 7;
    /// This message failed to mention some roles and add their members to the thread
    const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
    /// This message will not trigger push and desktop notifications
    const SUPPRESS_NOTIFICATIONS = 1 << 12;
    /// This message is a voice message
    const IS_VOICE_MESSAGE = 1 << 13;
  }
}

/// Discord Message Interaction Metadata Object
#[derive(Deserialize, Clone, Debug)]
pub struct MessageInteractionMetadata {
  /// Id of the interaction
  pub id: Snowflake,
  /// The type of interaction
  #[serde(rename = "type")]
  pub interaction_type: Option<InteractionType>,
  /// ID of the user who triggered the interaction
  pub user_id: Snowflake,
  /// IDs for installation context(s) related to an interaction. Details in [Authorizing Integration Owners Object](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-authorizing-integration-owners-object)
  pub authorizing_integration_owners: IntegrationOwners,
  /// ID of the original response message, present only on [follow-up messages](https://discord.com/developers/docs/interactions/receiving-and-responding)
  pub original_response_message_id: Option<Snowflake>,
  /// ID of the message that contained interactive component, present only on messages created from component interactions
  pub interacted_message_id: Option<Snowflake>,
  /// Metadata for the interaction that was used to open the modal, present only on modal submit interactions
  pub triggering_interaction_metadata: Option<Box<MessageInteractionMetadata>>,
}

/// Discord Allowed Mentions Object
#[derive(Serialize, Clone, Debug)]
pub struct AllowedMentions {
  /// An array of [allowed mention types](AllowedMentionType) to parse from the content.
  pub parse: Option<Vec<AllowedMentionType>>,
  /// Array of role_ids to mention (Max size of 100)
  pub roles: Option<Vec<Snowflake>>,
  /// Array of user_ids to mention (Max size of 100)
  pub users: Option<Vec<Snowflake>>,
  /// For replies, whether to mention the author of the message being replied to (default false)
  pub replied_user: Option<bool>
}

/// Discord Allowed Mention Types
#[derive(Serialize, Clone, Debug)]
#[allow(non_camel_case_types)]
#[serde(rename_all = "lowercase")]
pub enum AllowedMentionType {
  /// Allowed to mention roles
  ROLES,
  /// Allowed to mention users
  USERS,
  /// Allowed to mention @everyone and @here
  EVERYONE
}

/// Discord Role Subscription Data Object
#[derive(Deserialize, Clone, Debug)]
pub struct RoleSubscriptionData {
  /// The id of the sku and listing that the user is subscribed to
  pub role_subscription_listing_id: Snowflake,
  /// The name of the tier that the user is subscribed to
  pub tier_name: String,
  /// The cumulative number of months that the user has been subscribed for
  pub total_months_subscribed: i64,
  /// Whether this notification is for a renewal rather than a new purchase
  pub is_renewal: bool,
}

/// Options for fetching multiple messages with [fetch_many](Message::fetch_many).
/// Only one of `around`, `before`, or `after` can be passed at once.
#[derive(Serialize, Default, Clone, Debug)]
pub struct MessageFetchOptions {
  /// Get messages around this message ID
  pub around: Option<Snowflake>,
  /// Get messages before this message ID
  pub before: Option<Snowflake>,
  /// Get messages after this message ID
  pub after: Option<Snowflake>,
  /// Max number of messages to return (1-100). Defaults to 50.
  pub limit: Option<i64>,
}

/// Options for fetching reactions with [get_reactions](Message::get_reactions) or poll voters with [get_poll_voters](Message::get_poll_voters).
#[derive(Serialize, Default, Clone, Debug)]
pub struct ReactionFetchOptions {
  /// Get users after this user ID
  pub after: Option<Snowflake>,
  /// Max number of users to return (1-100) Defaults to 25.
  pub limit: Option<i64>,
}

impl Message {
  /// Fetch a single message with a channel and message ID
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::messages::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let message = Message::fetch(&input.rest, "613430047285706767", "916413462467465246").await?;
  /// # }
  /// ```
  pub async fn fetch<T: ToString, U: ToString>(rest: &Rest, channel_id: T, message_id: U) -> Result<Self, RestError> {
    rest.get(format!("channels/{}/messages/{}", channel_id.to_string(), message_id.to_string())).await
  }

  /// Fetch multiple messages with a channel ID and options
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::messages::{Message, MessageFetchOptions};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let options = MessageFetchOptions::new().set_before("940762083820175440").set_limit(5);
  /// let messages = Message::fetch_many(&input.rest, "697138785317814292", options).await?;
  /// # }
  /// ```
  pub async fn fetch_many<T: ToString>(rest: &Rest, channel_id: T, options: MessageFetchOptions) -> Result<Vec<Self>, RestError> {
    rest.get_query(format!("channels/{}/messages", channel_id.to_string()), options).await
  }

  /// Send a new message to a channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::messages::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581372137963522", "Hello!").await?;
  /// # }
  /// ```
  pub async fn create<T: ToString, U: Into<MessageResponse>>(rest: &Rest, channel_id: T, message: U) -> Result<Self, RestError> {
    let mut message = message.into();
    let path = format!("channels/{}/messages", channel_id.to_string());
    if let Some(files) = message.files.take() {
      rest.post_files(path, message, files).await
    } else {
      rest.post(path, message).await
    }
  }

  /// Edit a message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::messages::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581372137963522", "Hello!").await?;
  /// let edited_msg = msg.edit(&input.rest, "Bye!").await?;
  /// # }
  /// ```
  pub async fn edit<T: Into<MessageResponse>>(&self, rest: &Rest, message: T) -> Result<Message, RestError> {
    let mut message = message.into();
    let path = format!("channels/{}/messages/{}", self.channel_id, self.id);
    if let Some(files) = message.files.take() {
      rest.patch_files(path, message, files).await
    } else {
      rest.patch(path, message).await
    }
  }

  /// Delete a message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::messages::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581372137963522", "Hello!").await?;
  /// msg.delete(&input.rest).await?;
  /// # }
  /// ```
  pub async fn delete(&self, rest: &Rest) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/messages/{}", self.channel_id, self.id)).await
  }

  /// Publish a message that was posted in an [Announcement channel](ChannelType::GUILD_ANNOUNCEMENT)
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::messages::Message;
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581598878105605", "Hello!").await?;
  /// msg.crosspost(&input.rest).await?;
  /// # }
  /// ```
  pub async fn crosspost(&self, rest: &Rest) -> Result<Message, RestError> {
    rest.post(format!("channels/{}/messages/{}/crosspost", self.channel_id, self.id), Value::Null).await
  }

  /// Add a reaction to a message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{messages::Message, Emoji};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581598878105605", "Hello!").await?;
  /// msg.create_reaction(&input.rest, &Emoji::new_standard_emoji("👋")).await?;
  /// # }
  /// ```
  pub async fn create_reaction(&self, rest: &Rest, emoji: &Emoji) -> Result<(), RestError> {
    rest.put(format!("channels/{}/messages/{}/reactions/{}/@me", &self.channel_id, &self.id, emoji.to_url_format()), Value::Null).await
  }

  /// Remove the bot's reaction to a message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{messages::Message, Emoji};
  /// # #[command(name = "example", description = "An example command")]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = Message::create(&input.rest, "344581598878105605", "Hello!").await?;
  /// let emoji = Emoji::new_standard_emoji("👋");
  /// msg.create_reaction(&input.rest, &emoji).await?;
  /// msg.delete_reaction(&input.rest, &emoji).await?;
  /// # }
  /// ```
  pub async fn delete_reaction(&self, rest: &Rest, emoji: &Emoji) -> Result<(), RestError> {
    self.delete_user_reaction(rest, emoji, "@me").await
  }

  /// Remove someone else's reaction to a message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{Emoji, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.delete_user_reaction(&input.rest, &Emoji::new_standard_emoji("👋"), input.user.id).await?;
  /// # }
  /// ```
  pub async fn delete_user_reaction<T: ToString>(&self, rest: &Rest, emoji: &Emoji, user_id: T) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/messages/{}/reactions/{}/{}", &self.channel_id, &self.id, emoji.to_url_format(), user_id.to_string())).await
  }

  /// Get the users who reacted to a message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{Emoji, messages::ReactionFetchOptions, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// let options = ReactionFetchOptions::new().set_limit(5);
  /// let reactions = msg.get_reactions(&input.rest, &Emoji::new_standard_emoji("👋"), options).await?;
  /// println!("{:?}", reactions);
  /// # }
  /// ```
  pub async fn get_reactions(&self, rest: &Rest, emoji: &Emoji, options: ReactionFetchOptions) -> Result<Vec<User>, RestError> {
    rest.get_query(format!("channels/{}/messages/{}/reactions/{}", &self.channel_id, &self.id, emoji.to_url_format()), options).await
  }

  /// Delete all reactions from a message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{Emoji, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.delete_all_reactions(&input.rest).await?;
  /// # }
  /// ```
  pub async fn delete_all_reactions(&self, rest: &Rest) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/messages/{}/reactions", &self.channel_id, &self.id)).await
  }

  /// Delete all reactions for a single emoji from the message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{Emoji, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.delete_all_reactions_for_emoji(&input.rest, &Emoji::new_standard_emoji("👋")).await?;
  /// # }
  /// ```
  pub async fn delete_all_reactions_for_emoji(&self, rest: &Rest, emoji: &Emoji) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/messages/{}/reactions/{}", &self.channel_id, &self.id, emoji.to_url_format())).await
  }

  /// Pin the message to the channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::interactions::ApplicationCommandType;
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.pin(&input.rest).await?;
  /// # }
  /// ```
  pub async fn pin(&self, rest: &Rest) -> Result<(), RestError> {
    rest.put(format!("channels/{}/pins/{}", self.channel_id, self.id), Value::Null).await
  }

  /// Unpin the message from the channel
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::interactions::ApplicationCommandType;
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.unpin(&input.rest).await?;
  /// # }
  /// ```
  pub async fn unpin(&self, rest: &Rest) -> Result<(), RestError> {
    rest.delete(format!("channels/{}/pins/{}", self.channel_id, self.id)).await
  }

  /// Start a thread from the message
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{channels::ThreadCreateOptions, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.start_thread(&input.rest, ThreadCreateOptions::new("A thread")).await?;
  /// # }
  /// ```
  pub async fn start_thread(&self, rest: &Rest, options: ThreadCreateOptions) -> Result<Channel, RestError> {
    rest.post(format!("channels/{}/messages/{}/threads", self.channel_id, self.id), options).await
  }

  /// Get a list of users that voted for this specific answer.
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::{messages::ReactionFetchOptions, interactions::ApplicationCommandType};
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// let options = ReactionFetchOptions::new();
  /// let voters = msg.get_poll_voters(&input.rest, 1, options).await?;
  /// println!("{:?}", voters.users);
  /// # }
  /// ```
  pub async fn get_poll_voters(&self, rest: &Rest, answer_id: i64, options: ReactionFetchOptions) -> Result<PollVoters, RestError> {
    rest.get_query(format!("channels/{}/polls/{}/answers/{}", self.channel_id, self.id, answer_id), options).await
  }

  /// Immediately ends the poll. You cannot end polls from other users.
  /// ```
  /// # #[macro_use] extern crate slashook;
  /// # use slashook::commands::{CommandInput, CommandResponder};
  /// # use slashook::structs::interactions::ApplicationCommandType;
  /// # #[command(name = "Example Message Context", command_type = ApplicationCommandType::MESSAGE)]
  /// # fn example(input: CommandInput, res: CommandResponder) {
  /// let msg = input.target_message.unwrap();
  /// msg.end_poll(&input.rest).await?;
  /// # }
  /// ```
  pub async fn end_poll(&self, rest: &Rest) -> Result<Message, RestError> {
    rest.post(format!("channels/{}/polls/{}/expire", self.channel_id, self.id), Value::Null).await
  }
}

impl Attachment {
  /// Creates an attachment object that can be used to tell discord to keep the attachment when editing.
  pub fn keep_with_id<T: ToString>(id: T) -> Self {
    Self {
      id: id.to_string(),
      filename: String::from(""),
      description: None,
      content_type: None,
      size: 0,
      url: String::from(""),
      proxy_url: String::from(""),
      height: None,
      width: None,
      ephemeral: None,
      duration_secs: None,
      waveform: None,
      flags: None
    }
  }

  pub(crate) fn from_file(id: Snowflake, file: &File) -> Self {
    Self {
      id,
      filename: String::from(""),
      description: file.description.clone(),
      content_type: None,
      size: 0,
      url: String::from(""),
      proxy_url: String::from(""),
      height: None,
      width: None,
      ephemeral: None,
      duration_secs: file.duration_secs,
      waveform: file.waveform.clone(),
      flags: None
    }
  }
}

impl AllowedMentions {
  /// Create a new allowed mentions object. By default doesn't allow any mentions
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::{AllowedMentions, AllowedMentionType};
  /// let allowed_mentions = AllowedMentions::new();
  /// let response = MessageResponse::from("<@1234> @everyone <@&1235> No pings.")
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn new() -> Self {
    Self {
      parse: Some(Vec::new()),
      roles: None,
      users: None,
      replied_user: None
    }
  }

  /// Add an allowed mention type to parse. You cannot use the user or role type at the same time as an array of allowed ids for that type
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::{AllowedMentions, AllowedMentionType};
  /// let allowed_mentions = AllowedMentions::new().add_parse(AllowedMentionType::USERS);
  /// let response = MessageResponse::from("<@1234> Get pinged. Not @everyone or <@&1235> tho.")
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn add_parse(mut self, parsed: AllowedMentionType) -> Self {
    let mut parse = self.parse.unwrap_or_default();
    parse.push(parsed);
    self.parse = Some(parse);
    self
  }

  /// Add a user ID to the list of allowed user mentions
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::{AllowedMentions, AllowedMentionType};
  /// # let user_id = String::from("1234");
  /// let message = format!("<@{}> Get pinged.", user_id);
  /// let allowed_mentions = AllowedMentions::new().add_user(user_id);
  /// let response = MessageResponse::from(message)
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn add_user(mut self, user: Snowflake) -> Self {
    let mut users = self.users.unwrap_or_default();
    users.push(user);
    self.users = Some(users);
    self
  }

  /// Add a role ID to the list of allowed role mentions
  /// ```
  /// # use slashook::commands::MessageResponse;
  /// # use slashook::structs::messages::{AllowedMentions, AllowedMentionType};
  /// # let role_id = String::from("1235");
  /// let message = format!("<@&{}> Get pinged.", role_id);
  /// let allowed_mentions = AllowedMentions::new().add_role(role_id);
  /// let response = MessageResponse::from(message)
  ///   .set_allowed_mentions(allowed_mentions);
  /// ```
  pub fn add_role(mut self, role: Snowflake) -> Self {
    let mut roles = self.roles.unwrap_or_default();
    roles.push(role);
    self.roles = Some(roles);
    self
  }

  /// Set if the replied user should be mentioned or not
  pub fn set_replied_user(mut self, b: bool) -> Self {
    self.replied_user = Some(b);
    self
  }
}

impl MessageFetchOptions {
  /// Creates a new empty MessageFetchOptions
  pub fn new() -> Self {
    Self {
      around: None,
      before: None,
      after: None,
      limit: None,
    }
  }

  /// Sets the message ID to search around.
  /// Also removes `before` and `after` if set.
  pub fn set_around<T: ToString>(mut self, around: T) -> Self {
    self.around = Some(around.to_string());
    self.before = None;
    self.after = None;
    self
  }

  /// Sets the message ID to search before.
  /// Also removes `around` and `after` if set.
  pub fn set_before<T: ToString>(mut self, before: T) -> Self {
    self.around = None;
    self.before = Some(before.to_string());
    self.after = None;
    self
  }

  /// Sets the message ID to search after.
  /// Also removes `around` and `before` if set.
  pub fn set_after<T: ToString>(mut self, after: T) -> Self {
    self.around = None;
    self.before = None;
    self.after = Some(after.to_string());
    self
  }

  /// Sets the limit for the amount of messages to fetch
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }
}

impl ReactionFetchOptions {
  /// Creates a new empty ReactionFetchOptions
  pub fn new() -> Self {
    Self {
      after: None,
      limit: None,
    }
  }

  /// Sets the user ID to search after.
  pub fn set_after<T: ToString>(mut self, after: T) -> Self {
    self.after = Some(after.to_string());
    self
  }

  /// Sets the limit for the amount of reactions to fetch
  pub fn set_limit(mut self, limit: i64) -> Self {
    self.limit = Some(limit);
    self
  }
}

impl<'de> Deserialize<'de> for MessageFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}

impl Serialize for MessageFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}

impl<'de> Deserialize<'de> for AttachmentFlags {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let bits = u32::deserialize(d)?;
    Ok(Self::from_bits_retain(bits))
  }
}

impl Serialize for AttachmentFlags {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u32(self.bits())
  }
}

impl Default for AllowedMentions {
  fn default() -> Self {
    Self::new()
  }
}
