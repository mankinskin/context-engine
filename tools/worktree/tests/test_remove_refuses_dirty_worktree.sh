#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new remove dirty" >/dev/null
worktree="$main/.worktrees/remove-dirty"
printf 'do not lose me\n' > "$worktree/dirty.txt"
if GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh remove remove-dirty" >"$fixture/remove.out" 2>&1; then
    fail 'remove should refuse a dirty worktree without --force'
fi
grep -F 'dirty.txt' "$fixture/remove.out"
[[ -f "$worktree/dirty.txt" ]]
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh remove remove-dirty --force" >/dev/null
[[ ! -e "$worktree" ]]