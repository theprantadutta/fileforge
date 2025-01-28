use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    thread,
};

use crate::constants::SLEEP_DURATION;

pub fn generate_nginx_file_angular() -> io::Result<()> {
    println!("🚀 Starting nginx file generation...");
    thread::sleep(SLEEP_DURATION);

    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("📂 Current directory: {:?}", current_dir);
    thread::sleep(SLEEP_DURATION);

    // Base Nginx configuration template
    let template = r#"
events {}

http {
    include       /etc/nginx/mime.types;
    default_type  application/octet-stream;

    server {
        listen       80;

        root   /usr/share/nginx/html;
        index  index.html;

        location / {
            try_files $uri $uri/ /index.html;
        }

        error_page   500 502 503 504  /50x.html;
        location = /50x.html {
            root   /usr/share/nginx/html;
        }
    }
}
"#;

    // Determine the output directory based on the build mode
    let output_dir = if cfg!(debug_assertions) {
        current_dir.join("testing_directory").join("dotnet-testing")
    } else {
        current_dir.clone()
    };

    println!("📁 Ensuring output directory exists: {:?}", output_dir);
    thread::sleep(SLEEP_DURATION);
    fs::create_dir_all(&output_dir)?;

    // Path for the output compose file
    let output_path = output_dir.join("nginx.conf");

    // Delete any existing nginx.conf file
    let file_path = Path::new("nginx.conf");
    if file_path.exists() {
        println!("🗑️  Deleting previous nginx.conf file...");
        thread::sleep(SLEEP_DURATION);
        match fs::remove_file(file_path) {
            Ok(_) => {
                println!("✅ Previous file deleted successfully.");
                thread::sleep(SLEEP_DURATION);
            }
            Err(e) => eprintln!("❌ Error deleting file: {}", e),
        }
    } else {
        println!("🗂️  No previous nginx.conf file found.");
        thread::sleep(SLEEP_DURATION);
    }

    // Write the Nginx configuration to `nginx.conf`
    println!("📝 Writing the nginx configuration to {:?}", output_path);
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(template.as_bytes())?;
    println!("🎉 Nginx file generated successfully at {:?}", output_path);
    thread::sleep(SLEEP_DURATION);

    Ok(())
}
