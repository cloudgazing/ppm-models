use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewMessage {
	pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Notification {
	NewMessage(NewMessage),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeepAlive {
	pub time_stamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Welcome {
	pub signed_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
	Notification(Notification),
	KeepAlive(KeepAlive),
	Welcome(Welcome),
}
