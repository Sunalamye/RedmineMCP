#!/bin/bash
# Wrapper script to run redmine-mcp, downloading if necessary
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="$SCRIPT_DIR/../bin"

# Detect binary name
if [[ "$OS" == "Windows_NT" ]] || [[ "$(uname -s)" == MINGW* ]] || [[ "$(uname -s)" == MSYS* ]]; then
    BINARY="$BIN_DIR/redmine-mcp.exe"
else
    BINARY="$BIN_DIR/redmine-mcp"
fi

# Download if not exists
if [ ! -f "$BINARY" ]; then
    echo "Binary not found, downloading..." >&2
    "$SCRIPT_DIR/download-binary.sh" >&2
fi

# Run the binary
exec "$BINARY" "$@"
