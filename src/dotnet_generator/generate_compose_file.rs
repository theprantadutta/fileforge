use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

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

  // Base template without the healthcheck block
  let mut template = format!(
      r#"
version: '3.9'
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

  // Step 3: Append the healthcheck block if enabled
  if enable_healthcheck {
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

  // Step 4: Determine the output directory based on build mode
  #[cfg(debug_assertions)]
  let output_dir = current_dir.join("testing_directory").join("dotnet-testing");

  #[cfg(not(debug_assertions))]
  let output_dir = current_dir.clone();

  fs::create_dir_all(&output_dir)?;
  let output_path = output_dir.join("compose.yaml");

  // Delete if we have any previous docker-compose.yaml file
  let file_path = "docker-compose.yaml";

  if Path::new(file_path).exists() {
      match fs::remove_file(file_path) {
          Ok(_) => println!("Previous docker-compose.yaml file deleted successfully."),
          Err(e) => eprintln!("Error deleting docker-compose.yaml: {}", e),
      }
  } else {
      println!("No previous docker-compose.yaml file found.");
  }

  // Step 5: Write the generated content to `compose.yaml`
  let mut output_file = File::create(&output_path)?;
  output_file.write_all(template.as_bytes())?;

  println!("Compose file written successfully to {:?}", output_path);
  Ok(())
}
