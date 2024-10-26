#[cfg(feature = "actix")]
use actix_web::{body::BoxBody, http::StatusCode, HttpResponseBuilder, Responder};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

#[cfg(feature = "actix")]
impl Responder for AuthLoginResponse {
	type Body = BoxBody;

	fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
		let status = match self {
			AuthLoginResponse::Success(_) => StatusCode::OK,
			AuthLoginResponse::BadRequest => StatusCode::BAD_REQUEST,
			AuthLoginResponse::Unauthorized => StatusCode::UNAUTHORIZED,
			AuthLoginResponse::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
		};

		let body = serde_json::to_string(&self).unwrap();

		HttpResponseBuilder::new(status).body(body)
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

#[cfg(feature = "actix")]
impl Responder for AuthSignupResponse {
	type Body = BoxBody;

	fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
		let status = match self {
			AuthSignupResponse::Success(_) => StatusCode::OK,
			AuthSignupResponse::BadRequest => StatusCode::BAD_REQUEST,
			AuthSignupResponse::Conflict => StatusCode::CONFLICT,
			AuthSignupResponse::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
		};

		let body = serde_json::to_string(&self).unwrap();

		HttpResponseBuilder::new(status).body(body)
	}
}
