use std::{io, thread};

use crate::constants::SLEEP_DURATION;
use crate::dotnet_generator::generate_ansible_files_for_dotnet::generate_ansible_files_for_dotnet;
use crate::dotnet_generator::generate_compose_file_for_dotnet::generate_compose_file_for_dotnet;
use crate::dotnet_generator::generate_dockerfile_for_dotnet::generate_dockerfile_for_dotnet;
use crate::dotnet_generator::generate_gitlab_cil_file_for_dotnet::generate_gitlab_cil_file_for_dotnet;

pub fn handle_dotnet_generation() -> io::Result<()> {
    println!("\n🚀 Starting .NET Generator...");

    println!("\n🔧 Generating Dockerfile...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_dockerfile_for_dotnet() {
        eprintln!("❌ Error: Failed to generate Dockerfile. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ Dockerfile generated successfully!");

    println!("\n🔧 Generating Docker Compose file...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_compose_file_for_dotnet() {
        eprintln!(
            "❌ Error: Failed to generate Docker Compose file. Details: {}",
            e
        );
        std::process::exit(1);
    }
    println!("✅ Docker Compose file generated successfully!");

    println!("\n🔧 Generating GitLab CI file...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_gitlab_cil_file_for_dotnet() {
        eprintln!(
            "❌ Error: Failed to generate GitLab CI file. Details: {}",
            e
        );
        std::process::exit(1);
    }
    println!("✅ GitLab CI file generated successfully!");

    println!("\n🔧 Copying Ansible files...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_ansible_files_for_dotnet() {
        eprintln!("❌ Error: Failed to copy Ansible files. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ Ansible files copied successfully!");

    println!(
        "\n🎉 .NET Generator completed successfully! All required files have been generated.\n"
    );

    Ok(())
}
