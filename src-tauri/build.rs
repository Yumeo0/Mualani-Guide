use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-search=native=C:\\Program Files\\Npcap\\Lib\\x64");
    tauri_build::build();
    if env::consts::OS == "linux" && env::var("CARGO_FEATURE_SETCAP").is_ok() {
        let binary_path = "src-tauri/target/debug/mualani_guide";

        let status = Command::new("sudo")
            .arg("setcap")
            .arg("CAP_NET_RAW=+ep")
            .arg(binary_path)
            .status()
            .expect("Failed to run setcap");

        if !status.success() {
            panic!("setcap command failed");
        }
    }
}
