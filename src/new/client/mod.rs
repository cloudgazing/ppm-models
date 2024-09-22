pub mod auth;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
	pub message_id: String,
	pub user_id: String,
	pub text: String,
	pub timestamp: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientSocketMessage {
	token: String,
	receiver_id: String,
	encrypted_message_id: String,
	contents: Vec<u8>,
}
