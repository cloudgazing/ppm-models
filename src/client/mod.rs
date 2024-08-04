use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserMessage {
	pub access_token: String,
	pub user_id: String,
	// change to encrypted message
	pub content: String,
}

impl UserMessage {
	pub fn new(access_token: String, user_id: String, content: String) -> Self {
		Self {
			access_token,
			user_id,
			content,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeValidation {
	pub signed_key: String,
	pub token: String,
}

impl WelcomeValidation {
	pub fn new(signed_key: String, token: String) -> Self {
		Self { signed_key, token }
	}
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
	UserMessage(UserMessage),
	WelcomeValidation(WelcomeValidation),
}

impl ClientMessage {
	pub fn user_message(access_token: String, user_id: String, message_text: String) -> Self {
		Self::UserMessage(UserMessage::new(access_token, user_id, message_text))
	}
	pub fn welcome_validation(signed_key: String, token: String) -> Self {
		Self::WelcomeValidation(WelcomeValidation::new(signed_key, token))
	}
	pub fn to_data_string(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(&self)
	}
}

// HTTPS
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
	pub access_key: String,
	pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupData {
	pub access_key: String,
	pub password: String,
	pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthData {
	Login(LoginData),
	Signup(SignupData),
}
