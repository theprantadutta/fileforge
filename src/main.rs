mod angular_generator;
mod config;
mod constants;
mod dotnet_generator;
mod others;
mod shared;

use std::env;
use std::process::exit;

use config::handle_config_generation::handle_config_generation;
use others::generate_everything::generate_everything;
use others::print_usage::print_usage;
use others::show_config::show_config;

fn main() {
    // Get the command-line arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Check if the user provided at least one command
    if args.len() < 2 {
        print_usage(); // Display usage instructions
        std::process::exit(1); // Exit with an error code
    }

    // Match the command provided by the user
    match args[1].as_str() {
        // Handle the "init" command
        "init" => match handle_config_generation() {
            Ok(_) => {
                println!("🎉 Configuration generated successfully!"); // Success message
            }
            Err(e) => {
                eprintln!("❌ Error generating configuration: {}", e); // Error message
                exit(1); // Exit with an error code
            }
        },

        // Handle the "generate" command
        "generate" => match generate_everything(args.clone()) {
            Ok(_) => {
                println!("🚀 All files have been generated successfully!"); // Success message
            }
            Err(e) => {
                eprintln!("❌ Error generating files: {}", e); // Error message
                exit(1); // Exit with an error code
            }
        },

        // Handle the "version" command
        "version" => {
            println!("📦 Fileforge version: {}", env!("CARGO_PKG_VERSION")); // Display version
        }

        // Handle the "config" command
        "config" => match show_config() {
            Ok(_) => {
                println!("🔧 Configuration displayed successfully!"); // Success message
            }
            Err(e) => {
                eprintln!("❌ Error displaying configuration: {}", e); // Error message
                exit(1); // Exit with an error code
            }
        },

        // Handle unknown commands
        _ => {
            eprintln!("❌ Unknown command: {}", args[1]); // Error message for unknown command
            print_usage(); // Display usage instructions
            exit(1); // Exit with an error code
        }
    }
}
