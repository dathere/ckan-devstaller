#!/usr/bin/env bash

# Update/Upgrade system dependencies
sudo apt update -y
sudo apt upgrade -y

# Install curl
sudo apt install curl -y

# Change to the home directory
cd ~/

# Download the ckan-devstaller binary file
curl -LO https://github.com/dathere/ckan-devstaller/releases/download/0.1.0/ckan-devstaller

# Add execute permission to ckan-devstaller binary file
sudo chmod +x ./ckan-devstaller

# Run the ckan-devstaller binary file with defaults enabled
./ckan-devstaller --default

