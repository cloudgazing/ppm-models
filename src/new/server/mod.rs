use serde::{Deserialize, Serialize};

pub mod auth;
pub mod error;

#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
	pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageStatusConfirmation {
	Success,
	Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerSocketMessage {
	/// A `Client::Message` as a byte array.
	NewMessage(Vec<u8>),
	MessageStatusConfirmation(MessageStatusConfirmation),
}