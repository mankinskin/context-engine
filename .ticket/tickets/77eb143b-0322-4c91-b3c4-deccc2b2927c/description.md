## Problem

`run_in_terminal` was the single most-used tool in both analysed sessions — 177 calls in `3e9bc20b`, 121 in `41966513`. Classifying every command shows most of it duplicated capabilities already loaded into the same agent:

| category | `3e9bc20b` | `41966513` |
|---|---|---|
| `grep` / `find` / `ls` / `cat` / `wc` / `head` (peek-mcp + `grep_search` territory) | 83 | 18 |
| CLI shadowing a loaded MCP tool (`ticket.exe get`, `spec.exe get`, `spec.exe health`, `ticket.exe subgraph`) | 33 | 23 |
| `cargo run -p test-cli -- record` instead of `mcp_test-mcp_test_record_execution` | 3 | 0 |
| `cargo build` / `cargo test` (legitimate) | 25 | 34 |
| `git` (legitimate for Commit Agent) | 8 | 23 |

Worst cases:

- Subagent `[11] Materialize spec and validation files` — **72 terminal commands in 42 turns**, including `cargo build -p spec-cli --release` and `cargo run --release -p test-cli -- record ...` to do what `test_record_execution` does in one MCP call.
- Subagent `[9] Implement ticket 41ff230b` — 32 terminal calls, mostly `find . -name "Cargo.toml" | xargs grep -l`, `ls -la`, `grep -n` to locate a crate.
- Subagent `[0] Testing Agent` — 14 terminal commands including `cd ... && cat transcript.json`, when `session_peek_range` exists.

## Why it costs

Each shell round-trip is a full turn, and every turn re-sends the estimated ~37k-token fixed prefix (see `cd19fed4`). 116 of 298 terminal commands across both sessions were substitutable — roughly 116 avoidable turns. The derived figure of ~4.3M input tokens of prefix re-transmission is an estimate pending `9d527ad1`; the command counts themselves are measured.

Secondary effect: shell exploration returns unbounded output. `peek-mcp` and `grep_search` return bounded, structured results; `find`/`ls`/`cat` do not.

## Decision: measure before enforcing

Reviewed 2026-07-27. Three enforcement mechanisms were considered — a hard gate refusing `grep`/`find`/`ls` in `run_in_terminal` when `peek-mcp` is loaded, a soft context-injected substitution table, and measurement first. **Measurement first was chosen.**

Rationale: guidance-based enforcement already failed once here, and a hard gate risks blocking legitimate composed pipelines. Committing to a mechanism before a trend line exists would repeat the mistake this epic documents. This ticket therefore delivers the metric; the enforcement mechanism is a follow-up decision taken on evidence.

## Scope

- Implement a classifier that partitions `run_in_terminal` commands from a session event log into substitutable vs legitimate categories, using the taxonomy in the table above. Ship it in the `b7c61f0e` analyzer, not as a standalone script.
- Establish the baseline formally: 116 of 298 commands substitutable across the two analysed sessions, broken down per category and per sub-agent.
- Report the metric per session and per sub-agent so regressions are attributable.
- Investigate why agents reach for CLI binaries when the equivalent MCP tool is loaded. Two candidate causes are already observed: (a) the MCP call failed first (see `9faa3f5f`), (b) the agent did not know the MCP tool existed among 135 loaded tools (see `cd19fed4`). Report which dominates; do not assume it is defiance of guidance.
- Do **not** implement enforcement under this ticket. Record the trend across at least two post-change benchmark runs, then open a follow-up ticket proposing a mechanism with the evidence attached.

## Acceptance Criteria

1. A classifier exists that partitions `run_in_terminal` commands into substitutable vs legitimate categories, computable from any captured session event log.
2. The classifier reproduces the 116/298 baseline on the two analysed sessions, per category and per sub-agent.
3. The dominant cause of CLI-over-MCP preference is reported with evidence, distinguishing tool-failure fallback from tool-discovery failure.
4. Measured against the benchmark in `10d21210`, the substitutable-shell count is reported for at least two runs, establishing a trend rather than a single data point.
5. A follow-up ticket proposing an enforcement mechanism is opened, citing that trend. Enforcement is explicitly out of scope here.
6. No agent compiles a repo CLI at runtime to perform an operation that a loaded MCP tool already exposes — verified by the classifier flagging zero `cargo run -p *-cli` occurrences.

## Evidence

- Classification script and counts: `tmp/subagent_cost_probe.py`
- `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json`
- `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json`
- Existing guidance that was not followed: `.agents/instructions/orchestration/file-inspection.instructions.md`, `.agents/instructions/orchestration/compact-output.instructions.md`