use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    thread,
};

use serde_json::Value;

use crate::constants::SLEEP_DURATION;

pub fn generate_gitlab_ci_file_for_angular() -> io::Result<()> {
    println!("🚀 Starting gitlab ci file generation...");
    thread::sleep(SLEEP_DURATION);

    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("📂 Current directory: {:?}", current_dir);
    thread::sleep(SLEEP_DURATION);

    // Step 1: Check for `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    println!("🔍 Checking for config file at: {:?}", config_path);
    thread::sleep(SLEEP_DURATION);

    if !config_path.exists() {
        println!("Error: Config file not found at {:?}", config_path);
        thread::sleep(SLEEP_DURATION);
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "fileforge.config.json not found.",
        ));
    }

    let mut config_content = String::new();
    File::open(&config_path)?.read_to_string(&mut config_content)?;
    println!("✅ Config file found and loaded!");
    thread::sleep(SLEEP_DURATION);

    // Step 2: Parse the JSON file and get `project_location` and `project_directory`
    let config: Value = serde_json::from_str(&config_content)?;
    let project_location = config["project_location"]
        .as_str()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing project_location in config",
            )
        })?
        .to_string();

    let project_directory = config["project_directory"]
        .as_str()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing project_directory in config",
            )
        })?
        .to_string();

    println!("🌟 Project location: {}", project_location);
    thread::sleep(SLEEP_DURATION);
    println!("📁 Project directory: {}", project_directory);
    thread::sleep(SLEEP_DURATION);

    // Define the GitLab CI template with placeholders
    let gitlab_ci_template = r#"
variables:
  PROJECT_PATH: "{{ project_location }}/{{ project_directory }}"

stages:
  - deploy
  - start
  - ansible

deploy-job:
  stage: deploy
  before_script:
    - sudo mkdir -p $PROJECT_PATH
  script:
    - sudo find $PROJECT_PATH -mindepth 1 -maxdepth 1 ! \( -name "node_modules" \) -exec rm -rf {} +
    # COPY FILES FROM CURRENT GITLAB PATH TO PROJECT PATH
    - sudo cp -r * $PROJECT_PATH
  only:
    - master
  tags:
    - docker

start-job:
  stage: start
  script:
    - cd $PROJECT_PATH
    - |
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
  only:
    - master  # Add other branches/tags if needed
  tags:
    - docker

ansible-job:
  stage: ansible
  before_script:
    # Check if ansible-playbook is accessible, otherwise install it
    - if ! command -v ansible-playbook &> /dev/null; then
        echo "Installing Ansible...";
        pipx install --include-deps ansible;
        pipx ensurepath;
      fi
    - echo "Validating Ansible inventory..."
    - ansible-inventory -i ansible/hosts.yml --list  # Validate inventory
  script:
    - echo "Populating ansible_become_pass in inventory file"
    - sed -i "s/{{ ansible_host_name_placeholder }}/$ANSIBLE_HOST_NAME_DELL_SERVER/" ansible/hosts.yml
    - sed -i "s/{{ ansible_ssh_port_placeholder }}/$ANSIBLE_HOST_PORT_DELL_SERVER/" ansible/hosts.yml
    - sed -i "s/{{ ansible_host_placeholder }}/$ANSIBLE_HOST_DELL_SERVER/" ansible/hosts.yml
    - sed -i "s/{{ ansible_user_placeholder }}/$ANSIBLE_USER_DELL_SERVER/" ansible/hosts.yml
    - sed -i "s/{{ ansible_become_pass_placeholder }}/$ANSIBLE_BECOME_PASS_DELL_SERVER/" ansible/hosts.yml
    - cat ansible/hosts.yml # Optional: Verify the replacement (for debugging)
    - echo "Running Ansible playbook..."
    - ansible-playbook -i ansible/hosts.yml ansible/ansible-deploy.yml
  only:
    - master
  tags:
    - docker
"#;

    // Replace placeholders with values from the config
    let updated_gitlab_ci = gitlab_ci_template
        .replace("{{ project_location }}", &project_location)
        .replace("{{ project_directory }}", &project_directory);

    #[cfg(debug_assertions)]
    let output_dir = current_dir.join("testing_directory").join("dotnet-testing"); // Debug mode path

    #[cfg(not(debug_assertions))]
    let output_dir = current_dir.clone(); // Release mode path

    // Ensure the directory exists
    println!("📁 Ensuring directory exists: {:?}", output_dir);
    thread::sleep(SLEEP_DURATION);
    fs::create_dir_all(&output_dir)?;

    // Write the updated content to `.gitlab-ci.yml` in the output directory
    let output_path = output_dir.join(".gitlab-ci.yml");
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(updated_gitlab_ci.as_bytes())?;
    println!("🎉 Generated .gitlab-ci.yml in {:?}", output_path);
    thread::sleep(SLEEP_DURATION);

    Ok(())
}
