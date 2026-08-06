#!/usr/bin/env bash
set -euo pipefail

if grep -q 'worktree move' "$1/tools/worktree/worktree.sh"; then
    printf 'worktree move reference found\n' >&2
    exit 1
fi