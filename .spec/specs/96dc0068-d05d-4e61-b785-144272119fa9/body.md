# Goal
Generate the requested workflow prompt and agent files from canonical rule-api entries, then seed a bounded first session-api scaffold for storing Copilot chat sessions in the memory-api store.

# Scope
- add generated root guidance prompts for `/memory-setup`, `/interview`, `/user-training`, `/ticket-next`, `/next`, `/audit`, and `/tdd`
- add generated root agent files for `testing`, `research`, `interview`, `audit`, and `implement`
- keep rule-target configs and canonical rule entries as the source of truth for those generated files
- validate the new generated surfaces with focused target/config tests plus generation checks
- plan and scaffold a minimal `session-api` slice under `memory-viewers/memory-api`
- model validation and documentation evidence using existing `test-api`, `doc-api`, and `log-api` concepts

# Non-goals
- building every future MCP, HTTP, CLI, or UI surface for session capture in one change
- replacing the existing workflow metadata direction for `doc-api`, `test-api`, or `log-api`
- hand-authoring prompt or agent files without rule-api-backed generation

# Acceptance Criteria
1. The requested prompts and agents are generated from canonical rule entries and checked in as generated files.
2. Root rule-target config covers the new prompt and agent outputs with minimal additional structure.
3. Focused validation proves the rule-target wiring and generated output shape for the new files.
4. Ticket documentation for the implemented slice records validation specs/executions and generated-doc/log references using the current `test-api`, `doc-api`, and `log-api` model vocabulary.
5. A basic `session-api` scaffold exists with a clear boundary for Copilot hook ingestion and memory-store persistence follow-up.

# Traceability
- Tracker ticket: [d6f5f59e workflow guidance and session planning](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/d6f5f59e-3955-443f-9381-afc486d0b8ad/ticket.toml)
- Implemented guidance slice: [eaa42703 rule-api guidance generation](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/eaa42703-11f8-42dc-8c18-aec48101ed5e/ticket.toml)
- Incremental implement guidance slice: [bf8ef22e add implement agent target](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/bf8ef22e-ea06-45de-9f90-a2fee0e4cc6e/ticket.toml)
- Implemented session-api slice: [9491f6b7 session-api scaffold](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/9491f6b7-c11b-4d94-aed6-f5c6ea004e8a/ticket.toml)

# Implemented Slice
## Guidance Generation
- Added root prompt targets in `rule-targets/30-github-prompts.yaml` for the requested slash-command guidance files.
- Added root agent targets in `rule-targets/45-agents-agents.yaml` for the requested custom agents.
- Created canonical `.rule` entries for all new prompt and agent surfaces.
- Extended the generated agent set with `.agents/agents/implement.agent.md` from a matching root target and canonical `.rule` entry.
- Generated the following documentation surfaces from rule-api:
  - `.github/prompts/memory-setup.prompt.md`
  - `.github/prompts/interview.prompt.md`
  - `.github/prompts/user-training.prompt.md`
  - `.github/prompts/ticket-next.prompt.md`
  - `.github/prompts/next.prompt.md`
  - `.github/prompts/audit.prompt.md`
  - `.github/prompts/tdd.prompt.md`
  - `.agents/agents/testing.agent.md`
  - `.agents/agents/research.agent.md`
  - `.agents/agents/interview.agent.md`
  - `.agents/agents/audit.agent.md`
  - `.agents/agents/implement.agent.md`

## Session-Api Scaffold
- Added the new workspace member `memory-viewers/memory-api/crates/session-api` in the root workspace manifest.
- Added the new crate manifest `memory-viewers/memory-api/crates/session-api/Cargo.toml`.
- Added `src/lib.rs`, `src/error.rs`, `src/model.rs`, `src/hook.rs`, and `src/store.rs`.
- Defined the first typed mapping from Copilot hook payloads into stored session records and deterministic manifest/transcript paths.
- Kept persistence writes and higher-level ingestion surfaces as explicit follow-up work.

# Validation
- ValidationSpec: focused rule-target wiring and generation checks for the new guidance outputs.
- ValidationExecution: passed `./target/debug/rule.exe explain-target --config rule-targets.yaml --target context-engine-prompt-memory-setup`.
- ValidationExecution: passed `./target/debug/rule.exe generate-target --config rule-targets.yaml --target context-engine-prompt-memory-setup --json`.
- ValidationExecution: passed `./target/debug/rule.exe explain-target --config rule-targets.yaml --target context-engine-agent-implement --json`.
- ValidationExecution: passed `./target/debug/rule.exe generate-target --config rule-targets.yaml --target context-engine-agent-implement --json`.
- ValidationExecution: passed `./target/debug/rule.exe generate-target --config rule-targets.yaml --target context-engine-agent-implement --check --json`.
- ValidationExecution: passed `./target/debug/rule.exe sync-targets --config rule-targets/30-github-prompts.yaml --json`.
- ValidationExecution: passed `./target/debug/rule.exe sync-targets --config rule-targets/45-agents-agents.yaml --json`.
- ValidationExecution: passed `./target/debug/rule.exe sync-targets --config rule-targets/30-github-prompts.yaml --check --json`.
- ValidationExecution: passed `./target/debug/rule.exe sync-targets --config rule-targets/45-agents-agents.yaml --check --json`.
- ValidationSpec: focused compile and shallow unit coverage for the new `session-api` crate.
- ValidationExecution: passed `cargo test -p session-api`.

# Evidence Mapping
- DocEvidenceRecord candidates: the generated prompt and agent markdown files plus the new `session-api` crate files and updated workspace manifest.
- ValidationLogCapture / ValidationLogRetrieval: rule CLI JSON output from the generation and `--check` commands plus the `cargo test -p session-api` output in the current session terminal.
- No stronger repo-local workflow artifact has been written yet for either slice.

# Remaining Work
- Persist the planned session record into a concrete memory-api filesystem store.
- Add Copilot hook installation or ingestion surfaces that emit `SessionCaptureRequest` payloads.
- Add read/query tooling once the stored record format and write path are stabilized.