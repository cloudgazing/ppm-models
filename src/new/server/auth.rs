use super::error::{LoginError, SignupError, VerifyError};

use serde::{Deserialize, Serialize};

type AuthToken = String;

#[derive(Debug, Deserialize, Serialize)]
pub enum LoginConfirmation {
	Success(AuthToken),
	Error(LoginError),
}

impl LoginConfirmation {
	pub fn success(token: String) -> Self {
		Self::Success(token)
	}

	pub fn error(error: LoginError) -> Self {
		Self::Error(error)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SignupConfirmation {
	Success(AuthToken),
	Error(SignupError),
}

impl SignupConfirmation {
	pub fn success(token: String) -> Self {
		Self::Success(token)
	}

	pub fn error(error: SignupError) -> Self {
		Self::Error(error)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VerifyResponse {
	Ok { user_id: String, pin_hash: String },
	Err(VerifyError),
}
