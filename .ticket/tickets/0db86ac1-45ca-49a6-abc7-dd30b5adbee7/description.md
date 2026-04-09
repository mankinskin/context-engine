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
    pub ticket_id: Uuid,
    pub agent_id: String,
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
const BOARD_CONFIG: TableDefinition<&str, &[u8]> = TableDefinition::new("board_config");
```

### Check-in Validation Sequence

1. Read `BoardConfig` from `BOARD_CONFIG` table (or use `Default::default()` if absent).
2. Scan `BOARD_ENTRIES` table; count active entries.
3. If `active_count >= config.max_wip`, return `BoardError::WipLimitReached`.
4. If `(ticket_id, agent_id)` key already exists and status is Active/Stale, return `BoardError::AlreadyCheckedIn`.
5. Build file ownership map from all active entries. Check for overlap with requested `owned_files`.
6. If overlap found: mark conflicting entry as `Conflict`, return `BoardError::FileConflict`.
7. Insert new `BoardEntry` with status `Active`, `checked_in_at = now`, `last_heartbeat = now`.
8. Call `self.claim(ticket_id, agent_id, ttl_secs, Some(intent))` for backward-compatible lease.
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
- `board_clean()` is always an explicit operator-driven step.
- `board_clean(remove_stale = true)` is intended for post-review cleanup after the user confirms the stale entries should be removed.

## Acceptance Criteria

- [ ] `BoardEntry`, `BoardConfig`, `BoardSnapshot` types are public in `ticket_api`
- [ ] `BOARD_ENTRIES` and `BOARD_CONFIG` redb tables are created alongside existing tables
- [ ] `board_check_in()` enforces WIP limit, detects file conflicts, and creates backward-compatible lease
- [ ] `board_check_out()` marks entry completed and releases lease
- [ ] `board_heartbeat()` updates `last_heartbeat` and returns refreshed entry
- [ ] `board_show()` returns atomic read-only snapshot with stale computation, file ownership map, and warnings
- [ ] `board_configure()` reads/writes board config
- [ ] `board_clean()` removes only audit-window-eligible completed entries and optionally stale entries after explicit operator action
- [ ] `board_update_files()` modifies file ownership with conflict re-check
- [ ] File conflict detection rejects check-in and marks conflicting entry
- [ ] All board methods use redb write/read transactions for concurrency safety
- [ ] Default stale threshold is one hour and stale entries are surfaced as high-priority human-review items
- [ ] Unit tests cover: check-in happy path, WIP limit rejection, file conflict detection, stale detection, heartbeat renewal, explicit cleanup gating, and concurrent access
