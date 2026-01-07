//! Selective Forwarding Unit Implementation

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Voice channel participant.
pub struct Participant {
    pub user_id: Uuid,
    pub muted: bool,
    // TODO: WebRTC peer connection
}

/// Voice channel room.
pub struct Room {
    pub channel_id: Uuid,
    pub participants: HashMap<Uuid, Participant>,
}

/// SFU Server managing all voice rooms.
pub struct SfuServer {
    rooms: Arc<RwLock<HashMap<Uuid, Room>>>,
}

impl SfuServer {
    /// Create a new SFU server.
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create a room for a channel.
    pub async fn get_or_create_room(&self, channel_id: Uuid) -> Uuid {
        let mut rooms = self.rooms.write().await;
        if !rooms.contains_key(&channel_id) {
            rooms.insert(
                channel_id,
                Room {
                    channel_id,
                    participants: HashMap::new(),
                },
            );
        }
        channel_id
    }

    /// Add a participant to a room.
    pub async fn add_participant(&self, _channel_id: Uuid, _user_id: Uuid) {
        // TODO: Implement WebRTC peer connection setup
    }

    /// Remove a participant from a room.
    pub async fn remove_participant(&self, _channel_id: Uuid, _user_id: Uuid) {
        // TODO: Implement
    }
}

impl Default for SfuServer {
    fn default() -> Self {
        Self::new()
    }
}
