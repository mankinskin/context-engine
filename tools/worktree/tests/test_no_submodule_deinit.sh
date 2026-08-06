#!/usr/bin/env bash
set -euo pipefail

if grep -q 'submodule deinit' "$1/tools/worktree/worktree.sh"; then
    printf 'submodule deinit reference found\n' >&2
    exit 1
fi