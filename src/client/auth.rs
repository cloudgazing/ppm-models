use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginData {
	pub username: String,
	pub password: String,
}

impl LoginData {
	pub fn new(username: String, password: String) -> Self {
		Self { username, password }
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupData {
	pub username: String,
	pub password: String,
	pub display_name: String,
	pub pin_hash: Vec<u8>,
}

impl SignupData {
	pub fn new(username: String, password: String, display_name: String, pin_hash: Vec<u8>) -> Self {
		Self {
			username,
			password,
			display_name,
			pin_hash,
		}
	}
}
