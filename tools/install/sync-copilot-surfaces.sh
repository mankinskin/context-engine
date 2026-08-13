#!/usr/bin/env bash
# Regenerates the Copilot CLI auto-discovery directories from this repo's
# canonical `.agents/` sources.
#
# Copilot CLI only auto-loads custom agents, prompt templates, and
# path-scoped instructions from `.github/agents/`, `.github/prompts/`, and
# `.github/instructions/` respectively (repo/user/org/enterprise
# precedence). This repository keeps those sources under `.agents/` as the
# single source of truth (see `.github/copilot-instructions.md`), so this
# script copies them into the `.github/*` discovery paths Copilot CLI scans.
#
# Each destination directory is a symlink to its `.agents/` source whenever
# the host allows creating one — edits under `.github/*` then simply *are*
# edits under `.agents/`, so there is nothing to forget to sync back.
# Creating a symlink on Windows requires Developer Mode or an elevated
# shell, which isn't guaranteed to be available. When symlink creation
# fails, this script falls back to a plain recursive copy and marks the
# copy read-only, so an accidental edit fails loudly (permission denied)
# instead of silently disappearing on the next sync.
#
# Either way, the generated `.github/agents/`, `.github/prompts/`, and
# `.github/instructions/` directories are gitignored (regardless of whether
# an entry is a symlink or a real directory — see .gitignore) and must
# never be hand-edited — edit the corresponding file under `.agents/`
# instead.
set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$repo_root"

sync_dir() {
    local src=$1
    local dest=$2

    if [[ ! -d "$src" ]]; then
        printf 'sync-copilot-surfaces: skipping %s -> %s (source missing)\n' "$src" "$dest" >&2
        return 0
    fi

    rm -rf "$dest"

    # Try a real symlink first. A plain success/`-d` check is not enough:
    # on Windows, MSYS `ln -s` for a directory can return success and leave
    # behind a real recursive *copy* instead of a symlink when it lacks the
    # privilege (Developer Mode / elevation) to create a native one. `-L`
    # checks the link itself (lstat), so it catches that silent fallback;
    # `-d` (which follows the link) then confirms it actually resolves.
    if ln -s "$src" "$dest" 2>/dev/null && [[ -L "$dest" && -d "$dest" ]]; then
        printf 'sync-copilot-surfaces: %s -> %s (symlink: edits under %s ARE edits under %s)\n' \
            "$src" "$dest" "$dest" "$src"
        return 0
    fi

    rm -rf "$dest"
    mkdir -p "$dest"
    cp -R "$src/." "$dest/"
    chmod -R a-w "$dest"
    printf 'sync-copilot-surfaces: %s -> %s (read-only copy: symlink unavailable on this host — edit %s, never %s)\n' \
        "$src" "$dest" "$src" "$dest"
}

sync_dir "$repo_root/.agents/agents" "$repo_root/.github/agents"
sync_dir "$repo_root/.agents/prompts" "$repo_root/.github/prompts"
sync_dir "$repo_root/.agents/instructions" "$repo_root/.github/instructions"
