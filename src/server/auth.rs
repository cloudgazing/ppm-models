use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginSuccess {
	pub jwt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LoginConfirmation<'a> {
	Success(LoginSuccess),
	Failure(&'a str),
}

impl<'a> LoginConfirmation<'a> {
	pub fn success(jwt: String) -> Self {
		Self::Success(LoginSuccess { jwt })
	}

	pub fn failure(error: &'a str) -> Self {
		Self::Failure(error)
	}

	pub fn serialize(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupSuccess {
	pub jwt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SignupConfirmation<'a> {
	Success(SignupSuccess),
	Failure(&'a str),
}

impl<'a> SignupConfirmation<'a> {
	pub fn success(jwt: String) -> Self {
		Self::Success(SignupSuccess { jwt })
	}

	pub fn failure(error: &'a str) -> Self {
		Self::Failure(error)
	}

	pub fn serialize(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BasicResponse<T> {
	Ok(T),
	Err,
}
