use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMessage<'a> {
	/// The JWT authorization token.
	pub jwt: &'a str,
	/// The receiver's user ID.
	pub user_id: &'a str,
	/// A message id that is encrypted, preventing the server from seeing it.
	pub signed_id: &'a [u8],
	/// An encrypted message accessible only to the receiver.
	pub contents: &'a [u8],
}

impl<'a> UserMessage<'a> {
	pub fn new(jwt: &'a str, user_id: &'a str, signed_id: &'a [u8], contents: &'a [u8]) -> Self {
		Self {
			jwt,
			user_id,
			signed_id,
			contents,
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage<'a> {
	#[serde(borrow)]
	UserMessage(UserMessage<'a>),
}

impl<'a> From<UserMessage<'a>> for ClientMessage<'a> {
	fn from(user_message: UserMessage<'a>) -> Self {
		Self::UserMessage(user_message)
	}
}

impl<'a> ClientMessage<'a> {
	pub fn serialize(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(&self)
	}
}
