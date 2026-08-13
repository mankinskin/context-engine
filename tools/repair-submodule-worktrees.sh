#!/usr/bin/env bash
# Detects and repairs the "empty/corrupted working tree" failure mode seen
# when a submodule checkout is interrupted mid-bootstrap (e.g. by an
# earlier submodule failing fatally before this one's files are written
# out). Symptom: `git status` in the submodule shows every tracked file as
# staged-deleted, even though HEAD is at the correct, intact commit.
#
# Usage: repair-submodule-worktrees.sh [submodule-path ...]
# With no arguments, repairs every submodule listed in .gitmodules.
set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

submodule_paths=("$@")
if [[ ${#submodule_paths[@]} -eq 0 ]]; then
    while IFS= read -r path; do
        submodule_paths+=("$path")
    done < <(git config -f .gitmodules --get-regexp '\.path$' | awk '{print $2}')
fi

repaired=()
for path in "${submodule_paths[@]}"; do
    [[ -e "$path/.git" || -f "$path/.git" ]] || continue

    status_lines=$(git -C "$path" status --porcelain)
    [[ -n "$status_lines" ]] || continue

    total=$(printf '%s\n' "$status_lines" | grep -c . || true)
    # Staged deletion is "D  path" (D in column 1); unstaged deletion is
    # " D path" (D in column 2). Match either.
    deleted=$(printf '%s\n' "$status_lines" | grep -cE '^(D.| D)' || true)

    if [[ "$total" -gt 0 && "$total" == "$deleted" ]]; then
        printf 'repair-submodule-worktrees: %s has a fully-deleted working tree (%d files); running git reset --hard HEAD\n' "$path" "$total" >&2
        git -C "$path" reset --hard HEAD >/dev/null
        repaired+=("$path")
    fi
done

if [[ ${#repaired[@]} -gt 0 ]]; then
    printf 'repair-submodule-worktrees: repaired %d submodule(s): %s\n' "${#repaired[@]}" "${repaired[*]}"
fi
