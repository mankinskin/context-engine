Create generated workflow prompts and agents from canonical rule entries, then plan and scaffold a first session-api slice for saving Copilot chat sessions into the memory-api store.

# Completed Batch
- Added rule-target coverage for the requested workflow prompts and agents.
- Created canonical `.rule` entries and generated the new prompt and agent files.
- Added the first `session-api` scaffold crate under `memory-api/crates/session-api`.
- Updated the shared spec with ticket traceability, generated-doc references, and validation results.

# Validation
- Passed focused rule generation checks with `rule.exe generate-target` and fragment-scoped `rule.exe sync-targets --check`.
- Passed `cargo test -p session-api` with the initial scaffold tests.

# Evidence Trail
- Generated doc surfaces: `.agents/prompts/*` additions and `.agents/agents/*` additions from this batch.
- Source/doc surfaces: `Cargo.toml`, `rule-targets/30-github-prompts.yaml`, `rule-targets/45-agents-agents.yaml`, and the new `memory-api/crates/session-api` crate.
- Validation logs: rule CLI JSON output plus `cargo test -p session-api` terminal output.

# Follow-up
- The next batch should implement concrete session persistence and ingestion wiring on top of the new `session-api` types.