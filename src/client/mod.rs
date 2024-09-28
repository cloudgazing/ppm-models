pub mod auth;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
#[cfg(all(target_arch = "wasm32"))]
use tsify_next::Tsify;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Message {
	pub message_id: String,
	pub user_id: String,
	pub text: String,
	pub timestamp: DateTime<Local>,
}

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

#[derive(Debug, Deserialize, Serialize)]
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
