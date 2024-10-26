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

/// Success response returned by the server for both login and signup requests.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AuthResponseSuccess {
	pub user_id: String,
	pub display_name: String,
	pub auth_token: String,
}

/// The respnse returned by the server for a login request.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "camelCase"))]
pub enum AuthLoginResponse {
	/// Response status is `200 OK`.
	Success(AuthResponseSuccess),
	/// Response status is `400 Bad Request`.
	BadRequest,
	/// Response status is `401 Unauthorized`.
	Unauthorized,
	/// Response status is `500 Internal Server Error`.
	ServerError,
}

impl AuthLoginResponse {
	pub fn success(user_id: String, display_name: String, auth_token: String) -> Self {
		Self::Success(AuthResponseSuccess {
			user_id,
			display_name,
			auth_token,
		})
	}
	pub fn bad_request() -> Self {
		Self::BadRequest
	}
	pub fn unauthorized() -> Self {
		Self::Unauthorized
	}
	pub fn server_error() -> Self {
		Self::ServerError
	}
}

/// The response returned by the server for a signup request.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "camelCase"))]
pub enum AuthSignupResponse {
	/// Response status is `201 Created`.
	Success(AuthResponseSuccess),
	/// Response status is `400 Bad Request`.
	BadRequest,
	/// Response status is `409 Conflict`.
	Conflict,
	/// Response status is `500 Internal Server Error`.
	ServerError,
}

impl AuthSignupResponse {
	pub fn success(user_id: String, display_name: String, auth_token: String) -> Self {
		Self::Success(AuthResponseSuccess {
			user_id,
			display_name,
			auth_token,
		})
	}
	pub fn bad_request() -> Self {
		Self::BadRequest
	}
	pub fn conflict() -> Self {
		Self::Conflict
	}
	pub fn server_error() -> Self {
		Self::ServerError
	}
}
