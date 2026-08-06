#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
SCRIPT="$ROOT/tools/worktree/worktree.sh"

bash -n "$SCRIPT"

if awk '!/^[[:space:]]*#/ && /fetch[[:space:]]+origin|origin\/main/ { exit 1 }' "$SCRIPT"; then
    :
else
    printf 'functional origin reference found\n' >&2
    exit 1
fi

if grep -q 'submodule deinit' "$SCRIPT"; then
    printf 'submodule deinit reference found\n' >&2
    exit 1
fi

if grep -q 'worktree move' "$SCRIPT"; then
    printf 'worktree move reference found\n' >&2
    exit 1
fi