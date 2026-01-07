//! Message Handlers

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::AppState;

#[derive(Debug, Serialize)]
pub struct Message {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub encrypted: bool,
    pub nonce: Option<String>,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ListMessagesQuery {
    pub before: Option<Uuid>,
    pub after: Option<Uuid>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
    pub encrypted: bool,
    pub nonce: Option<String>,
}

pub async fn list(
    State(_state): State<AppState>,
    Path(_channel_id): Path<Uuid>,
    Query(_query): Query<ListMessagesQuery>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    // TODO: Implement
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn create(
    State(_state): State<AppState>,
    Path(_channel_id): Path<Uuid>,
    Json(_body): Json<CreateMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    // TODO: Implement
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn update(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<Message>, StatusCode> {
    // TODO: Implement
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete(State(_state): State<AppState>, Path(_id): Path<Uuid>) -> StatusCode {
    // TODO: Implement
    StatusCode::NOT_IMPLEMENTED
}
