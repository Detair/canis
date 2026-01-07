//! Channel Types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Channel type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelType {
    Text,
    Voice,
    Dm,
}

/// Channel data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub category_id: Option<Uuid>,
    pub topic: Option<String>,
    pub user_limit: Option<u32>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

/// Channel category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCategory {
    pub id: Uuid,
    pub name: String,
    pub position: i32,
    pub channels: Vec<Channel>,
}
