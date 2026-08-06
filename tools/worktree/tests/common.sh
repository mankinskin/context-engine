#!/usr/bin/env bash
set -euo pipefail

fail() {
    printf '%s\n' "$*" >&2
    exit 1
}

fixture_repo() {
    local root="$1" fixture source
    fixture=$(mktemp -d)
    source="$fixture/submodule-source"

    git init -q --initial-branch=main "$source"
    git -C "$source" config user.email test@example.invalid
    git -C "$source" config user.name test
    printf 'initial\n' > "$source/file.txt"
    git -C "$source" add file.txt
    git -C "$source" commit -qm initial

    git init -q --initial-branch=main "$fixture/main"
    git -C "$fixture/main" config user.email test@example.invalid
    git -C "$fixture/main" config user.name test
    printf 'fixture\n' > "$fixture/main/README"
    git -C "$fixture/main" add README
    git -C "$fixture/main" commit -qm initial
    git -C "$fixture/main" -c protocol.file.allow=always submodule add -q "$source" modules/example
    git -C "$fixture/main" commit -am 'add submodule' -q

    mkdir -p "$fixture/main/tools/worktree"
    cp "$root/tools/worktree/worktree.sh" "$fixture/main/tools/worktree/worktree.sh"
    chmod +x "$fixture/main/tools/worktree/worktree.sh"
    git -C "$fixture/main" add tools/worktree/worktree.sh
    git -C "$fixture/main" commit -qm 'add worktree helper'
    printf '%s\n' "$fixture"
}

recorded_sha() {
    git -C "$1" ls-tree HEAD -- "$2" | awk '{print $3}'
}