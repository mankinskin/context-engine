#!/usr/bin/env bash
set -euo pipefail

ROOT=$1
source "$ROOT/tools/worktree/tests/common.sh"
fixture=$(fixture_repo "$ROOT")
trap 'rm -rf "$fixture"' EXIT

plan=$(cd "$fixture/main" && bash tools/worktree/worktree.sh new dryrun plan --dry-run)
printf '%s\n' "$plan"
! grep -Eq 'fetch[[:space:]]+origin|origin/main' <<<"$plan"