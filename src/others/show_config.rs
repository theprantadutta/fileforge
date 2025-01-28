use std::{env, io, process::exit, thread};

use crate::{constants::SLEEP_DURATION, shared};

pub fn show_config() -> io::Result<()> {
    // Get the current working directory
    let current_dir = env::current_dir().unwrap_or_else(|e| {
        eprintln!("❌ Error getting current directory: {}", e);
        exit(1); // Exit with an error code if the current directory cannot be retrieved
    });
    println!("📂 Current directory: {:?}", current_dir);

    // Check if the `fileforge.config.json` file exists at the root of the project
    let config_path = std::path::Path::new("fileforge.config.json");

    if !config_path.exists() {
        eprintln!(
            "❌ Error: `fileforge.config.json` not found. Run 'fileforge init' to generate a config."
        );
        exit(1); // Exit with an error code if the config file is missing
    }

    // Load the configuration from `fileforge.config.json`
    let config = shared::get_current_config::get_current_config(current_dir);

    // Introduce a small delay for better user experience (optional)
    thread::sleep(SLEEP_DURATION);

    // Display the configuration in a pretty-printed JSON format
    println!(
        "🔧 Config File: \n\n{}",
        serde_json::to_string_pretty(&config).unwrap_or_else(|e| {
            eprintln!("❌ Error formatting configuration as JSON: {}", e);
            exit(1); // Exit with an error code if JSON formatting fails
        })
    );

    Ok(()) // Return `Ok` if everything succeeds
}
