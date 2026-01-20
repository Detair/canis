//! Voice metrics storage.
//!
//! This module provides functions for storing voice connection metrics
//! in TimescaleDB for historical analysis.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::stats::VoiceStats;

/// Store connection metrics in TimescaleDB (fire-and-forget).
///
/// This function is designed to be spawned as a background task.
/// Errors are logged but not propagated to avoid impacting the
/// caller's flow.
pub async fn store_metrics(
    pool: PgPool,
    stats: VoiceStats,
    user_id: Uuid,
    channel_id: Uuid,
    guild_id: Option<Uuid>,
) {
    let result = sqlx::query(
        r#"
        INSERT INTO connection_metrics
        (time, user_id, session_id, channel_id, guild_id, latency_ms, packet_loss, jitter_ms, quality)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(Utc::now())
    .bind(user_id)
    .bind(stats.session_id)
    .bind(channel_id)
    .bind(guild_id)
    .bind(stats.latency)
    .bind(stats.packet_loss)
    .bind(stats.jitter)
    .bind(stats.quality as i16)
    .execute(&pool)
    .await;

    if let Err(e) = result {
        tracing::warn!(
            user_id = %user_id,
            session_id = %stats.session_id,
            "Failed to store connection metrics: {}",
            e
        );
    }
}

/// Get guild_id from channel_id.
///
/// Returns `None` if the channel doesn't exist or doesn't belong to a guild.
pub async fn get_guild_id(pool: &PgPool, channel_id: Uuid) -> Option<Uuid> {
    sqlx::query_scalar("SELECT guild_id FROM channels WHERE id = $1")
        .bind(channel_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}
