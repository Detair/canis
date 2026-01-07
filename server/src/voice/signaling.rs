//! WebRTC Signaling Messages

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Signaling message types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SignalingMessage {
    /// Join voice channel
    Join { channel_id: Uuid },
    /// Leave voice channel
    Leave { channel_id: Uuid },
    /// SDP Offer
    Offer { channel_id: Uuid, sdp: String },
    /// SDP Answer
    Answer { channel_id: Uuid, sdp: String },
    /// ICE Candidate
    IceCandidate { channel_id: Uuid, candidate: String },
    /// Mute self
    Mute { channel_id: Uuid },
    /// Unmute self
    Unmute { channel_id: Uuid },
    /// User joined notification
    UserJoined { channel_id: Uuid, user_id: Uuid },
    /// User left notification
    UserLeft { channel_id: Uuid, user_id: Uuid },
    /// User speaking indicator
    Speaking { channel_id: Uuid, user_id: Uuid, speaking: bool },
}
