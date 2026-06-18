#!/bin/bash
# Stop hook for persisting agent chat transcripts through session-api
#
# Canonical location: tools/agent-hooks/session-capture-stop.sh
# Referenced by: .github/hooks/hooks.json, .clinerules/hooks/hooks.json

set -uo pipefail

CARGO_BIN=""
for candidate in \
    "${HOME:-}/.cargo/bin/cargo.exe" \
    "${HOME:-}/.cargo/bin/cargo" \
    "${USERPROFILE:-}\\.cargo\\bin\\cargo.exe" \
    "${USERPROFILE:-}/.cargo/bin/cargo.exe" \
    "/c/Users/${USERNAME:-}/.cargo/bin/cargo.exe" \
    "/c/Users/${USER:-}/.cargo/bin/cargo.exe"
do
    if [[ -n "$candidate" && -f "$candidate" ]]; then
        CARGO_BIN="$candidate"
        break
    fi
done

if [[ -z "$CARGO_BIN" ]]; then
    CARGO_BIN="$(command -v cargo 2>/dev/null || true)"
fi

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
MANIFEST_PATH="memory-api/crates/session-api/Cargo.toml"

if [[ -n "$CARGO_BIN" ]] && ! "$CARGO_BIN" run --quiet --manifest-path "$MANIFEST_PATH" --bin copilot-stop-hook -- \
    --transcript-path "$TRANSCRIPT_PATH" \
    --workspace-slug "$WORKSPACE_SLUG" \
    --trigger stop >/dev/null; then
    echo "[session-capture-stop] Failed to persist transcript from $TRANSCRIPT_PATH" >&2
fi

echo '{}'
exit 0