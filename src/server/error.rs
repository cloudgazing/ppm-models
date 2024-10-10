#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const SERVER_INTERNAL_ERROR: &str = "Internal server error";
pub const USER_INVALID_CLAIMS: &str = "Invalid user claims";

pub const AUTH_INVALID_FORMAT: &str = "Invalid data format";
pub const AUTH_INVALID_REQUEST_DATA: &str = "Invalid data";
pub const LOGIN_WRONG_CREDENTIALS: &str = "Wrong credentials";
pub const SIGNUP_USERNAME_TAKEN: &str = "Username already taken";

pub const MESSAGE_USER_NOT_FOUND: &str = "User not found";

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum LoginError {
	Internal,
	InvalidFormat,
	InvalidData,
	WrongCredentials,
}

impl LoginError {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Internal => SERVER_INTERNAL_ERROR,
			Self::InvalidFormat => AUTH_INVALID_FORMAT,
			Self::InvalidData => AUTH_INVALID_REQUEST_DATA,
			Self::WrongCredentials => LOGIN_WRONG_CREDENTIALS,
		}
	}
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum SignupError {
	Internal,
	InvalidFormat,
	InvalidData,
	UsernameTaken,
}

impl SignupError {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Internal => SERVER_INTERNAL_ERROR,
			Self::InvalidFormat => AUTH_INVALID_FORMAT,
			Self::InvalidData => AUTH_INVALID_REQUEST_DATA,
			Self::UsernameTaken => SIGNUP_USERNAME_TAKEN,
		}
	}
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum MessageStatusError {
	Internal,
	Claims,
	UserNotFound,
}

impl MessageStatusError {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Internal => SERVER_INTERNAL_ERROR,
			Self::Claims => USER_INVALID_CLAIMS,
			Self::UserNotFound => MESSAGE_USER_NOT_FOUND,
		}
	}
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum VerifyError {
	InvalidFormat,
	Claims,
}

impl VerifyError {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::InvalidFormat => AUTH_INVALID_FORMAT,
			Self::Claims => USER_INVALID_CLAIMS,
		}
	}
}
