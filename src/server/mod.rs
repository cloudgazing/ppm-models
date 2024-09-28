pub mod auth;
pub mod error;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
	pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageConfirmation {
	Success,
	Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerSocketMessage {
	/// A `Client::Message` as a byte array.
	NewMessage(Vec<u8>),
	MessageConfirmation(MessageConfirmation),
}
