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
                          cut from origin/main, with all submodules initialized.
  list                    List existing .worktrees/* entries: branch + submodule
                          init status for each.
  rebase <name>           Fetch origin, then rebase <name>'s branch onto origin/main.
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

    run git -C "$mw" fetch origin
    run git -C "$mw" checkout main
    if ! (( DRY_RUN )); then
        # Fast-forward main to origin/main. Prefer `merge --ff-only` over
        # `pull --ff-only`: a `pull.rebase=true` repo config routes the
        # latter through rebase, which refuses outright on any unstaged
        # change even when local main is already even with or ahead of
        # origin (nothing to integrate). Skip entirely when there is
        # genuinely nothing to fast-forward.
        if git -C "$mw" merge-base --is-ancestor origin/main main; then
            log "main already contains origin/main — nothing to pull"
        elif ! git -C "$mw" merge --ff-only origin/main; then
            die "fast-forwarding main to origin/main failed in $mw — likely unstaged/uncommitted changes in the main checkout. Commit or stash, then retry."
        fi
    else
        printf '[dry-run] would run: git -C %s pull --ff-only origin main\n' "$mw"
    fi
    run git -C "$mw" worktree add "$wtpath" -b "$branch" main

    if (( DRY_RUN )); then
        printf '[dry-run] would run: git -C %s submodule update --init --recursive\n' "$wtpath"
        printf '[dry-run] resolved worktree: %s\n' "$wtpath"
        printf '[dry-run] resolved branch:   %s\n' "$branch"
        return 0
    fi

    if ! git -C "$wtpath" submodule update --init --recursive; then
        # Sharp edge: a submodule commit checked out in the worktree may
        # exist only in the main checkout's local submodule clone (never
        # pushed to origin). Repair by fetching each submodule directly from
        # the main checkout's copy, then retry once before giving up.
        log "submodule update failed — attempting repair by fetching from the main checkout's submodule clones"
        local sm
        while IFS= read -r sm; do
            [[ -n "$sm" ]] || continue
            if [[ -d "$mw/$sm/.git" || -f "$mw/$sm/.git" ]] && [[ -d "$wtpath/$sm" ]]; then
                git -C "$wtpath/$sm" fetch "$mw/$sm" 2>/dev/null || true
            fi
        done <<<"$(submodule_paths "$mw")"

        if ! git -C "$wtpath" submodule update --init --recursive; then
            log "submodule init still failing after repair attempt — rolling back partial worktree to avoid leaving broken state"
            git -C "$wtpath" submodule deinit --all --force 2>/dev/null || true
            git -C "$mw" worktree remove --force "$wtpath" 2>/dev/null || true
            git -C "$mw" worktree prune
            git -C "$mw" branch -D "$branch" 2>/dev/null || true
            die "bootstrap failed during submodule init; rolled back — no partial worktree/branch left behind"
        fi
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

    run git -C "$mw" fetch origin

    if (( DRY_RUN )); then
        printf '[dry-run] would run: git -C %s rebase origin/main\n' "$wtpath"
        return 0
    fi

    if git -C "$wtpath" rebase origin/main; then
        log "rebase clean: $wtpath is now on top of origin/main"
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
    local mw wtpath branch remaining
    mw=$(main_worktree_path)
    name=$(normalize_name "$name")
    wtpath=$(resolve_worktree_path "$mw" "$name")
    require_worktree_exists "$wtpath"
    branch=$(git -C "$wtpath" branch --show-current || true)

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
