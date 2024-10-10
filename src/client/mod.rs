pub mod auth;

#[cfg(feature = "chrono")]
use chrono::{DateTime, Local};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[derive(Debug)]
#[cfg(feature = "chrono")]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct Message {
	pub message_id: String,
	pub user_id: String,
	pub text: String,
	pub timestamp: DateTime<Local>,
}

#[cfg(feature = "chrono")]
impl Message {
	pub fn new(message_id: String, user_id: String, text: String, timestamp: DateTime<Local>) -> Self {
		Self {
			message_id,
			user_id,
			text,
			timestamp,
		}
	}
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ClientSocketMessage {
	pub receiver_id: String,
	pub encrypted_message_id: String,
	pub contents: Vec<u8>,
}

impl ClientSocketMessage {
	pub fn new(receiver_id: String, encrypted_message_id: String, contents: Vec<u8>) -> Self {
		Self {
			receiver_id,
			encrypted_message_id,
			contents,
		}
	}
}
