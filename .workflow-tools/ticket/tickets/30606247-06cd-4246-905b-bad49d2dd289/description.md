## Summary
Added an explicit instruction precedence + exception matrix to AGENTS through the canonical rule system, then regenerated AGENTS via target-scoped generation.

## Canonical Rule Changes
- Created rule entry: `7a21f7ef-b76b-4cef-8a7b-7727958088cd`
- Slug: `shared/agent-rules/instruction-precedence-and-exceptions/l74`
- Section: `agent-rules/instruction-precedence-and-exceptions`

## Target Wiring
- Updated `rule-targets/10-agents.yaml` to include node:
  - name: `instruction-precedence-and-exceptions`
  - section: `agent-rules/instruction-precedence-and-exceptions`

## Generated Output
- Regenerated target: `context-engine-agents`
- Output updated: `AGENTS.md`

## Validation
- `rule explain-target --config rule-targets.yaml --target context-engine-agents --workspace . --toon` confirms new node is matched.
- `rule generate-target --workspace . --config rule-targets.yaml --target context-engine-agents` completed with status ok.
- Root-scoped `next` for `e1d8be15` confirms dependency frontier remains consistent and `f19dcafa` remains frontier-ready.
