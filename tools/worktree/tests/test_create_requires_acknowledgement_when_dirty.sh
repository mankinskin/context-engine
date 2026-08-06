#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
printf 'unacknowledged change\n' >> "$main/README"
if GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new acknowledge dirty" >"$fixture/create.out" 2>&1; then
    fail 'creation should require acknowledgement for a dirty main checkout'
fi
grep -F 'uncommitted changes' "$fixture/create.out"
grep -F 'unacknowledged change' "$main/README"
[[ ! -e "$main/.worktrees/acknowledge-dirty" ]]