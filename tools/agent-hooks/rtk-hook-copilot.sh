#!/usr/bin/env bash
# tools/agent-hooks/rtk-hook-copilot.sh
#
# Check if stdin has data. If not, exit immediately to prevent hang.
# Otherwise, pipe stdin to rtk hook copilot.

set -euo pipefail

if read -t 0; then
    rtk hook copilot
fi
