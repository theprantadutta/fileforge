mod dotnet_generator;
mod config;

use std::env;
use std::process::{Command, exit};

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: fileforge <command>");
        eprintln!("Commands:");
        eprintln!("  init      Generate configuration");
        eprintln!("  generate  Generate the Dockerfile");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "init" => {
            if let Err(e) = config::config::create_or_update_config() {
                eprintln!("Error generating configuration: {}", e);
                std::process::exit(1);
            }
        }
        "generate" => {
            // Check if the fileforge.config.json file exists at the root of the project
            let config_path = std::path::Path::new("fileforge.config.json");

            if !config_path.exists() {
                eprintln!("Error: fileforge.config.json not found. Run 'fileforge init' to generate a config.");
                exit(1);
            }

            // Check for unstaged git files
            let git_status_output = Command::new("git")
                .arg("status")
                .arg("--porcelain")
                .output()
                .expect("Failed to execute git status");

            let git_status = String::from_utf8_lossy(&git_status_output.stdout);

            // If there are any unstaged files, but not just fileforge.config.json
            let mut unstaged_files = false;
            let mut is_only_config_unstaged = true;

            for line in git_status.lines() {
                let status = line.split_whitespace().collect::<Vec<&str>>();

                if status.len() > 1 {
                    if status[1] != "fileforge.config.json" {
                        unstaged_files = true;
                    } else {
                        is_only_config_unstaged = false;
                    }
                }
            }
            
            println!("Unstaged files: {}", unstaged_files);
            println!("Is only config unstaged: {}", is_only_config_unstaged);

            if unstaged_files || !is_only_config_unstaged {
                eprintln!("Error: You have unstaged files. Please commit or stash your changes.");
                exit(1);
            }

            // Continue with the generation process
            if let Err(e) = dotnet_generator::dotnet_generator::dotnet_generator() {
                eprintln!("Error generating dotnet generator: {}", e);
                exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Available commands: init, generate");
            exit(1);
        }
    }
}
