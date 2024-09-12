use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A `sender_id` it is not sent as the server gets it from the JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
	/// Authorization token.
	jwt: String,
	/// The receiver's user id.
	receiver_id: String,
	message_id: String,
	chat_id: String,
	/// An encypted message.
	message: Vec<u8>,
	timestamp: NaiveDateTime,
}

impl WsMessage {
	pub fn new(
		jwt: String,
		receiver_id: String,
		message_id: String,
		chat_id: String,
		message: Vec<u8>,
		timestamp: NaiveDateTime,
	) -> Self {
		Self {
			jwt,
			receiver_id,
			message_id,
			chat_id,
			message,
			timestamp,
		}
	}
}
