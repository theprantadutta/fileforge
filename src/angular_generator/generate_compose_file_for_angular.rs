use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::{env, thread};

use crate::constants::SLEEP_DURATION;

pub fn generate_compose_file_for_angular() -> io::Result<()> {
    println!("🚀 Starting docker-compose file generation...");
    thread::sleep(SLEEP_DURATION);

    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("📂 Current directory: {:?}", current_dir);
    thread::sleep(SLEEP_DURATION);

    // Load the configuration from `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    println!("🔍 Checking for config file at: {:?}", config_path);
    thread::sleep(SLEEP_DURATION);

    let mut config_file = File::open(&config_path)?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    println!("✅ Config file found and loaded!");
    thread::sleep(SLEEP_DURATION);

    let config: Value = serde_json::from_str(&config_content)?;

    // Extract configuration values
    let service_name = config["service_name"].as_str().unwrap_or("default_service");
    let image_name = config["image_name"].as_str().unwrap_or("default_image");
    let container_name = config["container_name"]
        .as_str()
        .unwrap_or("default_container");
    let port = config["port"].as_u64().unwrap_or(5000);

    println!("⚙️  Extracted config values: service_name = {}, image_name = {}, container_name = {}, port = {}",
        service_name, image_name, container_name, port);
    thread::sleep(SLEEP_DURATION);

    // Base template without the healthcheck block
    let template = format!(
        r#"
services:
  {service_name}:
    image: "{image_name}"
    container_name: "{container_name}"
    restart: unless-stopped
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - '{port}:80'
    volumes:
      - ./node_modules:/app/node_modules
    environment:
      NODE_ENV: production
"#,
        service_name = service_name,
        image_name = image_name,
        container_name = container_name,
        port = port,
    );

    // Determine the output directory based on build mode
    #[cfg(debug_assertions)]
    let output_dir = current_dir.join("testing_directory").join("dotnet-testing");

    #[cfg(not(debug_assertions))]
    let output_dir = current_dir.clone();

    println!("📁 Ensuring output directory exists: {:?}", output_dir);
    thread::sleep(SLEEP_DURATION);
    fs::create_dir_all(&output_dir)?;

    // Delete any previous docker-compose.yaml file
    let file_paths = ["docker-compose.yaml", "docker-compose.yml"];

    for file_path in file_paths.iter() {
        let backup_path = format!("{}.backup", file_path);

        if Path::new(file_path).exists() {
            println!("📂 Backing up {} to {}...", file_path, backup_path);
            thread::sleep(SLEEP_DURATION);
            match fs::rename(file_path, &backup_path) {
                Ok(_) => {
                    println!("✅ Backup created successfully.");
                    thread::sleep(SLEEP_DURATION);
                }
                Err(e) => {
                    eprintln!("❌ Error creating backup: {}", e);
                    continue; // Skip deletion if backup fails
                }
            }

            // Delete the backup file (if needed)
            // Uncomment below if you want to delete the backup instead
            // match fs::remove_file(&backup_path) {
            //     Ok(_) => println!("🗑️ Backup file deleted."),
            //     Err(e) => eprintln!("❌ Error deleting backup: {}", e),
            // }
        } else {
            println!("🗂️ No previous {} file found.", file_path);
            thread::sleep(SLEEP_DURATION);
        }
    }

    let output_path = output_dir.join("compose.yaml");

    // Convert PathBuf to String for backup file
    let backup_path = output_path.with_extension("yaml.backup");

    // Check if the compose.yaml file exists and create a backup
    if output_path.exists() {
        println!("📂 Backing up {:?} to {:?}...", output_path, backup_path);
        fs::rename(&output_path, &backup_path)?;
        println!("✅ Backup created successfully.");
        thread::sleep(SLEEP_DURATION);
    }

    // Write the generated content to compose.yaml
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(template.as_bytes())?;
    thread::sleep(SLEEP_DURATION);

    println!(
        "🎉 Compose file generated successfully at {:?}",
        output_path
    );
    thread::sleep(SLEEP_DURATION);

    Ok(())
}
