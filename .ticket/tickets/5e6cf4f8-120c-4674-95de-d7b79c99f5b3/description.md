## Objective

Replace `tools/worktree/worktree.sh` with a Rust binary that drives git through
a library rather than shelling out, and give worktrees a lifecycle: locked
while a session is active, reclaimed when it completes and the worktree holds
nothing worth keeping.

## Why a rewrite

`worktree.sh` is a 500-line bash script encoding subtle git invariants —
offline submodule population via per-submodule linked worktrees, refusal to
fall back to a non-fast-forward merge, never touching `origin`. Its test suite
is 16 shell scripts, 10 of which fail. Those 10 are not flaky; they describe
lifecycle behavior the script never implemented. Rather than grow the script to
meet them, the contract moves to Rust with tests in Rust.

The existing shell suite is prior art, not a spec to satisfy in place. Its
intent carries over; the scripts and their `common.sh` harness retire with the
script they exercise.

## Lifecycle rules

A worktree is locked while its session is active and is never purged or
repurposed during that time.

On session completion, evaluate the worktree:

| State | Outcome |
|---|---|
| Uncommitted changes | Preserved. Never reclaimed automatically. |
| Commits not reachable from `main` | Preserved. Never reclaimed automatically. |
| Clean, and not ahead of `main` | Reclaimed for reuse. |

Reclamation is **rename/move in place**, re-pointing the branch, performed
automatically by the hook with no human in the loop. Moving in place preserves
the built `target/` directory and the rebuilt entity-store indexes; remove and
re-create would discard both and pay a full workspace rebuild (~2m26s measured)
plus a re-scan of ~1500 tickets.

A dirty worktree is never repurposed automatically. Repurposing one requires an
explicit agent decision, framed as re-topicing that worktree.

## Acceptance Criteria

1. A Rust binary replaces `worktree.sh` and covers its current subcommands:
   `new`, `list`, `rebase`, `merge`, `remove`, `doctor`.
2. Git is driven through a library, not by shelling out to the `git` binary.
3. Tests are written in Rust and cover the intent of the retiring shell suite,
   including the ten currently-failing contracts.
4. The `UserPromptSubmit` hook creates the session's worktree if absent, and is
   idempotent across repeated prompts within one session.
5. A worktree belonging to an active session is never purged or repurposed.
6. On session completion, a worktree with uncommitted changes is preserved.
7. On session completion, a worktree holding commits not reachable from `main`
   is preserved.
8. On session completion, a clean worktree not ahead of `main` is renamed/moved
   in place and its branch re-pointed, automatically.
9. Repurposing a dirty worktree requires an explicit agent decision and never
   happens automatically.
10. A recycled worktree is brought up to date with `main` before reassignment.
11. A newly created or recycled worktree has its machine-local entity-store
    indexes rebuilt before first use. A fresh worktree carries tracked
    `.ticket`, `.spec` and `.test` payloads but no `tickets.db` or
    `search_index/`, and the store refuses to open until those exist.
12. The retiring shell suite and its `common.sh` harness are removed in the
    same change that lands the Rust equivalent, not left behind red.

## Notes

Blocked on nothing, but sequenced after session-worktree discovery, which
supplies the resolution half of the same feature.

Separately tracked: `.worktrees/` currently holds nine directories while git
registers only three or four. Eager per-session creation will amplify this
orphan accumulation.



## 2026-08-07 Scope Update

The narrow capture-hook fix moved out of this ticket to ticket `a1b911ab-9394-4ba8-9134-1b2687e96ccd` and is already implemented there. This ticket now covers only eager worktree creation and the `worktree.sh`-to-Rust rewrite.
