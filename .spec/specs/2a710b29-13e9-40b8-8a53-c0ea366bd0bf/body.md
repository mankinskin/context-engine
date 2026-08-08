## Requirement

Every implementation agent session MUST operate in isolation from every other concurrent session: one implementation session, one git worktree, one branch, cut from `main`. This closes the failure mode where concurrent sessions overwrite each other's uncommitted edits in a shared root checkout and commits cannot be attributed to a session.

### Isolation invariant

- Each implementation task is assigned exactly one git worktree at `.worktrees/<ticket-short-id>-<slug>` and exactly one branch `agent/<ticket-short-id>-<slug>`, both cut from `main`.
- No two concurrent implementation sessions may share a worktree or branch.
- `.worktrees/` is git-ignored at the repository root.

### Two-claim protocol

Isolation is enforced by two independent, authoritative claims made before the first edit:

1. **Session store claim** (authoritative for worktree + branch): the implementation session calls `session_check_in` with `worktree_path` and `branch` set, recording the session's exclusive claim on that worktree/branch pair in `SessionWorktreeAssignment`.
2. **Board claim** (authoritative for ticket + file ownership): the implementation session calls `board_check_in` with the ticket id and the files it intends to touch, recording exclusive ownership of that scope on the draftboard.

Both claims are required. The session store claim alone does not reserve ticket/file scope, and the board claim alone does not reserve a worktree/branch.

### Intent-prefix encoding

`BoardEntry` has no dedicated branch or worktree column. Until such a column exists, the branch and worktree are carried in the board entry's `intent` field using the fixed prefix:

```
branch=<branch> worktree=<path> — <intent>
```

Any tooling or reviewer inspecting the board can recover the branch/worktree pair for an active entry by parsing this prefix.

### Rebase-before-ready rule

- Conflict resolution against `main` happens exclusively on the feature branch, via `git rebase main`. Conflicts are never resolved directly on `main`.
- After any rebase, validation (tests, build) is re-run on the feature branch before the branch is marked ready.

### Ready-to-merge signal

A branch is ready to merge when both of the following are true:

1. The implementation session calls `board_check_out` with a `handoff_reason` that starts with `ready-to-merge:`.
2. The ticket is moved to `in-review`.

Neither signal alone constitutes readiness; both must hold.

### Merge monopoly

- Only the root orchestrator session merges feature branches into `main`.
- The orchestrator asserts a fast-forward merge via `git merge --ff-only`.
- If the fast-forward fails (i.e., `main` has advanced past the branch's base), the branch is sent back to its owning session for another rebase; the orchestrator never performs a non-fast-forward merge or resolves conflicts itself.

### Submodule handling

- A superproject worktree created under this scheme requires `git submodule update --init --recursive` before work begins.
- Submodule-local work follows the same branch-per-task naming scheme within the submodule, and is committed deepest-first (submodules before the superproject pointer update).

## Non-goals

The following are explicitly out of scope for this requirement:

- Automated worktree provisioning in Rust (e.g., a tool or MCP command that runs `git worktree add` programmatically).
- Adding a dedicated branch/worktree column to `BoardEntry`'s schema.
- CI enforcement of the isolation invariant or the two-claim protocol.
