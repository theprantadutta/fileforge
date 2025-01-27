use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;

pub fn generate_dockerfile() -> io::Result<()> {
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
    let mut file = File::open(&config_path)?; // Open the file
    file.read_to_string(&mut config_content)?; // Read the content into the string

    // Parse the JSON file and get the `root_namespace`
    let config: Value = serde_json::from_str(&config_content)?;
    let root_namespace = config["root_namespace"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing root_namespace in config"))?
        .to_string();
    println!("Root namespace found in config: {}", root_namespace);

    // Step 2: Define the Dockerfile template directly in the function
    let docker_template = r#"
FROM mcr.microsoft.com/dotnet/aspnet:{{ dotnet_version }} AS base
#USER $APP_UID
WORKDIR /app
#EXPOSE 8080
#EXPOSE 8081

ENV ASPNETCORE_URLS=http://+:5000

# Set the timezone for the container by setting the TZ environment variable
# to the desired timezone (in this case, Asia/Dhaka).
ENV TZ=Asia/Dhaka

# Create a symbolic link from /etc/localtime to ensure the container uses the correct timezone
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
    echo $TZ > /etc/timezone

# For HealthChecks
RUN apt-get update && apt-get install -y curl

FROM mcr.microsoft.com/dotnet/sdk:{{ dotnet_version }} AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src
COPY ["{{ default_namespace }}.csproj", "./"]
RUN dotnet restore "{{ default_namespace }}.csproj"
COPY . .
WORKDIR "/src/"
RUN dotnet build "{{ default_namespace }}.csproj" -c $BUILD_CONFIGURATION -o /app/build

FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "{{ default_namespace }}.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "{{ default_namespace }}.dll"]
"#;

    // Step 3: Replace placeholder with namespace
    let updated_dockerfile = docker_template
        .replace("{{ dotnet_version }}", "9.0").replace("{{ default_namespace }}", &root_namespace);
    println!("Updated Dockerfile:\n{}", updated_dockerfile);

    // Step 4: Determine the output directory based on build mode
    #[cfg(debug_assertions)]
    let output_dir = current_dir.join("testing_directory").join("dotnet-testing"); // Debug mode path

    #[cfg(not(debug_assertions))]
    let output_dir = current_dir.clone(); // Release mode path

    // Ensure the directory exists
    println!("Ensuring directory exists: {:?}", output_dir);
    fs::create_dir_all(&output_dir)?;

    // Write the updated content to `Dockerfile` in the determined directory
    let output_path = output_dir.join("Dockerfile");
    println!("Output path: {:?}", output_path);

    let mut output_file = File::create(&output_path)?;
    output_file.write_all(updated_dockerfile.as_bytes())?;
    println!("Dockerfile written successfully to {:?}", output_path);

    Ok(())
}
