# indexed

Source: `crates/memory-api/src/storage/indexed.rs`

## Public API

### `IndexedEntity` (Struct)

Metadata stored per-entity in the redb index.
Does not hold full content — that lives in the manifest file on disk.

### `LeaseInfo` (Struct)

Lease record stored in the LEASES redb table.

### `LeaseInfo` (Impl)

