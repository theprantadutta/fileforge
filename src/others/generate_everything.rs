use std::{env, io, process::exit, thread};

use crate::{
    angular_generator::handle_angular_generation::handle_angular_generation,
    constants::SLEEP_DURATION,
    dotnet_generator::handle_dotnet_generation::handle_dotnet_generation,
    shared::{self, check_git_status::check_git_status},
};

pub fn generate_everything(args: Vec<String>) -> io::Result<()> {
    // Check if the `fileforge.config.json` file exists at the root of the project
    let config_path = std::path::Path::new("fileforge.config.json");

    if !config_path.exists() {
        eprintln!(
            "❌ Error: `fileforge.config.json` not found. Run 'fileforge init' to generate a config."
        );
        exit(1); // Exit with an error code if the config file is missing
    }

    // Check for the presence of the `--ignore-git` flag in the arguments
    let ignore_git = args.contains(&"--ignore-git".to_string());

    println!("🔍 Checking for unstaged git files...");

    if !ignore_git {
        // Check for unstaged git files if the `--ignore-git` flag is not present
        match check_git_status() {
            Ok(_) => {
                println!("✅ Git status check passed!"); // Success message
            }
            Err(e) => {
                eprintln!(
                    "❌ Error: You have unstaged files. Please commit or stash your changes: {}",
                    e
                );
                exit(1); // Exit with an error code if there are unstaged files
            }
        }
    } else {
        println!("⚠️ Skipping Git status check due to `--ignore-git` flag."); // Informational message
    }

    // Get the current working directory
    let current_dir = env::current_dir().unwrap_or_else(|e| {
        eprintln!("❌ Error getting current directory: {}", e);
        exit(1); // Exit with an error code if the current directory cannot be retrieved
    });
    println!("📂 Current directory: {:?}", current_dir);

    // Introduce a small delay for better user experience (optional)
    thread::sleep(SLEEP_DURATION);

    // Load the configuration from `fileforge.config.json`
    let config = shared::get_current_config::get_current_config(current_dir);

    // Extract the project type from the configuration (default to "dotnet" if not specified)
    let project_type = config["project_type"].as_str().unwrap_or("dotnet");
    println!("🔧 Project type: {}", project_type);

    // Handle generation based on the project type
    match project_type {
        "dotnet" => match handle_dotnet_generation() {
            Ok(_) => {
                println!("🎉 Dockerfile and other files for .NET project generated successfully!");
            }
            Err(_) => {
                eprintln!("❌ Error generating Dockerfile for .NET project.");
                exit(1); // Exit with an error code if generation fails
            }
        },
        "angular" => match handle_angular_generation() {
            Ok(_) => {
                println!(
                    "🎉 Dockerfile and other files for Angular project generated successfully!"
                );
            }
            Err(_) => {
                eprintln!("❌ Error generating Dockerfile for Angular project.");
                exit(1); // Exit with an error code if generation fails
            }
        },
        _ => {
            eprintln!("❌ Unknown project type: {}", project_type);
            exit(1); // Exit with an error code for unsupported project types
        }
    }

    Ok(()) // Return `Ok` if everything succeeds
}
