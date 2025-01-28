use std::{fs::File, io::Read, path::PathBuf, thread};

use serde_json::Value;

use crate::constants::SLEEP_DURATION;

/// Loads the configuration from `fileforge.config.json` in the specified directory.
///
/// # Parameters
/// - `current_dir`: The path to the current directory as a `PathBuf`.
///
/// # Returns
/// - `Value`: The parsed JSON configuration as a `serde_json::Value`.
pub fn get_current_config(current_dir: PathBuf) -> Value {
    // Construct the path to the configuration file
    let config_path = current_dir.join("fileforge.config.json");
    println!("🔍 Checking for config file at: {}", config_path.display()); // Log the config file path

    thread::sleep(SLEEP_DURATION);

    // Open the configuration file
    let mut config_file = File::open(&config_path).expect("❌ Failed to open config file.");
    let mut config_content = String::new();

    // Read the content of the configuration file
    config_file
        .read_to_string(&mut config_content)
        .expect("❌ Failed to read config file content.");
    println!("✅ Config file found and loaded successfully!"); // Log success

    thread::sleep(SLEEP_DURATION);

    // Parse the configuration file content as JSON
    let config: Value =
        serde_json::from_str(&config_content).expect("❌ Failed to parse config file as JSON.");
    config
}
