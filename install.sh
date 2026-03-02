#!/usr/bin/env bash
set -euo pipefail

# CAPPY Installation Script
# Installs the CAPPY MCP server binary and pattern database

CAPPY_HOME="${CAPPY_HOME:-$HOME/.cappy}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "Installing CAPPY to $CAPPY_HOME..."

# Detect platform
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
  arm64|aarch64) ARCH="arm64" ;;
  x86_64|amd64)  ARCH="x64" ;;
  *)
    echo "Error: Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

BINARY="servers/cappy-core"

if [ ! -f "$SCRIPT_DIR/$BINARY" ]; then
  echo "Error: Binary not found at $SCRIPT_DIR/$BINARY"
  echo "This distribution includes darwin-arm64 only."
  echo "Build from source for other platforms."
  exit 1
fi

# Create directories
mkdir -p "$CAPPY_HOME/bin"
mkdir -p "$CAPPY_HOME/databases"
mkdir -p "$CAPPY_HOME/logs"

# Copy binary
cp "$SCRIPT_DIR/$BINARY" "$CAPPY_HOME/bin/cappy-core"
chmod +x "$CAPPY_HOME/bin/cappy-core"

# Copy sample pattern database
if [ -f "$SCRIPT_DIR/databases/cappy-cache_sample.json" ]; then
  cp "$SCRIPT_DIR/databases/cappy-cache_sample.json" "$CAPPY_HOME/databases/"
fi

# Verify installation
if "$CAPPY_HOME/bin/cappy-core" --version 2>/dev/null; then
  echo ""
  echo "CAPPY installed successfully!"
else
  echo ""
  echo "Binary installed to $CAPPY_HOME/bin/cappy-core"
fi

echo ""
echo "Next steps:"
echo "  1. Add to PATH: export PATH=\"\$CAPPY_HOME/bin:\$PATH\""
echo "  2. Configure Claude Code: Add the MCP server config from .mcp.json"
echo "  3. Run: cappy-core --help"
