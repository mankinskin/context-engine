#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
before=$(git -C "$main" rev-parse main)
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new dry run --dry-run" >/dev/null
[[ ! -e "$main/.worktrees/dry-run" ]]
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new dry run" >/dev/null
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh rebase dry-run --dry-run" >/dev/null
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh rename dry-run dry-renamed --dry-run" >/dev/null
[[ -d "$main/.worktrees/dry-run" ]]
[[ ! -e "$main/.worktrees/dry-renamed" ]]
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh remove dry-run --dry-run" >/dev/null
[[ -d "$main/.worktrees/dry-run" ]]
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh finish dry-run --dry-run" >/dev/null
[[ -d "$main/.worktrees/dry-run" ]]
[[ "$before" == "$(git -C "$main" rev-parse main)" ]]