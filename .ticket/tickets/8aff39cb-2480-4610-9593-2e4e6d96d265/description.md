# [Board][Design] Draftboard Data Model, API Contract, and CLI/MCP Surface

## Objective

Produce the implementation-ready contract for the draftboard system: data model, store API, CLI subcommand surface, and MCP tool definitions. This design must be approved before any implementation begins.

## Context

The draftboard extends the existing ticket system with a workspace-scoped coordination layer. It builds on the existing `LeaseInfo` / `claim()` / `unclaim()` infrastructure in `ticket-api` but adds:

- **Board entries** with file ownership tracking
- **WIP limits** (workspace-configurable maximum concurrent entries)
- **Stale detection** with TTL-based heartbeat expiry
- **File conflict detection** across concurrent entries
- **Lock-gated snapshot** for new agent session onboarding
- **Human escalation** flags for conflicts that cannot be auto-resolved

## Data Model Design

### Board Entry

```rust
/// A draftboard entry representing one agent's active work on one ticket.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BoardEntry {
    /// The ticket being worked on.
    ticket_id: Uuid,
    /// Identity of the agent session (e.g. "copilot-session-abc123").
    agent_id: String,
    /// When the agent checked in.
    checked_in_at: DateTime<Utc>,
    /// Last heartbeat timestamp. Updated by `board heartbeat`.
    last_heartbeat: DateTime<Utc>,
    /// Heartbeat TTL in seconds. Entry becomes stale after last_heartbeat + ttl_secs.
    ttl_secs: u64,
    /// Short description of what the agent intends to do.
    intent: String,
    /// Files the agent declares ownership of (workspace-relative paths).
    owned_files: Vec<String>,
    /// Current entry status, computed from heartbeat freshness and conflict state.
    status: BoardEntryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum BoardEntryStatus {
    /// Agent is actively working; heartbeat is fresh.
    Active,
    /// Heartbeat expired past TTL — agent may have crashed or abandoned work.
    Stale,
    /// File ownership overlap detected with another entry — needs human resolution.
    Conflict,
    /// Agent checked out cleanly; entry retained briefly for audit before pruning.
    Completed,
}
```

### Board Configuration

```rust
/// Workspace-level board settings, stored in a redb config table or workspace TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BoardConfig {
    /// Maximum concurrent active entries. Check-in is rejected when this limit is reached.
    /// Default: 5.
    max_wip: u32,
    /// Seconds after last heartbeat before an entry is marked stale.
    /// Default: 600 (10 minutes).
    stale_after_secs: u64,
    /// Whether to auto-prune completed entries older than this many seconds.
    /// Default: 3600 (1 hour). Set to 0 to disable auto-prune.
    auto_prune_completed_after_secs: u64,
}
```

### Board Snapshot

```rust
/// Atomic snapshot of the entire board state, returned by `board show`.
struct BoardSnapshot {
    /// Timestamp of this snapshot.
    captured_at: DateTime<Utc>,
    /// All active entries (not completed/pruned).
    entries: Vec<BoardEntry>,
    /// Current board configuration.
    config: BoardConfig,
    /// Number of active (non-stale, non-completed) entries.
    active_count: u32,
    /// Number of stale entries (flagged for attention).
    stale_count: u32,
    /// Number of entries in conflict state.
    conflict_count: u32,
    /// Whether WIP limit is reached (active_count >= max_wip).
    wip_limit_reached: bool,
    /// File ownership map: file path → agent_id(s) owning it. Entries with >1 owner are conflicts.
    file_ownership: BTreeMap<String, Vec<String>>,
    /// Warnings for the onboarding agent session.
    warnings: Vec<String>,
}
```

### Storage

- **New redb table `BOARD_ENTRIES`**: Key = `"{ticket_id}:{agent_id}"` (compound string key), Value = bincode-serialized `BoardEntry`.
- **New redb table `BOARD_CONFIG`**: Key = `"default"` (single row), Value = bincode-serialized `BoardConfig`.
- Board entries are **ephemeral** — they are not stored on disk as TOML files. They live only in the redb index.
- On check-in, the board also calls `store.claim()` internally for backward compatibility with existing lease consumers.

## API Surface (ticket-api)

New methods on `TicketStore`:

```rust
impl TicketStore {
    /// Check in: register active work. Returns error if WIP limit reached or file conflict detected.
    pub fn board_check_in(&self, entry: BoardCheckIn) -> Result<BoardEntry, StorageError>;

    /// Check out: mark work completed and release file ownership.
    pub fn board_check_out(&self, ticket_id: &Uuid, agent_id: &str) -> Result<(), StorageError>;

    /// Heartbeat: renew TTL for an existing entry.
    pub fn board_heartbeat(&self, ticket_id: &Uuid, agent_id: &str) -> Result<BoardEntry, StorageError>;

    /// Show: lock-gated atomic snapshot of the entire board.
    pub fn board_show(&self) -> Result<BoardSnapshot, StorageError>;

    /// Configure: update board settings.
    pub fn board_configure(&self, config: BoardConfig) -> Result<BoardConfig, StorageError>;

    /// Clean: remove completed and optionally stale entries.
    pub fn board_clean(&self, remove_stale: bool) -> Result<BoardCleanResult, StorageError>;

    /// Update file ownership for an existing entry (add/remove files mid-session).
    pub fn board_update_files(&self, ticket_id: &Uuid, agent_id: &str, add: &[String], remove: &[String]) -> Result<BoardEntry, StorageError>;
}
```

### Check-in Validation Rules

1. **WIP limit**: Count active entries. If `active_count >= max_wip`, reject with `BoardError::WipLimitReached { current, max }`.
2. **File conflict**: Scan all active entries' `owned_files`. If any overlap with the new entry's files, reject with `BoardError::FileConflict { files, conflicting_agent }`. Mark both entries as `Conflict` status.
3. **Duplicate check-in**: If the same `(ticket_id, agent_id)` pair already has an active entry, reject with `BoardError::AlreadyCheckedIn`.
4. **Ticket existence**: Verify ticket exists in the store.
5. **Lease propagation**: On successful check-in, call `store.claim()` to create a backward-compatible lease.

### Error Types

```rust
enum BoardError {
    WipLimitReached { current: u32, max: u32 },
    FileConflict { files: Vec<String>, conflicting_agent: String, conflicting_ticket: Uuid },
    AlreadyCheckedIn { ticket_id: Uuid, agent_id: String },
    NotCheckedIn { ticket_id: Uuid, agent_id: String },
    TicketNotFound(Uuid),
}
```

## CLI Surface (ticket-cli)

New `board` subcommand with sub-subcommands:

```
ticket board show [--json]
    Lock-gated atomic snapshot. Returns BoardSnapshot.
    Default output: human-readable table + warnings.

ticket board check-in <TICKET_ID> --agent <AGENT_ID> [--intent "..."] [--files f1 f2 ...] [--ttl-secs N] [--json]
    Register active work. Validates WIP limit and file conflicts.

ticket board check-out <TICKET_ID> [--agent <AGENT_ID>] [--json]
    Deregister from the board. Releases file ownership.

ticket board heartbeat <TICKET_ID> --agent <AGENT_ID> [--json]
    Renew TTL. Returns updated entry with new expiry.

ticket board configure [--max-wip N] [--stale-after-secs N] [--auto-prune-secs N] [--json]
    View (no args) or update board configuration.

ticket board clean [--include-stale] [--json]
    Remove completed entries. With --include-stale, also remove stale entries.

ticket board update-files <TICKET_ID> --agent <AGENT_ID> [--add f1 f2] [--remove f3 f4] [--json]
    Modify file ownership for an existing entry.
```

## MCP Surface (ticket-mcp)

New MCP tools:

| Tool | Parameters | Returns |
|---|---|---|
| `board_show` | `workspace` | `BoardSnapshot` |
| `board_check_in` | `workspace`, `ticket_id`, `agent_id`, `intent?`, `files[]?`, `ttl_secs?` | `BoardEntry` |
| `board_check_out` | `workspace`, `ticket_id`, `agent_id?` | success/error |
| `board_heartbeat` | `workspace`, `ticket_id`, `agent_id` | `BoardEntry` |
| `board_configure` | `workspace`, `max_wip?`, `stale_after_secs?` | `BoardConfig` |
| `board_clean` | `workspace`, `include_stale?` | `BoardCleanResult` |

## Questions to Resolve

1. Should `board show` use a read-write transaction (exclusive lock) or a read-only transaction? Read-only is faster but cannot update stale status atomically.
2. Should the board auto-heartbeat on `board show` (extend TTL for the calling agent's entries)?
3. Should `board check-in` be integrated into `ticket update --state in-implementation` automatically?
4. Should completed entries be pruned immediately or retained for a configurable audit window?
5. What is the recommended default TTL for agent sessions? (Proposal: 600 seconds / 10 minutes with heartbeat renewal.)

## Alternatives Considered

### A: Extend LeaseInfo directly
- Pros: No new tables; reuses existing infrastructure
- Cons: LeaseInfo keyed by ticket_id only (one holder per ticket); no file ownership; would break existing consumers

### B: File-based board (TOML on disk)
- Pros: Visible in git; easy to inspect
- Cons: No atomic snapshots; file locking is fragile; cross-platform issues

### C: Separate redb tables (chosen)
- Pros: Atomic transactions; compound keys allow multiple agents per ticket; clean separation from core ticket data
- Cons: Board state not visible on disk; lost on redb rebuild (acceptable — board is ephemeral)

## Deliverables

- [ ] Finalized `BoardEntry`, `BoardConfig`, `BoardSnapshot` structs
- [ ] API contract approved (method signatures, validation rules, error types)
- [ ] CLI subcommand arguments and output format approved
- [ ] MCP tool schema approved
- [ ] Storage design approved (redb tables, key scheme, serialization)
- [ ] Question resolution documented

## Acceptance Criteria

- [ ] Data model is concrete, typed, and implementation-ready
- [ ] API contract specifies all validation rules and error cases
- [ ] CLI and MCP surfaces are fully specified with argument types and return shapes
- [ ] Storage design handles concurrent access safely (redb transactions)
- [ ] All open questions are resolved with documented decisions
- [ ] Architecture ticket updated to reference the draftboard design
