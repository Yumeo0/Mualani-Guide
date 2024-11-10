use crate::sniffer::PacketSniffer;
use pcap::Device;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::State;
use tracing::info;

pub struct SnifferState {
    app_handle: AppHandle,
    sniffer: Arc<Mutex<Option<PacketSniffer>>>,
    device_id: Arc<Mutex<Option<usize>>>,
}

impl SnifferState {
    pub fn create(app_handle: AppHandle) -> Self {
        Self {
            app_handle: app_handle.clone(),
            sniffer: Arc::new(Mutex::new(None)),
            device_id: Arc::new(Mutex::new(Some(0))),
        }
    }
}

#[tauri::command]
pub async fn set_device(state: State<'_, SnifferState>, index: usize) -> Result<(), String> {
    let mut device_id = state.device_id.lock().map_err(|e| e.to_string())?;
    *device_id = Some(index);
    Ok(())
}

#[tauri::command]
pub async fn start_packet_capture(state: State<'_, SnifferState>) -> Result<String, String> {
    let device_id = state.device_id.lock().map_err(|e| e.to_string())?;
    let device_index = device_id.ok_or_else(|| "No device selected".to_string())?;
    let mut sniffer_guard = state.sniffer.lock().map_err(|e| e.to_string())?;

    // Create new sniffer instance if none exists
    if sniffer_guard.is_none() {
        *sniffer_guard = Some(PacketSniffer::new());
    }

    if let Some(sniffer) = sniffer_guard.as_ref() {
        // Get default device
        let device = Device::list().unwrap()[device_index].clone();
        info!("Listening on device: {}", device.name);

        // Start capture
        sniffer
            .start_capture(device, state.app_handle.clone())
            .map_err(|e| e.to_string())?;
        Ok("Capture started successfully".to_string())
    } else {
        Err("Failed to initialize sniffer".to_string())
    }
}

#[tauri::command]
pub async fn stop_packet_capture(state: State<'_, SnifferState>) -> Result<String, String> {
    let sniffer_guard = state.sniffer.lock().map_err(|e| e.to_string())?;

    if let Some(sniffer) = sniffer_guard.as_ref() {
        sniffer.stop_capture();
        Ok("Capture stopped successfully".to_string())
    } else {
        Err("No active sniffer instance".to_string())
    }
}
