mod angular_generator;
mod config;
mod constants;
mod dotnet_generator;
mod shared;

use std::process::exit;
use std::time::Duration;
use std::{env, thread};

use angular_generator::handle_angular_generation::handle_angular_generation;
use config::handle_config_generation::handle_config_generation;
use constants::SLEEP_DURATION;
use dotnet_generator::handle_dotnet_generation::handle_dotnet_generation;
use shared::check_git_status::check_git_status;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: fileforge <command> [--ignore-git]");
        eprintln!("Commands:");
        eprintln!("  init      Generate configuration");
        eprintln!("  generate  Generate the Dockerfile");
        eprintln!("  config    Print the current configuration");
        eprintln!("  version   Print the version of fileforge");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "init" => match handle_config_generation() {
            Ok(_) => {
                println!("✅ Configuration generated successfully!");
            }
            Err(e) => {
                eprintln!("Error generating configuration, {}", e);
                exit(1);
            }
        },
        "generate" => {
            // Check if the fileforge.config.json file exists at the root of the project
            let config_path = std::path::Path::new("fileforge.config.json");

            if !config_path.exists() {
                eprintln!("Error: fileforge.config.json not found. Run 'fileforge init' to generate a config.");
                exit(1);
            }

            // Check for the presence of the --ignore-git flag
            let ignore_git = args.contains(&"--ignore-git".to_string());

            println!("🔍 Checking for unstaged git files...");

            if !ignore_git {
                // Check for unstaged git files
                match check_git_status() {
                    Ok(_) => {
                        println!("✅ Git status check passed!");
                    }
                    Err(e) => {
                        eprintln!("Error: You have unstaged files. Please commit or stash your changes, {}", e);
                        exit(1);
                    }
                }
            } else {
                println!("⚠️ Skipping Git status check due to --ignore-git flag.");
            }

            let current_dir = env::current_dir().unwrap();
            println!("📂 Current directory: {:?}", current_dir);
            thread::sleep(Duration::from_secs(1));

            // Load the configuration from `fileforge.config.json`
            let config = shared::get_current_config::get_current_config(current_dir);

            // Extract configuration values
            let project_type = config["project_type"].as_str().unwrap_or("dotnet");
            println!("🔧 Project type: {}", project_type);
            match project_type {
                "dotnet" => match handle_dotnet_generation() {
                    Ok(_) => {
                        println!("✅ Everything has been generated successfully!");
                    }
                    Err(_) => {
                        eprintln!("❌ Error generating Dockerfile for .NET project.");
                        exit(1);
                    }
                },
                "angular" => {
                    // angular_generator::handle_angular_generation::handle_angular_generation();
                    match handle_angular_generation() {
                        Ok(_) => {
                            println!("✅ Everything has been generated successfully!");
                        }
                        Err(_) => {
                            eprintln!("❌ Error generating Dockerfile for Angular project.");
                            exit(1);
                        }
                    }
                }
                _ => {
                    eprintln!("Unknown project type: {}", project_type);
                    exit(1);
                }
            }
        }
        "version" => {
            println!("📦 Fileforge version: {}", env!("CARGO_PKG_VERSION"));
        }
        "config" => {
            let current_dir = env::current_dir().unwrap();
            println!("📂 Current directory: {:?}", current_dir);
            // Check if the fileforge.config.json file exists at the root of the project
            let config_path = std::path::Path::new("fileforge.config.json");

            if !config_path.exists() {
                eprintln!("Error: fileforge.config.json not found. Run 'fileforge init' to generate a config.");
                exit(1);
            }
            // Load the configuration from `fileforge.config.json`
            let config = shared::get_current_config::get_current_config(current_dir);
            thread::sleep(SLEEP_DURATION);
            // show the output in json format
            println!(
                "🔧 Config File: \n\n{}",
                serde_json::to_string_pretty(&config).unwrap()
            );
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Available commands: init, generate, config, version");
            exit(1);
        }
    }
}
