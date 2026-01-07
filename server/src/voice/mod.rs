//! Voice Service (SFU)
//!
//! WebRTC Selective Forwarding Unit for voice channels.

mod handlers;
mod sfu;
mod signaling;

use axum::{
    routing::{get, post},
    Router,
};

use crate::api::AppState;

pub use sfu::SfuServer;

/// Create voice router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ice-servers", get(handlers::get_ice_servers))
        .route("/join/:channel_id", post(handlers::join_channel))
        .route("/leave/:channel_id", post(handlers::leave_channel))
}
