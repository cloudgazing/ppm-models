#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AuthLoginBody {
	pub username: String,
	pub password: String,
}

impl AuthLoginBody {
	pub fn new(username: String, password: String) -> Self {
		Self { username, password }
	}
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AuthSignupBody {
	pub username: String,
	pub password: String,
	pub display_name: String,
}

impl AuthSignupBody {
	pub fn new(username: String, password: String, display_name: String) -> Self {
		Self {
			username,
			password,
			display_name,
		}
	}
}
