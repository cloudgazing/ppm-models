use super::error::MessageStatusError;

use crate::database::MessageBundle;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
	Success,
	Error(MessageStatusError),
}

/// A confirmation from the server that lets the client know if the message was sent or something went wrong.
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageStatus {
	pub message_id: String,
	/// Status update from the server.
	pub status: Status,
}

impl MessageStatus {
	pub fn new(message_id: String, status: Status) -> Self {
		Self { message_id, status }
	}
}

/// The server sends a keep alive message roughly every 40 seconds to keep the connection alive.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeepAlive {
	pub time_stamp: String,
}

impl KeepAlive {
	pub fn new(time_stamp: String) -> Self {
		Self { time_stamp }
	}
}

/// Contains all the websocket messages the server sends to the client.
#[derive(Debug, Deserialize, Serialize)]
pub enum WsServerMessage {
	MessageBundle(MessageBundle),
	MessageStatus(MessageStatus),
	KeepAlive(KeepAlive),
}

impl From<MessageBundle> for WsServerMessage {
	fn from(chat_message: MessageBundle) -> Self {
		Self::MessageBundle(chat_message)
	}
}

impl From<MessageStatus> for WsServerMessage {
	fn from(message_status: MessageStatus) -> Self {
		Self::MessageStatus(message_status)
	}
}

impl From<KeepAlive> for WsServerMessage {
	fn from(keep_alive: KeepAlive) -> Self {
		Self::KeepAlive(keep_alive)
	}
}

impl WsServerMessage {
	pub fn message_bundle(
		message_id: String,
		chat_id: String,
		sender_id: String,
		message: Vec<u8>,
		timestamp: NaiveDateTime,
	) -> Self {
		Self::MessageBundle(MessageBundle::new(message_id, chat_id, sender_id, message, timestamp))
	}

	pub fn message_status(message_id: String, status: Status) -> Self {
		Self::MessageStatus(MessageStatus::new(message_id, status))
	}

	pub fn successful_message_status(message_id: String) -> Self {
		Self::message_status(message_id, Status::Success)
	}

	pub fn error_message_status(message_id: String, error: MessageStatusError) -> Self {
		Self::message_status(message_id, Status::Error(error))
	}

	pub fn keep_alive(time_stamp: String) -> Self {
		Self::KeepAlive(KeepAlive::new(time_stamp))
	}
}
