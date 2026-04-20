---
description: "Implement ticket 82652305: Refactor ticket-api to depend on memory-api"
---

# Ticket 82652305 — Refactor ticket-api to depend on memory-api

## Goal

Refactor `ticket-api` to be a thin domain layer on top of `memory-api`. Remove all duplicated generic code. Keep only ticket-specific domain logic.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update 82652305 --to-state in-implementation
./target/debug/ticket.exe board check-in 82652305 --agent-id copilot --intent "refactoring ticket-api to re-export memory-api" --files "crates/ticket-api/src/lib.rs,crates/ticket-api/src/storage/store.rs" --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update 82652305 --to-state in-review
# Then run the review checklist from .github/instructions/ticket-system.instructions.md
```

## Context

- `memory-api` already exists at `crates/memory-api/` and is a dependency in `crates/ticket-api/Cargo.toml`
- Both crates currently have parallel module structures: `model/`, `storage/`, `error.rs`, `workspace.rs`
- `ticket-api` has additional domain modules: `contracts/`, `execution/`, `watcher/`

## Current Structure (ticket-api/src/)

```
lib.rs           → pub mod + re-exports board types
error.rs         → ProtocolError + StorageError
model/           → ticket.rs (TicketManifest), edge.rs, schema_registry.rs, filesystem.rs, query.rs
storage/         → store.rs (TicketStore), index.rs (RedbIndexStore), search.rs (TantivySearchIndex),
                   ticket_fs.rs (TicketFs), indexed.rs, board.rs, board_audit.rs
workspace.rs     → workspace resolution
contracts/       → TicketCommand, command schema export (KEEP)
execution/       → sandbox, Copilot provider (KEEP)
watcher/         → file watcher (KEEP)
```

## Implementation Steps

### Step 1: Identify identical types

Compare these file pairs — they should be identical or near-identical after the memory-api extraction:

| ticket-api | memory-api |
|---|---|
| `model/ticket.rs` (TicketManifest, TicketId) | `model/entity.rs` (EntityManifest, EntityId) |
| `model/edge.rs` (EdgeRecord) | `model/edge.rs` (EdgeRecord) |
| `model/schema_registry.rs` | `model/schema_registry.rs` |
| `model/filesystem.rs` (ScanRoot) | `model/filesystem.rs` (ScanRoot) |
| `model/query.rs` | `model/query.rs` |
| `storage/index.rs` (RedbIndexStore) | `storage/index.rs` (RedbIndexStore) |
| `storage/search.rs` (TantivySearchIndex) | `storage/search.rs` (TantivySearchIndex) |
| `storage/ticket_fs.rs` (TicketFs) | `storage/entity_fs.rs` (EntityFs) |
| `storage/indexed.rs` | `storage/indexed.rs` |
| `storage/board.rs` | `storage/board.rs` |
| `storage/board_audit.rs` | `storage/board_audit.rs` |
| `error.rs` (StorageError) | `error.rs` (StorageError) |
| `workspace.rs` | `workspace.rs` |

### Step 2: Replace duplicated modules with re-exports

For each identical pair, replace the ticket-api file content with re-exports:

```rust
// crates/ticket-api/src/model/ticket.rs
pub use memory_api::model::entity::{EntityManifest as TicketManifest, EntityId as TicketId};
```

```rust
// crates/ticket-api/src/model/edge.rs
pub use memory_api::model::edge::*;
```

```rust
// crates/ticket-api/src/storage/index.rs
pub use memory_api::storage::index::*;
```

Do this for ALL duplicated modules. The goal: ticket-api has ZERO copies of generic storage code.

### Step 3: Update TicketStore

`TicketStore` in `storage/store.rs` should keep its domain methods but delegate storage to memory-api types:

```rust
use memory_api::storage::index::RedbIndexStore;
use memory_api::storage::search::TantivySearchIndex;
use memory_api::model::schema_registry::SchemaRegistry;
// ... etc
```

Update all `use crate::` imports that reference now-re-exported types to use either `crate::` (which re-exports from memory-api) or `memory_api::` directly.

### Step 4: Update lib.rs re-exports

```rust
// Re-export memory-api board types (these come from memory-api now)
pub use memory_api::{
    BoardCleanPreview, BoardCleanResult, BoardConfig, BoardEntry, BoardEntryStatus, BoardError,
    BoardReconcileResult, BoardSnapshot, ReconcileAction,
};
```

### Step 5: Fix all compilation errors

After replacing modules, fix any import paths in:
- `contracts/`
- `execution/`
- `watcher/`
- `storage/store.rs` (domain methods)

### Step 6: Remove heavy dependencies from ticket-api

After re-exporting from memory-api, ticket-api no longer needs these direct dependencies (they come transitively):
- `redb`
- `tantivy`
- `bincode`
- `sha2`
- `fs4`

Check if they can be removed from `ticket-api/Cargo.toml`. Only remove if all compilation succeeds without them.

## Validation

```bash
cargo test -p ticket-api
cargo test -p ticket-cli
cargo test -p ticket-http
cargo test -p ticket-mcp
cargo check -p ticket-viewer
```

ALL must pass. Public API surface must remain unchanged for downstream consumers.

## Key Constraint

Do NOT rename `TicketManifest` to `EntityManifest` in downstream code. Use type aliases to maintain backward compatibility:

```rust
pub type TicketManifest = memory_api::model::entity::EntityManifest;
pub type TicketId = memory_api::model::entity::EntityId;
```
