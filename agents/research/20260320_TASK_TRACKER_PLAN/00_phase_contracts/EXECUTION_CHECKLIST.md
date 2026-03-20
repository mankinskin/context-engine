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

- [ ] `crates/context-tasks/src/model/ticket.rs`
- [ ] `crates/context-tasks/src/model/schema.rs`
- [ ] `crates/context-tasks/src/model/edge.rs`
- [ ] `crates/context-tasks/src/model/event.rs`
- [ ] `crates/context-tasks/src/model/query.rs`
- [ ] `crates/context-tasks/src/storage/schema.rs`
- [ ] `crates/context-tasks/src/watcher/events.rs`
- [ ] `crates/context-tasks/src/contracts/command_schema.rs`
- [ ] `crates/context-tasks/tests/contracts_manifest_roundtrip.rs`
- [ ] `crates/context-tasks/tests/contracts_query_parser.rs`
- [ ] `crates/context-tasks/tests/contracts_schema_validation.rs`

## Step-by-Step Execution

### Step 0.1: Freeze Domain Types

- [ ] Define `TicketId` as UUID v4.
- [ ] Define `TicketManifest` with only required universal fields:
  - `id`
  - `created_at`
- [ ] Represent workflow-defined fields as extension map (`extra`).
- [ ] Define `TicketTypeSchema` for:
  - dynamic fields
  - configurable states
  - transition constraints

Acceptance criteria:
- [ ] Manifest parses with only universal fields.
- [ ] Unknown extra fields are preserved through roundtrip serialization.
- [ ] Invalid UUID or timestamp fails with structured error.

### Step 0.2: Freeze Relationship Model

- [ ] Define `EdgeRecord` with open string `kind`.
- [ ] Add edge constraint metadata in type/workflow schema:
  - directed vs undirected
  - acyclic-enforced kinds
- [ ] Specify uniqueness key `(from, to, kind)`.

Acceptance criteria:
- [ ] Duplicate edge insert attempts are idempotent.
- [ ] Invalid edge kind rejected by schema validation.

### Step 0.3: Freeze Filesystem Contract

- [ ] Define ticket folder minimum contract:
  - `ticket.toml`
  - optional `assets/`
  - `.ticket-lock`
- [ ] Define scan root registry format.
- [ ] Define orphan integration contract and parse diagnostics shape.

Acceptance criteria:
- [ ] A valid ticket folder can be discovered and normalized.
- [ ] Broken `ticket.toml` produces parse diagnostic with path + reason.

### Step 0.4: Freeze Global Index Contract

- [ ] Define redb table constants and key/value encoding contracts:
  - `TICKETS`
  - `EDGES`
  - `SCAN_ROOTS`
  - `LEASES`
  - `META`
- [ ] Define schema versioning policy in `META`.

Acceptance criteria:
- [ ] Table definitions compile and are version-gated.
- [ ] Schema version mismatch returns actionable migration error.

### Step 0.5: Freeze Query Language Contract

- [ ] Define query AST supporting:
  - free-text terms
  - field predicates
  - ranges
  - logical composition
- [ ] Define parser behavior and errors.
- [ ] Define field-namespace policy for dynamic type fields.

Acceptance criteria:
- [ ] Parsing succeeds for mixed FTS + metadata queries.
- [ ] Unknown field emits deterministic parse error and hint.

### Step 0.6: Freeze History Contract

- [ ] Define branch-aware history fields:
  - `created_on_branch`
  - `closed_on_branch`
  - `merge_commit`
- [ ] Decide git backing mode default:
  - embedded bare repo
  - optional workspace-git mode

Acceptance criteria:
- [ ] Decision documented and reflected in contract types.
- [ ] Merge-boundary closure path represented in schema.

### Step 0.7: Freeze Command Contracts

- [ ] Define machine-readable contract for CLI + HTTP command parity.
- [ ] Include `ticket` command set baseline:
  - `create`, `get`, `update`, `list`, `delete`
  - `scan`, `claim`, `unclaim`
  - `search`, `query`
  - `history`, `diff`, `revert`, `finalize-merge`
- [ ] Define structured error model with stable error codes.

Acceptance criteria:
- [ ] Command schemas exported in JSON for tooling.
- [ ] All contract tests verify response shape stability.

## Mandatory Test Gates (Phase 0 Exit)

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test -p context-tasks contracts_manifest_roundtrip`
- [ ] `cargo test -p context-tasks contracts_query_parser`
- [ ] `cargo test -p context-tasks contracts_schema_validation`
- [ ] `cargo test -p context-tasks -- --nocapture`

## Exit Criteria

Phase 0 is complete only if:

- [ ] All contract files exist and compile.
- [ ] All mandatory test gates pass.
- [ ] No unresolved contract TODO remains for Phase 1 blockers.
- [ ] Branch/history strategy decision is explicit.
- [ ] Query grammar and parse error behavior are frozen.

## Handoff To Phase 1

Before beginning Phase 1, produce a short handoff note with:

- [ ] final contract version id
- [ ] table schema version
- [ ] query grammar version
- [ ] known deferred items (non-blocking)
- [ ] exact command list guaranteed for Phase 1
