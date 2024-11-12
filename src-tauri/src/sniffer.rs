// sniffer.rs

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use artifactarium::{GameCommand, GamePacket, GameSniffer};
use base64::prelude::*;
use pcap::{Capture, Device};
use proto_gen::generated::{
    pack_ids,
    APIKeyInfo::APIKeyNotify,
    AchievementNotify::{AchievementNotify, AchievementUpdateNotify},
    AvatarNotify::{
        AvatarFightPropUpdate, AvatarNotify, AvatarPropertyUpdate, AvatarSkillUpdate,
        TeamSwapNotify,
    },
    FriendInit::FriendInit,
    PlayerInfo::PlayerInfo,
    PlayerTokenRsp::PlayerTokenRsp,
    PlayerUpdate::PlayerUpdate,
    QuestNotify::QuestNotify,
    SceneUpdate::{SceneEntityDieUpdate, SceneUpdate},
    StoreNotify::StoreNotify,
    StoreUpdate::StoreUpdate,
    WorldPlayerLocationNotify::WorldNotify,
};
use protobuf_json_mapping::print_to_string;
use std::result::Result::Ok;
use tauri::{AppHandle, Emitter};
use tracing::{debug, error, info};

pub struct PacketSniffer {
    shutdown_hook: Arc<Mutex<bool>>,
    capture_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}

pub fn handle_packet<T: protobuf::MessageFull>(
    event_name: &str,
    command: GameCommand,
    app_handle: AppHandle,
) {
    debug!("Handling packet: {:?}", command.command_id);
    if let Ok(data) = command.parse_proto::<T>() {
        app_handle
            .emit(event_name, print_to_string(&data).unwrap())
            .unwrap();
    }
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
        *shutdown_hook.lock().unwrap() = false;

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
                            // debug!("Received command: {:?}", command.command_id);
                            match command.command_id {
                                pack_ids::ACHIEVEMENTNOTIFY_ID => {
                                    handle_packet::<AchievementNotify>(
                                        "achievement_notify",
                                        command,
                                        app_handle.clone(),
                                    )
                                }
                                pack_ids::ACHIEVEMENTUPDATE_ID => {
                                    handle_packet::<AchievementUpdateNotify>(
                                        "achievement_update_notify",
                                        command,
                                        app_handle.clone(),
                                    )
                                }
                                pack_ids::APIKEYINFO_ID => handle_packet::<APIKeyNotify>(
                                    "api_key_notify",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::AVATARFIGHTPROP_ID => {
                                    handle_packet::<AvatarFightPropUpdate>(
                                        "avatar_fight_prop_update",
                                        command,
                                        app_handle.clone(),
                                    )
                                }
                                pack_ids::AVATARNOTIFY_ID => handle_packet::<AvatarNotify>(
                                    "avatar_notify",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::AVATARPROP_ID => handle_packet::<AvatarPropertyUpdate>(
                                    "avatar_property_update",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::FRIENDINIT_ID => handle_packet::<FriendInit>(
                                    "friend_init",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::PLAYERINFO_ID => handle_packet::<PlayerInfo>(
                                    "player_info",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::PLAYERTOKENRSP_ID => handle_packet::<PlayerTokenRsp>(
                                    "player_token_rsp",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::PLAYERUPDATE_ID => handle_packet::<PlayerUpdate>(
                                    "player_update",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::QUESTNOTIFY_ID => handle_packet::<QuestNotify>(
                                    "quest_notify",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::SCENEENTITYDIE_ID => {
                                    handle_packet::<SceneEntityDieUpdate>(
                                        "scene_entity_die_update",
                                        command,
                                        app_handle.clone(),
                                    )
                                }
                                pack_ids::SCENEUPDATE_ID => handle_packet::<SceneUpdate>(
                                    "scene_update",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::SKILLUPDATE_ID => handle_packet::<AvatarSkillUpdate>(
                                    "avatar_skill_update",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::STORENOTIFY_ID => handle_packet::<StoreNotify>(
                                    "store_notify",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::STOREUPDATE_ID => handle_packet::<StoreUpdate>(
                                    "store_update",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::TEAMSWAP_ID => handle_packet::<TeamSwapNotify>(
                                    "team_swap_notify",
                                    command,
                                    app_handle.clone(),
                                ),
                                pack_ids::WORLDNOTIFY_ID => handle_packet::<WorldNotify>(
                                    "world_notify",
                                    command,
                                    app_handle.clone(),
                                ),
                                _ => continue, //warn!("Packet not handled yet: {:?}", command.command_id),
                            };
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
        serde_json::from_slice(include_bytes!("../resources/gi/keys.json"))?;

    let mut keys_bytes = HashMap::new();

    for (k, v) in keys {
        keys_bytes.insert(k, BASE64_STANDARD.decode(v)?);
    }

    Ok(keys_bytes)
}
