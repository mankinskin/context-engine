#!/bin/bash
# Session sync hook for persisting agent chat transcripts through session-api
#
# Canonical location: tools/agent-hooks/session-capture-stop.sh
# Referenced by: .github/hooks/hooks.json, .clinerules/hooks/hooks.json

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MANIFEST_PATH="${SESSION_CAPTURE_MANIFEST_PATH:-$REPO_ROOT/memory-api/crates/session-api/Cargo.toml}"
CARGO_BIN="${SESSION_CAPTURE_CARGO_BIN:-cargo}"

swap_drive_prefix_if_missing() {
    local p="$1"
    if [[ "$p" =~ ^/mnt/([a-zA-Z])/(.*)$ ]]; then
        local alt="/${BASH_REMATCH[1],,}/${BASH_REMATCH[2]}"
        if [[ ! -e "$p" && -e "$alt" ]]; then
            echo "$alt"
            return
        fi
    elif [[ "$p" =~ ^/([a-zA-Z])/(.*)$ ]]; then
        local alt="/mnt/${BASH_REMATCH[1],,}/${BASH_REMATCH[2]}"
        if [[ ! -e "$p" && -e "$alt" ]]; then
            echo "$alt"
            return
        fi
    fi
    echo "$p"
}

MANIFEST_PATH="$(swap_drive_prefix_if_missing "$MANIFEST_PATH")"

if [[ "$CARGO_BIN" =~ ^[A-Za-z]:[\\/].* ]]; then
    if command -v cygpath >/dev/null 2>&1; then
        CARGO_BIN="$(cygpath -u "$CARGO_BIN" 2>/dev/null || echo "$CARGO_BIN")"
    else
        CARGO_BIN="${CARGO_BIN//\\//}"
        drive="${CARGO_BIN:0:1}"
        rest="${CARGO_BIN:2}"
        rest="${rest#/}"
        if [[ -d "/${drive,,}" ]]; then
            CARGO_BIN="/${drive,,}/$rest"
        elif [[ -d "/mnt/${drive,,}" ]]; then
            CARGO_BIN="/mnt/${drive,,}/$rest"
        fi
    fi
fi

CARGO_BIN="$(swap_drive_prefix_if_missing "$CARGO_BIN")"

if [[ "$CARGO_BIN" =~ \.exe$ && "$MANIFEST_PATH" =~ ^/([a-zA-Z]|mnt/[a-zA-Z])/ ]]; then
    if command -v cygpath >/dev/null 2>&1; then
        MANIFEST_PATH="$(cygpath -m "$MANIFEST_PATH" 2>/dev/null || echo "$MANIFEST_PATH")"
    elif [[ "$MANIFEST_PATH" =~ ^/mnt/([a-zA-Z])/(.*)$ ]]; then
        MANIFEST_PATH="${BASH_REMATCH[1]^}:/${BASH_REMATCH[2]}"
    elif [[ "$MANIFEST_PATH" =~ ^/([a-zA-Z])/(.*)$ ]]; then
        MANIFEST_PATH="${BASH_REMATCH[1]^}:/${BASH_REMATCH[2]}"
    fi
fi

if [[ -p /dev/stdin || ! -t 0 ]]; then
    INPUT="$(cat)"
else
    INPUT="{}"
fi

HOOK_ARGS=(--from-hook-stdin)
if [[ -n "${SESSION_CAPTURE_STORE_ROOT:-}" ]]; then
    HOOK_ARGS=(--store-root "$SESSION_CAPTURE_STORE_ROOT" "${HOOK_ARGS[@]}")
fi

printf '%s' "$INPUT" | "$CARGO_BIN" run --quiet --manifest-path "$MANIFEST_PATH" --bin copilot-capture-hook -- "${HOOK_ARGS[@]}"
