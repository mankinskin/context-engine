Define one global, format-switchable clickable-reference policy and render it into the global agent contract.

## Implementation
- Canonical source of truth: new `clickable-references` section in the recurring cross-cutting principles spec ([.spec/specs/954d9807-f357-41e5-9fd4-b1da39e0933d/sections/clickable-references.md](.spec/specs/954d9807-f357-41e5-9fd4-b1da39e0933d/sections/clickable-references.md)); spec body Sections list + related tickets updated.
- New canonical AGENTS.md rule entry `agent-rules/clickable-references` (`.rule/rules/ce05ee88-e2e0-494c-846a-270aa07c6638`), wired as a node in [rule-targets/10-agents.yaml](rule-targets/10-agents.yaml).
- Regenerated [AGENTS.md](AGENTS.md) (new "Clickable Reference Policy" section) and the rule catalog (`.rule/README.md`, `.rule/index.toon`, `.agents/rules-catalog.md`).

## Policy contents
Three reference modes selected by a single switch (default `manifest`): `viewer` (viewer-server deep link — ticket-viewer :3002, spec-viewer :4002, doc-viewer :3001, log-viewer :3000), `manifest` (relative link to entity manifest), `description` (relative link to top description/body file). Link text `<short-id> <title>`. Path normalization: forward-slash unix paths, repo-root-relative, no drive letters, mingw/WSL assumption, no backticks around file references.

## Validation
- `rule generate-target --config rule-targets.yaml --target context-engine-agents --check` → exit 0 (AGENTS.md byte-stable).
- `rule store-index` regenerated catalog artifacts cleanly (exit 0).
- Note: aggregated `rule sync-targets` reports pre-existing drift in the viewer-api submodule spec store (798c9a3c body.md) unrelated to this change; not touched. AGENTS.md was written via the scoped `generate-target`.

## Docs
AGENTS.md is the global render surface; copilot-instructions.md and all agents/prompts defer to it. Follow-up (optional): propagate the same policy into per-domain prompt/instruction targets (ticket/spec prompts) so their local link forms defer to this global switch.