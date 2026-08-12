#!/usr/bin/env bash
# Persist raw Copilot hook stdin before forwarding the exact bytes to
# session-capture-hook. The capture directory is already gitignored.

set -uo pipefail

capture_dir="${SESSION_HOOK_CAPTURE_DIR:-.session/local/hook-captures}"

if [[ "${SESSION_HOOK_CAPTURE:-1}" == "0" ]]; then
    session-capture-hook --from-hook-stdin
fi

mkdir -p "$capture_dir"
temporary_capture=$(mktemp "$capture_dir/.hook-stdin.XXXXXX")
trap 'rm -f "$temporary_capture"' EXIT
cat > "$temporary_capture"

event=$(jq -r '.hook_event_name // .hookEventName // "unknown"' "$temporary_capture" 2>/dev/null)
case "$event" in
    ''|*[!A-Za-z0-9_-]*) event="unknown" ;;
esac

capture_path="$capture_dir/$event.json"
mv "$temporary_capture" "$capture_path"
trap - EXIT

session-capture-hook --from-hook-stdin < "$capture_path"
