//! Chat Commands

use serde::Serialize;
use tauri::command;

#[derive(Debug, Serialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub channel_type: String,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub content: String,
    pub author_id: String,
}

#[command]
pub async fn get_channels() -> Result<Vec<Channel>, String> {
    // TODO: Implement
    Ok(vec![])
}

#[command]
pub async fn get_messages(channel_id: String, limit: Option<u32>) -> Result<Vec<Message>, String> {
    // TODO: Implement
    Ok(vec![])
}

#[command]
pub async fn send_message(channel_id: String, content: String) -> Result<Message, String> {
    // TODO: Implement
    Err("Not implemented".into())
}
