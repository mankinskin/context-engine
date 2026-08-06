#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new duplicate first" >/dev/null
if GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new duplicate second" >"$fixture/duplicate.out" 2>&1; then
    fail 'a second session worktree should require an explicit override'
fi
grep -F -- '--allow-additional' "$fixture/duplicate.out"
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new duplicate second --allow-additional" >/dev/null
[[ -d "$main/.worktrees/duplicate-first" ]]
[[ -d "$main/.worktrees/duplicate-second" ]]