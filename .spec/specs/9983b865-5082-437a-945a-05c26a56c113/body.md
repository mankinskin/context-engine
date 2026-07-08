<!-- aligned-structure:v1 -->

# Summary

Top-level submodules in the repository currently land in detached HEAD state, which makes local commits easy to create without advancing the intended `main` branch in each submodule.

## Behavior Story

Top-level submodules in the repository currently land in detached HEAD state, which makes local commits easy to create without advancing the intended `main` branch in each submodule.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Motivation

Top-level submodules in the repository currently land in detached HEAD state, which makes local commits easy to create without advancing the intended `main` branch in each submodule.

# Intended Behavior

Repository-owned submodule metadata should declare the intended tracking branch for each maintained submodule.

Repository-owned setup and update guidance should provide a standard command path that places each maintained submodule onto its tracked branch instead of leaving contributors on detached commits.

The standard workflow should make it clear that contributors publish submodule changes by pushing the checked-out branch in the submodule repository and then updating the superproject pointer.

# Constraints

- The repository must keep the superproject's pinned submodule commit model
- Existing clone and update flows must remain usable for contributors who only need the pinned submodule revisions
- The documented branch-tracking workflow should target maintained submodules whose remote default branch is `main`

# Acceptance Criteria

- `.gitmodules` records the intended `main` branch for maintained top-level submodules
- Repository guidance includes a standard command that switches maintained submodules to local branches tracking `origin/main`
- Focused validation demonstrates the changed configuration and the intended branch-tracking command path

# Implementation

- Root `.gitmodules` now records `branch = main` for `memory-viewers` and `context-stack`
- `tools/checkout-submodule-branches.sh` initializes missing submodules, resolves the intended branch from `.gitmodules` or `origin/HEAD`, and attaches detached commits to that branch when the move is a fast-forward
- `README.md` documents the pinned-versus-branch-tracking submodule workflow and the standard helper command

# Validation

- `git config -f .gitmodules --get-regexp '^submodule\..*\.(path|branch)$'`
- `bash tools/checkout-submodule-branches.sh`
- `git submodule foreach --recursive 'printf "path=%s\nbranch=%s\nhead=%s\nupstream=%s\n---\n" "$sm_path" "$(git symbolic-ref --quiet --short HEAD 2>/dev/null || echo DETACHED)" "$(git rev-parse --short HEAD)" "$(git rev-parse --abbrev-ref --symbolic-full-name @{upstream} 2>/dev/null || echo no-upstream)"'`

# Updated Artifacts

- [README.md](C:/Users/linus/git/graph_app/context-engine/README.md)
- [tools/checkout-submodule-branches.sh](C:/Users/linus/git/graph_app/context-engine/tools/checkout-submodule-branches.sh)
- [\.gitmodules](C:/Users/linus/git/graph_app/context-engine/.gitmodules)

# Traceability

- [0e1dca8b [repo] Keep submodules on main instead of detached HEADs](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/0e1dca8b-2869-4c43-b62b-79eb5f6f3a17/ticket.toml)
