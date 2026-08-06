#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
printf 'dirty main change\n' >> "$main/README"

if GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new preserve dirty" >"$fixture/create.out" 2>&1; then
    fail 'creation silently proceeded with dirty main checkout'
fi
grep -F 'README' "$fixture/create.out"
[[ ! -e "$main/.worktrees/preserve-dirty" ]]

GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new preserve dirty --preserve-main-changes" >/dev/null
git -C "$main" stash list | grep -F 'worktree.sh preserve-main-changes'
git -C "$main" stash pop -q
grep -F 'dirty main change' "$main/README"