<!-- aligned-structure:v1 -->

# Summary

Persist VS Code Copilot session state through `session-api` using periodic capture hooks so the latest transcript state is synced during the session, not only at terminal stop.

## Behavior Story

Persist VS Code Copilot session state through `session-api` using periodic capture hooks so the latest transcript state is synced during the session, not only at terminal stop.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Goal
Persist VS Code Copilot session state through `session-api` using periodic capture hooks so the latest transcript state is synced during the session, not only at terminal stop.

# Scope
- keep hook registration rooted at `.github/hooks/hooks.json`
- run transcript capture on periodic hook events (`UserPromptSubmit`, `PostToolUse`, `Stop`, `SessionEnd`)
- use a dedicated `session-api` executable hook adapter that reads hook stdin payloads and resolves transcript/store paths in cross-shell environments
- parse Copilot transcript JSONL into the existing `CopilotHookPayload` model with recursive de-stringification of embedded JSON-like strings
- preserve safe session merge semantics for periodic snapshots (accept newer divergent snapshots for the same session)
- keep session-local hook logs under `.session/sessions/<session-id>/session-capture-stop.log`

# Non-goals
- replacing existing non-capture reminder hooks
- introducing new query, MCP, or HTTP surfaces beyond the current `session-api` store API
- changing the on-disk session directory structure under `.session/sessions/<session-id>/`

# Acceptance Criteria
1. Hook commands invoke the capture executable directly via cargo using the renamed binary `copilot-capture-hook`.
2. The executable supports stdin hook mode (`--from-hook-stdin`) and captures periodically without transcript rewrite errors during normal sync.
3. Transcript ingest normalizes embedded JSON-like strings into native JSON values at all levels for persisted event metadata.
4. Existing session artifacts can be migrated to remove escaped JSON payload strings.
5. Focused and full `session-api` validation passes after rename and merge updates.

# Traceability
- Ticket: `.ticket/tickets/c991d769-27b4-4567-b9d1-95173af02c5a/`
- Ticket: `.ticket/tickets/5c7296f6-533f-47d9-8fe8-ffd4c80d8ca8/`
- Ticket: `memory-api/.ticket/tickets/b3155a94-230e-416b-be0e-5948d6d2193a/`
- Prior spec: `.spec/specs/96dc0068-d05d-4e61-b785-144272119fa9/`
- This spec: `.spec/specs/09f96d83-4795-4f19-9259-64ad0d452387/`

# Implemented Slice
- Renamed binary from `copilot-stop-hook` to `copilot-capture-hook` and updated hook configs/scripts accordingly.
- Added recursive JSON de-stringification in transcript parsing (`session-api` hook ingest path).
- Updated transcript merge behavior to treat periodic incoming snapshots as canonical sync updates when they are newer or longer, preventing false rewrite conflicts.
- Migrated existing `.session/sessions/*/{events,transcript}.json` payload strings to native JSON values.
- Hardened e2e fixture cleanup to prevent leaked `session-workspace-fixture-*` sessions in production workspace storage.
- Completed adjacent-suite migration by centralizing hook e2e fixture constants/scenarios into `tests/common/fixture_harness.rs` and reusing them across both stop/capture e2e suites.
- Updated adjacent `memory-matrix` session domain payload construction for new `session-api` hook fields (`event_meta`, `events`, `runtime`) and isolated matrix session-store usage to `.session-matrix` for deterministic in-process validation cells.

# Validation
- Passed `cargo test -p session-api` (latest run: 36 passed).
- Passed `cargo test -p session-api --test copilot_stop_hook_e2e --test copilot_capture_hook_e2e` (8 passed).
- Passed targeted e2e leak-prevention run: `cargo test -p session-api e2e_stop_hook_script_persists_fixture_from_nested_workspace_cwd` (renamed afterward to capture-hook naming).
- Passed capture-hook smoke run:
	`printf '{"transcript_path":"<current-session-transcript>","workspace_slug":"default","hook_event_name":"PostToolUse"}' | cargo run --quiet --manifest-path memory-api/crates/session-api/Cargo.toml --bin copilot-capture-hook -- --from-hook-stdin`
- Migration evidence: scanned 12 files across 4 sessions; changed 4 files; reduced stringified JSON scalar count from 3421 to 0.
- Passed adjacent-crate validation: `cargo test -p memory-matrix every_cell_records_an_execution_with_duration -- --nocapture`.
- Passed impacted full validation pass: `CARGO_TARGET_DIR=target/tmp/full-validate cargo test -p session-api -p memory-matrix`.
- Completed full audit pass: `cargo run -p audit-cli --bin audit -- --json run .` (run_id `295`, status `completed`; findings include `file_length` 185, `static_complexity` 15, `compiler_warning`/`compiler_check`, `coverage`, `test_execution`, and `ticket_graph` categories).

# Evidence Mapping
- Hook configs/scripts:
	- `.github/hooks/hooks.json`
	- `.clinerules/hooks/hooks.json`
	- `tools/agent-hooks/session-capture-stop.sh`
	- `.clinerules/hooks/session-capture-stop.sh`
- Session-api executable and merge/parser behavior:
	- `memory-api/crates/session-api/src/bin/copilot-capture-hook.rs`
	- `memory-api/crates/session-api/src/hook.rs`
	- `memory-api/crates/session-api/src/store_helpers.rs`
	- `memory-api/crates/session-api/src/store_tests.rs`
	- `memory-api/crates/session-api/tests/copilot_capture_hook_e2e.rs`

# Remaining Work
- Optional follow-up: rename `session-capture-stop.sh` script filename to a sync/capture-specific name and keep a compatibility alias for external callers.
