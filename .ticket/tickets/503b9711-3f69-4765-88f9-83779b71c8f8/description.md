## Problem

`tools/worktree/worktree.sh` bootstrapped agent worktrees by treating the remote `origin` as the source of truth for `main`:

- The only remote is `origin -> https://github.com/mankinskin/context-engine.git`.
- Local `main` is routinely ahead of `origin/main` (unpushed local commits).
- This repo has 5 submodules (`context-stack`, `memory-api`, `memory-kernel`, `memory-viewers`, `viewer-api`). The superproject's recorded submodule commits are also frequently local-only — never pushed to the submodules' own remotes.
- Consequence: `git fetch origin` + `submodule update --init --recursive` could not resolve local-only submodule commits from the network, leaving submodule directories as unpopulated stubs (just a `.git` file), which broke `cargo metadata` for the whole workspace (root `Cargo.toml` declares workspace members inside these submodules).
- Additional sharp edge: `git submodule update` run inside a linked worktree shares the submodule's `.git/modules/<name>` git directory with the main checkout. Since that shared repo has a single `core.worktree`, running `submodule update` from the worktree side can repoint it and empty/repoint the MAIN checkout's own submodule working tree.

## Required behavior (implemented)

1. `new` bootstraps a worktree directly from LOCAL `main`, with no `git fetch origin` and no fast-forward of `main` to `origin/main`.
2. `rebase` rebases onto LOCAL `main`, not `origin/main`, and does not fetch.
3. Submodule population happens OFFLINE: each submodule is populated as its own linked worktree (`git -C <main-checkout>/<submodule> worktree add --detach <target>/<submodule> <recorded-sha>`), resolving the target commit from the gitlink already recorded in the superproject tree and checking it out purely from objects already present in the shared `.git/modules/<name>` store. No network access is attempted.
4. This mechanism is worktree-safe: giving each submodule its own linked worktree (a private `.git/modules/<name>/worktrees/<uuid>`) never touches the main checkout's own worktree of that submodule, unlike a naive `submodule update` inside the linked worktree.
5. `git submodule deinit` is never invoked anywhere in the script (removed from the old rollback path too — it is not needed since submodules are never registered via `submodule init`/`update` in the new scheme).

Docs updated to match: `.agents/instructions/commit/branch-worktree.instructions.md`, `.agents/agents/orchestrator.agent.md`, `.agents/agents/implement.agent.md`, `.agents/agents/commit.agent.md`.

## Acceptance criteria

- AC1: `bash -n tools/worktree/worktree.sh` parses clean.
- AC2: `grep -n 'origin' tools/worktree/worktree.sh` shows no remaining `fetch origin` or `origin/main` functional reference (comment-only mentions explaining the local-main rationale are acceptable).
- AC3: `bash tools/worktree/worktree.sh new <id> <slug> --dry-run` prints a plan containing no `fetch origin` and no `origin/main`.
- AC4: No `git submodule deinit` remains anywhere in the script.
- AC5: Docs (`branch-worktree.instructions.md`, `orchestrator.agent.md`, `implement.agent.md`, `commit.agent.md`) describe the local-main sequence, not the old origin-based one.

## Residual risk (for a follow-up unit)

Repairing the existing broken worktree at `.worktrees/f3c2b8a9-install-ctl` (currently has unpopulated `memory-viewers`, `viewer-api`, `memory-kernel`) is explicitly a SEPARATE unit. The new offline per-submodule-worktree mechanism has not yet been exercised against a real worktree creation in this session (dry-run only, per hard safety constraints) — the next unit that creates or repairs a real worktree should verify empirically that populate_submodules_offline succeeds and does not disturb the main checkout's submodule state.

## Field evidence (2026-08-06): residual risk exercised against a real worktree

The "Residual risk" section above notes that `populate_submodules_offline` had only ever been dry-run, and that a real worktree creation should verify it empirically. A worktree was created and repaired by hand this session (using PLAIN `git worktree add`, NOT `worktree.sh`), which confirms the rationale for the offline per-submodule-worktree mechanism and adds two concrete findings.

### Finding 1 — a linked worktree gets a SEPARATE submodule clone, not the shared one

The submodule git dir for the new worktree resolved to:

- `.git/worktrees/fa2ba34b-worktree-session-proxy/modules/memory-api`

NOT the shared `.git/modules/memory-api`. Because that separate clone is populated from the submodule's own remote, local-only commits are simply absent. The recreated worktree's `memory-api` was missing BOTH:

- `6eb978fb26a73193da96f810211c406f8c0a91a7` — the gitlink the superproject records
- `67556a49c04c68b74d259f1c7f9702efa0ca9ed0` — `feat(ticket): add linked parts to ticket history`, a local commit ahead of the gitlink

Repair required fetching by local path from the main checkout's clone (`git fetch c:/Users/linus/git/context-engine/memory-api`), then explicitly checking out the target commit. Note that `git submodule update --init` reset the pointer back to the recorded gitlink and the ahead-of-gitlink commit had to be re-checked-out afterwards.

This is direct evidence for required behavior 3: resolving submodule commits from the network cannot work, and the offline mechanism is not merely a performance choice.

### Finding 2 — `git worktree move` is unusable on this repository

Renaming a worktree directory with `git worktree move` fails outright:

```
fatal: working trees containing submodules cannot be moved or removed
```

There is no flag that relaxes this. Any rename or relocation of a worktree in this repo must be implemented as remove + recreate on the same branch, followed by the local submodule-object fetch described in Finding 1. This constrains the worktree rename path and is tracked as an acceptance criterion on the managed session-worktree lifecycle ticket.

## Additional Acceptance Criteria

- AC6: Bootstrapping a worktree populates every submodule at the superproject's recorded gitlink with NO network access, and additionally resolves submodule commits that exist only in the main checkout's clone.
- AC7: A worktree created by `worktree.sh new` resolves the recorded gitlink for all 5 submodules immediately after creation, verified by `git -C <worktree>/<submodule> cat-file -e <recorded-sha>` for each, with networking unavailable or unused.
- AC8: The script does not depend on `git worktree move` anywhere, and any rename/relocate path it offers is implemented as remove + recreate + local submodule-object fetch.