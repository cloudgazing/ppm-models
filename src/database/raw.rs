use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RawUserInfo {
	pub user_id: Vec<u8>,
	pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawOwnUserInfo {
	pub user_id: Vec<u8>,
	pub display_name: String,
	pub pin_hash: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawSensitiveBundle {
	pub sensitive_hash: Vec<u8>,
	pub sensitive_data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawMessageBundle {
	pub message_id: Vec<u8>,
	pub chat_id: Vec<u8>,
	pub sender_id: Vec<u8>,
	pub message: Vec<u8>,
	pub timestamp: NaiveDateTime,
}
