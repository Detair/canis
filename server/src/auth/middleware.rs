//! Authentication Middleware

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::api::AppState;

/// Middleware to require authentication.
pub async fn require_auth(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Extract and validate JWT from Authorization header
    // TODO: Load user from database
    // TODO: Inject user into request extensions
    
    Ok(next.run(request).await)
}
