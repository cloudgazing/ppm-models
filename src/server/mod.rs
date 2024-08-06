pub mod auth;
pub mod error;
pub mod message;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TokenClaims {
	pub user_id: String,
}
