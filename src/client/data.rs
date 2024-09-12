use std::collections::HashMap;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

/// Represets a message sent by the user.
#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi)]
pub struct OwnMessage {
	pub message_id: String,
	/// The message in plain readable text format.
	pub text: String,
	pub timestamp: DateTime<Local>,
	pub is_sent: bool,
}

impl<'a> OwnMessage {
	pub fn new(message_id: String, text: String, timestamp: DateTime<Local>, is_sent: bool) -> Self {
		Self {
			message_id,
			text,
			timestamp,
			is_sent,
		}
	}

	pub fn new_now(message_id: String, text: String, is_sent: bool) -> Self {
		Self::new(message_id, text, Local::now(), is_sent)
	}

	pub fn new_pending(message_id: String, text: String) -> Self {
		Self {
			message_id,
			text,
			timestamp: Local::now(),
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
#[tsify(into_wasm_abi)]
pub struct OtherMessage {
	pub message_id: String,
	/// The message in plain readable text format.
	pub text: String,
	pub timestamp: DateTime<Local>,
}

impl<'a> OtherMessage {
	pub fn new(message_id: String, text: String, timestamp: DateTime<Local>) -> Self {
		Self {
			message_id,
			text,
			timestamp,
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[serde(tag = "type", rename_all = "camelCase")]
#[tsify(into_wasm_abi)]
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
	pub fn own_message(message_id: String, text: String, timestamp: DateTime<Local>, is_sent: bool) -> Self {
		Self::OwnMessage(OwnMessage::new(message_id, text, timestamp, is_sent))
	}

	pub fn other_message(message_id: String, text: String, timestamp: DateTime<Local>) -> Self {
		Self::OtherMessage(OtherMessage::new(message_id, text, timestamp))
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatInfo {
	pub user_id: String,
	/// The display name set by the user.
	pub display_name: String,
	pub shared_secret: String,
}

impl ChatInfo {
	pub fn new(user_id: String, display_name: String, shared_secret: String) -> Self {
		Self {
			user_id,
			display_name,
			shared_secret,
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatData {
	pub messages: Vec<ChatMessage>,
	/// Key: message id,
	///
	/// Value: index in the messages vec
	pub own_message_map: HashMap<String, usize>,
}

impl ChatData {
	pub fn new(messages: Vec<ChatMessage>, own_message_map: HashMap<String, usize>) -> Self {
		Self {
			messages,
			own_message_map,
		}
	}

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
