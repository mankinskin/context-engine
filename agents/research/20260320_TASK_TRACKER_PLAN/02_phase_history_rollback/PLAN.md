# Phase 2 — History and Rollback (git-backed)

**Status:** BLOCKED (requires Phase 1 complete)

## Objective

Provide full diff history with apply/revert per ticket using `git2` as the versioning
backend. Every mutation committed by Phase 1 already records a git commit (Phase 1
step 7); Phase 2 builds the user-facing history and rollback commands on top of that.

## Deliverables

- [ ] `GitHistory::log(id, limit)` — list commits touching a ticket folder, newest first
- [ ] `GitHistory::diff(id, from_sha, to_sha)` — unified diff between two versions
- [ ] `GitHistory::show(id, sha)` — reconstruct ticket manifest at a given commit
- [ ] `GitHistory::apply_diff(id, diff_patch)` — apply a patch to current working state
- [ ] `GitHistory::revert(id, sha)` — restore ticket folder to state at that commit,
      creating a new forward commit (no destructive rewrite of history)
- [ ] `ticket history <id>` CLI command with `--format diff|log|show` flags
- [ ] `ticket revert <id> --to <sha>` CLI + HTTP command
- [ ] `ticket diff <id> --from <sha> --to <sha>` CLI + HTTP command

## Git Repository Strategy (to be decided in Phase 0)

### Option A — Leverage existing workspace git repo

- Ticket commits go onto a dedicated branch (`refs/context-tickets`) or as regular
  commits on the working branch.
- Pro: users can see ticket history in `git log`; backup via normal git push.
- Con: pollutes user commit history unless isolated to a separate branch.

### Option B — Embedded bare repo under index root

- Initialise a bare git repo at `.context-engine/ticket-index/history.git`.
- Ticket folder snapshots are committed there, invisible to the user’s main repo.
- Pro: clean separation; no branch pollution.
- Con: user cannot leverage their own git tooling directly.

**Recommendation:** Default to Option B; provide an `--use-workspace-git` flag for
Option A as an opt-in.

## Commit Message Convention

```
ticket(<uuid>): <operation> [<field>]

Examples:
  ticket(a3f2c7b1): create
  ticket(a3f2c7b1): update status open→in-progress
  ticket(a3f2c7b1): add-edge depends_on b1c2d3e4
  ticket(a3f2c7b1): delete
```

## Revert Semantics

- Revert is always **forward**: it creates a new commit that restores old content.
- Never force-pushes or rewrites history (preserves auditability).
- After revert, the watcher fires a MODIFIED event — index is updated through normal
  reconciliation path.

## Key Interview Answers Applied Here

| Answer | History impact |
|--------|---------------|
| Q7 — Diff history + git2 | `git2` is the storage and diff engine |
| Q10 — FS tracking | Watcher reconciles revert-generated changes automatically |

## Risks

- `git2` / `libgit2` is a C dependency — accepted per interview answers but must be
  documented in workspace build instructions.
- Concurrent git commits from multiple Phase 1 writers need serialisation; use the
  per-ticket lock to gate the git commit as well (step 7 is within the lock).
- Large binary attachments (Q8) will grow the git object store rapidly; consider
  `.gitattributes` in the history repo to exclude binary files from diffs.

## TODO

- TODO: Decide Option A vs. B in Phase 0 and update this plan.
- TODO: Write round-trip test: create ticket → update N times → revert to version 1
        → assert manifest equals initial state.
- TODO: Define binary file policy in history repo (exclude? LFS?).
