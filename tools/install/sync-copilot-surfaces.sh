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
# Plain copies are used instead of symlinks: creating a symlink on Windows
# requires Developer Mode or an elevated shell, which is not guaranteed to
# be available. Copies are portable across every supported OS.
#
# The generated `.github/agents/`, `.github/prompts/`, and
# `.github/instructions/` directories are derived build output: gitignored,
# fully regenerated (not merged) on every run, and must never be hand-edited
# — edit the corresponding file under `.agents/` instead.
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
    mkdir -p "$dest"
    cp -R "$src/." "$dest/"
    printf 'sync-copilot-surfaces: %s -> %s\n' "$src" "$dest"
}

sync_dir "$repo_root/.agents/agents" "$repo_root/.github/agents"
sync_dir "$repo_root/.agents/prompts" "$repo_root/.github/prompts"
sync_dir "$repo_root/.agents/instructions" "$repo_root/.github/instructions"
