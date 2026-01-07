//! Authentication HTTP Handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::api::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub mfa_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub mfa_enabled: bool,
}

// ============================================================================
// Handlers
// ============================================================================

/// Register a new local user.
pub async fn register(
    State(_state): State<AppState>,
    Json(_body): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // TODO: Implement registration
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Login with username/password.
pub async fn login(
    State(_state): State<AppState>,
    Json(_body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // TODO: Implement login
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Logout and invalidate session.
pub async fn logout(State(_state): State<AppState>) -> StatusCode {
    // TODO: Implement logout
    StatusCode::NOT_IMPLEMENTED
}

/// Refresh access token.
pub async fn refresh_token(State(_state): State<AppState>) -> Result<Json<AuthResponse>, StatusCode> {
    // TODO: Implement token refresh
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Setup MFA (TOTP).
pub async fn mfa_setup(State(_state): State<AppState>) -> StatusCode {
    // TODO: Implement MFA setup
    StatusCode::NOT_IMPLEMENTED
}

/// Verify MFA code.
pub async fn mfa_verify(State(_state): State<AppState>) -> StatusCode {
    // TODO: Implement MFA verification
    StatusCode::NOT_IMPLEMENTED
}

/// Disable MFA.
pub async fn mfa_disable(State(_state): State<AppState>) -> StatusCode {
    // TODO: Implement MFA disable
    StatusCode::NOT_IMPLEMENTED
}

/// Get available OIDC providers.
pub async fn oidc_providers(State(_state): State<AppState>) -> StatusCode {
    // TODO: Implement OIDC provider list
    StatusCode::NOT_IMPLEMENTED
}

/// Initiate OIDC authorization.
pub async fn oidc_authorize(
    State(_state): State<AppState>,
    Path(_provider): Path<String>,
) -> StatusCode {
    // TODO: Implement OIDC authorization
    StatusCode::NOT_IMPLEMENTED
}

/// Handle OIDC callback.
pub async fn oidc_callback(State(_state): State<AppState>) -> StatusCode {
    // TODO: Implement OIDC callback
    StatusCode::NOT_IMPLEMENTED
}

/// Get current user profile.
pub async fn get_profile(State(_state): State<AppState>) -> Result<Json<UserProfile>, StatusCode> {
    // TODO: Implement get profile
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Update current user profile.
pub async fn update_profile(State(_state): State<AppState>) -> StatusCode {
    // TODO: Implement update profile
    StatusCode::NOT_IMPLEMENTED
}
