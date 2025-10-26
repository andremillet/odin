#!/bin/bash

# Installer script for Odin CLI

echo "Installing Odin CLI..."

# Check if binary exists
if [ ! -f "target/release/odin" ]; then
    echo "Error: Binary not found. Run 'cargo build --release' first."
    exit 1
fi

# Copy binary to /usr/local/bin
sudo cp target/release/odin /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/odin

echo "Odin installed successfully! Run 'odin --help' to get started."