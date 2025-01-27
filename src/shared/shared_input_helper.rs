use std::io;

pub fn get_input_from_user(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().to_string()
}

pub fn get_input_from_user_with_default(prompt: &str, default: &str) -> String {
    let mut input = String::new();
    println!("{} (default: {})", prompt, default);
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    if input.trim().is_empty() {
        default.to_string()
    } else {
        input.trim().to_string()
    }
}

pub fn get_port_from_user() -> u16 {
    loop {
        let port: String = get_input_from_user("Port (e.g., 80): ");
        if let Ok(port_num) = port.parse::<u16>() {
            return port_num;
        } else {
            println!("Invalid port. Please enter a valid integer.");
        }
    }
}