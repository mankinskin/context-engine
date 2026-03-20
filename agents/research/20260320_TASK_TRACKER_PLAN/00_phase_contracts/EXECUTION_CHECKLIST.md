# Phase 0 Execution Checklist

## Purpose

This checklist converts Phase 0 design work into execution-ready artifacts for implementation.
Phase 1 must not start until all mandatory gates are green.

## Problem/Solution/Reference

1. Problem: concurrent agent updates can diverge state definitions across tools.
Solution: finalize deterministic contracts and validation invariants first.
Reference: convergence and validation patterns from delightful-ai/beads-rs.

2. Problem: orchestration breaks when command/output contracts drift.
Solution: lock machine-readable command schemas with `TaskCommand` as canonical machine protocol and CLI as adapter.
Reference: JSON-first CLI patterns from Dicklesworthstone/beads_rust.

3. Problem: upstream architectures do not natively satisfy distributed ticket folders + schema pluggability.
Solution: implement contracts in this repository and treat upstream as pattern references only.
Reference: both projects.

## Deliverable Files

- [x] `crates/context-tasks/src/model/ticket.rs`
- [x] `crates/context-tasks/src/model/schema.rs`
- [x] `crates/context-tasks/src/model/edge.rs`
- [x] `crates/context-tasks/src/model/event.rs`
- [x] `crates/context-tasks/src/model/query.rs`
- [x] `crates/context-tasks/src/storage/schema.rs`
- [x] `crates/context-tasks/src/watcher/events.rs`
- [x] `crates/context-tasks/src/contracts/command_schema.rs`
- [x] `crates/context-tasks/tests/contracts_manifest_roundtrip.rs`
- [x] `crates/context-tasks/tests/contracts_query_parser.rs`
- [x] `crates/context-tasks/tests/contracts_schema_validation.rs`

## Step-by-Step Execution

### Step 0.1: Freeze Domain Types

- [x] Define `TicketId` as UUID v4.
- [x] Define `TicketManifest` with only required universal fields:
  - `id`
  - `created_at`
- [x] Represent workflow-defined fields as extension map (`extra`).
- [x] Define `TicketTypeSchema` for:
  - dynamic fields
  - configurable states
  - transition constraints

Acceptance criteria:
- [x] Manifest parses with only universal fields.
- [x] Unknown extra fields are preserved through roundtrip serialization.
- [x] Invalid UUID or timestamp fails with structured error.

### Step 0.2: Freeze Relationship Model

- [x] Define `EdgeRecord` with open string `kind`.
- [x] Add edge constraint metadata in type/workflow schema:
  - directed vs undirected
  - acyclic-enforced kinds
- [x] Specify uniqueness key `(from, to, kind)`.

Acceptance criteria:
- [x] Duplicate edge insert attempts are idempotent.
- [x] Invalid edge kind rejected by schema validation.

### Step 0.3: Freeze Filesystem Contract

- [x] Define ticket folder minimum contract:
  - `ticket.toml`
  - optional `assets/`
  - `.ticket-lock`
- [x] Define scan root registry format.
- [x] Define orphan integration contract and parse diagnostics shape.

Acceptance criteria:
- [x] A valid ticket folder can be discovered and normalized.
- [x] Broken `ticket.toml` produces parse diagnostic with path + reason.

### Step 0.4: Freeze Global Index Contract

- [x] Define redb table constants and key/value encoding contracts:
  - `TICKETS`
  - `EDGES`
  - `SCAN_ROOTS`
  - `LEASES`
  - `META`
- [x] Define schema versioning policy in `META`.

Acceptance criteria:
- [x] Table definitions compile and are version-gated.
- [x] Schema version mismatch returns actionable migration error.

### Step 0.5: Freeze Query Language Contract

- [x] Define query AST supporting:
  - free-text terms
  - field predicates
  - ranges
  - logical composition
- [x] Define parser behavior and errors.
- [x] Define field-namespace policy for dynamic type fields.

Acceptance criteria:
- [x] Parsing succeeds for mixed FTS + metadata queries.
- [x] Unknown field emits deterministic parse error and hint.

### Step 0.6: Freeze History Contract

- [x] Define branch-aware history fields:
  - `created_on_branch`
  - `closed_on_branch`
  - `merge_commit`
- [x] Decide git backing mode default:
  - embedded bare repo
  - optional workspace-git mode

Acceptance criteria:
- [x] Decision documented and reflected in contract types.
- [x] Merge-boundary closure path represented in schema.

### Step 0.7: Freeze Command Contracts

- [x] Define machine-readable contract for CLI adapter + agent protocol parity.
- [x] Include `ticket` command set baseline:
  - `create`, `get`, `update`, `list`, `delete`
  - `scan`, `claim`, `unclaim`
  - `search`, `query`
  - `history`, `diff`, `revert`, `finalize-merge`
- [ ] Define machine protocol transport variants:
  - `ticket exec` stdin JSON
  - `ticket exec --batch` transactional JSON batch
  - `ticket serve --stdio` JSONL request/response envelope
- [ ] Define machine protocol self-containment requirements:
  - explicit `index_root`
  - full UUIDs
  - structured patch objects
- [ ] Define optional response `fields` projection contract.
- [x] Define structured error model with stable error codes.

Acceptance criteria:
- [x] Command schemas exported in JSON for tooling.
- [x] All contract tests verify response shape stability.
- [ ] Request envelope shape is stable across `exec`, `serve`, HTTP, and MCP adapters.
- [ ] Transactional batch failure reports deterministic failing command index.

## Mandatory Test Gates (Phase 0 Exit) — SCOPED TO context-tasks

- [x] `cargo fmt -p context-tasks --check`
- [x] `cargo clippy -p context-tasks --all-targets --all-features -- -D warnings`
- [x] `cargo test -p context-tasks -- --nocapture`

Note: Workspace-wide fmt/clippy gates are out of scope for Phase 0 exit. Tracked separately.

## Exit Criteria — ALL GREEN

- [x] All contract files exist and compile.
- [x] All mandatory test gates pass (scoped to context-tasks).
- [x] Branch/history strategy decision is explicit (embedded bare repo default).
- [x] Query grammar and parse error behavior are frozen.
- [x] Schema compatibility policy documented.
- [x] No unresolved contract TODO remains for Phase 1 blockers.

## Handoff To Phase 1 — COMPLETE

- [x] Contract version: 0.1.0 (initial)
- [x] Table schema version: 1
- [x] Query grammar version: 1
- [x] Known deferred items: full runtime schema engine, executor abstraction
- [x] Phase 1 command set: create, get, update, list, delete, scan, search
- [ ] Phase 1 agent transport: `ticket exec`
- [ ] Phase 1.5 agent transport: `ticket serve --stdio`
- [x] Phase 1.5 command set: claim, unclaim
- [x] Phase 2 command set: history, diff, revert, finalize-merge
- [x] Phase 3 command set: search, query (also wired in Phase 1 for FTS)
