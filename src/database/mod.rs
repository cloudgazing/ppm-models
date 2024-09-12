pub mod raw;
pub mod sqlite;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Alice and Bob have a common shared secret key.
/// Sensitive data holds (for Alice) Bob's user id and the shared secret key.
/// This data is unique can be decrypted only by Alice.
/// This is stored on a server as an array of bytes.
#[derive(Debug, Serialize, Deserialize)]
pub struct SensitiveBundle {
	pub user_id: String,
	pub shared_secret: String,
}

impl SensitiveBundle {
	pub fn to_hashmap(bundles: Vec<SensitiveBundle>) -> SensitiveMap {
		bundles
			.into_iter()
			.map(|bundle| (bundle.user_id, bundle.shared_secret))
			.collect()
	}
}

pub type SensitiveMap = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
	pub user_id: String,
	pub display_name: String,
	pub unread_count: usize,
}

pub type UserInfoData = Vec<UserInfo>;

/// A message from a conversation betwen Alice and Bob.
/// `chat_id` is a unique identifier that can be computed only by Bob and Alice.
/// `sender_id` is an obfuscated user id that Alice and Bob can get using their shared secret.
/// `message_id` is the encrypted text message, with a key from the receiver.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
	/// A value that's unique across every single conversation.
	pub message_id: String,
	/// A value that's unique for each chat or conversation.
	pub chat_id: String,
	pub sender_id: String,
	/// The contents of the message, encrypted.
	pub message: Vec<u8>,
	pub timestamp: DateTime<Utc>,
}

impl Message {
	pub fn new(
		message_id: String,
		chat_id: String,
		sender_id: String,
		message: Vec<u8>,
		timestamp: DateTime<Utc>,
	) -> Self {
		Self {
			message_id,
			chat_id,
			sender_id,
			message,
			timestamp,
		}
	}
}
