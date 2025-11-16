#!/bin/bash
# Bash wrapper to focus VS Code chat using PowerShell

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
powershell.exe -ExecutionPolicy Bypass -File "$SCRIPT_DIR/focus-chat.ps1"
