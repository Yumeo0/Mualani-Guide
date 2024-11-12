// lib.rs

mod settings;
mod sniffer;
mod state;

use pcap::Device;
use serde::Serialize;
use settings::{load_settings, save_settings};
use state::{set_device, start_packet_capture, stop_packet_capture, SnifferState};
use std::path::PathBuf;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter("mualani_guide=debug,artifactarium=warn")
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Get the app_handle from the app instance
            let app_handle = app.handle();

            // Manage SnifferState with the app_handle
            app.manage(SnifferState::create(app_handle.clone()));

            Ok(())
        })
        .manage(AppState {
            settings_path: PathBuf::from(tauri::path::BaseDirectory::AppData.variable()),
        })
        .invoke_handler(tauri::generate_handler![
            start_packet_capture,
            stop_packet_capture,
            list_devices,
            set_device,
            load_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AppState {
    settings_path: PathBuf,
}

// Add this new struct to serialize device information
#[derive(Serialize)]
pub struct DeviceInfo {
    index: usize,
    name: String,
    description: Option<String>,
}

#[tauri::command]
async fn list_devices() -> Result<Vec<DeviceInfo>, String> {
    Device::list().map_err(|e| e.to_string()).map(|devices| {
        devices
            .into_iter()
            .enumerate()
            .map(|(index, device)| DeviceInfo {
                index,
                name: device.name,
                description: device.desc,
            })
            .collect()
    })
}
