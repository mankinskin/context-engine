# Verified observation (2026-08-08)

An agent ran `git checkout main` with its shell current directory inside `.worktrees/45bd0e3f-worktree-config-hijack`. Git returned `fatal: 'main' is already used by worktree at 'C:/Users/linus/git/context-engine'`. The agent then continued issuing commands against a state it had misread and reported internally contradictory results: `submodules=1/5`, `worktrees=8`, and `session_worktree_present=no` while the worktree existed.

Root-only operations, including merge into `main`, `worktree add`, `worktree remove`, `worktree prune`, `worktree repair`, and branch rename, must fail fast and loudly with an actionable message when invoked from inside a worktree rather than partially succeeding. The rename procedure is only safe from the repository root; see `.agents/instructions/commit/branch-worktree.instructions.md`.

## Acceptance criteria

1. Every root-only subcommand detects that the current directory is inside `.worktrees/` and aborts with a non-zero exit and a message naming the correct directory to run from.
2. The check uses the resolved real path, so a symlinked or renamed worktree is still detected.
3. A regression test invokes each root-only subcommand from inside a worktree and asserts a non-zero exit with no side effects.

## Related prior art

Ticket `b1f3e2a4` ("Enforce assignment start context branch and cwd checks", component `cli`, state `planned`) is related prior art. Keep this work separate: `b1f3e2a4` guards assignment-start context, while this ticket owns guards for root-only worktree and branch operations. Link the tickets for coordinated implementation.