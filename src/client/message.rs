use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMessage<'a> {
	/// The JWT authentication token.
	pub jwt: &'a str,
	/// The receiver's user ID.
	pub user_id: &'a str,
	pub message_id: &'a str,
	/// The content of the message encrypted with the receiver's key.
	pub content: String,
}

impl<'a> UserMessage<'a> {
	pub fn new(jwt: &'a str, user_id: &'a str, message_id: &'a str, content: String) -> Self {
		Self {
			jwt,
			user_id,
			message_id,
			content,
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage<'a> {
	#[serde(borrow)]
	UserMessage(UserMessage<'a>),
}

impl<'a> ClientMessage<'a> {
	pub fn user_message(jwt: &'a str, user_id: &'a str, message_id: &'a str, content: String) -> Self {
		Self::UserMessage(UserMessage::new(jwt, user_id, message_id, content))
	}

	pub fn serialize(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(&self)
	}
}
