#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
first=$(GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new reuse session" | sed -n 's/^WORKTREE_PATH=//p')
second=$(GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new reuse session" | sed -n 's/^WORKTREE_PATH=//p')
[[ "$first" == "$second" ]]
[[ $(find "$main/.worktrees" -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d ' ') == 1 ]]