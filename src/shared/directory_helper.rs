use std::env;

pub fn get_current_directory() -> Result<String, String> {
    // Get the current working directory
    match env::current_dir() {
        Ok(path) => {
            // Get the name of the current root directory
            if let Some(dir_name) = path.file_name() {
                let current_directory = dir_name.to_string_lossy().to_string();
                Ok(current_directory)
            } else {
                Err("Failed to get the directory name.".to_string())
            }
        }
        Err(e) => Err(format!("Error getting current directory: {}", e)),
    }
}