use std::io;

use super::{generate_ansible_files, generate_compose_file::{self}, generate_dockerfile, generate_gitlab_cil_file::generate_gitlab_ci_file};

pub fn dotnet_generator()-> io::Result<()> {
    if let Err(e) = generate_dockerfile::generate_dockerfile() {
        eprintln!("Error generating Dockerfile: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = generate_compose_file::generate_compose_file() {
        eprintln!("Error generating Compose File: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = generate_gitlab_ci_file() {
        eprintln!("Error generating Gitlab CI File: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = generate_ansible_files::copy_ansible_files() {
        eprintln!("Error generating Ansible Files: {}", e);
        std::process::exit(1);
    }
    return Ok(());
}