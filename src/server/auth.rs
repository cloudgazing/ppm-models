#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[cfg_attr(feature = "napi", napi_derive::napi)]
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
	pub fn success(user_id: String, display_name: String, auth_token: String) -> Self {
		Self::Success {
			user_id,
			display_name,
			auth_token,
		}
	}
	pub fn error(reason: String) -> Self {
		Self::Error { reason }
	}
}
