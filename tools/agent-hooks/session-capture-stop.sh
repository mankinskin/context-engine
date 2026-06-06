#!/bin/bash
# Stop hook for persisting agent chat transcripts through session-api
#
# Canonical location: tools/agent-hooks/session-capture-stop.sh
# Referenced by: .github/hooks/hooks.json, .clinerules/hooks/hooks.json

set -uo pipefail

INPUT=$(cat)
TRANSCRIPT_PATH=$(echo "$INPUT" | jq -r '.transcript_path // empty' 2>/dev/null)

if [[ -z "$TRANSCRIPT_PATH" || ! -f "$TRANSCRIPT_PATH" ]]; then
    echo '{}'
    exit 0
fi

WORKSPACE_SLUG=$(basename "$PWD")
MANIFEST_PATH="memory-viewers/memory-api/crates/session-api/Cargo.toml"
# The session store lives inside the memory-api submodule. Anchor it relative to
# the tool execution root so running from the repository root reuses the nested
# store instead of creating a duplicate .memory-api directory at the root.
STORE_ROOT="$PWD/memory-viewers/memory-api/.memory-api"

if ! cargo run --quiet --manifest-path "$MANIFEST_PATH" --bin copilot-stop-hook -- \
    --transcript-path "$TRANSCRIPT_PATH" \
    --store-root "$STORE_ROOT" \
    --workspace-slug "$WORKSPACE_SLUG" \
    --trigger stop >/dev/null; then
    echo "[session-capture-stop] Failed to persist transcript from $TRANSCRIPT_PATH" >&2
fi

echo '{}'
exit 0