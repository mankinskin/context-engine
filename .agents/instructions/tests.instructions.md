---
description: "Use when creating or editing tests, benchmarks, or stress tests. Covers tracing setup, test-log debugging, Criterion benchmarks, and HTTP-level validation."
applyTo: "**/tests/**,**/*test*,**/benches/**"
---

## Quick Reference — Common Commands

```bash
# Run a single test by name (fastest, use first)
cargo test -p <crate> <test_name> -- --nocapture

# Run all tests in a crate
cargo test -p ticket-api
cargo test -p ticket-http

# Run Criterion benchmarks (ticket-api graph pipeline)
cargo bench --bench graph_ops -p ticket-api

# HTTP-level stress test (requires running ticket-viewer server)
python tools/http/stress_graph.py          # concurrency sweep 2–32
python tools/http/bench2.py               # verbose per-request phase timing

# Full workspace test (slow — only after local crate tests pass)
cargo test
```

## Criterion Benchmarks

The BFS graph query pipeline is benchmarked in `crates/ticket-api/benches/graph_ops.rs`.
Run with: `cargo bench --bench graph_ops -p ticket-api`

| Benchmark | What it measures |
|---|---|
| `phase1_list_all_edges` | ReDB edge table scan (~630 edges) |
| `phase2_bfs_in_memory` | Pure in-memory BFS, no DB |
| `phase3_get_indexed_many` | Batch metadata fetch (1 ReDB transaction, 39 nodes) |
| `phase3_get_indexed_one_by_one` | Per-node fetch baseline (39 separate transactions) |
| `pipeline_full` | All 3 phases end-to-end |
| `pipeline_concurrent/{2,4,8,16,32}` | N threads barrier-synchronized |

The fixture builds 360 tickets + ~630 edges once per process (via `OnceLock`).

When adding a new storage-layer optimization, add a matching Criterion benchmark that shows the before/after comparison.

## HTTP-Level Stress Testing

`tools/http/stress_graph.py` — concurrency sweep (phases 1–3, with optional soak):

```bash
python tools/http/stress_graph.py                    # default workspace, depth=4
python tools/http/stress_graph.py --base-url http://127.0.0.1:3002 --depth 4
```

`tools/http/bench2.py` — verbose single-run timing including server-side phase breakdown from the `stats` field in the response body.

**Windows note**: always use `127.0.0.1` (not `localhost`) in `--base-url`. Windows resolves `localhost` to IPv6 (`::1`) first; the server only binds IPv4, causing ~2s connection timeout per request before fallback.

## Deploying ticket-viewer for HTTP Testing

```bash
# Build the binary (must build the viewer, not just the library)
cargo build -p ticket-viewer --release

# Deploy and restart
viewer-ctl stop ticket-viewer
viewer-ctl install ticket-viewer
viewer-ctl start ticket-viewer
```

The binary is `~/.cargo/bin/ticket-viewer.exe`. Building only `-p ticket-http` produces
the library but not the binary; the server will be stale until the viewer crate is rebuilt.

## Tracing Setup

For tracing-based tests, initialize tracing with graph context:

```rust
let _tracing = init_test_tracing!(&graph);
```

This improves readability of tokens and graph state in logs.

## Debug Workflow

When a test fails:
1. Run targeted tests first.
2. Inspect `target/test-logs/` for full trace output.
3. Use log-viewer MCP tools (`query_logs`, `search_all_logs`) with jq filters instead of parsing logs manually.
4. Re-run the nearest required validation after each local fix until it passes or the failure repeats without new signal.
5. If the failure remains a blocker, record the failing command, log path, and current diagnosis in the ticket/spec status summary instead of dropping the validation step.

## Test Execution Strategy

- Start with nearest unit/integration tests.
- Expand to crate-level runs once local failures are resolved.
- Keep working outward until the required validation passes or you have a clearly repeated blocker to report.
- Prefer the strongest focused validation surface already owned by the changed tool or crate; run the underlying command directly and record the exact command or manual step in ticket/spec summaries.
- For documentation or generated-guidance checks, run the relevant validation command directly and record unsupported coverage or manual follow-up explicitly.
- If dedicated automation is unavailable, use the closest manual or command-line validation path and record the limitation in the status summary.
- Avoid unrelated full-workspace test runs unless required.

For frontend-impacting changes:

- Run lint and typecheck in each affected frontend package.
- Run nearest unit/component tests for changed UI code.
- Run at least one browser-based end-to-end path that covers changed UX behavior.

For viewer/API integration changes:

- Add or run assertions that verify the viewer contract with context-api or ticket-api for changed endpoints.
- For filesystem-backed behaviors, include path-handling and access-boundary assertions.

For performance-sensitive paths (storage, BFS, graph queries):

- Add or run a Criterion benchmark in `crates/<crate>/benches/`.
- Confirm `phase3_get_indexed_many` is used instead of repeated `get_indexed()` calls.

For regression fixes:

- Prefer a failing reproducer assertion before or with the fix.
- Keep regression coverage focused on the reported failure mode.

## Assertions

- Prefer assertions that check behavior, not incidental implementation details.
- Keep regression tests focused on the bug or contract being changed.

## Recording Validation Evidence (test-api / test-mcp)

Validation results live in a queryable test-result store (`test-api`), not inline in tickets. Record a `ValidationSpec` for each check and a `ValidationExecution` for each run, then reference the stored entries from the ticket.

Store layout (mirrors `.ticket` / `.spec`):

```text
<store-root>/.test/<workspace>/specs/<spec-id>.json
<store-root>/.test/<workspace>/executions/<execution-id>.json
```

### Record via test-mcp (preferred)

Use the `test-mcp` MCP tools when available:

- `test_record_spec` — create/overwrite a validation spec (`id`, `title`, `command`, `detail`, `ticket_ids`, `spec_ids`, `acceptance_criterion_ids`).
- `test_record_execution` — record an outcome (`id`, `validation_spec_id`, `outcome` = `passed|failed|blocked`, `executed_at` RFC3339, `detail`, `ticket_ids`, `spec_ids`, `log_ids`).
- `test_get_spec` / `test_get_execution` — fetch one entry by id.
- `test_list_specs` — list all validation specs.
- `test_list_executions` — query executions by `ticket_id`, `validation_spec_id`, and/or `outcome`.

Always set `ticket_ids` on executions so the evidence can be queried back from the owning ticket.

### Record via the `test` CLI (fallback)

```bash
# Record a validation spec
./target/debug/test.exe --store-root "$PWD/.test" \
  record-spec --id vt-core-tests --title "Core unit tests" \
  --command "cargo test -p ticket-vscode-core" --ticket <ticket-id>

# Record an execution linked to the ticket
./target/debug/test.exe --store-root "$PWD/.test" \
  record --id exec-vt-core-tests-20260615 --spec-id vt-core-tests \
  --outcome passed --detail "16 passed" --ticket <ticket-id>

# Query the evidence linked to a ticket
./target/debug/test.exe --store-root "$PWD/.test" --toon list --ticket <ticket-id>
```

### Reference evidence from a ticket

Instead of pasting verbose results into the ticket description, add a concise pointer:

- the store root (e.g. `memory-api/.test/default/`),
- the validation spec ids and execution ids, and
- the `test ... list --ticket <id>` query that reproduces the evidence.

Keep `blocked` outcomes visible in the ticket with their reason, but let the store hold the full command and outcome trail.
