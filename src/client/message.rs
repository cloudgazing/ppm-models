use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
	/// Authorization token.
	jwt: String,
	/// The receiver's user id.
	receiver_id: String,
	message_id: String,
	chat_id: String,
	sender_id: String,
	/// An encypted message.
	message: Vec<u8>,
	timestamp: DateTime<Utc>,
}

impl WsMessage {
	pub fn new(
		jwt: String,
		receiver_id: String,
		message_id: String,
		chat_id: String,
		sender_id: String,
		message: Vec<u8>,
		timestamp: DateTime<Utc>,
	) -> Self {
		Self {
			jwt,
			receiver_id,
			message_id,
			chat_id,
			sender_id,
			message,
			timestamp,
		}
	}
}
