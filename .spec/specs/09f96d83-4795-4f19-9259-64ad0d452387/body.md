# Goal
Wire the repository's VS Code GitHub Copilot hook configuration to persist chat sessions through `session-api` after each agent response stops.

# Scope
- register the root hook configuration file in workspace settings via `chat.hookFilesLocations`
- extend the root hook configuration with a `Stop` hook alongside the existing `PostToolUse` hooks
- add a small executable `session-api` adapter that reads `Stop` hook input, resolves `transcript_path`, and persists session data into the existing store layout
- parse Copilot transcript JSONL into the existing `CopilotHookPayload` model while preserving append-only transcript semantics
- cover transcript normalization and executable hook capture behavior with focused tests
- verify the integration with a simulated `Stop` hook invocation

# Non-goals
- replacing or removing the existing `PostToolUse` hook reminders
- changing the existing session store layout or append-only merge semantics
- adding new query, MCP, or HTTP surfaces beyond the current `session-api` store API
- supporting every Copilot hook event type in one slice

# Acceptance Criteria
1. Workspace settings point Copilot at `.github/hooks/docs-validation.json` for this checkout.
2. `.github/hooks/docs-validation.json` includes a `Stop` hook command in addition to the current `PostToolUse` commands.
3. The `Stop` hook command reads the hook input and transcript, normalizes the session into the existing `session-api` model, and persists it with the current append-only store behavior.
4. Focused tests cover transcript parsing or normalization plus the executable hook capture path.
5. A focused simulated `Stop` hook invocation writes session files under the expected workspace and session directory.

# Traceability
- Ticket: [e663f9e9 stop-hook session capture](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e663f9e9-ac52-4c0e-8e07-d17c8a15b48d/ticket.toml)
- Prior root scaffold spec: [96dc0068 workflow guidance generation and session capture scaffolding](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.spec/specs/96dc0068-d05d-4e61-b785-144272119fa9/spec.toml)
- Prior `session-api` hook slice: [959c94bd session hook ingestion and read/query](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/memory-api/.ticket/tickets/959c94bd-4a42-47d6-bee4-a12332a23b52/ticket.toml)

# Planned Slice
- Added a transcript-driven hook adapter that converts Copilot JSONL transcripts into the existing `CopilotHookPayload` structure.
- Added `capture_copilot_transcript` plus a small `copilot-stop-hook` executable entrypoint so the repo hook can persist sessions without duplicating store logic.
- Registered the repo hook file in workspace settings and extended it with a `Stop` hook command while keeping the existing reminder hooks intact.
- Added a `memory-api/.gitignore` rule for `.memory-api/` so persisted session data stays local.

# Validation
- ValidationSpec: focused `session-api` tests for transcript normalization and hook-entry capture, a real simulated `Stop` hook invocation, and a full crate test run.
- ValidationExecution: passed `./target/debug/spec.exe health 09f96d83-4795-4f19-9259-64ad0d452387 --workspace-root . --json`.
- ValidationExecution: passed `cargo test -p session-api transcript -- --nocapture`.
- ValidationExecution: passed `git -C memory-viewers/memory-api check-ignore -v .memory-api/test/session.json`.
- ValidationExecution: passed `printf '{"transcript_path":"C:/Users/linus_behrbohm/AppData/Roaming/Code/User/workspaceStorage/2a3d14caf3f9407a57d20e903c05a6f8/GitHub.copilot-chat/transcripts/2b90cd39-cf55-4840-9b3e-ce38cde2f7b3.jsonl","stop_hook_active":false}' | bash .github/hooks/session-capture-stop.sh && test -f memory-api/.memory-api/sessions/context-engine/2b90cd39-cf55-4840-9b3e-ce38cde2f7b3/session.json && test -f memory-api/.memory-api/sessions/context-engine/2b90cd39-cf55-4840-9b3e-ce38cde2f7b3/transcript.json`.
- ValidationExecution: passed `cargo test -p session-api`.

# Evidence Mapping
- DocEvidenceRecord candidates: `.vscode/settings.json`, `.github/hooks/docs-validation.json`, `.github/hooks/session-capture-stop.sh`, `memory-api/.gitignore`, `memory-api/crates/session-api/src/hook.rs`, `memory-api/crates/session-api/src/store.rs`, and `memory-api/crates/session-api/src/bin/copilot-stop-hook.rs`.
- ValidationLogCapture / ValidationLogRetrieval: the commands above plus the persisted output in `memory-api/.memory-api/sessions/context-engine/2b90cd39-cf55-4840-9b3e-ce38cde2f7b3/`.

# Remaining Work
- Decide whether `Session End` should also persist or simply remain a future follow-up once the `Stop` hook path is stable.
- Add richer indexing or search behavior if the current filesystem-backed query path becomes too limited.
