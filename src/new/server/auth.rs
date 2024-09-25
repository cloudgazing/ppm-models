use super::error::{LoginError, SignupError};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthSuccess {
	pub user_id: String,
	pub display_name: String,
	pub auth_token: String,
}

impl AuthSuccess {
	pub fn new(user_id: String, display_name: String, auth_token: String) -> Self {
		Self {
			user_id,
			display_name,
			auth_token,
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LoginConfirmation {
	Success(AuthSuccess),
	Error(LoginError),
}

impl LoginConfirmation {
	pub fn success(user_id: String, display_name: String, auth_token: String) -> Self {
		Self::Success(AuthSuccess::new(user_id, display_name, auth_token))
	}

	pub fn error(error: LoginError) -> Self {
		Self::Error(error)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SignupConfirmation {
	Success(AuthSuccess),
	Error(SignupError),
}

impl SignupConfirmation {
	pub fn success(user_id: String, display_name: String, auth_token: String) -> Self {
		Self::Success(AuthSuccess::new(user_id, display_name, auth_token))
	}

	pub fn error(error: SignupError) -> Self {
		Self::Error(error)
	}
}

#[derive(Debug, Deserialize, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi)]
pub enum AuthResponse {
	Success {
		user_id: String,
		display_name: String,
		auth_token: String,
	},
	Error {
		reason: String,
	},
}

impl AuthResponse {
	pub fn from_login_confirmation(confirmation: LoginConfirmation) -> Self {
		match confirmation {
			LoginConfirmation::Success(success) => Self::Success {
				user_id: success.user_id,
				display_name: success.display_name,
				auth_token: success.auth_token,
			},
			LoginConfirmation::Error(error) => Self::Error {
				reason: error.as_str().to_string(),
			},
		}
	}

	pub fn from_signup_confirmation(confirmation: SignupConfirmation) -> Self {
		match confirmation {
			SignupConfirmation::Success(success) => Self::Success {
				user_id: success.user_id,
				display_name: success.display_name,
				auth_token: success.auth_token,
			},
			SignupConfirmation::Error(error) => Self::Error {
				reason: error.as_str().to_string(),
			},
		}
	}
}
