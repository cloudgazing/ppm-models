use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginData<'a> {
	pub username: &'a str,
	pub password: &'a str,
}

impl<'a> LoginData<'a> {
	pub fn new(username: &'a str, password: &'a str) -> Self {
		Self { username, password }
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupData<'a> {
	pub username: &'a str,
	pub password: &'a str,
	pub display_name: &'a str,
}

impl<'a> SignupData<'a> {
	pub fn new(username: &'a str, password: &'a str, display_name: &'a str) -> Self {
		Self {
			username,
			password,
			display_name,
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Auth<'a> {
	#[serde(borrow)]
	Login(LoginData<'a>),
	Signup(SignupData<'a>),
}
