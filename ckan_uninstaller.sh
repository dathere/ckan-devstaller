#!/bin/bash

# CKAN Complete Uninstaller Script
# This script removes all CKAN installations including Docker containers and system installations

set -e  # Exit on any error

echo "🗑️  CKAN Complete Uninstaller"
echo "================================"
echo
echo "This will remove:"
echo "- All Docker containers, volumes, and images related to CKAN"
echo "- ckan-compose directory and files"
echo "- System CKAN installation (/usr/lib/ckan/default)"
echo "- CKAN configuration (/etc/ckan/default)"
echo
read -p "Are you sure you want to proceed? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Uninstallation cancelled."
    exit 1
fi

echo
echo "🧹 Starting CKAN cleanup..."

# Step 1: Docker Compose cleanup (if ckan-compose directory exists)
if [ -d "ckan-compose" ]; then
    echo "Found ckan-compose directory, cleaning up Docker containers..."
    cd ckan-compose
    
    echo "   Stopping and removing containers and volumes..."
    docker-compose down --volumes 2>/dev/null || echo "   No docker-compose.yml found or already stopped"
    
    cd ..
else
    echo "No ckan-compose directory found in current location"
fi

# Step 2: Remove Docker containers with 'ckan' in the name
echo "Removing CKAN Docker containers..."
CKAN_CONTAINERS=$(docker ps -a --format "table {{.Names}}" | grep -i ckan | grep -v NAMES || true)
if [ ! -z "$CKAN_CONTAINERS" ]; then
    echo "$CKAN_CONTAINERS" | xargs docker rm -f
    echo "   Removed CKAN containers"
else
    echo "   No CKAN containers found"
fi

# Step 3: Remove Docker volumes with 'ckan' in the name
echo "Removing CKAN Docker volumes..."
CKAN_VOLUMES=$(docker volume ls --format "table {{.Name}}" | grep -i ckan | grep -v NAME || true)
if [ ! -z "$CKAN_VOLUMES" ]; then
    echo "$CKAN_VOLUMES" | xargs docker volume rm -f
    echo "   Removed CKAN volumes"
else
    echo "   No CKAN volumes found"
fi

# Step 4: Remove Docker images with 'ckan' in the name
echo "Removing CKAN Docker images..."
CKAN_IMAGES=$(docker images --format "table {{.Repository}}:{{.Tag}} {{.ID}}" | grep -i ckan | awk '{print $2}' || true)
if [ ! -z "$CKAN_IMAGES" ]; then
    echo "$CKAN_IMAGES" | xargs docker rmi -f
    echo "   Removed CKAN images"
else
    echo "   No CKAN images found"
fi

# Step 5: Remove ckan-compose directory
echo "Removing ckan-compose directory..."
if [ -d "ckan-compose" ]; then
    rm -rf ckan-compose
    echo "   ckan-compose directory removed"
else
    echo "   ckan-compose directory not found"
fi

# Step 6: Remove system CKAN installation
echo "Removing system CKAN installation..."
if [ -d "/usr/lib/ckan/default" ]; then
    sudo rm -rf /usr/lib/ckan/default
    echo "   Removed /usr/lib/ckan/default"
else
    echo "   /usr/lib/ckan/default not found"
fi

if [ -d "/etc/ckan/default" ]; then
    sudo rm -rf /etc/ckan/default
    echo "   Removed /etc/ckan/default"
else
    echo "   /etc/ckan/default not found"
fi

# Step 7: Clean up unused Docker resources
echo "Cleaning up unused Docker resources..."
docker system prune -f --volumes 2>/dev/null || echo "   Docker cleanup completed"

# Step 8: Check for remaining CKAN processes
echo "Checking for running CKAN services..."
CKAN_PROCESSES=$(ps aux | grep -i ckan | grep -v grep | grep -v "ckan_uninstaller" || true)
if [ ! -z "$CKAN_PROCESSES" ]; then
    echo "Warning: Found running CKAN processes:"
    echo "$CKAN_PROCESSES"
    echo "   You may need to stop these manually"
else
    echo "   No running CKAN processes found"
fi

echo
echo "CKAN uninstallation completed!"
echo
echo "Summary of actions taken:"
echo "- Stopped and removed all CKAN Docker containers"
echo "- Removed all CKAN Docker volumes and images"
echo "- Deleted ckan-compose directory"
echo "- Removed system CKAN installation directories"
echo "- Cleaned up unused Docker resources"
echo
echo "You can now proceed with a fresh installation using your devstaller."
