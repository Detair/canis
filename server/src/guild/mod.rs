//! Guild (Server) Management Module
//!
//! Handles guild creation, membership, invites, and management.

pub mod handlers;
pub mod invites;
pub mod types;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::api::AppState;

/// Create the guild router with all endpoints
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_guilds).post(handlers::create_guild))
        .route(
            "/:id",
            get(handlers::get_guild)
                .patch(handlers::update_guild)
                .delete(handlers::delete_guild),
        )
        .route("/:id/join", post(handlers::join_guild))
        .route("/:id/leave", post(handlers::leave_guild))
        .route("/:id/members", get(handlers::list_members))
        .route("/:id/members/:user_id", delete(handlers::kick_member))
        .route("/:id/channels", get(handlers::list_channels))
        // Invite routes
        .route(
            "/:id/invites",
            get(invites::list_invites).post(invites::create_invite),
        )
        .route("/:id/invites/:code", delete(invites::delete_invite))
}

/// Create the invite join router (separate for public access pattern)
pub fn invite_router() -> Router<AppState> {
    Router::new().route("/:code/join", post(invites::join_via_invite))
}
