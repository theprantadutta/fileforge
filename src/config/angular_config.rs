use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Write},
};

use crate::shared::{
    directory_helper,
    shared_input_helper::{
        get_input_from_user, get_input_from_user_with_default, get_port_from_user,
    },
};

#[derive(Serialize, Deserialize)]
pub struct AngularConfig {
    pub project_type: String,
    pub node_version: String,
    pub service_name: String,
    pub image_name: String,
    pub container_name: String,
    pub port: u16,
    pub project_location: String,
    pub project_directory: String,
}

pub fn create_or_update_config() -> io::Result<()> {
    // Get the root directory name
    let current_dir = directory_helper::get_current_directory().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        String::new()
    });
    println!("📂 Root Directory: {:?}", current_dir);

    let node_version = get_input_from_user("Node Version: ");
    let service_name = get_input_from_user_with_default("Service name: ", &current_dir);
    let image_name = get_input_from_user_with_default("Image name: ", &current_dir);
    let container_name = get_input_from_user_with_default("Container name: ", &current_dir);

    let port = get_port_from_user();
    let project_location = get_input_from_user_with_default(
        "Project location (default: /etc/www, don't include the trailing slash): ",
        "/etc/www",
    );
    let project_directory = get_input_from_user_with_default("Project directory: ", &current_dir);

    // Save to config file
    let config = AngularConfig {
        project_type: "angular".to_string(),
        node_version,
        service_name,
        image_name,
        container_name,
        port,
        project_location,
        project_directory,
    };

    let current_dir = std::env::current_dir()?;
    let config_path = current_dir.join("fileforge.config.json");
    let mut config_file = File::create(config_path)?;
    let config_json = serde_json::to_string_pretty(&config)?;
    config_file.write_all(config_json.as_bytes())?;

    println!("Angular Configuration saved to fileforge.config.json");

    Ok(())
}
