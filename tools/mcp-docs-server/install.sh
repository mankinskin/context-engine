#!/bin/bash
# Install script for mcp-docs-server
# Builds the server and copies it to the agents directory

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
TARGET_EXE="$SCRIPT_DIR/target/release/mcp-docs-server.exe"
INSTALL_DIR="$REPO_ROOT/context-engine/agents"
INSTALLED_EXE="$INSTALL_DIR/mcp-docs-server.exe"

echo -e "\033[36mBuilding mcp-docs-server...\033[0m"
cd "$SCRIPT_DIR"
cargo build --release

echo -e "\033[36mCopying to agents directory...\033[0m"
cp "$TARGET_EXE" "$INSTALLED_EXE"

echo -e "\033[33mPlease restart the MCP server in VS Code:\033[0m"
echo -e "\033[33m  1. Open Command Palette (Ctrl+Shift+P)\033[0m"
echo -e "\033[33m  2. Run 'MCP: List Servers'\033[0m"
echo -e "\033[33m  3. Click the refresh/restart button for 'docs'\033[0m"

echo ""
echo -e "\033[32mInstallation complete!\033[0m"
echo -e "\033[32mInstalled to: $INSTALLED_EXE\033[0m"
