#!/bin/bash

# Installer script for Odin CLI

echo "Installing Odin CLI..."

# Determine OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture
case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Download the latest release binary
RELEASE_URL="https://github.com/andremillet/odin/releases/latest/download/odin-${OS}-${ARCH}"
echo "Downloading from: $RELEASE_URL"

if command -v curl >/dev/null 2>&1; then
    curl -L -o /tmp/odin "$RELEASE_URL"
elif command -v wget >/dev/null 2>&1; then
    wget -O /tmp/odin "$RELEASE_URL"
else
    echo "Error: Neither curl nor wget is available."
    exit 1
fi

# Check if download succeeded
if [ ! -f "/tmp/odin" ]; then
    echo "Error: Failed to download the binary."
    exit 1
fi

# Validate the downloaded file
FILE_SIZE=$(stat -c%s "/tmp/odin" 2>/dev/null || stat -f%z "/tmp/odin" 2>/dev/null || echo 0)
if [ "$FILE_SIZE" -lt 100000 ]; then
    echo "Error: Downloaded file is too small ($FILE_SIZE bytes). Possibly a 404 or error page."
    rm -f /tmp/odin
    exit 1
fi

# Check if it's an ELF executable
if ! file "/tmp/odin" | grep -q "ELF.*executable"; then
    echo "Error: Downloaded file is not a valid executable."
    rm -f /tmp/odin
    exit 1
fi

# Copy binary to /usr/local/bin
sudo cp /tmp/odin /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/odin

# Clean up
rm /tmp/odin

echo "Odin installed successfully! Run 'odin --help' to get started."