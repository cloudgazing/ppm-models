use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMessage {
	pub jwt: String,
	pub user_id: String,
	pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WelcomeResponse {
	pub jwt: String,
	pub verification_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
	UserMessage(UserMessage),
	WelcomeResponse(WelcomeResponse),
}

impl Message {
	pub fn user_message(jwt: String, user_id: String, content: String) -> Self {
		Self::UserMessage(UserMessage { jwt, user_id, content })
	}

	pub fn welcome_response(jwt: String, verification_key: String) -> Self {
		Self::WelcomeResponse(WelcomeResponse { jwt, verification_key })
	}

	pub fn serialize(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(&self)
	}
}
