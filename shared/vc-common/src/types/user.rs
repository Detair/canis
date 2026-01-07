//! User Types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Online,
    Away,
    Busy,
    Offline,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Offline
    }
}

/// User profile (public information).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
}

/// Full user data (for authenticated user).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub mfa_enabled: bool,
    pub created_at: DateTime<Utc>,
}
