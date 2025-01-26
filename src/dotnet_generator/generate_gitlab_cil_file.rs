use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;

pub fn generate_gitlab_ci_file() -> io::Result<()> {
    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    // Step 1: Check for `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    println!("Config file path: {:?}", config_path);

    if !config_path.exists() {
        println!("Error: Config file not found at {:?}", config_path);
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "fileforge.config.json not found.",
        ));
    }

    let mut config_content = String::new();
    File::open(&config_path)?.read_to_string(&mut config_content)?;
    println!("Config file content loaded successfully.");

    // Step 2: Parse the JSON file and get `project_location` and `project_directory`
    let config: Value = serde_json::from_str(&config_content)?;
    let project_location = config["project_location"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing project_location in config"))?
        .to_string();

    let project_directory = config["project_directory"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing project_directory in config"))?
        .to_string();

    println!("Project location: {}", project_location);
    println!("Project directory: {}", project_directory);

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
    - mkdir -p $PROJECT_PATH
  script:
    - sudo find $PROJECT_PATH -mindepth 1 -maxdepth 1 ! \( -name "Logs" \) -exec rm -rf {} + # DELETE ALL FILES FIRST EXCEPT LOGS FOLDER
    - sudo cp -r * $PROJECT_PATH # COPY ALL FILES FROM CURRENT GITLAB DIRECTORY TO A SPECIFIC PROJECT PATH
  only:
    - master
  tags:
    - docker

start-job:
  stage: start
  script:
    - cd $PROJECT_PATH
    - sudo docker-compose up --build -d --remove-orphans
  # Only run pipeline for master branch push
  only:
    - master
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
    println!("Ensuring directory exists: {:?}", output_dir);
    fs::create_dir_all(&output_dir)?;

    // Write the updated content to `.gitlab-ci.yml` in the output directory
    let output_path = output_dir.join(".gitlab-ci.yml");
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(updated_gitlab_ci.as_bytes())?;
    println!("Generated .gitlab-ci.yml in {:?}", output_path);

    Ok(())
}
