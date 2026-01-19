#!/bin/bash
set -e

VERSION="1.0.0"
NAME="redmine-mcp"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RELEASE_DIR="$SCRIPT_DIR/release"

# Change to script directory for cargo
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "=== Building $NAME v$VERSION ==="
echo ""

# Clean release directory
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# Targets to build
TARGETS=(
    "aarch64-apple-darwin"      # macOS ARM64 (Apple Silicon)
    "x86_64-pc-windows-gnu"     # Windows x64
    "aarch64-unknown-linux-gnu" # Linux ARM64
)

# Build each target
for target in "${TARGETS[@]}"; do
    echo -e "${YELLOW}Building for $target...${NC}"

    # Choose build command based on target
    case "$target" in
        *apple*)
            # Native build for macOS
            cargo build --release --target "$target"
            ;;
        *windows*)
            # Use cargo + mingw-w64 for Windows (zigbuild has dlltool issues)
            cargo build --release --target "$target"
            ;;
        *)
            # Use zigbuild for Linux cross-compilation
            cargo zigbuild --release --target "$target"
            ;;
    esac

    # Determine output filename
    case "$target" in
        *windows*)
            EXT=".exe"
            PLATFORM="windows-x64"
            ;;
        *apple*)
            EXT=""
            PLATFORM="macos-arm64"
            ;;
        *aarch64*linux*)
            EXT=""
            PLATFORM="linux-arm64"
            ;;
        *x86_64*linux*)
            EXT=""
            PLATFORM="linux-x64"
            ;;
        *)
            EXT=""
            PLATFORM="$target"
            ;;
    esac

    # Copy to release directory
    SRC="$SCRIPT_DIR/target/$target/release/${NAME}${EXT}"
    DST="$RELEASE_DIR/${NAME}-${VERSION}-${PLATFORM}${EXT}"

    if [ -f "$SRC" ]; then
        cp "$SRC" "$DST"
        echo -e "${GREEN}  -> $DST${NC}"
    else
        echo "  Warning: $SRC not found"
    fi
    echo ""
done

# Create checksums
echo "Creating checksums..."
cd "$RELEASE_DIR"
shasum -a 256 * > checksums.txt
echo ""

# Show results
echo "=== Release files ==="
ls -lh "$RELEASE_DIR"
echo ""
cat "$RELEASE_DIR/checksums.txt"
