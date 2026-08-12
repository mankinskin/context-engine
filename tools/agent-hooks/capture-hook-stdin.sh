#!/usr/bin/env bash
# Persist raw Copilot hook stdin before forwarding the exact bytes to
# session-capture-hook. The capture directory is already gitignored.

set -uo pipefail

capture_dir="${SESSION_HOOK_CAPTURE_DIR:-.session/local/hook-captures}"

# The hook environment's PATH does not always include the cargo bin directory,
# so resolve the binary explicitly before falling back to PATH lookup.
resolve_capture_hook() {
    local candidate
    for candidate in \
        "${SESSION_CAPTURE_HOOK_BIN:-}" \
        "${CARGO_HOME:-$HOME/.cargo}/bin/session-capture-hook" \
        "${CARGO_HOME:-$HOME/.cargo}/bin/session-capture-hook.exe"; do
        if [[ -n "$candidate" && -x "$candidate" ]]; then
            printf '%s\n' "$candidate"
            return 0
        fi
    done
    command -v session-capture-hook 2>/dev/null
}

capture_hook_bin=$(resolve_capture_hook)
if [[ -z "$capture_hook_bin" ]]; then
    echo "capture-hook-stdin.sh: session-capture-hook not found on PATH or in ${CARGO_HOME:-$HOME/.cargo}/bin" >&2
    echo '{}'
    exit 0
fi

if [[ "${SESSION_HOOK_CAPTURE:-1}" == "0" ]]; then
    exec "$capture_hook_bin" --from-hook-stdin
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

"$capture_hook_bin" --from-hook-stdin < "$capture_path"
