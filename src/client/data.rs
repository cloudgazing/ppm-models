use std::collections::HashMap;

// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessagePackage<'a> {
	/// The sender's user id.
	pub user_id: &'a str,
	/// NewMessage struct encrypted and stored as bytes.
	pub contents: &'a [u8],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewMessage<'a> {
	// pub timestamp: DateTime<Utc>,
	pub text: &'a str,
}

impl<'a> NewMessage<'a> {
	pub fn new(text: &'a str) -> Self {
		Self { text }
	}
}

/// Represets a message sent by the user.
/// The message_id value is unique and can be used to verify it's integrity.
#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OwnMessage {
	// pub timestamp: DateTime<Utc>,
	/// Unique message identifier.
	pub message_id: String,
	pub is_sent: bool,
	/// The message in plain text format.
	pub text: String,
}

impl<'a> OwnMessage {
	pub fn new<S: Into<String>>(message_id: S, text: S) -> Self {
		Self {
			// timestamp: chrono::Utc::now(),
			message_id: message_id.into(),
			is_sent: false,
			text: text.into(),
		}
	}
}

/// Represends a message received from another user.
/// The message_id value is unique and used to verify it's integrity.
#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OtherMessage {
	// pub timestamp: DateTime<Utc>,
	/// Unique message identifier.
	pub message_id: String,
	/// The message in plain text format.
	pub text: String,
}

impl<'a> OtherMessage {
	pub fn new<S: Into<String>>(message_id: S, text: S) -> Self {
		Self {
			// timestamp: chrono::Utc::now(),
			message_id: message_id.into(),
			text: text.into(),
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[serde(tag = "type", rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ConversationMessage {
	OwnMessage(OwnMessage),
	OtherMessage(OtherMessage),
}

impl From<OwnMessage> for ConversationMessage {
	fn from(message: OwnMessage) -> Self {
		Self::OwnMessage(message)
	}
}

impl From<OtherMessage> for ConversationMessage {
	fn from(message: OtherMessage) -> Self {
		Self::OtherMessage(message)
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
	/// A unique readonly identifier.
	pub user_id: String,
	/// The display name set by the user.
	pub display_name: String,
	/// Stores the id of every user message along with it's index for a quick lookup
	pub own_msg_map: HashMap<String, usize>,
	/// The first value is the id of the first unread message, the latter is the total unread count
	pub unread_pointer: (String, usize),
	/// The Vec where all messages are stored.
	pub messages: Vec<ConversationMessage>,
}

impl Person {
	pub fn change_msg_to_sent(&mut self, message_id: &str) {
		let index = self.own_msg_map.get(message_id).unwrap();

		if let ConversationMessage::OwnMessage(msg) = &mut self.messages[*index] {
			msg.is_sent = true;
		}
	}
}

/// This may be stored on the server preferably encrypted with a key known only by the user.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserData {
	/// A unique readonly identifier.
	pub own_user_id: String,
	/// The display name set by the user.
	pub own_display_name: String,
	/// A map with a user_id as the key and it's correspondent person as the value.
	pub people: HashMap<String, Person>,
}
