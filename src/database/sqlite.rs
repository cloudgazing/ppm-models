use crate::database::raw::RawMessageBundle;

use super::raw::{RawOwnUserInfo, RawSensitiveBundle, RawUserInfo};

use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

pub async fn check_username_availability(pool: &SqlitePool, username: String) -> Result<bool, sqlx::Error> {
	let exists: i64 = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)", username)
		.fetch_one(pool)
		.await?;

	Ok(exists != 0)
}

pub async fn add_new_user(
	pool: &SqlitePool,
	username: String,
	password: String,
	display_name: String,
	pin_hash: Vec<u8>,
) -> Result<SqliteQueryResult, sqlx::Error> {
	let user_id = uuid::Uuid::new_v4().as_bytes().to_vec();
	let password_hash = blake3::hash(password.as_bytes()).as_bytes().to_vec();

	sqlx::query!(
		"INSERT INTO users (user_id, username, password_hash, display_name, pin_hash) VALUES (?, ?, ?, ?, ?)",
		user_id,
		username,
		password_hash,
		display_name,
		pin_hash
	)
	.execute(pool)
	.await
}

pub async fn get_raw_own_user_info(
	pool: &SqlitePool,
	username: String,
	password: String,
) -> Result<Option<RawOwnUserInfo>, sqlx::Error> {
	let password_hash = blake3::hash(password.as_bytes()).as_bytes().to_vec();

	sqlx::query_as!(
		RawOwnUserInfo,
		"SELECT user_id, display_name, pin_hash FROM users WHERE username = ? AND password_hash = ?",
		username,
		password_hash
	)
	.fetch_optional(pool)
	.await
}

pub async fn get_raw_user_info(pool: &SqlitePool, user_id: Vec<u8>) -> Result<Option<RawUserInfo>, sqlx::Error> {
	sqlx::query_as!(
		RawUserInfo,
		"SELECT user_id, display_name FROM users WHERE user_id = ?",
		user_id
	)
	.fetch_optional(pool)
	.await
}

pub async fn get_raw_user_info_vec(pool: &SqlitePool, user_ids: Vec<Vec<u8>>) -> Result<Vec<RawUserInfo>, sqlx::Error> {
	let placeholders = user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

	let query = format!(
		"SELECT user_id, display_name FROM users WHERE user_id IN ({})",
		placeholders
	);

	let mut query = sqlx::query_as::<_, RawUserInfo>(&query);

	for user_id in user_ids {
		query = query.bind(user_id);
	}

	query.fetch_all(pool).await
}

pub async fn get_raw_sensitive_bundle(
	pool: &SqlitePool,
	sensitive_hash: Vec<u8>,
) -> Result<Option<RawSensitiveBundle>, sqlx::Error> {
	sqlx::query_as!(
		RawSensitiveBundle,
		"SELECT sensitive_hash, sensitive_data FROM sensitive WHERE sensitive_hash = ?",
		sensitive_hash
	)
	.fetch_optional(pool)
	.await
}

pub async fn get_raw_chat_messages(pool: &SqlitePool, chat_id: Vec<u8>) -> Result<Vec<RawMessageBundle>, sqlx::Error> {
	sqlx::query_as!(
		RawMessageBundle,
		"SELECT message_id, chat_id, sender_id, message, timestamp FROM messages WHERE chat_id = ?",
		chat_id
	)
	.fetch_all(pool)
	.await
}
