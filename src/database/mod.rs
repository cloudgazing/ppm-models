pub mod raw;
pub mod sqlite;

use std::{collections::HashMap, string::FromUtf8Error};

use chrono::NaiveDateTime;
use raw::{RawMessageBundle, RawOwnUserInfo, RawUserInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
	pub user_id: String,
	pub display_name: String,
}

impl UserInfo {
	pub fn new(user_id: String, display_name: String) -> Self {
		Self { user_id, display_name }
	}

	pub fn from_raw(raw: RawUserInfo) -> Result<Self, FromUtf8Error> {
		Ok(Self::new(String::from_utf8(raw.user_id)?, raw.display_name))
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnUserInfo {
	pub user_id: String,
	pub display_name: String,
	pub pin_hash: Vec<u8>,
}

impl OwnUserInfo {
	pub fn new(user_id: String, display_name: String, pin_hash: Vec<u8>) -> Self {
		Self {
			user_id,
			display_name,
			pin_hash,
		}
	}

	pub fn from_raw(raw: RawOwnUserInfo) -> Result<Self, FromUtf8Error> {
		Ok(Self::new(
			String::from_utf8(raw.user_id)?,
			raw.display_name,
			raw.pin_hash,
		))
	}
}

/// A message from a conversation betwen Alice and Bob.
/// `chat_id` is a unique identifier that can be computed only by Bob and Alice.
/// `sender_id` is an obfuscated user id that Alice and Bob can get using their shared secret.
/// `message_id` is the encrypted text message, with a key from the receiver.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBundle {
	pub message_id: String,
	pub chat_id: String,
	pub sender_id: String,
	/// The contents of the message, encrypted.
	pub message: Vec<u8>,
	pub timestamp: NaiveDateTime,
}

impl MessageBundle {
	pub fn new(
		message_id: String,
		chat_id: String,
		sender_id: String,
		message: Vec<u8>,
		timestamp: NaiveDateTime,
	) -> Self {
		Self {
			message_id,
			chat_id,
			sender_id,
			message,
			timestamp,
		}
	}

	pub fn from_raw(raw: RawMessageBundle) -> Result<Self, FromUtf8Error> {
		Ok(Self::new(
			String::from_utf8(raw.message_id)?,
			String::from_utf8(raw.chat_id)?,
			String::from_utf8(raw.sender_id)?,
			raw.message,
			raw.timestamp,
		))
	}

	pub fn into_raw(self) -> RawMessageBundle {
		RawMessageBundle::new(
			self.message_id.into_bytes(),
			self.chat_id.into_bytes(),
			self.sender_id.into_bytes(),
			self.message,
			self.timestamp,
		)
	}
}

/// Alice and Bob have a common shared secret key.
/// Sensitive data holds (for Alice) Bob's user id and the shared secret key.
/// This data is unique can be decrypted only by Alice.
/// This is stored on a server as an array of bytes.
#[derive(Debug, Serialize, Deserialize)]
pub struct SensitiveData {
	pub user_id: String,
	pub shared_secret: String,
}

impl SensitiveData {
	/// turns it into a hashmap with the user_id as key and the shared secret as value
	pub fn to_secret_map<T>(bundles: Vec<Self>) -> HashMap<String, String> {
		bundles
			.into_iter()
			.map(|bundle| (bundle.user_id, bundle.shared_secret))
			.collect()
	}
}
