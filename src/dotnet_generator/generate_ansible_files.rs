use std::{thread, time::Duration};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;

pub fn copy_ansible_files() -> io::Result<()> {
    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("🌟 Starting Ansible File Copy Process...");
    thread::sleep(Duration::from_secs(1));

    println!("📂 Current directory: {:?}", current_dir);

    // Determine the output directory based on debug or release mode
    #[cfg(debug_assertions)]
    let output_dir = current_dir.join("testing_directory").join("dotnet-testing"); // Debug mode path

    #[cfg(not(debug_assertions))]
    let output_dir = current_dir.clone(); // Release mode path

    // Ensure the directory exists
    println!("🛠️ Ensuring directory exists: {:?}", output_dir);
    thread::sleep(Duration::from_secs(1));
    fs::create_dir_all(&output_dir)?;

    // Step 1: Check for `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    println!("🔍 Looking for config file at: {:?}", config_path);

    if !config_path.exists() {
        println!("❌ Error: Config file not found at {:?}", config_path);
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "fileforge.config.json not found.",
        ));
    }

    println!("✅ Config file found. Reading contents...");
    thread::sleep(Duration::from_secs(1));
    let mut config_content = String::new();
    File::open(&config_path)?.read_to_string(&mut config_content)?;

    // Step 2: Parse the JSON file and get `project_location` and `project_directory`
    println!("🔄 Parsing config file...");
    thread::sleep(Duration::from_secs(1));
    let config: Value = serde_json::from_str(&config_content)?;

    let project_location = config["project_location"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing project_location in config"))?
        .to_string();

    let project_directory = config["project_directory"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing project_directory in config"))?
        .to_string();

    println!("📁 Project location: {}", project_location);
    println!("📂 Project directory: {}", project_directory);

    // Step 3: Create an `ansible` directory inside the output directory
    let ansible_dir = output_dir.join("ansible");
    println!("🛠️ Creating Ansible directory at: {:?}", ansible_dir);
    thread::sleep(Duration::from_secs(1));
    fs::create_dir_all(&ansible_dir)?;

    // Step 4: Copy `hosts.yml` to the `ansible` directory
    println!("📋 Creating `hosts.yml` file...");
    thread::sleep(Duration::from_secs(1));
    let hosts_content = r#"target_servers:
  hosts:
    {{ ansible_host_name_placeholder }}:
      ansible_host: "{{ ansible_host_placeholder }}"
      ansible_user: "{{ ansible_user_placeholder }}"
      ansible_ssh_port: {{ ansible_ssh_port_placeholder }}
      ansible_become_user: root
      ansible_become_pass: "{{ ansible_become_pass_placeholder }}"
      ansible_ssh_common_args: '-o StrictHostKeyChecking=no'
"#;

    let hosts_file_path = ansible_dir.join("hosts.yml");
    let mut hosts_file = File::create(&hosts_file_path)?;
    hosts_file.write_all(hosts_content.as_bytes())?;
    println!("✅ `hosts.yml` file created at {:?}", hosts_file_path);

    // Step 5: Copy `ansible-deploy.yml` to the `ansible` directory, replacing placeholders
    println!("📋 Creating `ansible-deploy.yml` file...");
    thread::sleep(Duration::from_secs(1));
        // Step 5: Copy `ansible-deploy.yml` to the `ansible` directory, replacing placeholders
let ansible_deploy_content = r#"---
- name: Deploy files and start docker compose
  hosts: target_servers
  become: yes
  vars:
    project_directory: "[[ project_directory ]]"
    project_location: "[[ project_location ]]" # This is for both local and remote server
  tasks:
    - name: Remove the existing remote directory
      ansible.builtin.file:
        path: "{{ project_location }}/{{ project_directory }}"
        state: absent
    
    - name: Ensure the destination directory exists on the remote server
      ansible.builtin.file:
        path: "{{ project_location }}/{{ project_directory }}"
        state: directory
        mode: '0755'

    - name: Copy files to the remote server
      ansible.builtin.copy:
        src: "{{ project_location }}/{{ project_directory }}"
        dest: "{{ project_location }}"
        mode: '0755'
        
    - name: Run Sonatype Nexus Docker Login
      ansible.builtin.shell:
        cmd: |
          echo "Logging into Microsoft Docker Registry (Nexus)..."
          echo "Logging to url: {{ lookup('env', 'SONATYPE_NEXUS_URL') }} with username: {{ lookup('env', 'SONATYPE_NEXUS_USERNAME') }}"
          if echo "{{ lookup('env', 'SONATYPE_NEXUS_PASSWORD') }}" | docker login {{ lookup('env', 'SONATYPE_NEXUS_URL') }} -u {{ lookup('env', 'SONATYPE_NEXUS_USERNAME') }} --password-stdin; then
            echo "Successfully Logged into Microsoft Docker Registry";
          else
            echo "Failed to Login to Microsoft Docker Registry";
            exit 1;
          fi

          set -e  # Exit immediately if a command exits with a non-zero status
          echo "Ensuring Required Docker Images Exist..."
          docker pull {{ lookup('env', 'SONATYPE_NEXUS_URL') }}/dotnet/aspnet:9.0 || { echo "Failed to pull aspnet:9.0 image"; exit 1; }
          docker pull {{ lookup('env', 'SONATYPE_NEXUS_URL') }}/dotnet/sdk:9.0 || { echo "Failed to pull sdk:9.0 image"; exit 1; }
          echo "Docker Images Pulled Successfully"
      register: docker_login_result
    
    - name: Display docker-login result
      debug:
        var: docker_login_result.stdout

    - name: Run docker compose to start the services
      ansible.builtin.shell:
        cmd: |
          echo "Current Directory:"
          pwd
          export SONATYPE_NEXUS_URL=$SONATYPE_NEXUS_URL
          # Run docker compose up and retry once if it fails
          if ! docker compose up --build -d --remove-orphans; then
            echo "docker compose failed. Retrying with cache clearing..."
            echo "Stopping and removing existing containers..."
            docker compose down || echo "Failed to stop containers, continuing..."
            echo "Rebuilding containers without cache..."
            docker compose build --no-cache
            if ! docker compose up -d --remove-orphans; then
              echo "Retry failed. Exiting..."
              exit 1
            fi
          fi
        chdir: "{{ project_location }}/{{ project_directory }}"
      register: docker_compose_result
      environment:
        SONATYPE_NEXUS_URL: "{{ lookup('env', 'SONATYPE_NEXUS_URL') }}"
      ignore_errors: false
    
    - name: Display docker compose result
      debug:
        var: docker_compose_result.stdout
"#;

    let updated_ansible_deploy_content = ansible_deploy_content
        .replace("[[ project_location ]]", &project_location)
        .replace("[[ project_directory ]]", &project_directory);

    let ansible_deploy_file_path = ansible_dir.join("ansible-deploy.yml");
    let mut ansible_deploy_file = File::create(&ansible_deploy_file_path)?;
    ansible_deploy_file.write_all(updated_ansible_deploy_content.as_bytes())?;
    println!("✅ `ansible-deploy.yml` file created at {:?}", ansible_deploy_file_path);

    println!("🎉 Ansible File Copy Process Completed Successfully!");
    Ok(())
}
