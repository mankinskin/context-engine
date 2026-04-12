# [Board] ticket-api: Board Entries, WIP Limits, File Ownership, Conflict Detection

## Purpose

Implement the core draftboard data layer in `crates/ticket-api/`. This is the foundational storage and logic layer that all consumers (CLI, MCP, HTTP) build on. It adds new redb tables for board entries and configuration, and exposes board operations as methods on `TicketStore`.

## Component Boundaries

### In scope
- New `storage/board.rs` module containing board storage logic
- New redb table `BOARD_ENTRIES` with compound key `"{ticket_id}:{agent_id}"` → bincode `BoardEntry`
- New redb table `BOARD_CONFIG` with singleton key `"default"` → bincode `BoardConfig`
- `BoardEntry`, `BoardConfig`, `BoardSnapshot`, `BoardError` type definitions
- `board_check_in()`: validate WIP limit, check file conflicts, insert entry, claim legacy lease
- `board_check_out()`: mark completed, release lease, retain entry through the audit window
- `board_heartbeat()`: update `last_heartbeat` timestamp, reset TTL
- `board_show()`: read-only snapshot aggregating all entries into `BoardSnapshot`; never performs cleanup or heartbeat writes
- `board_configure()`: read/write board config
- `board_clean()`: explicit cleanup for audit-window-eligible completed entries and optionally stale entries after human review
- `board_update_files()`: modify file ownership mid-session
- Stale detection: compute `BoardEntryStatus::Stale` when `last_heartbeat + ttl_secs < now`
- File conflict detection: on check-in, scan all active entries for file path overlap
- Backward-compatible lease propagation: `board_check_in` calls `claim()`, `board_check_out` calls `unclaim()`

### Out of scope
- CLI argument parsing and output formatting (owned by `bcc111c6`)
- MCP tool registration and JSON schema (owned by `ec52f7cb`)
- Integration with `next`/`status` commands (owned by `74160bb8`)
- Persistent on-disk storage beyond redb (board is ephemeral)

## Key Data Types

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardEntry {
    pub entry_id: Uuid,
    pub ticket_id: Uuid,
    pub agent_id: String,
    pub previous_attempt: Option<Uuid>,
    pub checked_in_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub ttl_secs: u64,
    pub intent: String,
    pub owned_files: Vec<String>,
    pub status: BoardEntryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BoardEntryStatus {
    Active,
    Stale,
    Conflict,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardConfig {
    pub max_wip: u32,
    pub stale_after_secs: u64,
    pub completed_audit_window_secs: u64,
}

impl Default for BoardConfig {
    fn default() -> Self {
        Self {
            max_wip: 5,
            stale_after_secs: 3600,
            completed_audit_window_secs: 3600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardSnapshot {
    pub captured_at: DateTime<Utc>,
    pub entries: Vec<BoardEntry>,
    pub caller_entries: Vec<BoardEntry>,
    pub config: BoardConfig,
    pub active_count: u32,
    pub stale_count: u32,
    pub conflict_count: u32,
    pub wip_limit_reached: bool,
    pub file_ownership: BTreeMap<String, Vec<String>>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardCleanResult {
    pub removed_completed: u32,
    pub removed_stale: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardCleanPreview {
    pub token: String,
    pub completed_candidates: Vec<BoardEntry>,
    pub stale_candidates: Vec<BoardEntry>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum BoardError {
    #[error("WIP limit reached: {current}/{max} active entries")]
    WipLimitReached { current: u32, max: u32 },
    #[error("File conflict on {files:?} with agent {conflicting_agent} (ticket {conflicting_ticket})")]
    FileConflict {
        files: Vec<String>,
        conflicting_agent: String,
        conflicting_ticket: Uuid,
    },
    #[error("Already checked in: ticket {ticket_id} by {agent_id}")]
    AlreadyCheckedIn { ticket_id: Uuid, agent_id: String },
    #[error("Not checked in: ticket {ticket_id} by {agent_id}")]
    NotCheckedIn { ticket_id: Uuid, agent_id: String },
    #[error("Ticket not found: {0}")]
    TicketNotFound(Uuid),
}
```

## Implementation Notes

### Redb Table Schema

```rust
// New tables added alongside existing TICKETS, EDGES, LEASES tables
const BOARD_ENTRIES: TableDefinition<&str, &[u8]> = TableDefinition::new("board_entries");
const BOARD_ACTIVE_INDEX: TableDefinition<&str, &str> = TableDefinition::new("board_active_index");
const BOARD_CONFIG: TableDefinition<&str, &[u8]> = TableDefinition::new("board_config");
```

`BOARD_ENTRIES` is keyed by `entry_id` (UUID string). `BOARD_ACTIVE_INDEX` maps `"{ticket_id}:{agent_id}"` → `entry_id` and enforces one active entry per `(ticket_id, agent_id)` pair. Completed/cleaned entries are removed from the active index but remain in `BOARD_ENTRIES` until explicitly cleaned.

### Check-in Validation Sequence

1. Read `BoardConfig` from `BOARD_CONFIG` table (or use `Default::default()` if absent).
2. Scan `BOARD_ENTRIES` table; count active entries.
3. If `active_count >= config.max_wip`, return `BoardError::WipLimitReached`.
4. If `(ticket_id, agent_id)` key already exists in `BOARD_ACTIVE_INDEX` and the referenced entry’s status is Active/Stale, return `BoardError::AlreadyCheckedIn`.
5. Build file ownership map from all active entries. Check for overlap with requested `owned_files`.
6. If overlap found: mark conflicting entry as `Conflict`, return `BoardError::FileConflict`.
7. Generate a new `entry_id` (UUID). If a completed entry for the same `(ticket_id, agent_id)` exists, populate `previous_attempt` with its `entry_id`.
8. Insert new `BoardEntry` with status `Active`, `checked_in_at = now`, `last_heartbeat = now`.
9. Insert/overwrite `BOARD_ACTIVE_INDEX` with `"{ticket_id}:{agent_id}"` → new `entry_id`.
10. Call `self.claim(ticket_id, agent_id, ttl_secs, Some(intent))` for backward-compatible lease.
9. If the caller is using the `ticket update --board-check-in` convenience path, the CLI composes the ticket update and `board_check_in()` explicitly; the store API remains decomposed unless a transactional helper proves necessary.

### Stale Computation

Status is computed dynamically in `board_show()`:
- If `entry.status == Active && now > entry.last_heartbeat + Duration::seconds(entry.ttl_secs)`: set `status = Stale`.
- Stale entries count toward the WIP limit (to avoid ghost slots) but are flagged in warnings.
- The default threshold is one hour. Once stale, entries are surfaced as high-priority human-review items; no automatic cleanup occurs.

### Snapshot Atomicity

`board_show()` uses a single redb read transaction. All entries and config are read within this transaction, ensuring a consistent snapshot. Auto-heartbeat is handled by higher layers as a separate explicit write after the snapshot, so the store method stays read-only.

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
- If the ticket was reverted to an earlier state: surface a warning that the board entry’s intent may be stale.
- Reconciliation never silently deletes entries. It flags them for human review.

### File Rename Transition

`board_rename_file(ticket_id, agent_id, old_path, new_path)` provides an atomic rename: it removes `old_path` from `owned_files`, adds `new_path`, re-checks for conflicts on `new_path`, and records the operation as a single rename audit event. `board_update_files(add, remove)` can also express a rename when `add` and `remove` target the same logical file, but `board_rename_file` is the preferred explicit path.

## Acceptance Criteria

- [ ] `BoardEntry` includes `entry_id` and `previous_attempt` fields, public in `ticket_api`
- [ ] `BoardConfig`, `BoardSnapshot`, `BoardCleanPreview`, `BoardCleanResult` types are public in `ticket_api`
- [ ] `BOARD_ENTRIES`, `BOARD_ACTIVE_INDEX`, and `BOARD_CONFIG` redb tables are created alongside existing tables
- [ ] `board_check_in()` generates `entry_id`, populates `previous_attempt`, enforces WIP limit, detects file conflicts, and creates backward-compatible lease
- [ ] `board_check_out()` accepts optional `handoff_reason`, marks entry completed, and releases lease
- [ ] `board_heartbeat()` updates `last_heartbeat` and returns refreshed entry
- [ ] `board_show()` returns atomic read-only snapshot with `caller_entries` section, stale computation, file ownership map, and warnings
- [ ] `board_configure()` reads/writes board config
- [ ] `board_clean_preview()` returns candidates and a confirmation token
- [ ] `board_clean_apply()` removes only token-identified entries and rejects stale tokens
- [ ] `board_update_files()` modifies file ownership with conflict re-check
- [ ] `board_rename_file()` performs atomic rename with audit event
- [ ] `board_reconcile()` is called by `update_ticket`, `close_ticket`, `cancel_ticket`, `revert_ticket`
- [ ] File conflict detection rejects check-in and marks conflicting entry
- [ ] All board methods use redb write/read transactions for concurrency safety
- [ ] Default stale threshold is one hour and stale entries are surfaced as high-priority human-review items
- [ ] Public `claim()`/`unclaim()` methods are removed or made `pub(crate)` — external callers must use board commands
- [ ] Unit tests cover: check-in happy path, WIP limit rejection, file conflict detection, stale detection, heartbeat renewal, confirmation-token cleanup, reconciliation hooks, and concurrent access
