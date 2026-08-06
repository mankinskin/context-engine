#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
sha=$(recorded_sha "$main" modules/example)
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new offline bootstrap" >/dev/null
worktree="$main/.worktrees/offline-bootstrap"
[[ -f "$worktree/modules/example/file.txt" ]]
git -C "$worktree/modules/example" cat-file -e "$sha"