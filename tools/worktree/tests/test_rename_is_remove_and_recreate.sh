#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new rename source" >/dev/null
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh rename rename-source rename-target" >/dev/null
worktree="$main/.worktrees/rename-target"
sha=$(recorded_sha "$worktree" modules/example)
[[ ! -e "$main/.worktrees/rename-source" ]]
git -C "$worktree/modules/example" cat-file -e "$sha"