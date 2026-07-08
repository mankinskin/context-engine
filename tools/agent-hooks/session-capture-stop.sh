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
TRANSCRIPT_PATH=$(echo "$INPUT" | jq -r '(.transcript_path // .transcriptPath // empty)' 2>/dev/null)
HOOK_EVENT_NAME=$(echo "$INPUT" | jq -r '(.hook_event_name // .hookEventName // "unknown")' 2>/dev/null)
SESSION_ID=$(echo "$INPUT" | jq -r '(.session_id // .sessionId // "unknown")' 2>/dev/null)

# VS Code can provide Windows-style paths (c:\\Users\\...) even under bash.
# Normalize to a POSIX path so file existence checks and cargo args work.
if [[ -n "$TRANSCRIPT_PATH" ]] && command -v cygpath >/dev/null 2>&1; then
    TRANSCRIPT_PATH="$(cygpath -u "$TRANSCRIPT_PATH" 2>/dev/null || echo "$TRANSCRIPT_PATH")"
fi

if [[ -z "$TRANSCRIPT_PATH" || ! -f "$TRANSCRIPT_PATH" ]]; then
    echo "[session-capture-stop] skip: transcript not found (event=$HOOK_EVENT_NAME session=$SESSION_ID path=$TRANSCRIPT_PATH)" >&2
    echo '{}'
    exit 0
fi

WORKSPACE_SLUG=$(echo "$INPUT" | jq -r '(.workspace_slug // .workspaceSlug // "default")' 2>/dev/null)
if [[ -z "$WORKSPACE_SLUG" || "$WORKSPACE_SLUG" == "null" ]]; then
    WORKSPACE_SLUG="default"
fi
TRIGGER="${HOOK_EVENT_NAME:-stop}"
if [[ -z "$TRIGGER" || "$TRIGGER" == "unknown" || "$TRIGGER" == "null" ]]; then
    TRIGGER="stop"
fi
MANIFEST_PATH="memory-api/crates/session-api/Cargo.toml"
STORE_ROOT=".session"

if [[ -n "$CARGO_BIN" ]] && ! "$CARGO_BIN" run --quiet --manifest-path "$MANIFEST_PATH" --bin copilot-stop-hook -- \
    --transcript-path "$TRANSCRIPT_PATH" \
    --store-root "$STORE_ROOT" \
    --workspace-slug "$WORKSPACE_SLUG" \
    --trigger "$TRIGGER" >/dev/null; then
    echo "[session-capture-stop] Failed to persist transcript from $TRANSCRIPT_PATH" >&2
    exit 2
fi

if [[ -z "$CARGO_BIN" ]]; then
    echo "[session-capture-stop] Failed: cargo binary not found" >&2
    exit 2
fi

echo '{}'
exit 0