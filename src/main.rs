mod dotnet_generator;
mod config;

use std::env;

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
            if let Err(e) =  dotnet_generator::dotnet_generator::dotnet_generator() {
                eprintln!("Error generating dotnet generator: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Available commands: init, generate");
            std::process::exit(1);
        }
    }
}
