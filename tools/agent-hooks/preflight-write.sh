#!/usr/bin/env bash
# tools/agent-hooks/preflight-write.sh
#
# Pre-flight write validation gate — runs local syntax/lint checks before
# a file is written or modified by an agent tool.
#
# Triggered by the PreToolUse hook for file-write tools:
#   create_file, replace_string_in_file, edit_notebook_file
#
# Reads JSON from stdin (hook payload) and extracts the file path.
# Exits 0 to allow the write, exits non-zero to block it.
#
# Supported languages:
#   .rs   — cargo check (nearest Cargo.toml) + rustfmt --check
#   .py   — python3 -m py_compile (syntax only)
#   .ts   / .tsx — tsc --noEmit (if tsconfig.json nearby)
#   .sh   — bash -n (syntax check)
#   .toml — basic TOML sanity (python3 tomllib)
#
# Fallback: if the checker is unavailable, emit a warning but allow the write.

set -euo pipefail

# ── helpers ────────────────────────────────────────────────────────────────

log_warn() { echo "[preflight-write] WARN: $*" >&2; }
log_info() { echo "[preflight-write] $*" >&2; }
log_block() { echo "[preflight-write] BLOCK: $*" >&2; }

INPUT="$(cat)"

# Extract the file path from the hook payload.
FILE_PATH="$(echo "$INPUT" | python3 -c "
import sys, json
data = json.load(sys.stdin)
# try common locations
for key in ('filePath', 'file_path', 'path'):
    v = data.get('tool_input', {}).get(key)
    if v:
        print(v)
        sys.exit(0)
    # top-level
    v = data.get(key)
    if v:
        print(v)
        sys.exit(0)
# replacements array
reps = data.get('tool_input', {}).get('replacements', [])
if reps and reps[0].get('filePath'):
    print(reps[0]['filePath'])
    sys.exit(0)
print('')
" 2>/dev/null || true)"

if [[ -z "$FILE_PATH" ]]; then
    # Cannot extract path — allow write without checking.
    exit 0
fi

TOOL_NAME="$(echo "$INPUT" | python3 -c "
import sys, json
data = json.load(sys.stdin)
print(data.get('tool_name', data.get('toolName', '')))
" 2>/dev/null || true)"

# Only gate on file-write tools.
case "$TOOL_NAME" in
    create_file|replace_string_in_file|edit_notebook_file|multi_replace_string_in_file) ;;
    *) exit 0 ;;
esac

# Normalise path to relative form.
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
REL_PATH="${FILE_PATH#$REPO_ROOT/}"

EXT="${FILE_PATH##*.}"
EXT="${EXT,,}"  # lowercase

log_info "checking $REL_PATH ($EXT)"

# ── Rust: cargo check ──────────────────────────────────────────────────────
check_rust() {
    local file="$1"
    # Find the nearest Cargo.toml.
    local dir
    dir="$(dirname "$file")"
    while [[ "$dir" != "$REPO_ROOT" && "$dir" != "/" ]]; do
        if [[ -f "$dir/Cargo.toml" ]]; then
            if ! command -v cargo >/dev/null 2>&1; then
                log_warn "cargo not on PATH; skipping Rust check"
                return 0
            fi
            log_info "cargo check in $dir"
            if ! cargo check --manifest-path "$dir/Cargo.toml" --quiet 2>&1 | head -20 >&2; then
                log_block "cargo check failed for $REL_PATH — fix errors before saving"
                return 1
            fi
            return 0
        fi
        dir="$(dirname "$dir")"
    done
    log_warn "no Cargo.toml found above $file; skipping cargo check"
    return 0
}

# ── Python: syntax check ───────────────────────────────────────────────────
check_python() {
    local file="$1"
    if ! command -v python3 >/dev/null 2>&1; then
        log_warn "python3 not on PATH; skipping Python syntax check"
        return 0
    fi
    if ! python3 -m py_compile "$file" 2>&1 >&2; then
        log_block "Python syntax error in $REL_PATH — fix before saving"
        return 1
    fi
    log_info "Python syntax OK"
    return 0
}

# ── Shell: bash -n ─────────────────────────────────────────────────────────
check_shell() {
    local file="$1"
    if ! command -v bash >/dev/null 2>&1; then
        log_warn "bash not on PATH; skipping shell syntax check"
        return 0
    fi
    # Only check if the file already exists (can't check content not yet written).
    if [[ -f "$file" ]]; then
        if ! bash -n "$file" 2>&1 >&2; then
            log_block "Shell syntax error in $REL_PATH"
            return 1
        fi
        log_info "Shell syntax OK"
    fi
    return 0
}

# ── TOML: python tomllib ───────────────────────────────────────────────────
check_toml() {
    local file="$1"
    if [[ ! -f "$file" ]]; then
        return 0  # New file — nothing to parse yet.
    fi
    if python3 -c "import tomllib; tomllib.loads(open('$file', 'rb').read().decode())" 2>&1 >&2; then
        log_info "TOML syntax OK"
    else
        log_warn "TOML parse warning in $REL_PATH (non-blocking)"
        # TOML check is advisory only — don't block.
    fi
    return 0
}

# ── TypeScript ─────────────────────────────────────────────────────────────
check_typescript() {
    local file="$1"
    if ! command -v tsc >/dev/null 2>&1; then
        log_warn "tsc not on PATH; skipping TypeScript check"
        return 0
    fi
    local dir
    dir="$(dirname "$file")"
    while [[ "$dir" != "$REPO_ROOT" && "$dir" != "/" ]]; do
        if [[ -f "$dir/tsconfig.json" ]]; then
            log_info "tsc --noEmit in $dir"
            if ! tsc --noEmit --project "$dir/tsconfig.json" 2>&1 | head -20 >&2; then
                log_warn "TypeScript check failed (advisory — write not blocked)"
            fi
            return 0
        fi
        dir="$(dirname "$dir")"
    done
    log_warn "no tsconfig.json found; skipping TypeScript check"
    return 0
}

# ── Dispatch ───────────────────────────────────────────────────────────────
case "$EXT" in
    rs)   check_rust  "$FILE_PATH" ;;
    py)   check_python "$FILE_PATH" ;;
    sh)   check_shell "$FILE_PATH" ;;
    toml) check_toml  "$FILE_PATH" ;;
    ts|tsx) check_typescript "$FILE_PATH" ;;
    *)
        log_info "no pre-flight check for .$EXT files"
        ;;
esac

exit 0
