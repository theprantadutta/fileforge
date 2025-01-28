use std::{io, thread};

use crate::{
    angular_generator::{
        generate_ansible_files_for_angular::generate_ansible_files_for_angular,
        generate_compose_file_for_angular::generate_compose_file_for_angular,
        generate_dockerfile_for_angular::generate_dockerfile_for_angular,
        generate_gitlab_ci_file_for_angular::generate_gitlab_ci_file_for_angular,
        generate_nginx_file_for_angular::generate_nginx_file_angular,
    },
    constants::SLEEP_DURATION,
};

pub fn handle_angular_generation() -> io::Result<()> {
    println!("\n🚀 Starting Angular Generator...");

    println!("\n🔧 Generating Dockerfile...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_dockerfile_for_angular() {
        eprintln!("❌ Error: Failed to generate Dockerfile. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ Dockerfile generated successfully!");

    println!("\n🔧 Generating Docker Compose file...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_compose_file_for_angular() {
        eprintln!(
            "❌ Error: Failed to generate Docker Compose file. Details: {}",
            e
        );
        std::process::exit(1);
    }
    println!("✅ Docker Compose file generated successfully!");

    println!("\n🔧 Generating GitLab CI file...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_gitlab_ci_file_for_angular() {
        eprintln!(
            "❌ Error: Failed to generate GitLab CI file. Details: {}",
            e
        );
        std::process::exit(1);
    }
    println!("✅ GitLab CI file generated successfully!");

    println!("\n🔧 Copying Ansible files...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_ansible_files_for_angular() {
        eprintln!("❌ Error: Failed to copy Ansible files. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ Ansible files copied successfully!");

    println!("\n🔧 Copying Nginx file...");
    thread::sleep(SLEEP_DURATION); // Simulate progress
    if let Err(e) = generate_nginx_file_angular() {
        eprintln!("❌ Error: Failed to copy Nginx file. Details: {}", e);
        std::process::exit(1);
    }
    println!("✅ Nginx file copied successfully!");

    println!(
        "\n🎉 Angular Generator completed successfully! All required files have been generated.\n"
    );

    Ok(())
}
