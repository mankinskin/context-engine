## Problem

Handoff packages and delegation prompts name crates and components but not their physical location. Sub-agents then guess, fail, and fall back to expensive shell exploration.

Observed failures in `3e9bc20b`:

- 5 `read_file` failures. The agent's own recovery reasoning: *"Hmm, let me check the correct path. According to the validation command `rtk cargo test -p session-api --lib`, the crate is session-api. Let me find where it actually is."* It had guessed `memory-api/session-api/src`; the real path is `memory-api/crates/session-api/src`.
- Subagent `[9]` then spent 32 terminal calls on `find memory-api/session-api/src -name "*.rs"`, `find . -name "Cargo.toml" -type f | xargs grep -l 'name = "session-api"'`, `ls -la memory-api/crates/session-api/src/`, and similar.
- Subagent `[11]` hit 4 more `read_file` failures on the same class of wrong path and eventually gave up: *"The path issues are problematic. Let me try a different approach - just read the file directly with cat."*

Observed failures in `41966513`:

- 3 `list_dir` failures on `agent-tooling/peek-*` and `memory-api/crate*` — the tree is `memory-api/tools/mcp/peek-mcp` and `memory-api/crates/`.

## Why it costs

Each failed path is a wasted turn plus a recovery turn plus several exploration turns. In `3e9bc20b` this pattern accounts for the majority of the 83 `grep`/`find`/`ls` commands. At an estimated ~37k tokens of fixed prefix per turn, path-guessing is one of the most expensive avoidable behaviours in the log. The failure and command counts are measured; the token cost is an estimate pending `9d527ad1`.

**Recurrence during review.** While reviewing this very ticket on 2026-07-27, the Review Agent emitted ticket paths prefixed with `memory-api/crates/session-api/`, producing links to files that do not exist. A separate lookup in the same session guessed `45ff05c9-1c86-4a9d-9c0b-f1e6bd7bb1f1` for a ticket whose real id is `45ff05c9-7608-43c4-a98a-e1c44e4b7fbd`, and the read failed. The failure mode reproduces inside the review of the ticket describing it, which raises the priority of the resolver in scope below.

**Nested stores are part of this problem.** Ticket and spec entities do not all live in the root store. `0d3fdba6` resolves to `memory-api/.ticket/tickets/0d3fdba6-45e6-4129-84f7-d98324c9519d/`, not `.ticket/tickets/...`. An agent that assumes a single root store constructs a path that does not exist, and the failure looks identical to a wrong-crate-path failure. Any resolver built under this ticket must return the owning store, not just the entity id — and handoff `context_anchors` must record the store-qualified path.

`repo_map.toon` exists at the repo root and encodes exactly this information, but nothing in the delegation path injects it and no sub-agent read it in either session.

## Scope

- Extend the handoff package `target_files` / `context_anchors` fields to carry repo-root-relative physical paths, not crate or component names. Coordinate with `8c67b96a` (handoff record should own the full package) and `0d3fdba6` (handoff completeness gate).
- Require the orchestrator's delegation prompt to include resolved physical paths for every file or crate it names. The orchestrator already knows them; the sub-agent does not.
- Make `repo_map.toon` part of the delegation context bundle, or expose a cheap MCP resolver (`crate name -> path`) so a single bounded call replaces a `find` sweep.
- Add a completeness check: a handoff whose `target_files` contain non-existent paths should fail its gate rather than be handed to an implementer.

## Acceptance Criteria

1. Handoff packages store repo-root-relative, forward-slash, verified-to-exist paths for every named target. **MET.**
2. A handoff containing a path that does not exist fails validation at creation time, not at consumption time. **MET.**
3. Delegation prompts emitted by the Orchestrator/Iteration agents include physical paths for every crate, module, or file they reference. **MET.**
4. Measured against the benchmark in `10d21210` — whose scenario includes a handoff naming a crate without its physical path — `read_file` / `list_dir` path-resolution failures drop to zero versus the checked-in baseline. **DEFERRED-pending-10d21210.** `10d21210` (the synthetic benchmark + checked-in baseline) is not yet built, so there is no benchmark run to measure against. The code path is measurable today: `create_handoff_record` now rejects any handoff naming a crate/file without a verified physical path, which is the mechanism `10d21210`'s scenario will exercise.
5. In that same benchmark run, exploratory `find` / `ls` commands issued solely to locate a named crate drop to zero, as classified by the `77eb143b` classifier. **DEFERRED-pending-77eb143b-and-10d21210.** The `77eb143b` classifier and the `10d21210` benchmark run are both downstream and not yet built; no classified command counts exist to report.

## Implementation Summary

- [memory-api/crates/session-api/src/model/handoff.rs](memory-api/crates/session-api/src/model/handoff.rs) — `SessionHandoffPackage`/`SessionHandoffRecord` schema unchanged in shape; `target_files`/`context_anchors` now carry normalized, verified paths after `create_handoff_record` runs (see below).
- [memory-api/crates/session-api/src/store/config/handoff_finish.rs](memory-api/crates/session-api/src/store/config/handoff_finish.rs) — `create_handoff_record` now validates every `target_files` entry, and every path-shaped `context_anchors` entry (anchors containing `/` that are not a `scheme://` or `prefix:id` form), against the discovered workspace root at creation time. A missing/non-existent path returns `SessionError::HandoffPathNotFound` before anything is persisted (AC2); valid entries are normalized to forward-slash form (AC1).
- [memory-api/crates/session-api/src/store/helpers/storage.rs](memory-api/crates/session-api/src/store/helpers/storage.rs) — added `workspace_root()` (repo-root discovery via the `repo_map.toon` marker file, since this repo nests submodules at multiple levels and `.git` presence alone is ambiguous), `normalize_repo_relative_path()`, `looks_like_path()`, and `verify_repo_relative_path_exists()`.
- [memory-api/crates/session-api/src/error.rs](memory-api/crates/session-api/src/error.rs) — added `SessionError::HandoffPathNotFound { path, workspace_root }`.
- [memory-api/crates/session-api/src/store_tests/workflow/snapshot_and_handoff.rs](memory-api/crates/session-api/src/store_tests/workflow/snapshot_and_handoff.rs) — added `handoff_package_with_nonexistent_target_file_fails_at_creation_time` (AC2) and `handoff_package_normalizes_backslash_target_files_to_forward_slash` (AC1).
- [memory-api/crates/session-api/tests/handoff_folder_storage.rs](memory-api/crates/session-api/tests/handoff_folder_storage.rs), [memory-api/crates/session-api/tests/handoff_roundtrip.rs](memory-api/crates/session-api/tests/handoff_roundtrip.rs) — pre-existing fixtures used synthetic non-existent paths (`src/main.rs`, `src/lib.rs`, `tests/integration.rs`); updated to real repo-relative paths so creation-time validation no longer rejects them.
- [.agents/agents/orchestrator.agent.md](.agents/agents/orchestrator.agent.md), [.agents/agents/handoff.agent.md](.agents/agents/handoff.agent.md), [.agents/prompts/handoff.prompt.md](.agents/prompts/handoff.prompt.md), [.agents/prompts/iteration.prompt.md](.agents/prompts/iteration.prompt.md) — require physical, repo-root-relative, forward-slash, verified-to-exist paths for every crate/module/file named in delegation prompts and handoff packages (AC3).

## Validation

- `rtk cargo build -p session-api` — pass.
- `rtk cargo test -p session-api` — 195 passed (10 suites), 0 failed.
- Recorded as [vt-session-api-handoff-path-validation](memory-api/.test/default/specs/vt-session-api-handoff-path-validation.json) / [exec-vt-session-api-handoff-path-validation-20260728](memory-api/.test/default/executions/exec-vt-session-api-handoff-path-validation-20260728.json) in test-mcp.

## Evidence

- `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json` — failures at events 810, 1256, 1258, 1351, 1365 with recovery reasoning attached
- `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json` — `list_dir` failures
- `tmp/subagent_cost_probe.py`
- Unused asset: `repo_map.toon`