use rust_embed::RustEmbed;
use std::path::PathBuf;
use std::fs;
use std::io::Read;

use crate::config::MODDING;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub fn load_biomes() {
    let mut json_files = Vec::new();
    let mut exe_dir: PathBuf = PathBuf::new();
    if MODDING {
        let exe_path = std::env::current_exe().expect("Failed to get current executable path");
        exe_dir = exe_path.parent().expect("Failed to get executable directory").to_path_buf();
        let models_dir = exe_dir.join("assets/biomes");
        if models_dir.exists() && models_dir.is_dir() {
            println!("Found the modded directory for biomes");
            for entry in fs::read_dir(&models_dir).expect("Failed to read models directory") {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "json") {
                        if let Some(file_name) = path.strip_prefix(&exe_dir).ok().and_then(|p| p.to_str()) {
                            println!("Found the modded biome file: {}", file_name);
                            json_files.push(file_name.to_string());
                        }
                    }
                }
            }
        }
    }
    json_files.extend(
        Assets::iter()
            .filter(|file| file.starts_with("biomes/") && file.ends_with(".json"))
            .map(|file| file.to_string())
    );
    json_files.sort();
    json_files.dedup();
    for file in json_files {
        println!("Found JSON file: {}", file);
        let file_path = exe_dir.join(&file);
        if file_path.exists() {
            let mut file_content = String::new();
            let mut file = fs::File::open(&file_path).expect("Failed to open file");
            file.read_to_string(&mut file_content).expect("Failed to read file");
            //handle_biome_data(&mut world_data, &file_content);
        } else if let Some(asset) = Assets::get(&file) {
            let json_content = std::str::from_utf8(asset.data.as_ref()).expect("Invalid UTF-8");
            //handle_biome_data(&mut world_data, json_content);
        }
    }
}