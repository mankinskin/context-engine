#!/bin/bash
# Stop hook for persisting agent chat transcripts through session-api
#
# Canonical location: tools/agent-hooks/session-capture-stop.sh
# Referenced by: .github/hooks/hooks.json, .clinerules/hooks/hooks.json

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DEFAULT_SESSION_ROOT="$REPO_ROOT/.session/sessions"
TRACE_LOG_PATH=""

log_trace() {
    local level="$1"
    local message="$2"
    local target_log="${TRACE_LOG_PATH:-${SESSION_CAPTURE_TRACE_LOG:-$DEFAULT_SESSION_ROOT/unknown/session-capture-stop.log}}"
    mkdir -p "$(dirname "$target_log")" 2>/dev/null || true
    printf '%s [%s] %s\n' "$(date -u +"%Y-%m-%dT%H:%M:%SZ")" "$level" "$message" >>"$target_log" 2>/dev/null || true
}

sanitize_session_id_for_filename() {
    local raw="$1"
    raw="${raw//[^A-Za-z0-9._-]/_}"
    raw="${raw#_}"
    raw="${raw%_}"
    if [[ -z "$raw" ]]; then
        raw="unknown"
    fi
    echo "$raw"
}

normalize_path() {
    local p="$1"
    p="${p//\\//}"
    if [[ "$p" =~ ^[A-Za-z]:/ ]]; then
        local drive="${p:0:1}"
        if [[ -d "/${drive,,}" ]]; then
            p="/${drive,,}${p:2}"
        elif [[ -d "/mnt/${drive,,}" ]]; then
            p="/mnt/${drive,,}${p:2}"
        else
            p="/${drive,,}${p:2}"
        fi
    fi
    echo "$p"
}

maybe_swap_drive_prefix() {
    local p="$1"
    if [[ "$p" =~ ^/([a-z])/(.*)$ ]]; then
        local alt="/mnt/${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
        if [[ -e "$alt" ]]; then
            echo "$alt"
            return
        fi
    elif [[ "$p" =~ ^/mnt/([a-z])/(.*)$ ]]; then
        local alt="/${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
        if [[ -e "$alt" ]]; then
            echo "$alt"
            return
        fi
    fi
    echo "$p"
}

path_for_cargo_arg() {
    local p="$1"
    # When a Windows cargo.exe is launched from a WSL-style runtime, convert
    # Linux mount paths so cargo.exe can resolve them.
    if [[ -n "$CARGO_BIN" && "$CARGO_BIN" =~ \.exe$ && "$p" =~ ^/mnt/[a-z]/ ]]; then
        if command -v wslpath >/dev/null 2>&1; then
            wslpath -w "$p" 2>/dev/null || echo "$p"
            return
        fi
    fi
    echo "$p"
}

CARGO_BIN=""
for candidate in \
    "${SESSION_CAPTURE_CARGO_BIN:-}" \
    "${HOME:-}/.cargo/bin/cargo.exe" \
    "${HOME:-}/.cargo/bin/cargo" \
    "${USERPROFILE:-}\\.cargo\\bin\\cargo.exe" \
    "${USERPROFILE:-}/.cargo/bin/cargo.exe" \
    "/c/Users/${USERNAME:-}/.cargo/bin/cargo.exe" \
    "/c/Users/${USER:-}/.cargo/bin/cargo.exe" \
    "/mnt/c/Users/${USERNAME:-}/.cargo/bin/cargo.exe" \
    "/mnt/c/Users/${USER:-}/.cargo/bin/cargo.exe" \
    "cargo"
do
    [[ -z "$candidate" ]] && continue

    candidate_norm="$candidate"
    if [[ "$candidate_norm" =~ ^[A-Za-z]:[\\/].* ]]; then
        candidate_norm="$(normalize_path "$candidate_norm")"
    fi

    if [[ -f "$candidate_norm" ]]; then
        CARGO_BIN="$candidate_norm"
        break
    fi

    resolved_candidate="$(command -v "$candidate_norm" 2>/dev/null || true)"
    if [[ -n "$resolved_candidate" ]]; then
        CARGO_BIN="$resolved_candidate"
        break
    fi
done

if [[ -z "$CARGO_BIN" && -n "${CARGO:-}" ]]; then
    CARGO_BIN="$CARGO"
fi

if [[ -n "$CARGO_BIN" && "$CARGO_BIN" =~ ^[A-Za-z]:[\\/].* ]]; then
    CARGO_BIN="$(normalize_path "$CARGO_BIN")"
fi

if [[ -z "$CARGO_BIN" ]]; then
    CARGO_BIN="$(command -v cargo 2>/dev/null || true)"
fi

log_trace "INFO" "cargo candidate resolved path=${CARGO_BIN:-<empty>}"

if [[ -p /dev/stdin || ! -t 0 ]]; then
    INPUT=$(cat)
else
    INPUT="{}"
fi

log_trace "INFO" "stdin captured bytes=${#INPUT}"

if command -v jq >/dev/null 2>&1; then
    TRANSCRIPT_PATH=$(echo "$INPUT" | jq -r '(.transcript_path // .transcriptPath // empty)' 2>/dev/null)
    HOOK_EVENT_NAME=$(echo "$INPUT" | jq -r '(.hook_event_name // .hookEventName // "unknown")' 2>/dev/null)
    SESSION_ID=$(echo "$INPUT" | jq -r '(.session_id // .sessionId // "unknown")' 2>/dev/null)
    WORKSPACE_SLUG=$(echo "$INPUT" | jq -r '(.workspace_slug // .workspaceSlug // "default")' 2>/dev/null)
elif command -v python3 >/dev/null 2>&1; then
    TRANSCRIPT_PATH=$(echo "$INPUT" | python3 -c 'import json,sys; d=json.load(sys.stdin); print(d.get("transcript_path") or d.get("transcriptPath") or "")' 2>/dev/null)
    HOOK_EVENT_NAME=$(echo "$INPUT" | python3 -c 'import json,sys; d=json.load(sys.stdin); print(d.get("hook_event_name") or d.get("hookEventName") or "unknown")' 2>/dev/null)
    SESSION_ID=$(echo "$INPUT" | python3 -c 'import json,sys; d=json.load(sys.stdin); print(d.get("session_id") or d.get("sessionId") or "unknown")' 2>/dev/null)
    WORKSPACE_SLUG=$(echo "$INPUT" | python3 -c 'import json,sys; d=json.load(sys.stdin); print(d.get("workspace_slug") or d.get("workspaceSlug") or "default")' 2>/dev/null)
else
    TRANSCRIPT_PATH=""
    HOOK_EVENT_NAME="unknown"
    SESSION_ID="unknown"
    WORKSPACE_SLUG="default"
fi

# Prefer explicit override; otherwise write one log file per session.
if [[ -n "${SESSION_CAPTURE_TRACE_LOG:-}" ]]; then
    TRACE_LOG_PATH="$SESSION_CAPTURE_TRACE_LOG"
else
    SESSION_LOG_ID="$SESSION_ID"
    if [[ -z "$SESSION_LOG_ID" || "$SESSION_LOG_ID" == "unknown" || "$SESSION_LOG_ID" == "null" ]]; then
        session_from_name="$(basename "$TRANSCRIPT_PATH")"
        session_from_name="${session_from_name%.jsonl}"
        if [[ -n "$session_from_name" && "$session_from_name" != "$TRANSCRIPT_PATH" ]]; then
            SESSION_LOG_ID="$session_from_name"
        fi
    fi
    SESSION_LOG_ID="$(sanitize_session_id_for_filename "$SESSION_LOG_ID")"
    TRACE_LOG_PATH="$DEFAULT_SESSION_ROOT/$SESSION_LOG_ID/session-capture-stop.log"
fi

log_trace "INFO" "hook start pwd=$PWD"
log_trace "INFO" "selected trace log path=$TRACE_LOG_PATH"

if [[ -n "$TRANSCRIPT_PATH" ]]; then
    if command -v cygpath >/dev/null 2>&1; then
        TRANSCRIPT_PATH="$(cygpath -u "$TRANSCRIPT_PATH" 2>/dev/null || echo "$TRANSCRIPT_PATH")"
    else
        TRANSCRIPT_PATH="$(normalize_path "$TRANSCRIPT_PATH")"
    fi
    TRANSCRIPT_PATH="$(maybe_swap_drive_prefix "$TRANSCRIPT_PATH")"
fi

log_trace "INFO" "parsed payload event=$HOOK_EVENT_NAME session=$SESSION_ID workspace=$WORKSPACE_SLUG transcript_path=$TRANSCRIPT_PATH"

if [[ -z "$TRANSCRIPT_PATH" || ! -f "$TRANSCRIPT_PATH" ]]; then
    if [[ -n "$TRANSCRIPT_PATH" ]]; then
        log_trace "WARN" "transcript missing path=$TRANSCRIPT_PATH"
    else
        log_trace "WARN" "transcript path empty in hook payload"
    fi
    echo "[session-capture-stop] skip: transcript not found (event=$HOOK_EVENT_NAME session=$SESSION_ID path=$TRANSCRIPT_PATH)" >&2
    echo '{}'
    exit 0
fi

log_trace "INFO" "transcript found path=$TRANSCRIPT_PATH"

if [[ -z "$WORKSPACE_SLUG" || "$WORKSPACE_SLUG" == "null" ]]; then
    WORKSPACE_SLUG="default"
fi
TRIGGER="${HOOK_EVENT_NAME:-stop}"
if [[ -z "$TRIGGER" || "$TRIGGER" == "unknown" || "$TRIGGER" == "null" ]]; then
    TRIGGER="stop"
fi
MANIFEST_PATH="${SESSION_CAPTURE_MANIFEST_PATH:-$REPO_ROOT/memory-api/crates/session-api/Cargo.toml}"
STORE_ROOT="${SESSION_CAPTURE_STORE_ROOT:-$REPO_ROOT/.session}"

if command -v cygpath >/dev/null 2>&1; then
    MANIFEST_PATH="$(cygpath -u "$MANIFEST_PATH" 2>/dev/null || echo "$MANIFEST_PATH")"
    STORE_ROOT="$(cygpath -u "$STORE_ROOT" 2>/dev/null || echo "$STORE_ROOT")"
else
    MANIFEST_PATH="$(normalize_path "$MANIFEST_PATH")"
    STORE_ROOT="$(normalize_path "$STORE_ROOT")"
fi

MANIFEST_PATH="$(maybe_swap_drive_prefix "$MANIFEST_PATH")"
STORE_ROOT="$(maybe_swap_drive_prefix "$STORE_ROOT")"

if [[ ! -f "$MANIFEST_PATH" ]]; then
    log_trace "ERROR" "manifest missing path=$MANIFEST_PATH"
    echo "[session-capture-stop] Failed: manifest not found at $MANIFEST_PATH" >&2
    exit 2
fi

log_trace "INFO" "store_root=$STORE_ROOT manifest_path=$MANIFEST_PATH trigger=$TRIGGER"

CARGO_TRANSCRIPT_PATH="$(path_for_cargo_arg "$TRANSCRIPT_PATH")"
CARGO_STORE_ROOT="$(path_for_cargo_arg "$STORE_ROOT")"
CARGO_MANIFEST_PATH="$(path_for_cargo_arg "$MANIFEST_PATH")"

log_trace "INFO" "cargo args transcript=$CARGO_TRANSCRIPT_PATH store_root=$CARGO_STORE_ROOT manifest=$CARGO_MANIFEST_PATH"

if [[ -n "$CARGO_BIN" ]]; then
    CARGO_STDERR_FILE="$(mktemp 2>/dev/null || echo "")"
    if ! "$CARGO_BIN" run --quiet --manifest-path "$CARGO_MANIFEST_PATH" --bin copilot-stop-hook -- \
        --transcript-path "$CARGO_TRANSCRIPT_PATH" \
        --store-root "$CARGO_STORE_ROOT" \
        --workspace-slug "$WORKSPACE_SLUG" \
        --trigger "$TRIGGER" >/dev/null 2>"$CARGO_STDERR_FILE"; then
        if [[ -n "$CARGO_STDERR_FILE" && -f "$CARGO_STDERR_FILE" ]]; then
            while IFS= read -r line; do
                log_trace "ERROR" "cargo run stderr: $line"
            done <"$CARGO_STDERR_FILE"
        fi
        rm -f "$CARGO_STDERR_FILE" 2>/dev/null || true
        log_trace "ERROR" "failed to persist transcript path=$TRANSCRIPT_PATH"
        echo "[session-capture-stop] Failed to persist transcript from $TRANSCRIPT_PATH (see $TRACE_LOG_PATH)" >&2
        exit 2
    fi
    rm -f "$CARGO_STDERR_FILE" 2>/dev/null || true
else
    log_trace "ERROR" "cargo binary unresolved"
fi

if [[ -z "$CARGO_BIN" ]]; then
    echo "[session-capture-stop] Failed: cargo binary not found" >&2
    exit 2
fi

log_trace "INFO" "hook success session=$SESSION_ID transcript_path=$TRANSCRIPT_PATH"

echo '{}'
exit 0
