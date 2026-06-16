#!/bin/bash
# Stop hook for persisting agent chat transcripts through session-api
#
# Canonical location: tools/agent-hooks/session-capture-stop.sh
# Referenced by: .github/hooks/hooks.json, .clinerules/hooks/hooks.json

set -uo pipefail

# Ensure ~/.cargo/bin is in PATH since this hook may run in a non-interactive shell where profile files are not sourced
for dir in \
    "$HOME/.cargo/bin" \
    "${USERPROFILE:-}/.cargo/bin" \
    "/c/Users/${USERNAME:-}/.cargo/bin" \
    "/c/Users/${USER:-}/.cargo/bin" \
    "C:\\Users\\${USERNAME:-}\\.cargo\\bin" \
    "C:\\Users\\${USER:-}\\.cargo\\bin"
do
    if [[ -n "$dir" && -d "$dir" && ( -f "$dir/cargo.exe" || -f "$dir/cargo" ) ]]; then
        export PATH="$dir:$PATH"
        break
    fi
done

if read -t 0; then
    INPUT=$(cat)
else
    INPUT="{}"
fi
TRANSCRIPT_PATH=$(echo "$INPUT" | jq -r '.transcript_path // empty' 2>/dev/null)

if [[ -z "$TRANSCRIPT_PATH" || ! -f "$TRANSCRIPT_PATH" ]]; then
    echo '{}'
    exit 0
fi

WORKSPACE_SLUG=$(basename "$PWD")
MANIFEST_PATH="memory-viewers/memory-api/crates/session-api/Cargo.toml"

if ! cargo run --quiet --manifest-path "$MANIFEST_PATH" --bin copilot-stop-hook -- \
    --transcript-path "$TRANSCRIPT_PATH" \
    --workspace-slug "$WORKSPACE_SLUG" \
    --trigger stop >/dev/null; then
    echo "[session-capture-stop] Failed to persist transcript from $TRANSCRIPT_PATH" >&2
fi

echo '{}'
exit 0