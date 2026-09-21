## Summary
Final consolidation pass completed for instruction-governance track.

## Consolidation Outcomes
- `applyTo` scopes are now selective in active instruction files updated by this track (token-efficiency, spec-system, tests, ticket-system).
- AGENTS now has explicit instruction precedence + exception matrix and a canonical formatting conflict policy.
- `spec-system.instructions.md` duplicate full-body repetition removed.
- Formatting conflict resolved with canonical rule: linkified file/path references take precedence over backtick file references for file citations.

## Files/Surfaces Consolidated
- `.agents/instructions/token-efficiency.instructions.md`
- `.agents/instructions/spec-system.instructions.md`
- `.agents/instructions/tests.instructions.md`
- `.agents/instructions/ticket-system.instructions.md`
- `AGENTS.md`
- `rule-targets/10-agents.yaml`
- `.rule/rules/7a21f7ef-b76b-4cef-8a7b-7727958088cd/body.md`

## Validation
- `rule explain-target --config rule-targets.yaml --target context-engine-agents --workspace . --toon`
- `rule generate-target --workspace . --config rule-targets.yaml --target context-engine-agents`
- `rg -n "^## " .agents/instructions/spec-system.instructions.md` confirms unique top-level sections.
- Tracker subgraph and health checks run for root `e1d8be15`.

## Limitation
- `rule update` path intermittently failed with search-index writer collision (`FileAlreadyExists(... .del)`), so canonical rule text update used direct rule body edit followed by target regeneration.
