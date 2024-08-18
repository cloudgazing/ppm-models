use super::error::MessageStatusError;

use serde::{Deserialize, Serialize};

/// A new message from another user.
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMessage<'a> {
	/// The receiver's user ID.
	pub user_id: &'a str,
	/// The content of the message encrypted with the receiver's key.
	pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
	Success,
	Error(MessageStatusError),
}

/// A confirmation from the server that lets the client know if the message was sent or something went wrong.
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageStatus {
	/// The receiver's user ID.
	pub user_id: String,
	/// Status update from the server.
	pub status: Status,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Notification<'a> {
	#[serde(borrow)]
	NewMessage(NewMessage<'a>),
	MessageStatus(MessageStatus),
}

/// The server sends a keep alive message roughly every 40 seconds to keep the connection alive.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeepAlive {
	pub time_stamp: String,
}

/// Contains all the websocket messages the server sends to the client.
#[derive(Debug, Deserialize, Serialize)]
pub enum ServerMessage<'a> {
	#[serde(borrow)]
	Notification(Notification<'a>),
	KeepAlive(KeepAlive),
}

impl<'a> ServerMessage<'a> {
	pub fn notification(notification: Notification<'a>) -> Self {
		Self::Notification(notification)
	}

	pub fn new_message(user_id: &'a str, content: String) -> Self {
		Self::Notification(Notification::NewMessage(NewMessage { user_id, content }))
	}

	pub fn message_status(user_id: String, status: Status) -> Self {
		Self::Notification(Notification::MessageStatus(MessageStatus { user_id, status }))
	}

	pub fn successful_message_status(user_id: String) -> Self {
		Self::Notification(Notification::MessageStatus(MessageStatus {
			user_id,
			status: Status::Success,
		}))
	}

	pub fn error_message_status(user_id: String, error: MessageStatusError) -> Self {
		Self::Notification(Notification::MessageStatus(MessageStatus {
			user_id,
			status: Status::Error(error),
		}))
	}

	pub fn keep_alive(time_stamp: String) -> Self {
		Self::KeepAlive(KeepAlive { time_stamp })
	}

	pub fn serialize(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}
