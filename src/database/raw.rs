use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RawUserInfo {
	pub user_id: Vec<u8>,
	pub display_name: String,
}

impl RawUserInfo {
	pub fn new(user_id: Vec<u8>, display_name: String) -> Self {
		Self { user_id, display_name }
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawOwnUserInfo {
	pub user_id: Vec<u8>,
	pub display_name: String,
	pub pin_hash: Vec<u8>,
}

impl RawOwnUserInfo {
	pub fn new(user_id: Vec<u8>, display_name: String, pin_hash: Vec<u8>) -> Self {
		Self {
			user_id,
			display_name,
			pin_hash,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawSensitiveBundle {
	pub sensitive_hash: Vec<u8>,
	pub sensitive_data: Vec<u8>,
}

impl RawSensitiveBundle {
	pub fn new(sensitive_hash: Vec<u8>, sensitive_data: Vec<u8>) -> Self {
		Self {
			sensitive_hash,
			sensitive_data,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawMessageBundle {
	pub message_id: Vec<u8>,
	pub chat_id: Vec<u8>,
	pub sender_id: Vec<u8>,
	pub message: Vec<u8>,
	pub timestamp: NaiveDateTime,
}

impl RawMessageBundle {
	pub fn new(
		message_id: Vec<u8>,
		chat_id: Vec<u8>,
		sender_id: Vec<u8>,
		message: Vec<u8>,
		timestamp: NaiveDateTime,
	) -> Self {
		Self {
			message_id,
			chat_id,
			sender_id,
			message,
			timestamp,
		}
	}
}
