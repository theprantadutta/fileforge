use std::io;

/// Prompts the user for input and returns the trimmed input as a `String`.
///
/// # Parameters
/// - `prompt`: A message to display to the user before accepting input.
///
/// # Returns
/// - `String`: The user's input, trimmed of whitespace.
pub fn get_input_from_user(prompt: &str) -> String {
    let mut input = String::new();
    println!("➡️  {}", prompt); // Prompt the user
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Failed to read input.");
    let trimmed_input = input.trim();
    println!("✅ Received input: {}", trimmed_input); // Log the trimmed input
    trimmed_input.to_string()
}

/// Prompts the user for input with a default value and returns the trimmed input or the default.
///
/// # Parameters
/// - `prompt`: A message to display to the user before accepting input.
/// - `default`: The default value to use if no input is provided.
///
/// # Returns
/// - `String`: The user's input, trimmed of whitespace, or the default value.
pub fn get_input_from_user_with_default(prompt: &str, default: &str) -> String {
    let mut input = String::new();
    println!("➡️  {} (default: {})", prompt, default); // Prompt the user with default value
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Failed to read input.");
    if input.trim().is_empty() {
        println!("⚠️  No input provided. Using default value: {}", default); // Log the use of default
        default.to_string()
    } else {
        let trimmed_input = input.trim();
        println!("✅ Received input: {}", trimmed_input); // Log the trimmed input
        trimmed_input.to_string()
    }
}

/// Prompts the user to enter a valid port number and returns it as a `u16`.
///
/// # Returns
/// - `u16`: The valid port number entered by the user.
pub fn get_port_from_user() -> u16 {
    loop {
        let port: String = get_input_from_user("Port (e.g., 80): ");
        match port.parse::<u16>() {
            Ok(port_num) => {
                println!("✅ Using port: {}", port_num); // Log the valid port
                return port_num;
            }
            Err(_) => {
                println!("❌ Invalid port. Please enter a valid integer."); // Log invalid input
            }
        }
    }
}
