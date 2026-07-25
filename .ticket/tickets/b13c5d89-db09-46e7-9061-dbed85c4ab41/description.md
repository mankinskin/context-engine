Foundation slice of the agent skill infrastructure, unblocking the critical path (ingestion debug tooling -> context/log UI -> UI dev/test/review loop).

Anchor spec: agents/skill-infrastructure (a9b7ef39).

Confirmed decisions (interview batches 1-3):
- Buy-first: adopt proven skills.sh skills; hand-author only Dioxus.
- Full migration: convert all 12 .agents/instructions/*.md off the rule-store generator to hand-owned by-description files; delete agent-guidance targets from rule-targets.
- Bold prune: delete fixture/empty specs + consolidate ultra-granular specs (reviewable ticket).
- Skill format: skills.sh-native SKILL.md shape, compatible with VS Code Copilot by-description loading.
- Adoption: vendor normalized copies in-repo.
- Migration safety: content-parity gate (seed-from-current + git-diff before retiring generator).
- Sequencing: skills land first; migration + prune strictly last; validation after.

Children:
(a) Skill directory contract + re-home orphan + master index
(b) Vendor 8 adopted skills.sh skills
(c) Author Dioxus skill (5 scope areas)
(d) Migrate 12 instructions off generator + delete rule-targets agent-guidance targets
(e) Prune fixtures + consolidate ultra-granular specs
(f) Automated prompt-replay validation matrix