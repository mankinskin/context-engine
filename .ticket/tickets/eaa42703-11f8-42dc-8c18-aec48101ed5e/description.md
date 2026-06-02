Generate the requested workflow prompt and agent files from canonical rule-api entries, wire them into root rule-target configs, generate the outputs, and keep the implementation traceability explicit.

# Implemented Slice
- Added root prompt targets for `/memory-setup`, `/interview`, `/user-training`, `/ticket-next`, `/next`, `/audit`, and `/tdd`.
- Added a new root `.agents/agents` target fragment for `testing`, `research`, `interview`, and `audit` agents.
- Created canonical `.rule` entries for each new prompt and agent surface.
- Generated the new prompt and agent files from those canonical rule entries.

# Validation
- ValidationSpec: focused rule-target wiring and generation checks for the new guidance surfaces.
- ValidationExecution: passed `./target/debug/rule.exe explain-target --config rule-targets.yaml --target context-engine-prompt-memory-setup`.
- ValidationExecution: passed `./target/debug/rule.exe generate-target --config rule-targets.yaml --target context-engine-prompt-memory-setup --json`.
- ValidationExecution: passed `./target/debug/rule.exe sync-targets --config rule-targets/30-github-prompts.yaml --json`.
- ValidationExecution: passed `./target/debug/rule.exe sync-targets --config rule-targets/45-agents-agents.yaml --json`.
- ValidationExecution: passed `./target/debug/rule.exe sync-targets --config rule-targets/30-github-prompts.yaml --check --json`.
- ValidationExecution: passed `./target/debug/rule.exe sync-targets --config rule-targets/45-agents-agents.yaml --check --json`.

# Evidence Trail
- DocEvidenceRecord candidates: generated prompt files under `.github/prompts/` and generated agent files under `.agents/agents/`.
- ValidationLogCapture / ValidationLogRetrieval: rule CLI JSON output captured in the current session terminal; no repo-local workflow artifact was written for this slice.
- The linked spec records the ticket paths, generated documentation surfaces, and the passing validation commands.

# Remaining Work
- The session-api planning/scaffolding slice is tracked separately by the dependent follow-up ticket.
- If a stronger evidence artifact is needed later, record the same command set through the repo-local workflow tooling instead of duplicating metadata structures here.