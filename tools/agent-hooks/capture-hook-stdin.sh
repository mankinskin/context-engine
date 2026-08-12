#!/usr/bin/env bash
# Records the raw hook stdin payload, then forwards it unchanged to
# session-capture-hook.
#
# Why this exists: `.github/hooks/hooks.json` previously used a bare
# `tee ./session.log | session-capture-hook --from-hook-stdin`. On Windows the
# hook command is not run through a POSIX shell, so `tee` resolved to
# PowerShell's `Tee-Object`, which wrote a UTF-16LE BOM and zero payload bytes.
# Routing through `bash tools/agent-hooks/capture-hook-stdin.sh` matches the
# pattern already proven by the other hooks in this directory.
#
# Canonical location: tools/agent-hooks/capture-hook-stdin.sh
# Referenced by: .github/hooks/hooks.json
#
# Capture output: .session/local/hook-captures/<event>.json (gitignored).
# One file per event name, latest write wins, so the capture directory stays
# bounded regardless of session length.
#
# Disable capture (still forwards to session-capture-hook) with:
#   SESSION_HOOK_CAPTURE=0

set -uo pipefail

payload=$(cat)

if [[ "${SESSION_HOOK_CAPTURE:-1}" != "0" ]]; then
    event=$(printf '%s' "$payload" | jq -r '.hook_event_name // .hookEventName // "unknown"' 2>/dev/null)
    case "$event" in
        ''|*[!A-Za-z0-9_-]*) event="unknown" ;;
    esac

    capture_dir="${SESSION_HOOK_CAPTURE_DIR:-.session/local/hook-captures}"
    if mkdir -p "$capture_dir" 2>/dev/null; then
        printf '%s' "$payload" > "$capture_dir/$event.json" 2>/dev/null || true
    fi
fi

if command -v session-capture-hook >/dev/null 2>&1; then
    printf '%s' "$payload" | session-capture-hook --from-hook-stdin
    exit 0
fi

# session-capture-hook is not installed: stay non-blocking.
echo "capture-hook-stdin: session-capture-hook not on PATH" >&2
echo '{}'
