//! Voice HTTP Handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::api::AppState;

#[derive(Debug, Serialize)]
pub struct IceServer {
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IceServersResponse {
    pub ice_servers: Vec<IceServer>,
}

/// Get ICE server configuration.
pub async fn get_ice_servers(State(state): State<AppState>) -> Json<IceServersResponse> {
    let mut servers = vec![IceServer {
        urls: vec![state.config.stun_server.clone()],
        username: None,
        credential: None,
    }];

    if let Some(turn) = &state.config.turn_server {
        servers.push(IceServer {
            urls: vec![turn.clone()],
            username: state.config.turn_username.clone(),
            credential: state.config.turn_credential.clone(),
        });
    }

    Json(IceServersResponse { ice_servers: servers })
}

/// Join a voice channel.
pub async fn join_channel(
    State(_state): State<AppState>,
    Path(_channel_id): Path<Uuid>,
) -> StatusCode {
    // TODO: Implement - returns SDP offer
    StatusCode::NOT_IMPLEMENTED
}

/// Leave a voice channel.
pub async fn leave_channel(
    State(_state): State<AppState>,
    Path(_channel_id): Path<Uuid>,
) -> StatusCode {
    // TODO: Implement
    StatusCode::NOT_IMPLEMENTED
}
