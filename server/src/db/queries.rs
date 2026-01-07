//! Database Queries

use chrono::{DateTime, Utc};
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

/// Find user by email.
pub async fn find_user_by_email(pool: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(pool)
        .await
}

/// Check if username exists.
pub async fn username_exists(pool: &PgPool, username: &str) -> sqlx::Result<bool> {
    let result = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1) as exists",
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(false))
}

/// Check if email exists.
pub async fn email_exists(pool: &PgPool, email: &str) -> sqlx::Result<bool> {
    let result = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists",
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(false))
}

/// Create a new local user.
pub async fn create_user(
    pool: &PgPool,
    username: &str,
    display_name: &str,
    email: Option<&str>,
    password_hash: &str,
) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, display_name, email, password_hash, auth_method)
        VALUES ($1, $2, $3, $4, 'local')
        RETURNING
            id, username, display_name, email, password_hash,
            auth_method as "auth_method: AuthMethod",
            external_id, avatar_url,
            status as "status: UserStatus",
            mfa_secret, created_at, updated_at
        "#,
        username,
        display_name,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await
}

/// Update user's MFA secret.
pub async fn set_mfa_secret(
    pool: &PgPool,
    user_id: Uuid,
    mfa_secret: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE users SET mfa_secret = $1, updated_at = NOW() WHERE id = $2",
        mfa_secret,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// Session Queries
// ============================================================================

/// Create a new session (for refresh token tracking).
pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    token_hash: &str,
    expires_at: DateTime<Utc>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> sqlx::Result<Session> {
    sqlx::query_as!(
        Session,
        r#"
        INSERT INTO sessions (user_id, token_hash, expires_at, ip_address, user_agent)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, token_hash, expires_at, ip_address, user_agent, created_at
        "#,
        user_id,
        token_hash,
        expires_at,
        ip_address,
        user_agent
    )
    .fetch_one(pool)
    .await
}

/// Find session by token hash.
pub async fn find_session_by_token_hash(
    pool: &PgPool,
    token_hash: &str,
) -> sqlx::Result<Option<Session>> {
    sqlx::query_as!(
        Session,
        r#"
        SELECT id, user_id, token_hash, expires_at, ip_address, user_agent, created_at
        FROM sessions
        WHERE token_hash = $1 AND expires_at > NOW()
        "#,
        token_hash
    )
    .fetch_optional(pool)
    .await
}

/// Delete a session by ID.
pub async fn delete_session(pool: &PgPool, session_id: Uuid) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM sessions WHERE id = $1", session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete a session by token hash.
pub async fn delete_session_by_token_hash(pool: &PgPool, token_hash: &str) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM sessions WHERE token_hash = $1", token_hash)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all sessions for a user (logout everywhere).
pub async fn delete_all_user_sessions(pool: &PgPool, user_id: Uuid) -> sqlx::Result<u64> {
    let result = sqlx::query!("DELETE FROM sessions WHERE user_id = $1", user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

/// Clean up expired sessions (for background job).
pub async fn cleanup_expired_sessions(pool: &PgPool) -> sqlx::Result<u64> {
    let result = sqlx::query!("DELETE FROM sessions WHERE expires_at < NOW()")
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
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
