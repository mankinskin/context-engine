#!/bin/bash
# Stop hook for persisting Copilot chat transcripts through session-api

set -uo pipefail

INPUT=$(cat)
TRANSCRIPT_PATH=$(echo "$INPUT" | jq -r '.transcript_path // empty' 2>/dev/null)

if [[ -z "$TRANSCRIPT_PATH" || ! -f "$TRANSCRIPT_PATH" ]]; then
    echo '{}'
    exit 0
fi

WORKSPACE_SLUG=$(basename "$PWD")
STORE_ROOT="memory-viewers/memory-api/.memory-api"
MANIFEST_PATH="memory-viewers/memory-api/crates/session-api/Cargo.toml"

if ! cargo run --quiet --manifest-path "$MANIFEST_PATH" --bin copilot-stop-hook -- \
    --transcript-path "$TRANSCRIPT_PATH" \
    --store-root "$STORE_ROOT" \
    --workspace-slug "$WORKSPACE_SLUG" \
    --trigger stop >/dev/null; then
    echo "[session-capture-stop] Failed to persist transcript from $TRANSCRIPT_PATH" >&2
fi

echo '{}'
exit 0