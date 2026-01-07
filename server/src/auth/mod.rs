//! Authentication Service
//!
//! Handles local authentication, SSO/OIDC, MFA, and session management.

mod handlers;
mod jwt;
mod middleware;
mod oidc;
mod password;

use axum::{
    routing::{get, post},
    Router,
};

use crate::api::AppState;

pub use middleware::require_auth;

/// Create authentication router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Local auth
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout))
        .route("/refresh", post(handlers::refresh_token))
        // MFA
        .route("/mfa/setup", post(handlers::mfa_setup))
        .route("/mfa/verify", post(handlers::mfa_verify))
        .route("/mfa/disable", post(handlers::mfa_disable))
        // OIDC
        .route("/oidc/providers", get(handlers::oidc_providers))
        .route("/oidc/authorize/:provider", get(handlers::oidc_authorize))
        .route("/oidc/callback", get(handlers::oidc_callback))
        // Profile
        .route("/me", get(handlers::get_profile))
        .route("/me", post(handlers::update_profile))
}
