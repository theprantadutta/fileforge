use std::{fs::File, io::Read, path::PathBuf, thread};

use serde_json::Value;

use crate::constants::SLEEP_DURATION;

pub fn get_current_config(current_dir: PathBuf) -> Value {
    // Load the configuration from `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    println!("🔍 Checking for config file at: {:?}", config_path);
    thread::sleep(SLEEP_DURATION);

    let mut config_file = File::open(&config_path).unwrap();
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).unwrap();
    println!("✅ Config file found and loaded!");
    thread::sleep(SLEEP_DURATION);

    let config: Value = serde_json::from_str(&config_content).unwrap();
    config
}
