#!/bin/bash
# Download redmine-mcp binary for the current platform
set -e

VERSION="1.0.0"
REPO="Sunalamye/RedmineMCP"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="$SCRIPT_DIR/../bin"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        case "$ARCH" in
            arm64) PLATFORM="macos-arm64" ;;
            x86_64) PLATFORM="macos-x64" ;;
            *) echo "Unsupported macOS architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Linux)
        case "$ARCH" in
            aarch64|arm64) PLATFORM="linux-arm64" ;;
            x86_64) PLATFORM="linux-x64" ;;
            *) echo "Unsupported Linux architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT)
        PLATFORM="windows-x64"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Determine binary name
if [[ "$PLATFORM" == windows* ]]; then
    BINARY_NAME="redmine-mcp-${VERSION}-${PLATFORM}.exe"
    LOCAL_NAME="redmine-mcp.exe"
else
    BINARY_NAME="redmine-mcp-${VERSION}-${PLATFORM}"
    LOCAL_NAME="redmine-mcp"
fi

# Download URL
URL="https://github.com/${REPO}/releases/download/v${VERSION}/${BINARY_NAME}"

# Create bin directory
mkdir -p "$BIN_DIR"

# Download binary
echo "Downloading redmine-mcp v${VERSION} for ${PLATFORM}..."
if command -v curl &> /dev/null; then
    curl -fsSL "$URL" -o "$BIN_DIR/$LOCAL_NAME"
elif command -v wget &> /dev/null; then
    wget -q "$URL" -O "$BIN_DIR/$LOCAL_NAME"
else
    echo "Error: curl or wget is required"
    exit 1
fi

# Make executable
chmod +x "$BIN_DIR/$LOCAL_NAME"

# Verify checksum
echo "Verifying checksum..."
CHECKSUM_URL="https://github.com/${REPO}/releases/download/v${VERSION}/checksums.txt"
if command -v curl &> /dev/null; then
    CHECKSUMS=$(curl -fsSL "$CHECKSUM_URL" 2>/dev/null || echo "")
elif command -v wget &> /dev/null; then
    CHECKSUMS=$(wget -qO- "$CHECKSUM_URL" 2>/dev/null || echo "")
fi

if [ -n "$CHECKSUMS" ]; then
    EXPECTED=$(echo "$CHECKSUMS" | grep "$BINARY_NAME" | awk '{print $1}')
    if [ -n "$EXPECTED" ]; then
        if command -v shasum &> /dev/null; then
            ACTUAL=$(shasum -a 256 "$BIN_DIR/$LOCAL_NAME" | awk '{print $1}')
        elif command -v sha256sum &> /dev/null; then
            ACTUAL=$(sha256sum "$BIN_DIR/$LOCAL_NAME" | awk '{print $1}')
        else
            echo "Warning: Cannot verify checksum (no shasum or sha256sum)"
            ACTUAL=""
        fi

        if [ -n "$ACTUAL" ] && [ "$EXPECTED" != "$ACTUAL" ]; then
            echo "ERROR: Checksum verification failed!"
            echo "  Expected: $EXPECTED"
            echo "  Actual:   $ACTUAL"
            rm -f "$BIN_DIR/$LOCAL_NAME"
            exit 1
        fi
        echo "Checksum verified."
    else
        echo "Warning: Checksum not found for $BINARY_NAME"
    fi
else
    echo "Warning: Could not download checksums file"
fi

echo "Downloaded to: $BIN_DIR/$LOCAL_NAME"
