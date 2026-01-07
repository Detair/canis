//! Authentication Commands

use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub server_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[command]
pub async fn login(_request: LoginRequest) -> Result<User, String> {
    // TODO: Implement
    Err("Not implemented".into())
}

#[command]
pub async fn logout() -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[command]
pub async fn get_current_user() -> Result<Option<User>, String> {
    // TODO: Implement
    Ok(None)
}
