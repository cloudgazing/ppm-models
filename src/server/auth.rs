use serde::{Deserialize, Serialize};
#[cfg(all(target_arch = "wasm32"))]
use tsify_next::Tsify;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi))]
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
