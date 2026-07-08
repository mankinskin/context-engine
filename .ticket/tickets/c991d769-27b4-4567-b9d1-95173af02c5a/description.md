# Session Objective
Resolve the current file_length batch for memory-api and reduce findings from the baseline.

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

# Progress Log (2026-07-08)
- Baseline at start of this session: file_length=185, total_findings=210.
- Chunk 1: split helpers out of `memory-api/crates/audit-api/src/index.rs` into `memory-api/crates/audit-api/src/index_helpers.rs`.
  - Result: `index.rs` line count 438 -> 350; specific file_length finding cleared.
  - Validation: cargo check -p audit-api (pass).
- Chunk 2: split move JSON helpers and tests from `memory-api/tools/mcp/audit-mcp/src/server.rs` into `server_move_json.rs` and `server_tests.rs`.
  - Result: `server.rs` line count 485 -> 378; specific file_length finding cleared.
  - Validation: cargo test -p audit-mcp --lib (pass).
- Chunk 3: split tests from `memory-api/crates/test-api/src/lib.rs` into `memory-api/crates/test-api/src/lib_tests.rs`.
  - Result: `lib.rs` line count 422 -> 276; specific file_length finding cleared.
  - Validation: cargo test -p test-api --lib (pass).
- Current audit summary after chunk 3: file_length=182, total_findings=207.

# Acceptance Criteria
- Findings in this batch are resolved or have explicit blocker tickets linked.
- No increase in other categories caused by this batch.
- Batch notes include before and after counts and next unresolved action.

# Handoff Notes
Record exact commands run, resulting counts, and files changed so the next session can continue without rediscovery.