# Status: TODO

# Project Baseline: Current Persistence and Interface Patterns

## Why this matters

The current `context-engine` workspace already implements a practical persistence model and adapter architecture. Any future task-tracker subsystem should align with these strengths.

## Confirmed Current Persistence Model

From `crates/context-stack/context-api` docs and source:
- Workspace data stored under `.context-engine/<workspace>/`
- Binary graph payload in `graph.bin` (bincode)
- Human-readable metadata in `metadata.json`
- Locking via `.lock` file (`fs2`)
- Explicit `save_workspace` semantics (not auto-save every mutation)
- Atomic writes via temporary-file + rename pattern

Observed from code/docs:
- `crates/context-stack/context-api/README.md`
- `crates/context-stack/context-api/src/workspace/persistence.rs`

## Interface/Adapter Pattern Already In Place

Current tools show a storage-agnostic command interface pattern:
- CLI adapter (`context-cli`)
- HTTP adapter (`context-http`)
- MCP adapter (`context-mcp`)

The system centralizes operations through command dispatch and `WorkspaceManager`, then exposes those over transport-specific layers.

## Architecture Takeaways For Task Tracker

1. Keep core domain storage logic in a central crate/service.
2. Preserve command-style API for parity across CLI/HTTP/MCP.
3. Maintain explicit durability operations (`save`, `checkpoint`, `compact`).
4. Continue atomic-write and lock discipline for crash safety.
5. Reuse tracing/log pipeline for auditable mutation workflows.

## Gaps Relative To Requested Task-Tracker Features

- No native ticket folder schema with required/optional files.
- No first-class dependency graph for tickets/issues.
- No built-in versioned event journal per ticket.
- No full-text ticket index/highlighting subsystem.
- No formal validation framework for ticket manifests and required fields.

## Suggested Integration Strategy

- Add a dedicated task-tracker crate with a storage abstraction.
- Keep compatibility with existing command execution model.
- Introduce a migration-safe persistence backend trait:
  - `FilesystemHybridBackend`
  - `SqliteBackend`
  - `RedbBackend` (experimental)
- Add explicit import/export pathways between folder form and compact binary snapshots.

## TODO

- TODO: Draft trait definitions for backend abstraction.
- TODO: Map current command model to ticket command surface.
- TODO: Prototype ticket workspace under `.context-engine/tickets/<id>/`.
- TODO: Define compatibility story with existing graph operations.
