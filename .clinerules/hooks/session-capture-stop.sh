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
MANIFEST_PATH="memory-api/crates/session-api/Cargo.toml"

# Prefer the installed binary so hooks do not rebuild per invocation.
if command -v copilot-capture-hook >/dev/null 2>&1; then
    CAPTURE_CMD=(copilot-capture-hook)
else
    CAPTURE_CMD=(cargo run --quiet --manifest-path "$MANIFEST_PATH" --bin copilot-capture-hook --)
fi

if ! "${CAPTURE_CMD[@]}" \
    --transcript-path "$TRANSCRIPT_PATH" \
    --workspace-slug "$WORKSPACE_SLUG" \
    --trigger stop >/dev/null; then
    echo "[session-capture-stop] Failed to persist transcript from $TRANSCRIPT_PATH" >&2
fi

echo '{}'
exit 0