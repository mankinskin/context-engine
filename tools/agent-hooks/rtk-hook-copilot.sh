#!/usr/bin/env bash
# tools/agent-hooks/rtk-hook-copilot.sh
#
# Check if stdin has data. If not, exit immediately to prevent hang.
# Otherwise, pipe stdin to rtk hook copilot.

set -euo pipefail

# Ensure ~/.cargo/bin is in PATH since this hook may run in a non-interactive shell where profile files are not sourced
for dir in \
    "$HOME/.cargo/bin" \
    "${USERPROFILE:-}/.cargo/bin" \
    "/c/Users/${USERNAME:-}/.cargo/bin" \
    "/c/Users/${USER:-}/.cargo/bin" \
    "C:\\Users\\${USERNAME:-}\\.cargo\\bin" \
    "C:\\Users\\${USER:-}\\.cargo\\bin"
do
    if [[ -n "$dir" && -d "$dir" && ( -f "$dir/rtk.exe" || -f "$dir/rtk" ) ]]; then
        export PATH="$dir:$PATH"
        break
    fi
done

if read -t 0; then
    rtk hook copilot
fi
