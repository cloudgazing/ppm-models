use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
	pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
	// change to encrypted message
	pub content: String,
}

#[derive(Serialize, Deserialize)]
pub enum Notification {
	NewMessage(NewMessage),
}

#[derive(Serialize, Deserialize)]
pub struct Welcome {
	pub signed_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct KeepAlive {
	pub time_stamp: String,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
	Notification(Notification),
	Welcome(Welcome),
	KeepAlive(KeepAlive),
}

// HTTPS
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginConfirmation {
	pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupConfirmation {
	pub access_token: String,
}
