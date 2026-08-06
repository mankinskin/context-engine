#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new ahead source" >/dev/null
worktree="$main/.worktrees/ahead-source"
submodule="$worktree/modules/example"
git -C "$submodule" config user.email test@example.invalid
git -C "$submodule" config user.name test
printf 'ahead\n' >> "$submodule/file.txt"
git -C "$submodule" commit -am ahead -q
ahead_sha=$(git -C "$submodule" rev-parse HEAD)
GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh rename ahead-source ahead-target" >/dev/null
git -C "$main/.worktrees/ahead-target/modules/example" cat-file -e "$ahead_sha"