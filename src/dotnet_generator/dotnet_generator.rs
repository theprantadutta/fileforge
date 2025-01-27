use std::{io, thread, time};

use super::{generate_ansible_files, generate_compose_file::{self}, generate_dockerfile, generate_gitlab_cil_file::generate_gitlab_ci_file};

pub fn dotnet_generator() -> io::Result<()> {
    let sleep_duration = time::Duration::from_secs(2);

    println!("\n🚀 Starting .NET Generator...");

    println!("\n🔧 Generating Dockerfile...");
    thread::sleep(sleep_duration); // Simulate progress
    if let Err(e) = generate_dockerfile::generate_dockerfile() {
        eprintln!("❌ Error: Failed to generate Dockerfile. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ Dockerfile generated successfully!");

    println!("\n🔧 Generating Docker Compose file...");
    thread::sleep(sleep_duration); // Simulate progress
    if let Err(e) = generate_compose_file::generate_compose_file() {
        eprintln!("❌ Error: Failed to generate Docker Compose file. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ Docker Compose file generated successfully!");

    println!("\n🔧 Generating GitLab CI file...");
    thread::sleep(sleep_duration); // Simulate progress
    if let Err(e) = generate_gitlab_ci_file() {
        eprintln!("❌ Error: Failed to generate GitLab CI file. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ GitLab CI file generated successfully!");

    println!("\n🔧 Copying Ansible files...");
    thread::sleep(sleep_duration); // Simulate progress
    if let Err(e) = generate_ansible_files::copy_ansible_files() {
        eprintln!("❌ Error: Failed to copy Ansible files. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ Ansible files copied successfully!");

    println!("\n🎉 .NET Generator completed successfully! All required files have been generated.\n");

    Ok(())
}