use std::{env, thread};
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::Path;
use serde_json::Value;
use std::time::Duration;

pub fn generate_compose_file() -> io::Result<()> {
    println!("🚀 Starting docker-compose file generation...");
    thread::sleep(Duration::from_secs(1));

    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("📂 Current directory: {:?}", current_dir);
    thread::sleep(Duration::from_secs(1));

    // Load the configuration from `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    println!("🔍 Checking for config file at: {:?}", config_path);
    thread::sleep(Duration::from_secs(1));

    let mut config_file = File::open(&config_path)?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    println!("✅ Config file found and loaded!");
    thread::sleep(Duration::from_secs(1));

    let config: Value = serde_json::from_str(&config_content)?;

    // Extract configuration values
    let service_name = config["service_name"].as_str().unwrap_or("default_service");
    let image_name = config["image_name"].as_str().unwrap_or("default_image");
    let container_name = config["container_name"].as_str().unwrap_or("default_container");
    let port = config["port"].as_u64().unwrap_or(5000);
    let enable_healthcheck = config["enable_healthcheck"].as_bool().unwrap_or(false);

    println!("⚙️  Extracted config values: service_name = {}, image_name = {}, container_name = {}, port = {}, enable_healthcheck = {}",
        service_name, image_name, container_name, port, enable_healthcheck);
    thread::sleep(Duration::from_secs(1));

    // Base template without the healthcheck block
    let mut template = format!(
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
      - '{port}:5000'
"#,
        service_name = service_name,
        image_name = image_name,
        container_name = container_name,
        port = port,
    );

    // Append the healthcheck block if enabled
    if enable_healthcheck {
        println!("🩺 Adding healthcheck block...");
        thread::sleep(Duration::from_secs(1));
        let healthcheck_block = r#"
    healthcheck:
      test: curl --fail http://localhost:5000/health || exit 1
      interval: 40s
      timeout: 30s
      retries: 3
      start_period: 60s
  "#;
        template.push_str(healthcheck_block);
    }

    // Append the network configuration
    template.push_str(
        r#"
    networks:
      - api-network

networks:
  api-network:
    external: true
"#,
    );
    println!("✅ Network configuration added.");
    thread::sleep(Duration::from_secs(1));

    // Determine the output directory based on build mode
    #[cfg(debug_assertions)]
    let output_dir = current_dir.join("testing_directory").join("dotnet-testing");

    #[cfg(not(debug_assertions))]
    let output_dir = current_dir.clone();

    println!("📁 Ensuring output directory exists: {:?}", output_dir);
    thread::sleep(Duration::from_secs(1));
    fs::create_dir_all(&output_dir)?;

    let output_path = output_dir.join("compose.yaml");

    // Delete any previous docker-compose.yaml file
    let file_path = "docker-compose.yaml";
    if Path::new(file_path).exists() {
        println!("🗑️  Deleting previous docker-compose.yaml file...");
        thread::sleep(Duration::from_secs(1));
        match fs::remove_file(file_path) {
            Ok(_) => {
              println!("✅ Previous file deleted successfully.");
              thread::sleep(Duration::from_secs(1));
            },
            Err(e) => eprintln!("❌ Error deleting file: {}", e),
        }
    } else {
        println!("🗂️  No previous docker-compose.yaml file found.");
        thread::sleep(Duration::from_secs(1));
    }

    // Write the generated content to `compose.yaml`
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(template.as_bytes())?;
    println!("🎉 Compose file generated successfully at {:?}", output_path);
    thread::sleep(Duration::from_secs(1));

    Ok(())
}
