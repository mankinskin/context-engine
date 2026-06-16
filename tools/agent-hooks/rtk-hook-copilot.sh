#!/usr/bin/env bash
# tools/agent-hooks/rtk-hook-copilot.sh
#
# Check if stdin has data. If not, exit immediately to prevent hang.
# Otherwise, pipe stdin to rtk hook copilot.

set -euo pipefail

RTK_BIN=""
for candidate in \
    "${HOME:-}/.cargo/bin/rtk.exe" \
    "${HOME:-}/.cargo/bin/rtk" \
    "${USERPROFILE:-}\\.cargo\\bin\\rtk.exe" \
    "${USERPROFILE:-}/.cargo/bin/rtk.exe" \
    "/c/Users/${USERNAME:-}/.cargo/bin/rtk.exe" \
    "/c/Users/${USER:-}/.cargo/bin/rtk.exe"
do
    if [[ -n "$candidate" && -f "$candidate" ]]; then
        RTK_BIN="$candidate"
        break
    fi
done

if [[ -z "$RTK_BIN" ]]; then
    RTK_BIN="$(command -v rtk 2>/dev/null || true)"
fi

if read -t 0; then
    if [[ -n "$RTK_BIN" ]]; then
        "$RTK_BIN" hook copilot
    else
        echo "[rtk-hook-copilot] WARN: rtk not found; skipping rtk hook copilot" >&2
    fi
fi
