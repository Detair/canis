//! File redirect endpoint — generates presigned S3 URLs on-the-fly.
//!
//! `GET /api/files/{key...}` → 302 redirect to a fresh presigned S3 URL.
//! No auth required (URLs are shared in messages, profiles).
//! Browser caches the redirect for ~1 hour via `Cache-Control`.

use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;

use super::AppState;

/// Convert an S3 key to an API file URL.
///
/// Returns `/api/files/{key}` — the redirect endpoint that generates presigned URLs.
pub fn file_url(s3_key: &str) -> String {
    format!("/api/files/{s3_key}")
}

/// Redirect to a presigned S3 URL for the given file key.
pub async fn redirect(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let s3 = match &state.s3 {
        Some(s3) => s3,
        None => {
            return (StatusCode::SERVICE_UNAVAILABLE, "File storage not configured")
                .into_response()
        }
    };

    match s3.presign_get(&key).await {
        Ok(presigned_url) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::LOCATION, presigned_url.parse().unwrap());
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=3500".parse().unwrap(),
            );
            (StatusCode::FOUND, headers, "").into_response()
        }
        Err(e) => {
            tracing::warn!(key = %key, error = %e, "Failed to generate presigned URL for file redirect");
            (StatusCode::NOT_FOUND, "File not found").into_response()
        }
    }
}
