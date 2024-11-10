// main.rs

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pcap::Device;
use tracing::info;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter("mualani_guide=info")
        .init();

    info!("Starting up...");

    info!("Available devices:");
    let mut index = 0;
    for device in Device::list().unwrap() {
        info!("{}: {} <{}>", index, device.name, device.desc.unwrap());
        index += 1;
    }

    mualani_guide_lib::run();
}
