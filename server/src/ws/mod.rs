//! WebSocket Handler
//!
//! Real-time communication for chat and voice signaling.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::AppState;

/// WebSocket message envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    /// Message ID for acknowledgment
    pub id: Option<String>,
    /// Message type/event name
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Message payload
    pub payload: serde_json::Value,
}

/// Client-to-server events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientEvent {
    /// Ping for keepalive
    Ping,
    /// Subscribe to channel events
    Subscribe { channel_id: Uuid },
    /// Unsubscribe from channel events
    Unsubscribe { channel_id: Uuid },
    /// Send typing indicator
    Typing { channel_id: Uuid },
    /// Stop typing indicator
    StopTyping { channel_id: Uuid },
    /// Voice signaling
    VoiceSignal(crate::voice::signaling::SignalingMessage),
}

/// Server-to-client events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    /// Pong response
    Pong,
    /// New message in channel
    MessageNew {
        channel_id: Uuid,
        message: serde_json::Value,
    },
    /// Message edited
    MessageEdit {
        channel_id: Uuid,
        message_id: Uuid,
        content: String,
    },
    /// Message deleted
    MessageDelete {
        channel_id: Uuid,
        message_id: Uuid,
    },
    /// User typing
    TypingStart {
        channel_id: Uuid,
        user_id: Uuid,
    },
    /// User stopped typing
    TypingStop {
        channel_id: Uuid,
        user_id: Uuid,
    },
    /// Presence update
    PresenceUpdate {
        user_id: Uuid,
        status: String,
    },
    /// Voice signaling
    VoiceSignal(crate::voice::signaling::SignalingMessage),
    /// Error
    Error {
        code: String,
        message: String,
    },
}

/// WebSocket upgrade handler.
pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handle WebSocket connection.
async fn handle_socket(socket: WebSocket, _state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // TODO: Authenticate connection via token
    // TODO: Track connection in Redis for presence
    // TODO: Subscribe to Redis pub/sub for real-time events

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Parse and handle message
                if let Ok(event) = serde_json::from_str::<ClientEvent>(&text) {
                    match event {
                        ClientEvent::Ping => {
                            let pong = serde_json::to_string(&ServerEvent::Pong).unwrap();
                            let _ = sender.send(Message::Text(pong)).await;
                        }
                        // TODO: Handle other events
                        _ => {}
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }

    // TODO: Clean up connection, update presence
}
