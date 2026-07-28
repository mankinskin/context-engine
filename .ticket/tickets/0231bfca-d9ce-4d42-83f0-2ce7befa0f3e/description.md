## Problem

`77eb143b` shipped the classifier and measured — not enforced — CLI-over-MCP shell usage. On the two checked-in baseline sessions (`3e9bc20b…`, `41966513…`), replayed through `compute_delegation_cost_report_from_events`:

| category | `3e9bc20b…` | `41966513…` | combined |
|---|---|---|---|
| `read_like_exploratory` | 77 | 28 | 105 |
| `cli_shadowing_mcp` | 18 | 20 | 38 |
| `cargo_run_cli` | 16 | 0 | 16 |
| `legitimate_dev` | 41 | 79 | 120 |
| `other` | 30 | 25 | 55 |
| total `run_in_terminal` | 182 | 152 | 334 |

`cargo_run_cli` (AC6: compiling/running a repo CLI at runtime instead of calling an already-loaded MCP tool) is **16, not 0**, entirely from one sub-agent span ("Materialize spec and validation files").

`mcp_tool_discovery_failure_count` (17 + 19 = 36) dominates `mcp_tool_failure_fallback_count` (0 + 1 = 1) by roughly 36:1 — the dominant cause of `cli_shadowing_mcp` shell usage is that the equivalent MCP tool was never invoked in the span at all, not that it was tried and failed. This points at a tool-discovery/guidance problem, not a reliability problem with the MCP tools themselves.

This is a **two-session trend**, not enforcement. `77eb143b` deliberately deferred building a mechanism until this trend existed (see its ticket description's "Decision: measure before enforcing").

## Scope

Propose (and, once approved, implement) one concrete enforcement mechanism for the recurring patterns above:

1. A hard or soft gate on `run_in_terminal` when a `read_like_exploratory` or `cli_shadowing_mcp` head is detected and the equivalent tool (`peek-mcp`, `grep_search`, `read_file`, or the matching `*-mcp` tool) is already loaded in the calling agent's grant set.
2. Specific handling for `cargo_run_cli`: block or strongly warn on `cargo run -p *-cli` when a loaded MCP tool covers the same operation (per the classifier's `CLI_SHADOW_BASENAMES`/`mcp_family_for_command` mapping in `memory-api/crates/session-api/src/delegation_cost.rs`).
3. Given the dominant cause is tool-discovery failure (not tool-failure fallback), prioritize a mechanism that improves *discovery* (e.g. context-injected substitution table, per-agent-template tool reminders) over one that only blocks after the fact.

Do not implement without first deciding the mechanism against the evidence above — this ticket starts in `new`/planning, same review gate `77eb143b` used.

## Acceptance Criteria

1. A concrete enforcement mechanism is proposed and reviewed against the `77eb143b` classifier's category/per-sub-agent evidence (cite the table above).
2. The proposal explicitly addresses the tool-discovery-failure dominant cause (36:1 over tool-failure fallback), not just a blanket shell gate.
3. The proposal addresses `cargo_run_cli` (currently 16, non-zero) specifically, since AC6 in `77eb143b` requires this to trend toward zero.
4. Once implemented, a post-change session capture is replayed through the same `compute_delegation_cost_report_from_events` harness and the `cli_shadowing_mcp`/`cargo_run_cli`/`read_like_exploratory` counts and the `substitutable_shell_count`/`run_in_terminal` ratio are compared against the `77eb143b` baseline above per the `10d21210` benchmark's replay-only architecture (no new live A/B run).

## Evidence

- Classifier: [memory-api/crates/session-api/src/delegation_cost.rs](../../../memory-api/crates/session-api/src/delegation_cost.rs)
- Baseline report: [.benchmark/10d21210/baseline/delegation_cost_report.json](../../../.benchmark/10d21210/baseline/delegation_cost_report.json)
- Benchmark scenario/thresholds: [.benchmark/10d21210/README.md](../../../.benchmark/10d21210/README.md)
- Originating ticket: [77eb143b](../77eb143b-0322-4c91-b3c4-deccc2b2927c/ticket.toml)