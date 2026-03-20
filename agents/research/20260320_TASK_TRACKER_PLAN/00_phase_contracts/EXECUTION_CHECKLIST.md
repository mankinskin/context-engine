# Phase 0 Execution Checklist

## Purpose

This checklist converts Phase 0 design work into execution-ready artifacts for implementation.
Phase 1 must not start until all mandatory gates are green.

## Problem/Solution/Reference

1. Problem: concurrent agent updates can diverge state definitions across tools.
Solution: finalize deterministic contracts and validation invariants first.
Reference: convergence and validation patterns from delightful-ai/beads-rs.

2. Problem: orchestration breaks when command/output contracts drift.
Solution: lock machine-readable command schemas with acceptance tests.
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

- [x] Define machine-readable contract for CLI + HTTP command parity.
- [x] Include `ticket` command set baseline:
  - `create`, `get`, `update`, `list`, `delete`
  - `scan`, `claim`, `unclaim`
  - `search`, `query`
  - `history`, `diff`, `revert`, `finalize-merge`
- [x] Define structured error model with stable error codes.

Acceptance criteria:
- [x] Command schemas exported in JSON for tooling.
- [x] All contract tests verify response shape stability.

## Mandatory Test Gates (Phase 0 Exit)

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] `cargo test -p context-tasks contracts_manifest_roundtrip`
- [x] `cargo test -p context-tasks contracts_query_parser`
- [x] `cargo test -p context-tasks contracts_schema_validation`
- [x] `cargo test -p context-tasks -- --nocapture`

Current status note:
- Workspace-wide fmt/clippy currently fail in existing, unrelated crates/files outside `context-tasks`.
- `context-tasks` gates are green (`cargo clippy -p context-tasks ...` and `cargo test -p context-tasks ...`).

## Exit Criteria

Phase 0 is complete only if:

- [ ] All contract files exist and compile.
- [ ] All mandatory test gates pass.
- [ ] No unresolved contract TODO remains for Phase 1 blockers.
- [x] All contract files exist and compile.
- [x] Branch/history strategy decision is explicit.
- [x] Query grammar and parse error behavior are frozen.

## Handoff To Phase 1

Before beginning Phase 1, produce a short handoff note with:

- [ ] final contract version id
- [ ] table schema version
- [ ] query grammar version
- [ ] known deferred items (non-blocking)
- [ ] exact command list guaranteed for Phase 1
