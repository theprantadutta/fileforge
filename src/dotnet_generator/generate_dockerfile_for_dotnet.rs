use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::{env, thread};

use crate::constants::SLEEP_DURATION;

pub fn generate_dockerfile_for_dotnet() -> io::Result<()> {
    println!("🚀 Starting Dockerfile generation...");
    thread::sleep(SLEEP_DURATION);

    // Get the current directory
    let current_dir = env::current_dir()?;
    println!("📂 Current directory: {:?}", current_dir);
    thread::sleep(SLEEP_DURATION);

    // Check for `fileforge.config.json`
    let config_path = current_dir.join("fileforge.config.json");
    println!("🔍 Checking for config file at: {:?}", config_path);
    thread::sleep(SLEEP_DURATION);

    if !config_path.exists() {
        println!("❌ Config file not found!");
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

    // Parse the JSON file
    let config: Value = serde_json::from_str(&config_content)?;
    let project_directory = config["project_directory"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing 'project_directory'"))?
        .to_string();
    println!("📂 Found project directory: {}", project_directory);
    thread::sleep(SLEEP_DURATION);

    let dotnet_version = config["dotnet_version"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing 'dotnet_version'"))?
        .to_string();
    println!("📂 Found dotnet_version: {}", dotnet_version);
    thread::sleep(SLEEP_DURATION);

    // Prepare Dockerfile template
    let docker_template = r#"
# Use an ARG for the Nexus URL and set a default fallback value
ARG SONATYPE_NEXUS_URL=mcr.microsoft.com

FROM ${SONATYPE_NEXUS_URL}/dotnet/aspnet:{{ dotnet_version }} AS base
# USER $APP_UID
WORKDIR /app
# EXPOSE 8080
# EXPOSE 8081

ENV ASPNETCORE_URLS=http://+:5000

# Set the timezone for the container
ENV TZ=Asia/Dhaka

# Create a symbolic link for the timezone
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
    echo $TZ > /etc/timezone

# For HealthChecks
RUN apt-get update && apt-get install -y curl

FROM ${SONATYPE_NEXUS_URL}/dotnet/sdk:{{ dotnet_version }} AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src
COPY ["{{ project_directory }}.csproj", "./"]
RUN dotnet restore "{{ project_directory }}.csproj"
COPY . .
WORKDIR "/src/"
RUN dotnet build "{{ project_directory }}.csproj" -c $BUILD_CONFIGURATION -o /app/build

FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "{{ project_directory }}.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "{{ project_directory }}.dll"]
"#;

    let updated_dockerfile = docker_template
        .replace("{{ dotnet_version }}", &dotnet_version)
        .replace("{{ project_directory }}", &project_directory);
    println!("✅ Dockerfile template updated with project directory.");
    thread::sleep(SLEEP_DURATION);

    // Write Dockerfile to output directory
    #[cfg(debug_assertions)]
    let output_dir = current_dir.join("testing_directory").join("dotnet-testing");
    #[cfg(not(debug_assertions))]
    let output_dir = current_dir.clone();

    println!("📁 Ensuring output directory exists: {:?}", output_dir);
    fs::create_dir_all(&output_dir)?;
    thread::sleep(SLEEP_DURATION);

    let output_path = output_dir.join("Dockerfile");
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(updated_dockerfile.as_bytes())?;
    println!("✅ Dockerfile written to: {:?}", output_path);
    thread::sleep(SLEEP_DURATION);

    println!("🎉 Dockerfile generation completed successfully!");
    thread::sleep(SLEEP_DURATION);
    Ok(())
}
