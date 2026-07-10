Goal: reduce the high-overlap rule pairs surfaced by the fresh rule-overlap audit and leave a cleaner, more canonical prompt/instruction rule set for future regenerations.

Why this exists:
- Review-close audit evidence for ticket 474eb962-b68f-4651-b980-c4c9233b2710 showed metrics.rule_overlap is present and usable.
- Fresh audit result: rules_considered=298, compared_pairs=44253, high_overlap_pairs=53, max_similarity=1.0.
- The remaining work is no longer audit-output ergonomics; it is content deduplication and canonical-owner cleanup.

Initial scope:
1. Start with the highest-value prompt/instruction overlaps surfaced by the audit, especially handoff/prompt and recurring-principle duplicates that affect new-session guidance.
2. Choose one canonical owner per repeated rule, keep thin references in secondary scopes, and regenerate targets after each batch.
3. Re-run audit run . or rtk audit --toon run . after each batch and record whether high_overlap_pairs decreases.

Acceptance criteria:
- At least one concrete overlap batch is deduplicated with canonical ownership documented.
- rule sync-targets --config rule-targets.yaml remains deterministic after the cleanup.
- The ticket records before/after overlap evidence and any intentionally retained duplicates with rationale.

Evidence anchors:
- owning completed review ticket: .ticket/tickets/474eb962-b68f-4651-b980-c4c9233b2710/ticket.toml
- audit surfaced exact rule_overlap findings including 100% duplicates across handoff-adjacent and recurring-principle entries.
