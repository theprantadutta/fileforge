use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use regex::Regex;

pub fn generate_compose_file() -> io::Result<()> {
    // Step 1: Get the current directory
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    // Step 2: Load the configuration from `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    let mut config_file = File::open(&config_path)?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    let config: serde_json::Value = serde_json::from_str(&config_content)?;

    // Extract configuration values
    let service_name = config["service_name"].as_str().unwrap_or("default_service");
    let image_name = config["image_name"].as_str().unwrap_or("default_image");
    let container_name = config["container_name"].as_str().unwrap_or("default_container");
    let port = config["port"].as_u64().unwrap_or(5000);
    let enable_healthcheck = config["enable_healthcheck"].as_bool().unwrap_or(false);

    // Step 3: Load the template
    let template = r#"
version: '3.9'
services:
  {{ service_name }}:
    image: "{{ image_name }}"
    container_name: "{{ container_name }}"
    restart: unless-stopped
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - '{{ port }}:5000'
    healthcheck:
      test: curl --fail http://localhost:5000/health || exit 1
      interval: 40s
      timeout: 30s
      retries: 3
      start_period: 60s
"#;

    // Step 4: Replace placeholders in the template
    let mut updated_compose = template
        .replace("{{ service_name }}", service_name)
        .replace("{{ image_name }}", image_name)
        .replace("{{ container_name }}", container_name)
        .replace("{{ port }}", &port.to_string());

    // Step 5: Remove healthcheck block if `enable_healthcheck` is false
    if !enable_healthcheck {
        let healthcheck_pattern = r"(?m)^\s*healthcheck:\n(?:\s+.*\n?)*";
        let re = Regex::new(healthcheck_pattern).unwrap();
        updated_compose = re.replace_all(&updated_compose, "").to_string();
    }

    // Step 6: Determine the output directory based on build mode
    #[cfg(debug_assertions)]
    let output_dir = current_dir.join("testing_directory").join("dotnet-testing");

    #[cfg(not(debug_assertions))]
    let output_dir = current_dir.clone();

    fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("compose.yaml");

    // Step 7: Write the generated content to `compose.yaml`
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(updated_compose.as_bytes())?;

    println!("Compose file written successfully to {:?}", output_path);
    Ok(())
}
