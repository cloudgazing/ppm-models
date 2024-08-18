use super::error::{LoginError, SignupError, VerifyError};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum LoginConfirmation<'a> {
	Success(&'a str),
	Error(LoginError),
}

impl<'a> LoginConfirmation<'a> {
	pub fn success(token: &'a str) -> Self {
		Self::Success(token)
	}

	pub fn error(error: LoginError) -> Self {
		Self::Error(error)
	}

	pub fn serialize(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SignupConfirmation<'a> {
	Success(&'a str),
	Error(SignupError),
}

impl<'a> SignupConfirmation<'a> {
	pub fn success(token: &'a str) -> Self {
		Self::Success(token)
	}

	pub fn error(error: SignupError) -> Self {
		Self::Error(error)
	}

	pub fn serialize(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VerifyResponse {
	Ok,
	Err(VerifyError),
}
