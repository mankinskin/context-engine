## Summary
Resolved the formatting-rule conflict by making precedence/exception behavior explicit and adding a canonical formatting policy that selects linkified file references over backticks for file/path citations.

## Changes
- Updated canonical AGENTS rule body at `.rule/rules/7a21f7ef-b76b-4cef-8a7b-7727958088cd/body.md`.
- Regenerated AGENTS via `rule generate-target --target context-engine-agents`.
- Confirmed `AGENTS.md` now includes a single precedence section with explicit conflict resolution and formatting policy.

## Canonical Policy
- Linkified markdown file references are canonical for workspace file/path/line citations.
- Backtick-wrapped file references are not used when linkified-file policy is active.
- If another instruction requests backticks for file references, linkified-file policy takes precedence for file/path citations.

## Validation
- `rule explain-target --config rule-targets.yaml --target context-engine-agents --workspace . --toon` confirms one matching rule for `agent-rules/instruction-precedence-and-exceptions`.
- `rule generate-target --workspace . --config rule-targets.yaml --target context-engine-agents` succeeded.
- `rg` verification confirms single precedence entry in `AGENTS.md`.

## Limitation
- `rule update` command path failed repeatedly with search-index writer collision (`FileAlreadyExists(... .del)`), so the canonical body update was applied directly to the rule body file and then regenerated.
