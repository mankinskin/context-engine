#!/usr/bin/env bash
set -euo pipefail

if ! awk '!/^[[:space:]]*#/ && /fetch[[:space:]]+origin|origin\/main/ { exit 1 }' "$1/tools/worktree/worktree.sh"; then
    printf 'functional origin reference found\n' >&2
    exit 1
fi