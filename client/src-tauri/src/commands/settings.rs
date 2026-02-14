//! Settings Commands
//!
//! Persistent settings and UI state stored as JSON files in the app data directory.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::command;
use tauri::Manager;

// ============================================================================
// Settings Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AudioSettings {
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    pub input_volume: f32,
    pub output_volume: f32,
    pub noise_suppression: bool,
    pub echo_cancellation: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            input_device: None,
            output_device: None,
            input_volume: 100.0,
            output_volume: 100.0,
            noise_suppression: true,
            echo_cancellation: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct VoiceSettings {
    pub push_to_talk: bool,
    pub push_to_talk_key: Option<String>,
    pub voice_activity_detection: bool,
    pub vad_threshold: f32,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            push_to_talk: false,
            push_to_talk_key: None,
            voice_activity_detection: true,
            vad_threshold: 0.5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Settings {
    pub audio: AudioSettings,
    pub voice: VoiceSettings,
    pub theme: String,
    pub notifications_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            audio: AudioSettings::default(),
            voice: VoiceSettings::default(),
            theme: "dark".into(),
            notifications_enabled: true,
        }
    }
}

// ============================================================================
// UI State Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UiState {
    pub category_collapse: HashMap<String, bool>,
}

// ============================================================================
// File Persistence Helpers
// ============================================================================

fn get_settings_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {e}"))?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {e}"))?;

    Ok(app_data_dir.join("settings.json"))
}

fn get_ui_state_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {e}"))?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {e}"))?;

    Ok(app_data_dir.join("ui_state.json"))
}

fn load_settings_from_file(path: &PathBuf) -> Settings {
    match std::fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
            tracing::warn!("Corrupt settings file, using defaults: {e}");
            Settings::default()
        }),
        Err(_) => Settings::default(),
    }
}

fn save_settings_to_file(path: &PathBuf, settings: &Settings) -> Result<(), String> {
    let json =
        serde_json::to_string_pretty(settings).map_err(|e| format!("Failed to serialize settings: {e}"))?;
    std::fs::write(path, json).map_err(|e| format!("Failed to write settings file: {e}"))
}

fn load_ui_state_from_file(path: &PathBuf) -> UiState {
    match std::fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
            tracing::warn!("Corrupt UI state file, using defaults: {e}");
            UiState::default()
        }),
        Err(_) => UiState::default(),
    }
}

fn save_ui_state_to_file(path: &PathBuf, state: &UiState) -> Result<(), String> {
    let json =
        serde_json::to_string_pretty(state).map_err(|e| format!("Failed to serialize UI state: {e}"))?;
    std::fs::write(path, json).map_err(|e| format!("Failed to write UI state file: {e}"))
}

// ============================================================================
// Settings Commands
// ============================================================================

#[command]
pub async fn get_settings(app_handle: tauri::AppHandle) -> Result<Settings, String> {
    let path = get_settings_path(&app_handle)?;
    Ok(load_settings_from_file(&path))
}

#[command]
pub async fn update_settings(
    app_handle: tauri::AppHandle,
    settings: Settings,
) -> Result<(), String> {
    let path = get_settings_path(&app_handle)?;
    save_settings_to_file(&path, &settings)
}

// ============================================================================
// UI State Commands
// ============================================================================

#[command]
pub async fn get_ui_state(app_handle: tauri::AppHandle) -> Result<UiState, String> {
    let path = get_ui_state_path(&app_handle)?;
    Ok(load_ui_state_from_file(&path))
}

#[command]
pub async fn update_category_collapse(
    app_handle: tauri::AppHandle,
    category_id: String,
    collapsed: bool,
) -> Result<(), String> {
    let path = get_ui_state_path(&app_handle)?;
    let mut state = load_ui_state_from_file(&path);
    state.category_collapse.insert(category_id, collapsed);
    save_ui_state_to_file(&path, &state)
}
