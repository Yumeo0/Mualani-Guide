fn main() {
    println!("cargo:rustc-link-search=native=C:\\Program Files\\Npcap\\Lib\\x64");
    tauri_build::build()
}
