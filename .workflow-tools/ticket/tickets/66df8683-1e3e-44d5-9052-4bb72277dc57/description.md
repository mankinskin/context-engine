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

Progress update 2026-07-10:
- Completed one concrete dedup batch for the recurring-principles `related-tickets` entries owned by:
	- context-engine/recurring-principles/recurring-cross-cutting-principles/related-tickets/l14
	- memory-api/recurring-principles/memory-api-recurring-principles/related-tickets/l19
	- viewer-api/recurring-principles/viewer-api-recurring-principles/related-tickets/l12
- Canonical owner decision: keep the context-engine root recurring-principles `related-tickets` entry as the full ticket list; reduce the `memory-api` and `viewer-api` entries to thin references that only need local links when workspace-specific follow-up exists.
- Files changed for this batch:
	- memory-api/.rule/rules/74d940e6-88b0-4c20-be70-72cdd9db2b89/body.md
	- viewer-api/.rule/rules/925ea219-ff23-45cf-9ced-de2a23c3bac5/body.md
	- regenerated spec outputs in memory-api/.spec/specs/f9c32554-9884-41c4-8b5b-d1d32b37e341/body.md and viewer-api/.spec/specs/798c9a3c-404a-4842-874d-484edb4209ef/body.md

Validation evidence 2026-07-10:
- Before edit: `metrics.rule_overlap.high_overlap_pairs = 53`.
- After edit: `metrics.rule_overlap.high_overlap_pairs = 51`.
- Targeted finding check after the edit: the exact recurring-principles triplet above no longer appears in `findings[]` with `category == "rule_overlap"` (`targeted_pair_count = 0`).
- Regeneration passed with:
	- `cargo run -p rule-cli --bin rule -- sync-targets --config memory-api/rule-targets.yaml`
	- `cargo run -p rule-cli --bin rule -- sync-targets --config viewer-api/rule-targets.yaml`

Remaining work:
- Completed on 2026-07-10. No remaining `rule_overlap` findings were left above threshold after the final dedup pass.

Final completion update 2026-07-10:
- Completed the remaining overlap-cleanup batches for:
	- duplicated `roast` agent entries across `memory-api`, `memory-viewers`, and `viewer-api`
	- duplicated `ticket-refinement` agent sections across `memory-api`, `memory-viewers`, and `viewer-api`
	- duplicated shared agent-rule sections mirrored into `memory-api` and `viewer-api`
	- the shared `implement` prompt, which now delegates to the Implement Agent instead of reprinting the same contract
- Canonical-owner pattern used for the final pass:
	- keep one fuller owner per repeated guidance family
	- convert secondary copies into shorter workspace-specific references when the shared owner already covers the contract
	- keep repo-local detail only when that workspace genuinely needs additional routing, validation, escalation, or traceability guidance

Final validation evidence 2026-07-10:
- Final audit result after all cleanup batches: `rules_considered = 277`, `compared_pairs = 38226`, `high_overlap_pairs = 0`, `max_similarity = 0.7045454545454546`.
- Final `rule_overlap` finding count: `0`.
- Final regeneration passes completed with:
	- `cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets.yaml`
	- `cargo run -p rule-cli --bin rule -- sync-targets --config memory-viewers/rule-targets.yaml`
	- `cargo run -p rule-cli --bin rule -- sync-targets --config memory-api/rule-targets.yaml`
	- `cargo run -p rule-cli --bin rule -- sync-targets --config viewer-api/rule-targets.yaml`

Definition-of-done check:
- At least one concrete overlap batch deduplicated: satisfied; multiple batches completed.
- Canonical owner established per cleaned overlap family: satisfied.
- Regeneration remained deterministic after cleanup: satisfied.
- Fresh overlap audit recorded with before/after evidence: satisfied.
- No unrelated worktree noise mixed into the overlap-cleanup scope: satisfied.
