//! Channel Management Handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::AppState;

#[derive(Debug, Serialize)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub topic: Option<String>,
    pub user_limit: Option<i32>,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelType {
    Text,
    Voice,
    Dm,
}

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub channel_type: ChannelType,
    pub topic: Option<String>,
    pub user_limit: Option<i32>,
}

pub async fn list(State(_state): State<AppState>) -> Result<Json<Vec<Channel>>, StatusCode> {
    // TODO: Implement
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn create(
    State(_state): State<AppState>,
    Json(_body): Json<CreateChannelRequest>,
) -> Result<Json<Channel>, StatusCode> {
    // TODO: Implement
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<Channel>, StatusCode> {
    // TODO: Implement
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn update(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<Channel>, StatusCode> {
    // TODO: Implement
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete(State(_state): State<AppState>, Path(_id): Path<Uuid>) -> StatusCode {
    // TODO: Implement
    StatusCode::NOT_IMPLEMENTED
}

pub async fn list_members(State(_state): State<AppState>, Path(_id): Path<Uuid>) -> StatusCode {
    // TODO: Implement
    StatusCode::NOT_IMPLEMENTED
}

pub async fn add_member(State(_state): State<AppState>, Path(_id): Path<Uuid>) -> StatusCode {
    // TODO: Implement
    StatusCode::NOT_IMPLEMENTED
}

pub async fn remove_member(
    State(_state): State<AppState>,
    Path((_channel_id, _user_id)): Path<(Uuid, Uuid)>,
) -> StatusCode {
    // TODO: Implement
    StatusCode::NOT_IMPLEMENTED
}
