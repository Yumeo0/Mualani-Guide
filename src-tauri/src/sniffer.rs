use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use artifactarium::{GamePacket, GameSniffer};
use base64::prelude::*;
use pcap::{Capture, Device};
use proto_gen::generated::{pack_ids, PlayerInfo::PlayerInfo, PlayerTokenRsp::PlayerTokenRsp};
use protobuf_json_mapping::print_to_string;
use std::result::Result::Ok;
use tauri::{AppHandle, Emitter};
use tracing::{debug, error, info};

pub struct PacketSniffer {
    shutdown_hook: Arc<Mutex<bool>>,
    capture_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl PacketSniffer {
    pub fn new() -> Self {
        PacketSniffer {
            shutdown_hook: Arc::new(Mutex::new(false)),
            capture_thread: Mutex::new(None),
        }
    }

    pub fn start_capture(&self, device: Device, app_handle: AppHandle) -> anyhow::Result<()> {
        let keys = load_keys()?;
        let mut sniffer = GameSniffer::new().set_initial_keys(keys);
        let shutdown_hook = Arc::clone(&self.shutdown_hook);

        let handle = thread::spawn(move || {
            let mut capturer = Capture::from_device(device)
                .expect("Failed to open device")
                .timeout(1000)
                .open()
                .expect("Failed to start packet capture");
            _ = capturer.filter("udp portrange 22101-22102", true);

            while *shutdown_hook.lock().unwrap() == false {
                match capturer.next_packet() {
                    Ok(packet) => {
                        let Some(GamePacket::Commands(commands)) =
                            sniffer.receive_packet(packet.data.to_vec())
                        else {
                            continue;
                        };

                        for command in commands {
                            if command.command_id == pack_ids::PLAYERTOKENRSP_ID {
                                if let Ok(data) = command.parse_proto::<PlayerTokenRsp>() {
                                    info!("PlayerTokenRsp: {:?}", data);
                                    app_handle
                                        .emit("player_token_rsp", print_to_string(&data).unwrap())
                                        .unwrap();
                                }
                            } else if command.command_id == pack_ids::PLAYERINFO_ID {
                                if let Ok(data) = command.parse_proto::<PlayerInfo>() {
                                    info!("PlayerInfo: {}", data);
                                    app_handle
                                        .emit("player_info", print_to_string(&data).unwrap())
                                        .unwrap();
                                }
                            }
                        }
                    }
                    Err(pcap::Error::TimeoutExpired) => {
                        debug!("Packet capture timed out, continuing...");
                        continue;
                    }
                    Err(e) => {
                        error!("Failed to capture packet: {}", e);
                    }
                }
            }

            info!("Packet capture thread shutting down.");
        });

        *self.capture_thread.lock().unwrap() = Some(handle);

        Ok(())
    }

    pub fn stop_capture(&self) {
        *self.shutdown_hook.lock().unwrap() = true;
        if let Some(handle) = self.capture_thread.lock().unwrap().take() {
            info!("Waiting for capture thread to finish...");
            handle.join().expect("Failed to join capture thread");
        }
    }
}

fn load_keys() -> anyhow::Result<HashMap<u16, Vec<u8>>> {
    let keys: HashMap<u16, String> =
        serde_json::from_slice(include_bytes!("../../public/keys.json"))?;

    let mut keys_bytes = HashMap::new();

    for (k, v) in keys {
        keys_bytes.insert(k, BASE64_STANDARD.decode(v)?);
    }

    Ok(keys_bytes)
}
