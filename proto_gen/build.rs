use protobuf_codegen::Codegen;
use std::fs;
use std::path::PathBuf;
use tracing::warn;

fn main() {
    // Define paths
    let proto_src = "resources/protos";
    let proto_out = "generated";
    let pack_ids_path = "resources/PackIDS.inc";
    let pack_ids_out_path = "generated/pack_ids.rs";
    let mod_path = "generated/mod.rs";

    // Create the output directory if it doesn't exist
    fs::create_dir_all(proto_out).expect("Failed to create output directory");

    let proto_files = get_proto_files(proto_src);
    if proto_files.len() == 0 {
        warn!(
            "Skipping protobuf generation: no .proto files found in {}",
            proto_src
        );
        return;
    }

    // Generate Rust files from .proto
    Codegen::new()
        .out_dir(proto_out)
        .inputs(&proto_files)
        .includes(&[proto_src])
        .run()
        .expect("Failed to generate protobuf Rust files");

    // Extend generated files with IDs
    extend_with_ids(&pack_ids_path, pack_ids_out_path);

    // Now, update the mod.rs file to include PackIDs
    let mut mod_content = fs::read_to_string(mod_path).expect("Failed to read mod.rs");

    // Append the `pub mod PackIDs;` if it's not already present
    mod_content.push_str("\npub mod pack_ids;\n");
    // Write the updated mod.rs back to disk
    fs::write(mod_path, mod_content).expect("Failed to update mod.rs");
}

fn get_proto_files(proto_src: &str) -> Vec<PathBuf> {
    fs::read_dir(proto_src)
        .expect("Failed to read proto directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "proto" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

// Helper function to parse a line of the form "#define NAME VALUE"
fn parse_define(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 3 && parts[0] == "#define" {
        // Return the name and value (without "#define")
        return Some((parts[1].to_string(), parts[2].to_string()));
    }
    None
}

fn extend_with_ids(in_dir: &str, out_dir: &str) {
    let content = fs::read_to_string(in_dir).expect("Failed to read PackIDs.inc file");

    // Start building the output content
    let mut output = String::from("// Generated file. Do not modify manually.\n\n");

    // Parse each line in the input file
    for line in content.lines() {
        // Ignore empty lines or comments
        if line.trim().is_empty() || line.starts_with("//") {
            continue;
        }

        // Match lines that look like "#define NAME VALUE"
        if let Some((name, hex_value)) = parse_define(line) {
            // Convert the hex value to decimal
            let decimal_value =
                u32::from_str_radix(&hex_value[2..], 16).expect("Invalid hex value");

            // Write the definition in Rust format
            output.push_str(&format!("pub const {}: u16 = {};\n", name, decimal_value));
        }
    }

    // Write the generated content to the output file
    fs::write(out_dir, output).expect("Failed to write pack_ids.rs");
}
