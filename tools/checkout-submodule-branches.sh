#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)

ensure_submodule_branch() {
    local parent_repo="$1"
    local rel_path="$2"
    local name="$3"
    local gitmodules="$parent_repo/.gitmodules"
    local submodule_path="$parent_repo/$rel_path"
    local branch=""
    local current_branch=""

    if [[ ! -e "$submodule_path/.git" && ! -f "$submodule_path/.git" ]]; then
        git -C "$parent_repo" submodule update --init -- "$rel_path" >/dev/null
    fi

    branch=$(git -C "$parent_repo" config -f "$gitmodules" --get "submodule.$name.branch" 2>/dev/null || true)
    if [[ -z "$branch" ]]; then
        branch=$(git -C "$submodule_path" symbolic-ref --quiet --short refs/remotes/origin/HEAD 2>/dev/null || true)
        branch=${branch#origin/}
    fi

    if [[ -z "$branch" ]]; then
        printf 'Skipping %s: no configured branch and origin/HEAD is unavailable.\n' "$rel_path" >&2
        return 0
    fi

    if git -C "$submodule_path" show-ref --verify --quiet "refs/remotes/origin/$branch"; then
        git -C "$submodule_path" fetch origin "$branch" >/dev/null
    fi

    current_branch=$(git -C "$submodule_path" symbolic-ref --quiet --short HEAD 2>/dev/null || true)

    if [[ "$current_branch" == "$branch" ]]; then
        :
    elif [[ -z "$current_branch" ]]; then
        if git -C "$submodule_path" show-ref --verify --quiet "refs/heads/$branch"; then
            # Preserve detached commits by attaching the branch only when that is a fast-forward.
            if git -C "$submodule_path" merge-base --is-ancestor "refs/heads/$branch" HEAD; then
                git -C "$submodule_path" branch -f "$branch" HEAD >/dev/null
                git -C "$submodule_path" checkout "$branch" >/dev/null
            elif git -C "$submodule_path" merge-base --is-ancestor HEAD "refs/heads/$branch"; then
                git -C "$submodule_path" checkout "$branch" >/dev/null
            else
                printf 'Skipping %s: detached HEAD does not fast-forward local %s.\n' "$rel_path" "$branch" >&2
                return 0
            fi
        else
            git -C "$submodule_path" checkout -b "$branch" >/dev/null
        fi
    elif git -C "$submodule_path" show-ref --verify --quiet "refs/heads/$branch"; then
        git -C "$submodule_path" checkout "$branch" >/dev/null
    else
        git -C "$submodule_path" checkout -b "$branch" >/dev/null
    fi

    if git -C "$submodule_path" show-ref --verify --quiet "refs/remotes/origin/$branch"; then
        git -C "$submodule_path" branch --set-upstream-to="origin/$branch" "$branch" >/dev/null
    fi

    printf '%s -> %s @ %s\n' "$rel_path" "$branch" "$(git -C "$submodule_path" rev-parse --short HEAD)"
    visit_submodules "$submodule_path"
}

visit_submodules() {
    local parent_repo="$1"
    local gitmodules="$parent_repo/.gitmodules"
    local line=""
    local key=""
    local rel_path=""
    local name=""

    [[ -f "$gitmodules" ]] || return 0

    while IFS= read -r line; do
        key=${line%% *}
        rel_path=${line#* }
        name=${key#submodule.}
        name=${name%.path}
        ensure_submodule_branch "$parent_repo" "$rel_path" "$name"
    done < <(git -C "$parent_repo" config -f "$gitmodules" --get-regexp '^submodule\..*\.path$')
}

visit_submodules "$repo_root"