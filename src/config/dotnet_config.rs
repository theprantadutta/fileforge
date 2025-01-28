use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};

use crate::shared::shared_input_helper::{
    get_input_from_user, get_input_from_user_with_default, get_port_from_user,
};

#[derive(Serialize, Deserialize)]
pub struct DotnetConfig {
    pub project_type: String,
    pub root_namespace: String,
    pub dotnet_version: String,
    pub service_name: String,
    pub image_name: String,
    pub container_name: String,
    pub port: u16,
    pub enable_healthcheck: bool,
    pub project_location: String,
    pub project_directory: String,
}

pub fn create_or_update_config() -> io::Result<()> {
    // Check for .csproj file
    let current_dir = std::env::current_dir()?;
    let csproj_path = fs::read_dir(&current_dir)?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.path().extension() == Some("csproj".as_ref()));

    let root_namespace = if let Some(ref csproj_entry) = csproj_path {
        // Borrowing instead of moving
        println!("Found .csproj file: {:?}", csproj_entry.path());
        let mut csproj_content = String::new();
        File::open(csproj_entry.path())?.read_to_string(&mut csproj_content)?;

        // Get root namespace
        if let Some(start) = csproj_content.find("<RootNamespace>") {
            if let Some(end) = csproj_content[start..].find("</RootNamespace>") {
                // Adjust the start index to skip over the '<RootNamespace>' and the '>' character
                let namespace = &csproj_content[start + 15..start + end]; // 15 accounts for '<RootNamespace>' length + '>'
                println!("Root namespace found: {}", namespace.trim());
                namespace.trim().to_string()
            } else {
                println!(
                    "No <RootNamespace> closing tag found. Please provide the root namespace:"
                );
                get_input_from_user("Root namespace: ")
            }
        } else {
            println!("No <RootNamespace> tag found. Please provide the root namespace:");
            get_input_from_user("Root namespace: ")
        }
    } else {
        println!("No .csproj file found. Please provide the root namespace:");
        get_input_from_user("Root namespace: ")
    };

    // Get .NET version
    let dotnet_version = if let Some(ref csproj_entry) = csproj_path {
        // Borrowing instead of moving
        let csproj_content = fs::read_to_string(csproj_entry.path())?;
        if let Some(start) = csproj_content.find("<TargetFramework>net") {
            if let Some(end) = csproj_content[start..].find("</TargetFramework>") {
                let version = &csproj_content[start + 17..start + end];
                println!("Found .NET version: {}", version);
                version.trim().to_string()
            } else {
                println!(
                    "No <TargetFramework> closing tag found. Please provide the .NET version:"
                );
                get_input_from_user("Dotnet version (e.g., net9.0): ")
            }
        } else {
            println!("No <TargetFramework> tag found. Please provide the .NET version:");
            get_input_from_user("Dotnet version (e.g., net9.0): ")
        }
    } else {
        println!("No .csproj file found. Please provide the .NET version:");
        get_input_from_user("Dotnet version (e.g., net9.0): ")
    };

    let hyphened_root_namespace = root_namespace.replace("_", "-").to_lowercase();

    // Ask for other configurations
    let service_name = get_input_from_user_with_default("Service name: ", &hyphened_root_namespace);
    let image_name = get_input_from_user_with_default("Image name: ", &hyphened_root_namespace);
    let container_name =
        get_input_from_user_with_default("Container name: ", &hyphened_root_namespace);

    let port = get_port_from_user();
    let enable_healthcheck = get_healthcheck_from_user();
    let project_location = get_input_from_user_with_default(
        "Project location (default: /etc/www, don't include the trailing slash): ",
        "/etc/www",
    );
    let project_directory =
        get_input_from_user_with_default("Project directory: ", &hyphened_root_namespace);

    // Save to config file
    let config = DotnetConfig {
        project_type: "dotnet".to_string(),
        root_namespace,
        dotnet_version,
        service_name,
        image_name,
        container_name,
        port,
        enable_healthcheck,
        project_location,
        project_directory,
    };

    let config_path = current_dir.join("fileforge.config.json");
    let mut config_file = File::create(config_path)?;
    let config_json = serde_json::to_string_pretty(&config)?;
    config_file.write_all(config_json.as_bytes())?;

    println!("DotnetConfiguration saved to fileforge.config.json");

    Ok(())
}

fn get_healthcheck_from_user() -> bool {
    loop {
        let healthcheck: String =
            get_input_from_user("Enable healthcheck (yes/no), defaults to no: ");

        // Set default to "no" if the input is empty
        let healthcheck = if healthcheck.trim().is_empty() {
            println!("Using default value: no");
            "no".to_string()
        } else {
            println!("Using provided value: {}", healthcheck);
            healthcheck
        };

        if healthcheck.to_lowercase() == "yes" {
            return true;
        } else if healthcheck.to_lowercase() == "no" {
            return false;
        } else {
            println!("Invalid input. Please enter 'yes' or 'no'.");
        }
    }
}
