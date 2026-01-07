//! Voice Commands

use tauri::command;

#[command]
pub async fn join_voice(channel_id: String) -> Result<(), String> {
    // TODO: Implement
    Err("Not implemented".into())
}

#[command]
pub async fn leave_voice() -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[command]
pub async fn set_mute(muted: bool) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}

#[command]
pub async fn set_deafen(deafened: bool) -> Result<(), String> {
    // TODO: Implement
    Ok(())
}
