#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new finish ready" >/dev/null
worktree="$main/.worktrees/finish-ready"
printf 'completed\n' > "$worktree/completed.txt"
git -C "$worktree" add completed.txt
git -C "$worktree" commit -qm completed
before=$(git -C "$main" rev-parse main)
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh finish finish-ready" >"$fixture/finish.out" 2>&1
after=$(git -C "$main" rev-parse main)
[[ "$before" == "$after" ]]
[[ ! -e "$worktree" ]]
git -C "$main" branch --list 'agent/finish-ready' | grep -F 'agent/finish-ready'
grep -F 'ready-to-merge' "$fixture/finish.out"