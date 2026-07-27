## Commit Gate (D3): WIP Decision on Failed Review

On a failed review, the Iteration Agent asks the user whether to commit the partial work as WIP before stopping. The decision is binary:

- **User approves WIP commit:** The commit is delegated to the Commit Agent, which commits the work with a conventional message (e.g., `wip: <scope>`). The ticket state remains as-is, and the worktree becomes clean.
- **User declines:** The worktree is left dirty (no commit). The summary explicitly states "Commit: skipped (user declined)", and the ticket is re-packaged in-place for the next iteration without state change.

This gate ensures that partial progress is not lost while respecting the user's intent to iterate further before committing.