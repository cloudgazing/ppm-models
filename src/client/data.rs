use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

/// Represets a message sent by the user.
#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OwnMessage {
	pub message_id: String,
	/// The message in plain readable text format.
	pub text: String,
	pub is_sent: bool,
}

impl<'a> OwnMessage {
	pub fn new(message_id: String, text: String, is_sent: bool) -> Self {
		Self {
			message_id,
			text,
			is_sent,
		}
	}

	pub fn new_pending(message_id: String, text: String) -> Self {
		Self {
			message_id,
			text,
			is_sent: false,
		}
	}

	pub fn change_sent(&mut self) {
		self.is_sent = true;
	}
}

/// Represends a message received from someone else.
#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OtherMessage {
	pub message_id: String,
	/// The message in plain readable text format.
	pub text: String,
}

impl<'a> OtherMessage {
	pub fn new(message_id: String, text: String) -> Self {
		Self { message_id, text }
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[serde(tag = "type", rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ChatMessage {
	OwnMessage(OwnMessage),
	OtherMessage(OtherMessage),
}

impl From<OwnMessage> for ChatMessage {
	fn from(message: OwnMessage) -> Self {
		Self::OwnMessage(message)
	}
}

impl From<OtherMessage> for ChatMessage {
	fn from(message: OtherMessage) -> Self {
		Self::OtherMessage(message)
	}
}

impl ChatMessage {
	pub fn own_message(message_id: String, text: String, is_sent: bool) -> Self {
		Self::OwnMessage(OwnMessage::new(message_id, text, is_sent))
	}

	pub fn other_message(message_id: String, text: String) -> Self {
		Self::OtherMessage(OtherMessage::new(message_id, text))
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatInfo {
	pub user_id: String,
	pub shared_secret: String,
	/// The display name set by the user.
	pub display_name: String,
	/// The number of unread messages.
	pub unread_count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatData {
	/// The Vec where all messages are stored.
	pub messages: Vec<ChatMessage>,
	/// Stores the id of every user message along with it's index for a quick lookup
	pub own_message_map: HashMap<String, usize>,
}

impl ChatData {
	pub fn update_msg_to_sent(&mut self, message_id: &str) {
		let index = self.own_message_map.get(message_id).unwrap();

		if let ChatMessage::OwnMessage(msg) = &mut self.messages[*index] {
			msg.is_sent = true;
		}
	}
}

pub type ChatId = String;
pub type ChatInfoMap = HashMap<ChatId, ChatInfo>;
pub type ChatDataMap = HashMap<ChatId, ChatData>;
