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
# Each destination directory is EITHER a symlink to its `.agents/` source
# OR a read-only copy of it — never an editable copy. A symlink means edits
# under `.github/*` simply *are* edits under `.agents/`, so there is nothing
# to forget to sync back. A read-only copy means an accidental edit fails
# loudly (permission denied) instead of silently disappearing on the next
# sync. See `usage()` below for the three `--mode` choices and how the
# script picks one when none is given.
#
# Either way, the generated `.github/agents/`, `.github/prompts/`, and
# `.github/instructions/` directories are gitignored (regardless of whether
# an entry is a symlink or a real directory — see .gitignore) and must
# never be hand-edited — edit the corresponding file under `.agents/`
# instead.
set -euo pipefail

usage() {
    cat <<'EOF'
usage: sync-copilot-surfaces.sh [--mode=auto|symlink|copy]

  --mode=auto     (default) try a symlink; if it can't be created (e.g. no
                  Windows Developer Mode / elevation), print a WARNING and
                  fall back to a read-only copy instead.
  --mode=symlink  require a real symlink for every directory; ABORT the
                  whole run if any one of them can't be created.
  --mode=copy     always use a read-only copy; never attempt a symlink.

Run with no --mode in an interactive terminal to be prompted instead. Any
non-interactive caller (e.g. bootstrap.sh/init.sh) that passes no --mode
gets --mode=auto with no prompt, so automation never blocks on input.
EOF
}

mode=""
for arg in "$@"; do
    case "$arg" in
        --mode=*) mode="${arg#--mode=}" ;;
        --symlink) mode=symlink ;;
        --copy) mode=copy ;;
        -h|--help) usage; exit 0 ;;
        *)
            printf 'sync-copilot-surfaces: unknown argument %s\n' "$arg" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [[ -n "$mode" ]]; then
    case "$mode" in
        auto|symlink|copy) ;;
        *)
            printf 'sync-copilot-surfaces: invalid --mode=%s (expected auto, symlink, or copy)\n' "$mode" >&2
            exit 2
            ;;
    esac
else
    # No --mode given: ask when there is an actual human to ask (stdin AND
    # stdout are a terminal). A non-interactive caller — bootstrap.sh,
    # init.sh, CI — has neither, so it silently gets the safe "auto"
    # default instead of hanging on a read that will never be answered.
    if [[ -t 0 && -t 1 ]]; then
        printf 'sync-copilot-surfaces: how should .github/{agents,prompts,instructions} be installed?\n' >&2
        printf '  1) symlink  - edits under .github/* ARE edits under .agents/*; abort if a symlink cannot be created\n' >&2
        printf '  2) copy     - read-only copies; edits under .github/* always fail loudly instead of syncing back\n' >&2
        printf '  3) auto     - try symlink, warn and fall back to a read-only copy if unavailable (default)\n' >&2
        read -r -p 'Choice [1/2/3, default 3]: ' reply || reply=""
        case "$reply" in
            1|symlink) mode=symlink ;;
            2|copy) mode=copy ;;
            3|auto|"") mode=auto ;;
            *)
                printf 'sync-copilot-surfaces: unrecognized choice %s, defaulting to auto\n' "$reply" >&2
                mode=auto
                ;;
        esac
    else
        mode=auto
        printf 'sync-copilot-surfaces: non-interactive run, no --mode given — defaulting to --mode=auto\n' >&2
    fi
fi

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$repo_root"

sync_dir() {
    local src=$1
    local dest=$2

    if [[ ! -d "$src" ]]; then
        printf 'sync-copilot-surfaces: skipping %s -> %s (source missing)\n' "$src" "$dest" >&2
        return 0
    fi

    if [[ "$mode" == copy ]]; then
        read_only_copy "$src" "$dest"
        return 0
    fi

    rm -rf "$dest"

    # Try a real symlink. A plain success/`-d` check is not enough: on
    # Windows, MSYS `ln -s` for a directory can return success and leave
    # behind a real recursive *copy* instead of a symlink when it lacks the
    # privilege (Developer Mode / elevation) to create a native one. `-L`
    # checks the link itself (lstat), so it catches that silent fallback;
    # `-d` (which follows the link) then confirms it actually resolves.
    if ln -s "$src" "$dest" 2>/dev/null && [[ -L "$dest" && -d "$dest" ]]; then
        printf 'sync-copilot-surfaces: %s -> %s (symlink: edits under %s ARE edits under %s)\n' \
            "$src" "$dest" "$dest" "$src"
        return 0
    fi

    if [[ "$mode" == symlink ]]; then
        printf 'sync-copilot-surfaces: ABORT — --mode=symlink was requested but %s -> %s could not be created as a real symlink on this host (likely missing Windows Developer Mode / elevation).\n' \
            "$src" "$dest" >&2
        printf 'sync-copilot-surfaces: re-run with --mode=auto (falls back to a read-only copy with a warning) or --mode=copy (always uses a read-only copy).\n' >&2
        exit 1
    fi

    printf 'sync-copilot-surfaces: WARNING — could not create a symlink for %s -> %s on this host (likely missing Windows Developer Mode / elevation); falling back to a read-only copy.\n' \
        "$src" "$dest" >&2
    read_only_copy "$src" "$dest"
}

read_only_copy() {
    local src=$1
    local dest=$2

    rm -rf "$dest"
    mkdir -p "$dest"
    cp -R "$src/." "$dest/"
    chmod -R a-w "$dest"
    printf 'sync-copilot-surfaces: %s -> %s (read-only copy: edit %s, never %s)\n' \
        "$src" "$dest" "$src" "$dest"
}

printf 'sync-copilot-surfaces: mode=%s\n' "$mode" >&2

sync_dir "$repo_root/.agents/agents" "$repo_root/.github/agents"
sync_dir "$repo_root/.agents/prompts" "$repo_root/.github/prompts"
sync_dir "$repo_root/.agents/instructions" "$repo_root/.github/instructions"
