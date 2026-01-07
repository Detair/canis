//! Database Queries

use sqlx::PgPool;
use uuid::Uuid;

use super::models::*;

// ============================================================================
// User Queries
// ============================================================================

/// Find user by ID.
pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}

/// Find user by username.
pub async fn find_user_by_username(pool: &PgPool, username: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_optional(pool)
        .await
}

/// Find user by external ID (for OIDC).
pub async fn find_user_by_external_id(
    pool: &PgPool,
    external_id: &str,
) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE external_id = $1",
        external_id
    )
    .fetch_optional(pool)
    .await
}

// ============================================================================
// Channel Queries
// ============================================================================

/// List all channels.
pub async fn list_channels(pool: &PgPool) -> sqlx::Result<Vec<Channel>> {
    sqlx::query_as!(Channel, "SELECT * FROM channels ORDER BY position ASC")
        .fetch_all(pool)
        .await
}

/// Find channel by ID.
pub async fn find_channel_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Channel>> {
    sqlx::query_as!(Channel, "SELECT * FROM channels WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}

// ============================================================================
// Message Queries
// ============================================================================

/// List messages in a channel with pagination.
pub async fn list_messages(
    pool: &PgPool,
    channel_id: Uuid,
    before: Option<Uuid>,
    limit: i64,
) -> sqlx::Result<Vec<Message>> {
    if let Some(before_id) = before {
        sqlx::query_as!(
            Message,
            r#"
            SELECT * FROM messages 
            WHERE channel_id = $1 
              AND deleted_at IS NULL
              AND id < $2
            ORDER BY created_at DESC 
            LIMIT $3
            "#,
            channel_id,
            before_id,
            limit
        )
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as!(
            Message,
            r#"
            SELECT * FROM messages 
            WHERE channel_id = $1 
              AND deleted_at IS NULL
            ORDER BY created_at DESC 
            LIMIT $2
            "#,
            channel_id,
            limit
        )
        .fetch_all(pool)
        .await
    }
}
