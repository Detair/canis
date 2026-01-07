//! File Upload Handling

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::api::AppState;

#[derive(Debug, Serialize)]
pub struct UploadedFile {
    pub id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub size: i64,
    pub url: String,
}

pub async fn upload_file(
    State(_state): State<AppState>,
    // TODO: Use axum Multipart
) -> Result<Json<UploadedFile>, StatusCode> {
    // TODO: Implement file upload to S3
    Err(StatusCode::NOT_IMPLEMENTED)
}
