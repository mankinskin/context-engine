# Session Objective
Resolve the current static_complexity batch for memory-api and reduce 28 findings from the baseline.

# Scope Guardrails
- Stay inside memory-api unless a blocker requires a dependency fix outside scope.
- Do not start the next batch until this ticket meets done criteria.

# Implementation Steps
1. Capture exact finding rows for this batch from the baseline audit artifact.
2. Group findings into 2 to 5 micro-chunks and handle one chunk at a time.
3. After each chunk, run the narrowest compile/test check relevant to touched files.
4. Re-run audit summary and record count delta.
5. If blockers remain, create follow-up tickets and link them before handoff.

# Validation Commands
- Full category summary: cargo run -p audit-cli --bin audit -- --json summary --by category .
- Full baseline refresh when needed: cargo run -p audit-cli --bin audit -- --json run .
- Ticket health sanity: ./target/debug/ticket.exe health --workspace . --all --toon

# Acceptance Criteria
- Findings in this batch are resolved or have explicit blocker tickets linked.
- No increase in other categories caused by this batch.
- Batch notes include before and after counts and next unresolved action.

# Progress Log

## Baseline (2026-07-07)
- Artifact: target/tmp/batch3_memory_api_audit_before.json
- Baseline rows (memory-api static_complexity only): target/tmp/batch3_memory_api_baseline_rows.json
- Baseline count: 28

## Chunk 1 (ticket-http: error/graph/workflow)
- Edited:
  - memory-api/tools/http/ticket-http/src/serve/error.rs
  - memory-api/tools/http/ticket-http/src/serve/handlers/graph/traversal.rs
  - memory-api/tools/http/ticket-http/src/serve/handlers/workflow.rs
- Narrow validation:
  - cargo test --manifest-path memory-api/tools/http/ticket-http/Cargo.toml storage_err
  - cargo test --manifest-path memory-api/tools/http/ticket-http/Cargo.toml workflow_next
  - cargo test --manifest-path memory-api/tools/http/ticket-http/Cargo.toml workspace_graph
- Post artifact: target/tmp/batch3_memory_api_chunk1_after.json
- Delta vs baseline: 28 -> 24 (resolved 4, added 0)
  - resolved: storage_err, workspace_graph, workflow_next, workflow_tree_response

## Chunk 2 (ticket-http: list handler)
- Edited:
  - memory-api/tools/http/ticket-http/src/serve/handlers/tickets/read.rs
- Narrow validation:
  - cargo test --manifest-path memory-api/tools/http/ticket-http/Cargo.toml list_tickets
- Post artifact: target/tmp/batch3_memory_api_chunk2_after.json
- Delta vs chunk1: 24 -> 23 (resolved 1, added 0)
  - resolved: list_tickets

## Chunk 3 (ticket-cli/test-cli dispatch routing)
- Edited:
  - memory-api/tools/cli/ticket-cli/src/cli/dispatch.rs
  - memory-api/tools/cli/test-cli/src/lib.rs
- Narrow validation:
  - cargo test --manifest-path memory-api/tools/cli/ticket-cli/Cargo.toml dispatch_
  - cargo test --manifest-path memory-api/tools/cli/test-cli/Cargo.toml dispatch
  - cargo check --manifest-path memory-api/tools/cli/test-cli/Cargo.toml
- Post artifact: target/tmp/batch3_memory_api_chunk3_after.json
- Delta vs chunk2: 23 -> 20 (resolved 3, added 0)
  - resolved: dispatch_store_command_graph, dispatch_store_command_ops, test-cli dispatch

## Chunk 4b (core low-risk routing/parser/store updates)
- Edited:
  - memory-api/crates/memory-matrix/src/matrix.rs
  - memory-api/crates/memory-api/src/model/query.rs
  - memory-api/crates/rule-api/src/store.rs
- Narrow validation:
  - cargo check -p memory-api -p rule-api -p memory-matrix
  - cargo test -p rule-api update
  - cargo test -p memory-api query
  - cargo test -p memory-matrix dispatch
- Post artifact: target/tmp/batch3_memory_api_chunk4b_after.json
- Delta vs chunk3: 20 -> 17 (resolved 3, added 0)
  - resolved: matrix dispatch, parse_field_value, rule-api update

## Category Regression Check
- Compared full findings by category (baseline vs chunk4b):
  - static_complexity: 41 -> 30 (-11)
  - all other categories unchanged
  - increases: none

## Chunk 5b (spec index tree + matrix transport test predicate)
- Edited:
  - memory-api/crates/spec-api/src/store_index.rs
  - memory-api/crates/memory-matrix/tests/matrix.rs
- Narrow validation:
  - cargo test -p spec-api store_index -- --nocapture
  - cargo test -p memory-matrix unwired_transports_are_explicitly_blocked_with_reason -- --nocapture
- Post artifact: target/tmp/batch3_memory_api_chunk5b_after.json
- Delta vs chunk4b: 17 -> 15 (resolved 2, added 0)
  - resolved: render_tree_entry_page, unwired_transports_are_explicitly_blocked_with_reason

## Category Regression Check (chunk4b -> chunk5b)
- static_complexity: 30 -> 28 (-2)
- ticket_graph: 3 -> 6 (+3, pre-existing/parallel drift outside this batch scope)
- all other categories unchanged

## Chunk 6a (memory-matrix MCP dispatch helper split)
- Edited:
  - memory-api/crates/memory-matrix/src/mcp.rs
- Narrow validation:
  - cargo test -p memory-matrix ticket_get_mcp_cell_is_wired_and_passes -- --nocapture
  - cargo test -p memory-matrix ticket_update_mcp_cell_is_wired_and_passes -- --nocapture
  - cargo test -p memory-matrix spec_scan_mcp_cell_is_wired_and_passes -- --nocapture
- Post artifact: target/tmp/batch3_memory_api_chunk6a_after.json
- Delta vs chunk5b (memory-api static_complexity only): 15 -> 13 (resolved 2, added 0)
  - resolved: dispatch_ticket_mcp, dispatch_spec_mcp
- Remaining high-priority hotspot in planned sequence:
  - memory-api/crates/memory-api/src/storage/move_kernel.rs (plan_move, execute_or_resume)

## Ticket Health Sanity
- Ran earlier in this session: ./target/debug/ticket.exe health --workspace . --all --toon
- Result: store-wide warnings exist (mostly missing effort/description on other tickets); no blocker discovered for this batch execution flow.

## Remaining Work
- memory-api static_complexity remaining in this batch: 13
- Next suggested chunk: memory-api/crates/memory-api/src/storage/move_kernel.rs (2 findings), then memory-api/crates/ticket-api/src/storage/store.rs (1 finding), then memory-api/crates/session-api/src/hook.rs (1 finding).

# Handoff Notes
Record exact commands run, resulting counts, and files changed so the next session can continue without rediscovery.