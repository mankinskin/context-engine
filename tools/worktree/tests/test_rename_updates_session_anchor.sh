#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
script_main=$(GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && pwd -W")
mkdir -p "$main/.session-routing"
printf '{"anchor":{"worktree_path":"%s","branch":"agent/anchor-old","status":"Active"}}\n' "$script_main/.worktrees/anchor-old" > "$main/.session-routing/worktree-index.json"
git -C "$main" add .session-routing/worktree-index.json
git -C "$main" commit -qm 'add session anchor'
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new anchor old" >/dev/null
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh rename anchor-old anchor-new" >/dev/null
grep -F "$script_main/.worktrees/anchor-new" "$main/.session-routing/worktree-index.json"
! grep -F "$script_main/.worktrees/anchor-old" "$main/.session-routing/worktree-index.json"