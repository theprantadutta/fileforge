#!/bin/bash

set -e

# GitHub repo details
REPO_OWNER="theprantadutta"
REPO_NAME="fileforge"

# Get the latest release version from GitHub API
LATEST_RELEASE=$(curl -s https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest)
VERSION=$(echo $LATEST_RELEASE | grep -oP '"tag_name": "\K(.*?)(?=")')

# Define the download URL for the Unix version
ASSET_URL=$(echo $LATEST_RELEASE | grep -oP '"browser_download_url": "\K(.*?)(?=")' | grep "unix")

# Download the binary for Unix
echo "Downloading FileForge version $VERSION for Unix..."
curl -L -o fileforge_$VERSION $ASSET_URL

# Make the binary executable
chmod +x fileforge_$VERSION

# Move it to /usr/local/bin
sudo mv fileforge_$VERSION /usr/local/bin/fileforge

echo "FileForge v$VERSION installed successfully!"