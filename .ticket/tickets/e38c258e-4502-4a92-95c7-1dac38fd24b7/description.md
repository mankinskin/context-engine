Problem: every agent works in the same root checkout on `main`. Concurrent sessions overwrite each other's uncommitted edits, a `cargo fmt` or `git add -A` from one session swallows another's in-progress work, and the resulting commits cannot be attributed to a session. A recent incident required a full forensic review of the session store to work out which of three sessions produced a dirty tree.

Goal: instrument a single implementation loop so that each implementation task runs in its own git worktree on its own branch cut from `main`, claims that worktree in the session store and the ticket board before its first edit, rebases onto the updated `main` and resolves conflicts on its own side, marks the branch ready to merge, and leaves the actual merge into `main` to the root orchestrator session.

Scope: guidance only. No Rust changes — the session store and board already expose everything needed.

Acceptance criteria:
1. A canonical instruction file at `.agents/instructions/commit/branch-worktree.instructions.md` defines the seven-step loop (bootstrap, claim, work, commit, rebase, mark-ready, merge), the branch and worktree naming scheme, and the escalation triggers.
2. Branch naming is `agent/<ticket-short-id>-<slug>` and worktree path is `.worktrees/<ticket-short-id>-<slug>`; `.worktrees/` is git-ignored at the repo root.
3. The implementation agent claims its worktree via `session_check_in` (with `worktree_path` and `branch`) and its ticket/file scope via `board_check_in`, before the first edit.
4. Because `BoardEntry` has no branch column, the branch and worktree are carried in the board `intent` field using the fixed prefix `branch=<branch> worktree=<path> — <intent>`.
5. Conflict resolution happens on the feature branch via `git rebase origin/main`, never on `main`; validation is re-run after the rebase.
6. Ready-to-merge is signalled by a `board_check_out` whose `handoff_reason` starts with `ready-to-merge:` and by moving the ticket to `in-review`.
7. Only the root orchestrator session merges into `main`, and it asserts a fast-forward with `git merge --ff-only`; a failed fast-forward sends the branch back for another rebase.
8. `AGENTS.md`, `.agents/instructions/commit/workflow.instructions.md`, `.agents/instructions/ticket/board.instructions.md`, `.agents/agents/implement.agent.md`, `.agents/agents/commit.agent.md`, and `.agents/agents/orchestrator.agent.md` all point at the new instruction file and state the rule relevant to their own role.
9. Submodule handling is stated: a superproject worktree needs `git submodule update --init --recursive`, and submodule-local work gets a matching branch, committed deepest-first.