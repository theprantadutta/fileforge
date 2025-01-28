use std::{io, process::Command};

pub fn check_git_status() -> io::Result<()> {
    // Execute the `git status --porcelain` command to check for unstaged files
    let git_status_output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .map_err(|e| {
            eprintln!("❌ Failed to execute `git status`: {}", e);
            io::Error::new(io::ErrorKind::Other, "Failed to execute git status")
        })?;

    // Convert the command output to a string
    let git_status = String::from_utf8_lossy(&git_status_output.stdout);

    // Track if there are any unstaged files and whether only `fileforge.config.json` is unstaged
    let mut unstaged_files = false;
    let mut is_only_config_unstaged = true;

    // Iterate through each line of the `git status` output
    for line in git_status.lines() {
        let status = line.split_whitespace().collect::<Vec<&str>>();

        // Check if the line represents a file change
        if status.len() > 1 {
            if status[1] != "fileforge.config.json" {
                unstaged_files = true; // Found an unstaged file other than `fileforge.config.json`
            } else {
                is_only_config_unstaged = false; // Found `fileforge.config.json` as an unstaged file
            }
        }
    }

    // Log the results of the Git status check
    println!("🔍 Unstaged files detected: {}", unstaged_files);
    println!(
        "🔍 Is only `fileforge.config.json` unstaged: {}",
        is_only_config_unstaged
    );

    // In non-debug mode, enforce the rule that no unstaged files (except `fileforge.config.json`) are allowed
    #[cfg(not(debug_assertions))]
    if unstaged_files || !is_only_config_unstaged {
        eprintln!("❌ Error: You have unstaged files. Please commit or stash your changes.");
        std::process::exit(1); // Exit with an error code if there are unstaged files
    }

    Ok(()) // Return `Ok` if everything succeeds
}
