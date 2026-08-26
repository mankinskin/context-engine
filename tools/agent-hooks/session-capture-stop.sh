#!/bin/bash
# Session sync hook for persisting agent chat transcripts through session-api
#
# Canonical location: tools/agent-hooks/session-capture-stop.sh
# Referenced by: .github/hooks/hooks.json, .clinerules/hooks/hooks.json

set -euo pipefail

HOOK_BIN="${SESSION_CAPTURE_HOOK_BIN:-session-capture-hook}"

if [[ -p /dev/stdin || ! -t 0 ]]; then
    INPUT="$(cat)"
else
    INPUT="{}"
fi

HOOK_ARGS=(--from-hook-stdin)
if [[ -n "${SESSION_CAPTURE_STORE_ROOT:-}" ]]; then
    HOOK_ARGS=(--store-root "$SESSION_CAPTURE_STORE_ROOT" "${HOOK_ARGS[@]}")
fi

if ! command -v "$HOOK_BIN" >/dev/null 2>&1; then
    printf 'error: %s is not installed; run the commit-pinned workflow-tools install.sh bootstrap first\n' "$HOOK_BIN" >&2
    exit 1
fi

printf '%s' "$INPUT" | "$HOOK_BIN" "${HOOK_ARGS[@]}"
