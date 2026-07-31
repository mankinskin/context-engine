Two copies of the same board module exist, confirmed by skeleton inspection:

- LIVE copy: memory-api/crates/memory-api/src/storage/board.rs — 425 lines, defines BoardEntry, BoardEntryStatus, BoardConfig, BoardSnapshot, ActiveWorktree, BoardHistorySnapshot, BoardCleanPreview, BoardCleanResult, ReconcileAction, BoardReconcileResult, BoardError.
- STALE copy: memory-kernel/src/storage/board.rs — 295 lines, defines the same set of types MINUS `ActiveWorktree`.

The two copies have now diverged: ticket c060bf94-2435-4cc5-8016-ca1d2c8264f5 ("Bind board entries to sessions and worktrees; add active-worktree discovery") added `session_id`/`worktree_path`/`branch` fields to `BoardEntry`, the `ActiveWorktree` type, a `WorktreeConflict` error, and `board_worktrees` surfaces — all applied only to the memory-api copy. The memory-kernel copy is off the live call path and was never updated. The user has explicitly confirmed this duplication is NOT intentional.

Acceptance criteria:
1. Establish whether memory-kernel/src/storage/board.rs has ANY live callers anywhere in the workspace before making any change.
2. If it has no live callers, delete memory-kernel/src/storage/board.rs (and any now-dead re-exports/module declarations pointing at it). If it does have live callers, replace its body with a re-export of the memory-api definitions instead of deleting it outright.
3. `cargo check` passes across the whole workspace after the change.

Reference: ticket c060bf94-2435-4cc5-8016-ca1d2c8264f5 is the ticket whose changes caused this divergence.