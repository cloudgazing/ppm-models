use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginData {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupData {
	pub username: String,
	pub password: String,
	pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Auth {
	Login(LoginData),
	Signup(LoginData),
}
