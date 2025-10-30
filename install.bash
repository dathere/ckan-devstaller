#!/usr/bin/env bash

# Update/Upgrade system dependencies
sudo apt update -y
sudo apt upgrade -y

# Install curl
sudo apt install curl -y

# Change to the home directory
cd ~/

# Download the ckan-devstaller binary file
curl -LO https://github.com/dathere/ckan-devstaller/releases/download/0.3.1/ckan-devstaller

# Add execute permission to ckan-devstaller binary file
sudo chmod +x ./ckan-devstaller

# Run the ckan-devstaller binary file with the specified preset and (non-)interactive mode
preset=$1
skip_interactive=$2

if [ $preset == "dathere-default" ]; then
    if [ $skip_interactive == "skip-interactive" ]; then
        ./ckan-devstaller --ckan-version 2.11.4 --extensions ckanext-scheming DataStore DataPusher+ --features enable-ssh --skip-interactive
    else
        ./ckan-devstaller --ckan-version 2.11.4 --extensions ckanext-scheming DataStore DataPusher+ --features enable-ssh
    fi
else
    if [ $preset == "skip-interactive" ]; then
        ./ckan-devstaller --skip-interactive
    else
        ./ckan-devstaller
    fi
fi
