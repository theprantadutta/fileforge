mod dotnet_generator;
mod config;
mod shared;

use std::fs::File;
use std::io::Read;
use std::{env, fs};
use std::process::exit;

use dotnet_generator::handle_dotnet_generation::{self};

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
            // Check if the project is a .NET project or an Angular project
            let is_dotnet_project = fs::read_dir(".")
                .map(|mut entries| entries.any(|entry| entry
                    .map(|e| e.path().extension().map_or(false, |ext| ext == "csproj" || ext == "sln"))
                    .unwrap_or(false)))
                .unwrap_or(false);
            
            let is_angular_project = fs::read_dir(".")
                .map(|mut entries| entries.any(|entry| entry
                    .map(|e| e.path().file_name().map_or(false, |name| name == "package.json"))
                    .unwrap_or(false)))
                .unwrap_or(false);
           
            if is_angular_project {
                println!("Detected Angular project. Checking for @angular/core...");
    
                let package_json_path = "package.json";
                let mut package_json = String::new();
                
                if let Ok(mut file) = File::open(package_json_path) {
                    if file.read_to_string(&mut package_json).is_ok() {
                        if package_json.contains("@angular/core") {
                            println!("@angular/core found in package.json. Generating Angular configuration...");
                            if let Err(e) = config::angular_config::create_or_update_config() {
                                eprintln!("Error generating Angular configuration: {}", e);
                                exit(1);
                            }
                        } else {
                            eprintln!("@angular/core not found in package.json. This does not appear to be an Angular project.");
                            eprintln!("Right now, we only support Angular and Dotnet Core projects.");
                            exit(0);
                        }
                    } else {
                        eprintln!("Error reading package.json.");
                        exit(1);
                    }
                } else {
                    eprintln!("package.json not found.");
                    exit(1);
                }
            } else if is_dotnet_project {
                println!("Detected .NET project. Generating .NET configuration...");
                if let Err(e) = config::dotnet_config::create_or_update_config() {
                    eprintln!("Error generating .NET configuration: {}", e);
                    exit(1);
                }
            } else {
                eprintln!("Neither .NET nor Angular project detected. Cannot generate configuration.");
                eprintln!("Right now, we only support Angular and Dotnet Core projects.");
                exit(0);
            }

            // if let Err(e) = config::dotnet_config::create_or_update_config() {
            //     eprintln!("Error generating configuration: {}", e);
            //     std::process::exit(1);
            // }
        }
        "generate" => {
            handle_dotnet_generation::handle_dotnet_generation();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Available commands: init, generate");
            exit(1);
        }
    }
}
