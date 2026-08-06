#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

main="$fixture/main"
submodule="$main/modules/example"
printf 'main-only\n' >> "$submodule/file.txt"
git -C "$submodule" config user.email test@example.invalid
git -C "$submodule" config user.name test
git -C "$submodule" commit -am 'main-only commit' -q
git -C "$main" add modules/example
git -C "$main" commit -m 'record main-only submodule commit' -q
sha=$(recorded_sha "$main" modules/example)

GIT_ALLOW_PROTOCOL=file bash -c "cd '$main' && bash tools/worktree/worktree.sh new local object" >/dev/null
worktree="$main/.worktrees/local-object"
git -C "$worktree/modules/example" cat-file -e "$sha"
[[ $(git -C "$worktree/modules/example" rev-parse HEAD) == "$sha" ]]