---
description: "Implement ticket d5722e8e: EntityStore convenience facade in memory-api"
---

# Ticket d5722e8e — EntityStore Convenience Facade

## Goal

Add an `EntityStore` struct to `memory-api` that composes `RedbIndexStore`, `EntityFs`, and `TantivySearchIndex` into a single type. This is a pure composition convenience — no new logic.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update d5722e8e --to-state in-implementation
./target/debug/ticket.exe board check-in d5722e8e --agent-id copilot --intent "adding EntityStore facade to memory-api" --files "crates/memory-api/src/storage/mod.rs,crates/memory-api/src/storage/entity_store.rs" --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update d5722e8e --to-state in-review
```

## Context

- `memory-api` at `crates/memory-api/` has three storage components:
  - `storage::index::RedbIndexStore` — SQLite-backed metadata index (despite the name, now backed by SQLite)
  - `storage::entity_fs::EntityFs` — filesystem operations (scan, read/write TOML manifests)
  - `storage::search::TantivySearchIndex` — full-text search
- Currently `TicketStore` in ticket-api manually composes `RedbIndexStore` + `TantivySearchIndex` + `SchemaRegistry`
- The new `EntityStore` gives downstream crates a single entry point

## Implementation

### Step 1: Create `crates/memory-api/src/storage/entity_store.rs`

```rust
use std::path::{Path, PathBuf};

use crate::error::StorageError;
use crate::model::entity::EntityManifest;
use crate::model::schema_registry::SchemaRegistry;
use crate::storage::entity_fs::EntityFs;
use crate::storage::index::RedbIndexStore;
use crate::storage::search::TantivySearchIndex;

/// Convenience facade composing all three storage layers.
pub struct EntityStore {
    pub index: RedbIndexStore,
    pub fs: EntityFs,  // Note: EntityFs may not exist yet — check if memory-api has it
    pub search: TantivySearchIndex,
    pub schema_registry: SchemaRegistry,
    pub index_root: PathBuf,
}

impl EntityStore {
    /// Open (or create) an entity store.
    ///
    /// `index_root` is the directory for SQLite + Tantivy index files.
    pub fn open(index_root: &Path) -> Result<Self, StorageError> {
        Self::open_with(index_root, SchemaRegistry::with_builtins())
    }

    /// Open with a custom schema registry.
    pub fn open_with(index_root: &Path, schema_registry: SchemaRegistry) -> Result<Self, StorageError> {
        std::fs::create_dir_all(index_root)?;
        let db_path = index_root.join("tickets.db");
        let search_dir = index_root.join("search_index");

        let index = RedbIndexStore::open(&db_path)?;
        let search = TantivySearchIndex::open_or_create(&search_dir)?;

        Ok(Self {
            index,
            fs: EntityFs::new(),  // or however EntityFs is constructed
            search,
            schema_registry,
            index_root: index_root.to_path_buf(),
        })
    }

    pub fn schema_registry(&self) -> &SchemaRegistry {
        &self.schema_registry
    }
}
```

**IMPORTANT**: Before writing this, check the actual API of `EntityFs`, `RedbIndexStore`, and `TantivySearchIndex` in memory-api. The constructor signatures above are approximate — match the real ones.

### Step 2: Register the module

In `crates/memory-api/src/storage/mod.rs`, add:

```rust
pub mod entity_store;
pub use entity_store::EntityStore;
```

### Step 3: Re-export from lib.rs

In `crates/memory-api/src/lib.rs`, add `EntityStore` to the re-exports:

```rust
pub use storage::EntityStore;
```

### Step 4: Delegate common operations

Look at what `TicketStore` does in `crates/ticket-api/src/storage/store.rs` and identify methods that are purely generic (not ticket-domain-specific). Move those to `EntityStore`:

- `open()` / `open_with()` — already done above
- `add_scan_root()` / `list_scan_roots()`
- `scan()` — coordinate fs scan + index update + search index update
- `search()` — delegate to TantivySearchIndex
- `get_indexed()` / `list_indexed()` — delegate to RedbIndexStore
- Edge management: `add_edge()`, `remove_edge()`, `list_edges()`

Only move methods that have NO ticket-specific logic. If a method references `TicketManifest` specifically (not `EntityManifest`), leave it in `TicketStore`.

### Step 5: Tests

Add a basic test in `crates/memory-api/src/storage/entity_store.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_store_open() {
        let tmp = tempfile::tempdir().unwrap();
        let store = EntityStore::open(tmp.path()).unwrap();
        assert!(store.index_root.exists());
    }
}
```

## Validation

```bash
cargo test -p memory-api
cargo check -p ticket-api  # ensure no breakage
```

## Key Constraints

- Do NOT break any existing memory-api public API
- Do NOT modify ticket-api in this ticket (that's ticket 82652305)
- EntityStore is opt-in — downstream crates can use it or continue using individual stores
