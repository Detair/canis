//! WebSocket Protocol Messages
//!
//! Shared message types for real-time communication.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{Message, UserProfile, UserStatus};

/// Client-to-server WebSocket events.
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

    /// Voice: Join channel
    VoiceJoin { channel_id: Uuid },

    /// Voice: Leave channel
    VoiceLeave { channel_id: Uuid },

    /// Voice: SDP Offer
    VoiceOffer { channel_id: Uuid, sdp: String },

    /// Voice: SDP Answer
    VoiceAnswer { channel_id: Uuid, sdp: String },

    /// Voice: ICE Candidate
    VoiceIce { channel_id: Uuid, candidate: String },

    /// Voice: Mute self
    VoiceMute { channel_id: Uuid },

    /// Voice: Unmute self
    VoiceUnmute { channel_id: Uuid },
}

/// Server-to-client WebSocket events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    /// Pong response
    Pong,

    /// Connection ready with user info
    Ready { user: UserProfile },

    /// New message
    MessageCreate { message: Message },

    /// Message updated
    MessageUpdate {
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
    TypingStart { channel_id: Uuid, user: UserProfile },

    /// User stopped typing
    TypingStop { channel_id: Uuid, user_id: Uuid },

    /// User presence changed
    PresenceUpdate { user_id: Uuid, status: UserStatus },

    /// Voice: User joined channel
    VoiceUserJoined {
        channel_id: Uuid,
        user: UserProfile,
    },

    /// Voice: User left channel
    VoiceUserLeft { channel_id: Uuid, user_id: Uuid },

    /// Voice: SDP Offer from another user
    VoiceOffer {
        channel_id: Uuid,
        user_id: Uuid,
        sdp: String,
    },

    /// Voice: SDP Answer from another user
    VoiceAnswer {
        channel_id: Uuid,
        user_id: Uuid,
        sdp: String,
    },

    /// Voice: ICE Candidate from another user
    VoiceIce {
        channel_id: Uuid,
        user_id: Uuid,
        candidate: String,
    },

    /// Voice: User speaking indicator
    VoiceSpeaking {
        channel_id: Uuid,
        user_id: Uuid,
        speaking: bool,
    },

    /// Error
    Error { code: String, message: String },
}

/// WebSocket message wrapper with optional request ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage<T> {
    /// Optional request ID for request-response correlation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The actual event
    #[serde(flatten)]
    pub event: T,
}
