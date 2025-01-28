use std::env;

/// Retrieves the name of the current working directory.
///
/// # Returns
/// - `Ok(String)`: The name of the current directory as a `String`.
/// - `Err(String)`: An error message if the current directory cannot be determined.
pub fn get_current_directory() -> Result<String, String> {
    // Attempt to get the current working directory
    match env::current_dir() {
        Ok(path) => {
            // Extract the directory name from the path
            if let Some(dir_name) = path.file_name() {
                // Convert the directory name to a String and return it
                let current_directory = dir_name.to_string_lossy().to_string();
                println!("📂 Current directory: {}", current_directory); // Log the result
                Ok(current_directory)
            } else {
                // Return an error if the directory name cannot be extracted
                let error_message = "❌ Failed to get the directory name.".to_string();
                eprintln!("{}", error_message); // Log the error
                Err(error_message)
            }
        }
        Err(e) => {
            // Handle errors when getting the current working directory
            let error_message = format!("❌ Error getting current directory: {}", e);
            eprintln!("{}", error_message); // Log the error
            Err(error_message)
        }
    }
}
