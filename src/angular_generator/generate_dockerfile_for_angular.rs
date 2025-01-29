use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::{env, thread};

use crate::constants::SLEEP_DURATION;

pub fn generate_dockerfile_for_angular() -> io::Result<()> {
    println!("🚀 Starting Dockerfile generation...");
    thread::sleep(SLEEP_DURATION);

    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("📂 Current directory: {:?}", current_dir);
    thread::sleep(SLEEP_DURATION);

    // Check for `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    println!("🔍 Checking for config file at: {:?}", config_path);
    thread::sleep(SLEEP_DURATION);

    if !config_path.exists() {
        println!("❌ Config file not found!");
        thread::sleep(SLEEP_DURATION);
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "fileforge.config.json not found.",
        ));
    }

    let mut config_content = String::new();
    File::open(&config_path)?.read_to_string(&mut config_content)?;
    println!("✅ Config file found and loaded!");
    thread::sleep(SLEEP_DURATION);

    // Parse the JSON file
    let config: Value = serde_json::from_str(&config_content)?;
    let node_version = config["node_version"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing 'node_version'"))?
        .to_string();
    println!("📂 Found Node Version: {}", node_version);
    thread::sleep(SLEEP_DURATION);

    // Prepare Dockerfile template
    let docker_template = r#"
### STAGE 1: Build ###
# Use an official Node runtime as a parent image
FROM node:{{ node_version }}-alpine AS build

# Set the working directory to /app
WORKDIR /app

# Copy package.json and package-lock.json to the container
COPY package*.json ./

# Update the NPM
# RUN npm i -g npm

# Install dependencies
RUN npm install --legacy-peer-deps

# Copy the rest of the application code to the container
COPY . .

# Build the app
RUN npm run build:prod

### STAGE 2: Run ###
# Use an official Nginx image
FROM nginx:1.24.0-alpine

# Use Nginx Default configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copying New angular build files to nginx default folder
COPY --from=build /app/dist/angular-app/browser /usr/share/nginx/html
"#;

    let updated_dockerfile = docker_template.replace("{{ node_version }}", "20.10.0");
    println!("✅ Dockerfile template updated with project directory.");
    thread::sleep(SLEEP_DURATION);

    // Write Dockerfile to output directory
    #[cfg(debug_assertions)]
    let output_dir = current_dir.join("testing_directory").join("dotnet-testing");
    #[cfg(not(debug_assertions))]
    let output_dir = current_dir.clone();

    println!("📁 Ensuring output directory exists: {:?}", output_dir);
    fs::create_dir_all(&output_dir)?;
    thread::sleep(SLEEP_DURATION);

    // Create a backup file path
    let output_path = output_dir.join("Dockerfile");
    let backup_path = output_dir.join("Dockerfile.backup");

    // Check if Dockerfile exists and create a backup
    if output_path.exists() {
        println!("📂 Backing up {:?} to {:?}...", output_path, backup_path);
        fs::rename(&output_path, &backup_path)?;
        println!("✅ Backup created successfully.");
    }

    // Write the updated Dockerfile
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(updated_dockerfile.as_bytes())?;

    println!("✅ Dockerfile written to: {:?}", output_path);
    thread::sleep(SLEEP_DURATION);

    println!("🎉 Dockerfile generation completed successfully!");
    thread::sleep(SLEEP_DURATION);
    Ok(())
}
