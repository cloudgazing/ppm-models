#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LoginData {
	pub username: String,
	pub password: String,
}

impl LoginData {
	pub fn new(username: String, password: String) -> Self {
		Self { username, password }
	}
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SignupData {
	pub username: String,
	pub password: String,
	pub display_name: String,
}

impl SignupData {
	pub fn new(username: String, password: String, display_name: String) -> Self {
		Self {
			username,
			password,
			display_name,
		}
	}
}
