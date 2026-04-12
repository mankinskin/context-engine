# [Board] ticket-api: Cleanup, File Ops, Reconciliation, Claim Deprecation

## Purpose

Add the operational maintenance layer to the draftboard in `crates/ticket-api/`. This builds on the core board storage (types, tables, check-in/out/heartbeat/show/configure) established by `0db86ac1` and adds:

- Confirmation-token cleanup workflow (`board_clean_preview`, `board_clean_apply`)
- Mid-session file ownership mutation (`board_update_files`, `board_rename_file`)
- Lifecycle reconciliation hooks in existing ticket methods
- Public `claim()`/`unclaim()` deprecation

## Component Boundaries

### In scope
- `BoardCleanPreview`, `BoardCleanResult` types (public in `ticket_api`)
- `board_clean_preview()`: returns cleanup candidates and a confirmation token bound to the current board state
- `board_clean_apply(token, include_stale)`: removes only token-identified entries; rejects stale tokens if the board changed materially since preview
- `board_update_files(ticket_id, agent_id, add, remove)`: modify file ownership mid-session with conflict re-check on newly added files
- `board_rename_file(ticket_id, agent_id, old_path, new_path)`: atomic rename transition — releases old path, claims new path, re-checks conflicts, records as single audit event
- `board_reconcile(ticket_id)`: internal helper called by `update_ticket`, `close_ticket`, `cancel_ticket`, `revert_ticket`
- Public `claim()`/`unclaim()` methods removed or made `pub(crate)` — external callers must use board commands

### Out of scope
- Core board types and CRUD (established by `0db86ac1`)
- CLI subcommands (owned by `bcc111c6`)
- MCP tools (owned by `ec52f7cb`)
- Integration with `next`/`status` (owned by `74160bb8`)

### Depends on
- `0db86ac1` — core board storage must exist before operations can be added

## Key Data Types (added by this ticket)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardCleanPreview {
    pub token: String,
    pub completed_candidates: Vec<BoardEntry>,
    pub stale_candidates: Vec<BoardEntry>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardCleanResult {
    pub removed_completed: u32,
    pub removed_stale: u32,
}
```

## Implementation Notes

### Cleanup Semantics

- Completed entries are not auto-pruned. They become cleanup-eligible only after `completed_audit_window_secs` elapses.
- `board_clean_preview()` returns a `BoardCleanPreview` with candidates and a confirmation token.
- `board_clean_apply(token, include_stale)` removes only the entries identified in the preview. The token is rejected if the board has changed materially since the preview was generated (e.g. an entry was renewed or a new entry was added that changes the candidate set).
- `include_stale` means stale entries are also removed, after operator confirmation.

### Board Reconciliation Hooks

Per Q9 (all mutating lifecycle operations trigger reconciliation), the following existing `TicketStore` methods must call a new internal `board_reconcile(ticket_id)` helper:

- `update_ticket()` (state transitions)
- `close_ticket()`
- `cancel_ticket()`
- `revert_ticket()`

`board_reconcile(ticket_id)` checks whether the ticket has an active board entry. If so:
- If the ticket reached a terminal state (`done`, `cancelled`): mark the board entry as `Completed` and surface a cleanup recommendation.
- If the ticket was reverted to an earlier state: surface a warning that the board entry's intent may be stale.
- Reconciliation never silently deletes entries. It flags them for human review.

### File Rename Transition

`board_rename_file(ticket_id, agent_id, old_path, new_path)` provides an atomic rename: it removes `old_path` from `owned_files`, adds `new_path`, re-checks for conflicts on `new_path`, and records the operation as a single rename audit event. `board_update_files(add, remove)` can also express a rename when `add` and `remove` target the same logical file, but `board_rename_file` is the preferred explicit path.

### Claim/Unclaim Deprecation

Public `claim()`/`unclaim()` methods are removed or made `pub(crate)`. Board commands (`board_check_in`, `board_check_out`) handle lease management internally. Any external code calling `claim()`/`unclaim()` directly must migrate to the board API.

## Acceptance Criteria

- [ ] `BoardCleanPreview` and `BoardCleanResult` types are public in `ticket_api`
- [ ] `board_clean_preview()` returns candidates and a confirmation token
- [ ] `board_clean_apply()` removes only token-identified entries and rejects stale tokens
- [ ] `board_update_files()` modifies file ownership with conflict re-check on added files
- [ ] `board_rename_file()` performs atomic rename with audit event
- [ ] `board_reconcile()` is called by `update_ticket`, `close_ticket`, `cancel_ticket`, `revert_ticket`
- [ ] Reconciliation marks entries `Completed` on terminal ticket states and warns on reverts
- [ ] Public `claim()`/`unclaim()` methods are removed or made `pub(crate)`
- [ ] Unit tests cover: confirmation-token cleanup, stale-token rejection, file update conflicts, rename transitions, reconciliation hooks on close/cancel/revert
