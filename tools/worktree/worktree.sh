#!/usr/bin/env bash
set -euo pipefail

# Implements the agent worktree-isolation protocol described in
# .agents/instructions/commit/branch-worktree.instructions.md — this script
# is the single source of truth for the exact git sequences that protocol
# requires, so the sharp edges around submodules cannot be re-broken by hand.

DRY_RUN=0

usage() {
    cat <<'EOF'
usage: tools/worktree/worktree.sh <subcommand> [args] [--dry-run]

subcommands:
  new <short-id> <slug>   Bootstrap a worktree + branch for agent/<short-id>-<slug>,
                          cut from local main, with all submodules initialized
                          offline (no network access, no origin dependency).
  list                    List existing .worktrees/* entries: branch + submodule
                          init status for each.
  rebase <name>           Rebase <name>'s branch onto local main. No fetch.
                          Stops on conflict; never auto-resolves or aborts.
  merge <name>            Fast-forward-only merge <name>'s branch into main.
                          Fails loudly (never falls back to a real merge) if main
                          has moved past what <name> rebased onto.
  remove <name>           Full teardown of <name>: worktree remove --force,
                          worktree prune, branch -d.
  doctor                  Detect + repair deinitialized submodules in the MAIN
                          checkout (sharp edge from `submodule deinit` run inside
                          a worktree), and report stale worktree registrations.
  -h, --help              Show this usage text.

<name> is the directory name under .worktrees/, e.g. "4ef88dbc-worktree-helper".
It is accepted with or without the leading ".worktrees/".

--dry-run may be passed to any mutating subcommand (new, rebase, merge, remove,
doctor). It prints the exact commands that would run and executes none of them.

new, rebase, merge, remove, and doctor all refuse to run from inside a linked
worktree — run them from the main checkout.
EOF
}

log() { printf '%s\n' "$*" >&2; }
die() { log "error: $*"; exit 1; }

# run <cmd...> — executes, or under --dry-run just prints what would run.
run() {
    if (( DRY_RUN )); then
        printf '[dry-run] would run: %s\n' "$*"
    else
        "$@"
    fi
}

require_git() {
    command -v git >/dev/null 2>&1 || die "git not found on PATH"
}

# Absolute path of the current process's git-dir / git-common-dir. Differ iff
# the cwd is inside a linked worktree rather than the main checkout.
current_git_dir() { (cd "$(git rev-parse --git-dir)" && pwd); }
current_git_common_dir() { (cd "$(git rev-parse --git-common-dir)" && pwd); }

# Resolve the main checkout's working-tree path. `git worktree list
# --porcelain` cannot be trusted for this: when this repo is itself nested as
# a submodule, git misreports the main worktree's entry as its git-common-dir
# instead of its working tree. Instead: if we ARE the main checkout
# (git-dir == git-common-dir), our own toplevel IS the answer. Otherwise we
# are inside a linked worktree, which by this tool's own convention always
# lives at <main>/.worktrees/<name> — strip that suffix from our toplevel.
main_worktree_path() {
    local gd gcd top
    gd=$(current_git_dir)
    gcd=$(current_git_common_dir)
    if [[ "$gd" == "$gcd" ]]; then
        git rev-parse --show-toplevel
        return 0
    fi
    top=$(git rev-parse --show-toplevel)
    case "$top" in
        */.worktrees/*)
            printf '%s\n' "${top%/.worktrees/*}"
            ;;
        *)
            die "cannot determine main checkout path from linked worktree at $top (unexpected layout)"
            ;;
    esac
}

guard_main_checkout() {
    local gd gcd mw
    gd=$(current_git_dir)
    gcd=$(current_git_common_dir)
    if [[ "$gd" != "$gcd" ]]; then
        mw=$(main_worktree_path)
        log "refused: this subcommand must run from the main checkout, not a linked worktree."
        log "  you are in:     $(pwd)"
        log "  main checkout:  $mw"
        die "cd to the main checkout and re-run"
    fi
}

# All submodule paths declared in the main checkout's .gitmodules.
submodule_paths() {
    local mw="$1"
    if [[ -f "$mw/.gitmodules" ]]; then
        git -C "$mw" config -f .gitmodules --get-regexp '\.path$' 2>/dev/null \
            | awk '{print $2}'
    fi
}

# Prints deinitialized submodule paths (leading '-' in `git submodule status`).
deinitialized_submodules() {
    local repo="$1"
    git -C "$repo" submodule status 2>/dev/null | awk '/^-/{print $2}'
}

# Populates every submodule inside a linked worktree OFFLINE and without
# disturbing the main checkout's own submodule working trees. `git submodule
# update` in a linked worktree is unsafe here: a submodule's git directory
# under .git/modules/<name> is shared, and `submodule update` treats it as
# having a single core.worktree — running it from a linked worktree repoints
# that core.worktree at the new location and empties/repoints the MAIN
# checkout's submodule working tree. Instead, treat each submodule exactly
# like the superproject itself: give it a proper linked worktree of its OWN
# repo (`git -C <main>/<submodule> worktree add`), which creates a private
# `.git/modules/<name>/worktrees/<uuid>` and never touches the main
# checkout's existing worktree of that submodule. This also requires no
# network access: the target commit is the gitlink already recorded in the
# superproject tree, resolved and checked out purely from objects already
# present in the shared .git/modules/<name> object store.
populate_submodules_offline() {
    local mw="$1" wtpath="$2"
    local sm sha smdir failed=0
    while IFS= read -r sm; do
        [[ -n "$sm" ]] || continue
        smdir="$mw/$sm"
        if [[ ! -e "$smdir/.git" ]]; then
            log "warning: submodule $sm not initialized in main checkout ($smdir) — skipping"
            continue
        fi
        sha=$(git -C "$wtpath" ls-tree HEAD -- "$sm" | awk '{print $3}')
        if [[ -z "$sha" ]]; then
            log "warning: could not resolve recorded commit for submodule $sm — skipping"
            failed=1
            continue
        fi
        mkdir -p "$(dirname "$wtpath/$sm")"
        if ! run git -C "$smdir" worktree add --detach "$wtpath/$sm" "$sha"; then
            log "warning: offline worktree add failed for submodule $sm at $sha"
            failed=1
        fi
    done <<<"$(submodule_paths "$mw")"
    return "$failed"
}

# Newline-separated absolute worktree paths git itself considers registered,
# per the main checkout's `git worktree list --porcelain`. This is the only
# reliable membership test: a directory under .worktrees/ can exist without
# being a registered worktree (stray debris), and asking git for a branch
# from inside such a directory silently resolves to the enclosing repo.
registered_worktree_paths() {
    local mw="$1"
    git -C "$mw" worktree list --porcelain | awk '/^worktree /{print $2}'
}

normalize_name() {
    local n="$1"
    n="${n#.worktrees/}"
    n="${n%/}"
    printf '%s\n' "$n"
}

resolve_worktree_path() {
    local mw="$1" name="$2"
    printf '%s\n' "$mw/.worktrees/$name"
}

require_worktree_exists() {
    local wtpath="$1"
    [[ -d "$wtpath" ]] || die "no worktree at $wtpath"
    git -C "$wtpath" rev-parse --git-dir >/dev/null 2>&1 \
        || die "$wtpath exists but is not a git worktree"
}

remove_submodule_worktrees() {
    local mw="$1" wtpath="$2" sm
    while IFS= read -r sm; do
        [[ -n "$sm" ]] || continue
        [[ -e "$mw/$sm/.git" ]] || continue
        if (( DRY_RUN )); then
            printf '[dry-run] would run: git -C %s worktree remove --force %s\n' "$mw/$sm" "$wtpath/$sm"
            printf '[dry-run] would run: git -C %s worktree prune\n' "$mw/$sm"
        else
            git -C "$mw/$sm" worktree remove --force "$wtpath/$sm" 2>/dev/null || true
            git -C "$mw/$sm" worktree prune
        fi
    done <<<"$(submodule_paths "$mw")"
}

# ---------------------------------------------------------------------------
# new <short-id> <slug>
# ---------------------------------------------------------------------------
cmd_new() {
    local short_id="${1:-}" slug="${2:-}"
    [[ -n "$short_id" && -n "$slug" ]] \
        || die "usage: worktree.sh new <short-id> <slug>"
    [[ "$short_id" =~ ^[A-Za-z0-9]+$ && "$slug" =~ ^[A-Za-z0-9._-]+$ ]] \
        || die "usage: worktree.sh new <short-id> <slug> — both must be plain identifiers"

    guard_main_checkout
    local mw name branch wtpath
    mw=$(main_worktree_path)
    name="${short_id}-${slug}"
    branch="agent/${name}"
    wtpath=$(resolve_worktree_path "$mw" "$name")

    # Fail cleanly before mutating anything if the branch or dir already exist.
    if git -C "$mw" show-ref --verify --quiet "refs/heads/$branch"; then
        die "branch $branch already exists — refusing to bootstrap over it"
    fi
    if [[ -e "$wtpath" ]]; then
        die "$wtpath already exists — refusing to bootstrap over it"
    fi

    # Branch directly from LOCAL main — no fetch, no origin dependency. Local
    # main (and the local-only submodule commits it records) is frequently
    # ahead of, or entirely absent from, origin in this repo, so origin is
    # never an authoritative source for either.
    run git -C "$mw" worktree add "$wtpath" -b "$branch" main

    if (( DRY_RUN )); then
        printf '[dry-run] would populate submodules offline: for each submodule, git -C <main-checkout>/<submodule> worktree add --detach %s/<submodule> <recorded-sha>\n' "$wtpath"
        printf '[dry-run] resolved worktree: %s\n' "$wtpath"
        printf '[dry-run] resolved branch:   %s\n' "$branch"
        return 0
    fi

    if ! populate_submodules_offline "$mw" "$wtpath"; then
        log "offline submodule population failed — rolling back partial worktree to avoid leaving broken state"
        local sm
        while IFS= read -r sm; do
            [[ -n "$sm" ]] || continue
            git -C "$mw/$sm" worktree remove --force "$wtpath/$sm" 2>/dev/null || true
            git -C "$mw/$sm" worktree prune 2>/dev/null || true
        done <<<"$(submodule_paths "$mw")"
        git -C "$mw" worktree remove --force "$wtpath" 2>/dev/null || true
        git -C "$mw" worktree prune
        git -C "$mw" branch -D "$branch" 2>/dev/null || true
        die "bootstrap failed during submodule population; rolled back — no partial worktree/branch left behind"
    fi

    # Machine-consumable result line, then a human-readable echo.
    printf 'WORKTREE_PATH=%s\n' "$wtpath"
    printf 'BRANCH=%s\n' "$branch"
}

# ---------------------------------------------------------------------------
# list
# ---------------------------------------------------------------------------
cmd_list() {
    local mw base dir name branch missing registered
    mw=$(main_worktree_path)
    base="$mw/.worktrees"
    if [[ ! -d "$base" ]]; then
        log "no .worktrees directory under $mw"
        return 0
    fi
    registered=$(registered_worktree_paths "$mw")
    local found=0
    for dir in "$base"/*/; do
        [[ -d "$dir" ]] || continue
        dir="${dir%/}"
        name=$(basename "$dir")
        found=1
        if ! grep -Fxq "$dir" <<<"$registered"; then
            printf '%s\tUNREGISTERED-DEBRIS\t-\n' "$name"
            continue
        fi
        branch=$(git -C "$dir" branch --show-current)
        missing=$(deinitialized_submodules "$dir")
        if [[ -z "$missing" ]]; then
            printf '%s\tbranch=%s\tsubmodules=initialized\n' "$name" "$branch"
        else
            printf '%s\tbranch=%s\tsubmodules=MISSING(%s)\n' "$name" "$branch" "$(tr '\n' ',' <<<"$missing")"
        fi
    done
    if (( ! found )); then
        log "no worktrees found under $base"
    fi
}

# ---------------------------------------------------------------------------
# rebase <name>
# ---------------------------------------------------------------------------
cmd_rebase() {
    local name="${1:-}"
    [[ -n "$name" ]] || die "usage: worktree.sh rebase <name>"
    guard_main_checkout
    local mw wtpath
    mw=$(main_worktree_path)
    name=$(normalize_name "$name")
    wtpath=$(resolve_worktree_path "$mw" "$name")
    require_worktree_exists "$wtpath"

    if (( DRY_RUN )); then
        printf '[dry-run] would run: git -C %s rebase main\n' "$wtpath"
        return 0
    fi

    if git -C "$wtpath" rebase main; then
        log "rebase clean: $wtpath is now on top of local main"
    else
        log "rebase stopped with conflicts in $wtpath"
        log "resolve them there, then run:"
        log "  git -C \"$wtpath\" status"
        log "  # fix conflicts, then:"
        log "  git -C \"$wtpath\" add <resolved files>"
        log "  git -C \"$wtpath\" rebase --continue"
        log "  # or to cancel: git -C \"$wtpath\" rebase --abort"
        exit 1
    fi
}

# ---------------------------------------------------------------------------
# merge <name>
# ---------------------------------------------------------------------------
cmd_merge() {
    local name="${1:-}"
    [[ -n "$name" ]] || die "usage: worktree.sh merge <name>"
    guard_main_checkout
    local mw wtpath branch
    mw=$(main_worktree_path)
    name=$(normalize_name "$name")
    wtpath=$(resolve_worktree_path "$mw" "$name")
    require_worktree_exists "$wtpath"
    branch=$(git -C "$wtpath" branch --show-current)
    [[ -n "$branch" ]] || die "$wtpath has no current branch (detached HEAD?)"

    run git -C "$mw" checkout main

    if (( DRY_RUN )); then
        printf '[dry-run] would run: git -C %s merge --ff-only %s\n' "$mw" "$branch"
        return 0
    fi

    if git -C "$mw" merge --ff-only "$branch"; then
        log "fast-forwarded main to $branch"
    else
        die "merge --ff-only failed: main has moved since $branch last rebased. Re-run: tools/worktree/worktree.sh rebase $name — never falling back to a non-ff merge."
    fi
}

# ---------------------------------------------------------------------------
# remove <name>
# ---------------------------------------------------------------------------
cmd_remove() {
    local name="${1:-}"
    [[ -n "$name" ]] || die "usage: worktree.sh remove <name>"
    guard_main_checkout
    local mw wtpath branch remaining registered
    mw=$(main_worktree_path)
    name=$(normalize_name "$name")
    [[ "$name" =~ ^[A-Za-z0-9._-]+$ ]] \
        || die "worktree name must be a plain identifier"
    wtpath=$(resolve_worktree_path "$mw" "$name")
    [[ -d "$wtpath" ]] || die "no worktree at $wtpath"

    registered=$(registered_worktree_paths "$mw")
    if ! grep -Fxq "$wtpath" <<<"$registered"; then
        log "removing unregistered debris at $wtpath"
        remove_submodule_worktrees "$mw" "$wtpath"
        if ! run rm -rf -- "$wtpath"; then
            die "could not remove unregistered debris at $wtpath; close shells or processes using the directory and retry"
        fi
        run git -C "$mw" worktree prune
        return 0
    fi

    require_worktree_exists "$wtpath"
    branch=$(git -C "$wtpath" branch --show-current || true)

    # Tear down each submodule's own linked worktree (registered by
    # populate_submodules_offline via `git -C <main>/<submodule> worktree add`)
    # before removing the superproject worktree. Without this, every removal
    # leaks a stale worktree registration under the submodule's
    # .git/modules/<name>/worktrees/<uuid> that later blocks re-adding the
    # same path. Tolerate submodules that were never registered this way
    # (e.g. stubs left by the old `submodule update` bootstrap path).
    remove_submodule_worktrees "$mw" "$wtpath"

    # --force alone handles initialized submodules; a prior `submodule deinit`
    # would instead corrupt the shared .git/config (see instructions).
    run git -C "$mw" worktree remove --force "$wtpath"
    run git -C "$mw" worktree prune

    if [[ -n "$branch" ]]; then
        run git -C "$mw" branch -d "$branch"
    fi

    if (( DRY_RUN )); then
        return 0
    fi

    remaining=$(deinitialized_submodules "$mw")
    if [[ -z "$remaining" ]]; then
        log "verified: main checkout's submodules are still initialized after teardown"
    else
        log "WARNING: main checkout has deinitialized submodules after teardown: $remaining"
        log "run: tools/worktree/worktree.sh doctor"
    fi
}

# ---------------------------------------------------------------------------
# doctor
# ---------------------------------------------------------------------------
cmd_doctor() {
    guard_main_checkout
    local mw missing p stale
    mw=$(main_worktree_path)

    missing=$(deinitialized_submodules "$mw")
    if [[ -z "$missing" ]]; then
        log "submodules: all healthy in main checkout ($mw)"
    else
        log "submodules: deinitialized in main checkout: $missing"
        while IFS= read -r p; do
            [[ -n "$p" ]] || continue
            run git -C "$mw" submodule init -- "$p"
            run git -C "$mw" submodule update --init --recursive -- "$p"
        done <<<"$missing"
        if ! (( DRY_RUN )); then
            missing=$(deinitialized_submodules "$mw")
            if [[ -z "$missing" ]]; then
                log "submodules: repaired — all healthy now"
            else
                log "submodules: still deinitialized after repair attempt: $missing"
            fi
        fi
    fi

    stale=$(git -C "$mw" worktree prune --dry-run -v 2>&1 || true)
    if [[ -n "$stale" ]]; then
        log "stale worktree registrations (would be cleared by 'git worktree prune'):"
        log "$stale"
        run git -C "$mw" worktree prune
    else
        log "worktree registrations: none stale"
    fi

    # Debris: directories under .worktrees/ that git does not consider a
    # registered worktree at all (not stale-registered, just never a worktree).
    local base dir orphans registered name
    base="$mw/.worktrees"
    orphans=()
    if [[ -d "$base" ]]; then
        registered=$(registered_worktree_paths "$mw")
        for dir in "$base"/*/; do
            [[ -d "$dir" ]] || continue
            dir="${dir%/}"
            grep -Fxq "$dir" <<<"$registered" || orphans+=("$(basename "$dir")")
        done
    fi
    if (( ${#orphans[@]} )); then
        log "unregistered debris under .worktrees/ (not a git worktree, safe to inspect/remove manually): ${orphans[*]}"
    else
        log "unregistered debris: none"
    fi
}

# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------
main() {
    require_git

    local args=() a
    for a in "$@"; do
        if [[ "$a" == "--dry-run" ]]; then
            DRY_RUN=1
        else
            args+=("$a")
        fi
    done
    set -- "${args[@]+"${args[@]}"}"

    local sub="${1:-}"
    if [[ -z "$sub" || "$sub" == "-h" || "$sub" == "--help" ]]; then
        usage
        exit 0
    fi
    shift || true

    case "$sub" in
        new)    cmd_new "$@" ;;
        list)   cmd_list "$@" ;;
        rebase) cmd_rebase "$@" ;;
        merge)  cmd_merge "$@" ;;
        remove) cmd_remove "$@" ;;
        doctor) cmd_doctor "$@" ;;
        *)
            usage
            die "unknown subcommand: $sub"
            ;;
    esac
}

main "$@"
