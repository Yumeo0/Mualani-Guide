use crate::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    theme: String,
    notifications: bool,
    language: String,
}

#[tauri::command]
pub fn load_settings(state: State<AppState>) -> Result<Settings, String> {
    let settings_path = &state.settings_path;

    if !settings_path.exists() {
        let default_settings = Settings {
            theme: "light".to_string(),
            notifications: true,
            language: "en".to_string(),
        };
        save_settings(default_settings.clone(), state)?;
        return Ok(default_settings);
    }

    let settings = fs::read_to_string(settings_path)
        .map_err(|_| "Failed to read settings file".to_string())?;
    serde_json::from_str(&settings).map_err(|_| "Failed to parse settings".to_string())
}

#[tauri::command]
pub fn save_settings(settings: Settings, state: State<AppState>) -> Result<(), String> {
    let settings_path = &state.settings_path;
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|_| "Failed to serialize settings".to_string())?;
    fs::write(settings_path, json).map_err(|_| "Failed to write settings".to_string())?;
    Ok(())
}
