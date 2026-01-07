//! Message Types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserProfile;

/// Message data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub author: UserProfile,
    pub content: String,
    pub encrypted: bool,
    pub attachments: Vec<Attachment>,
    pub reply_to: Option<Uuid>,
    pub edited_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// File attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub url: String,
}
