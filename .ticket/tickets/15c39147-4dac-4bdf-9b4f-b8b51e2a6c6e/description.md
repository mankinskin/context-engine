Define one global, format-switchable clickable-reference policy and render it into the global agent contract.

## Design (revised after review)
Single canonical owner: the AGENTS.md rule entry `agent-rules/clickable-references` (.rule/rules/ce05ee88-e2e0-494c-846a-270aa07c6638). There is no second copy:
- The "Formatting conflict policy (canonical)" block in the Instruction Precedence entry (7a21f7ef) no longer restates reference-formatting rules — it now defers to the Clickable Reference Policy. The Clickable Reference Policy back-references it. One definition, cross-linked both ways.
- The earlier decorative recurring-principles spec section was deleted (spec 954d9807 section clickable-references removed; body reverted). Generation reads the rule entry via rule-targets/10-agents.yaml, not the spec, so a spec copy would have required manual sync for no benefit. The rule entry is the source of truth.

## Policy contents
Three reference modes selected by one switch (default manifest):
- viewer — real routes only: ticket-viewer /workspace/{workspace}/ticket/{id} (:3002), spec-viewer /specs/{id} (:4002), log-viewer /#/file/{url-encoded-log-name} (:3000). doc-viewer (:3001) has no per-entity deep-link route (artifacts keyed by package::target); docs fall back to manifest/description mode — stated explicitly instead of a placeholder.
- manifest — relative link to the entity manifest (ticket.toml, spec.toml).
- description — relative link to the top description/body file (body.md, description.md).
Link text "{short-id} {title}". Path normalization: forward-slash unix, repo-root-relative, no drive letters, mingw/WSL assumption. Anti-backtick rule is explicitly scoped to the emitted reference token (not policy prose or shell commands), and the policy prose no longer backtick-wraps path examples — the rule no longer violates itself.

## Validation (honest)
- `rule generate-target --config rule-targets.yaml --target context-engine-agents --check` → exit 0. This proves the AGENTS.md render is deterministic/byte-stable for this target; it is NOT an end-to-end gate.
- `rule store-index` → exit 0 (catalog regenerated: .rule/README.md, .rule/index.toon, .agents/rules-catalog.md).
- Aggregated `rule sync-targets --config rule-targets.yaml --check` remains RED, but only on a viewer-api submodule spec artifact (798c9a3c body.md) that is untouched by this change — confirmed by stashing this change and re-running the gate (still red on the same file). This change is not committed through the aggregated writer; the viewer-api drift is a separate, pre-existing problem to fix in that submodule.

## Files
.rule/rules/ce05ee88.../body.md (policy), .rule/rules/7a21f7ef.../body.md (deferral), rule-targets/10-agents.yaml (node), AGENTS.md (regenerated), .spec/specs/954d9807.../ (section deleted, body reverted), .rule/README.md + index.toon + .agents/rules-catalog.md (catalog).