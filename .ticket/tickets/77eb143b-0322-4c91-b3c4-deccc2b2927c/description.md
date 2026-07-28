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
2. **(Restated after review round 2 — see "Post-fix corrected numbers" below.)** The classifier's `shell_command_categories.read_like_exploratory` count reproduces **118 of 334** combined `run_in_terminal` calls on the two analysed sessions (`3e9bc20b`: 87/182, `41966513`: 31/152), per category and per sub-agent. This replaces the prior 116/298 target: the corrected classifier numerator (118) essentially reproduces the original ad-hoc 116 (off by 2, ≈1.7%), while the denominator (334 vs 298, off by 36, ≈12%) does not fully reproduce — most plausibly because the original `tmp/subagent_cost_probe.py` script (now deleted, not available to re-diff) counted `run_in_terminal` calls or bucketed categories differently than this analyzer's raw-event reconstruction. Given how closely the numerator lands, 116/298 is best read as a classifier-implementation difference (bug-for-bug), not evidence of a genuinely different session slice — the "unreproducible due to different session slice" justification used to originally amend this AC does not hold up under the corrected numbers.
3. The dominant cause of CLI-over-MCP preference is reported with evidence, distinguishing tool-failure fallback from tool-discovery failure.
4. Measured against the benchmark in `10d21210`, the substitutable-shell count is reported for at least two runs, establishing a trend rather than a single data point.
5. A follow-up ticket proposing an enforcement mechanism is opened, citing that trend. Enforcement is explicitly out of scope here.
6. **(Rewritten after review round 2.)** The classifier detects and reports every `cargo run -p *-cli` occurrence in the baseline (currently **16**, non-zero) so it can be driven toward zero by the `0231bfca` enforcement follow-up; this ticket does not require the baseline count to already be zero. (Previously worded as "verified by the classifier flagging zero `cargo run -p *-cli` occurrences," which is self-contradictory against a historical, pre-enforcement baseline — the zero-occurrence property belongs to future post-change sessions, not this baseline.)

## Post-fix corrected numbers (review round 2, 2026-07-28)

**Defect fixed:** `classify_shell_command`'s read-like branch ran `command_head` on the raw, un-stripped command while every other category branch stripped a leading `cd <path> &&`/`;` chain first. Commands like `cd .session/sessions/<id> && grep -A 30 '...' transcript.json | head -40` therefore had head `"cd"`, missed the read-like check, and fell through to `Other`. Fixed by stripping the cd-chain before the read-like check too (`memory-api/crates/session-api/src/delegation_cost.rs`), with a regression test using a real cd-chain + read-like shape drawn from the `3e9bc20b` baseline log (event `3fc6d1d5-1e14-4195-9d76-cf0ded4fa8ed`).

**Corrected combined `shell_command_categories` (334 total `run_in_terminal` calls, both sessions):**

| category | `3e9bc20b` (182 calls) | `41966513` (152 calls) | combined | pre-fix combined |
|---|---|---|---|---|
| `read_like_exploratory` | 87 | 31 | **118** | 105 |
| `other` | 20 | 22 | **42** | 55 |
| `cli_shadowing_mcp` | 18 | 20 | 38 | 38 (unchanged) |
| `cargo_run_cli` | 16 | 0 | 16 | 16 (unchanged) |
| `legitimate_dev` | 41 | 79 | 120 | 120 (unchanged) |

Only `read_like_exploratory`/`other` moved (13 commands reclassified, matching the "10 in `3e9bc20b`, 3 in `41966513`" impact estimate); `cli_shadowing_mcp`, `cargo_run_cli`, and `legitimate_dev` are unaffected because their branches already stripped the cd-chain correctly before this fix.

**`substitutable_shell_count` (10d21210's AC4/epic-AC4 threshold metric) is unchanged at 105/334** — that metric intentionally classifies by the raw command head (a different, narrower definition than `shell_command_categories`) and was never affected by this bug, so none of the `10d21210` README's cited threshold rows (fb14754e AC4/AC5, 66acb737 AC5, 46d8b25d AC5, 77eb143b AC4, cc3324c9 AC5, epic 79c4ac3e AC4) change numerically. The `77eb143b` AC4 row's baseline-note text was updated to explain this distinction.

**AC3 discovery-vs-fallback split (combined): 36 discovery-failure : 1 failure-fallback : 1 ambiguous** — unchanged by this fix (the `cli_shadowing_mcp` branch was never affected by the cd-strip bug). The parent-bucket `ever_failed = false` hardcode is a documented known limitation (see code comment at the AC3 split computation in `delegation_cost.rs`): parent-level tool calls have no failure log parallel to `SubAgentDelegationReport::failures`, so a parent-level `cli_shadowing_mcp` call can only land in discovery-failure or ambiguous, never failure-fallback. All `cli_shadowing_mcp` occurrences in this baseline are sub-agent-bucketed, so this does not change the 36:1 conclusion.

**Honest verdict on 116/298:** the corrected numerator (118) is within 2 of the original ad-hoc 116 — essentially reproducible. The denominator (334 vs 298) is not reproduced and cannot be fully explained since `tmp/subagent_cost_probe.py` no longer exists to diff against; the most likely explanation is a different definition of "total `run_in_terminal` calls counted" (e.g. parent-bucket inclusion) rather than a different session slice, given the two analysed session IDs are identical. This ticket does not retro-fit category rules to hit exactly 116 — 118 is what the corrected classifier computes and is the number this ticket's evidence trail now cites.

## Evidence

- Classification script and counts (historical only; deleted, not available for diffing): `tmp/subagent_cost_probe.py`
- `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json`
- `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json`
- Corrected baseline artifact: `.benchmark/10d21210/baseline/delegation_cost_report.json` (regenerated via `generate_checked_in_baseline_report --ignored`)
- Existing guidance that was not followed: `.agents/instructions/orchestration/file-inspection.instructions.md`, `.agents/instructions/orchestration/compact-output.instructions.md`