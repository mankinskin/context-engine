# Status: TODO

# Embedded SQL Options: SQLite (`rusqlite`) and libSQL

## Scope

Evaluate SQL-first embedded approaches for task tracker metadata, dependencies, transitions, and validation state.

## SQLite via rusqlite

### What exists
- `rusqlite` provides ergonomic SQLite bindings in Rust.
- Optional `bundled` feature simplifies cross-platform builds (especially Windows).
- Supports blob I/O features and many SQLite capabilities via crate features.

### Why it fits this project
- Strong ACID transactions and proven durability.
- Natural representation for ticket dependencies and workflow queries.
- Easy to model:
  - `tickets`
  - `ticket_dependencies`
  - `ticket_states`
  - `ticket_events`
  - `ticket_assets`
  - `ticket_validation_findings`

### Concurrency notes
- WAL mode supports concurrent readers with a single writer.
- Must manage checkpoint strategy and long-running readers.
- WAL has specific constraints (notably same-host shared memory assumptions in default modes).

### Risks / concerns
- Schema migrations require discipline.
- Very large write bursts need careful tuning.
- FS + DB consistency still needs explicit design if artifacts are in folders.

### Source links
- https://github.com/rusqlite/rusqlite
- https://www.sqlite.org/wal.html

## libSQL / Turso ecosystem

### What exists
- SQLite-compatible family targeting distributed and agent/local-first scenarios.
- Ecosystem messaging emphasizes replication/sync, browser/on-device execution, and branching capabilities.

### Potential fit
- Future-ready if multi-device synchronization becomes first-class.
- Keeps SQL model while allowing broader deployment topologies.

### Risks / concerns
- Need precise feature validation against required local-only workflows.
- Operational complexity can exceed pure embedded SQLite for single-machine use.

### Source links
- https://www.turso.tech/libsql
- https://docs.turso.tech/

## Suggested SQL schema sketch (conceptual)

- `tickets(id, title, status, progress, created_at, updated_at, schema_version, manifest_path)`
- `ticket_dependencies(ticket_id, depends_on_ticket_id, kind, created_at)`
- `ticket_events(id, ticket_id, event_type, payload_json, author, ts)`
- `ticket_assets(id, ticket_id, rel_path, mime, hash, size_bytes, required_flag)`
- `ticket_validation(id, ticket_id, rule_id, level, message, rel_path, line, ts)`
- `ticket_refs(id, src_ticket_id, dst_ticket_id, ref_type, context)`

## TODO

- TODO: Create concrete migration plan and naming conventions.
- TODO: Benchmark WAL settings for expected write/read mix.
- TODO: Define transaction wrappers for multi-step ticket updates.
- TODO: Design SQL constraints for required files/fields by ticket type.
