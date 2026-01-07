//! VoiceChat Desktop Client Library
//!
//! Tauri backend for the desktop application.

mod audio;
mod commands;
mod crypto;
mod network;
mod webrtc;

use tauri::Manager;

/// Run the Tauri application.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize logging
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "vc_client=debug".into()),
                )
                .init();

            tracing::info!("VoiceChat Client starting");

            // Store app state
            app.manage(AppState::new());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::login,
            commands::auth::logout,
            commands::auth::get_current_user,
            commands::chat::get_channels,
            commands::chat::get_messages,
            commands::chat::send_message,
            commands::voice::join_voice,
            commands::voice::leave_voice,
            commands::voice::set_mute,
            commands::voice::set_deafen,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Application state.
pub struct AppState {
    // TODO: Add state fields
}

impl AppState {
    fn new() -> Self {
        Self {}
    }
}
