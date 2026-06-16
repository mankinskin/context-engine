#!/usr/bin/env bash
# tools/agent-hooks/rtk-hook-copilot.sh
#
# Check if stdin has data. If not, exit immediately to prevent hang.
# Otherwise, pipe stdin to rtk hook copilot.

set -euo pipefail

# Ensure ~/.cargo/bin is in PATH since this hook may run in a non-interactive shell where profile files are not sourced
if [[ -d "$HOME/.cargo/bin" ]]; then
    export PATH="$HOME/.cargo/bin:$PATH"
fi

if read -t 0; then
    rtk hook copilot
fi
