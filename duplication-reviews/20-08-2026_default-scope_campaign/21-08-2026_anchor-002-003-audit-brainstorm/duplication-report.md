# Duplication Report — anchor slice [2,3] (`audit.agent.md`, `brainstorm.agent.md`)

Scope note: this report's synthesis is scoped to this run's own [duplicate-passages.md](duplicate-passages.md) only, per [Anchor-Subset Scope for Large Corpora](../../../.agents/instructions/orchestration/duplication-review.instructions.md#anchor-subset-scope-for-large-corpora) — anchors remain open beyond anchor 3, so no cross-run/campaign-level synthesis is attempted here.

## Coverage

- Pairs evaluated this run: **279** (anchor `F_2` = `audit.agent.md` × 140 targets, anchor `F_3` = `brainstorm.agent.md` × 139 targets), matching [pair-ledger.md](pair-ledger.md) row count exactly.
- Verdicts: 5 exact duplicate / 21 near-duplicate / 205 thematic overlap / 48 no overlap.

## Ranked duplicated ideas (surviving clusters)

### 1. Shared agent-template skeleton (thematic overlap — recurs across ~180+ occurrences)

Both `audit.agent.md` and `brainstorm.agent.md` share the same five-section agent template (frontmatter → persona line → Input Contract/Scope → Constraints → Required Workflow → Output Format) as every other file in `.agents/agents/*.agent.md`. Nearly every anchor/target pair against another `.agent.md` file surfaced this as thematic overlap (structure repeats, wording is agent-specific). This is expected template-level similarity, not a duplicated rule — see the [Duplication Consolidation Agent](../../../.agents/agents/duplication-consolidation.agent.md) template-conformance note rather than a content fix.

**Occurrences:** all 42 `audit.agent.md`-vs-agent pairs (batches 1-6) and all 27 `brainstorm.agent.md`-vs-agent pairs (batches 21-26); see [pair-ledger.md](pair-ledger.md) rows 1-42 and 141-180.

### 2. "Do not edit code, create tickets, update specifications, or choose a final direction for the user." (near-duplicate — 6 occurrences)

A verbatim constraint sentence recurring in `brainstorm.agent.md` and independently in `mission-planning.agent.md`, `online-research.agent.md`, `orchestrator.agent.md`, `roast.agent.md`, `model-prices.instructions.md`, and `transform-transcript.prompt.md`. This is a real cross-file duplicated rule (advisory-only agent posture), not coincidental phrasing.

**Occurrences:** [pair-ledger.md](pair-ledger.md) rows 162, 163, 164, 168, 207, and the `transform-transcript.prompt.md` pair (batch 39); see [duplicate-passages.md](duplicate-passages.md) near-duplicate rows 7-10, 12, 20.

### 3. "Inspect only the closest relevant tickets, specifications, and repository surfaces needed to avoid contradicted or duplicate ideas." (near-duplicate — 7 occurrences)

A verbatim scoping-of-research-effort sentence recurring in `brainstorm.agent.md` and independently in `prompts/audit.prompt.md`, `prompts/debug-test.prompt.md`, `prompts/interview.prompt.md`, `prompts/research.prompt.md`, `prompts/reviews.prompt.md`, `prompts/user-training.prompt.md` (and reported again for `memory-setup.prompt.md`/`refine-ingest.prompt.md`). A genuine repeated guidance sentence across the `prompts/` corpus, not coincidental.

**Occurrences:** [pair-ledger.md](pair-ledger.md) rows 241, 244, 253, 258, 259, 268; see [duplicate-passages.md](duplicate-passages.md) near-duplicate rows 14-19.

### 4. Agent catalog description strings duplicated verbatim in `model-routing.instructions.md` (near-duplicate — 2 occurrences)

`audit.agent.md`'s one-line persona description ("Use for honest repository audits, findings-first reviews, and automated validation triage.") and `brainstorm.agent.md`'s `model: "Claude Sonnet 5"` frontmatter value both reappear verbatim inside `instructions/orchestration/model-routing.instructions.md`'s agent/model catalog table. Expected: that file is a routing catalog that intentionally mirrors each agent's frontmatter, so this is catalog-consistency duplication rather than an accidental rule duplication — worth confirming the catalog is generated/kept in sync rather than hand-duplicated.

**Occurrences:** [pair-ledger.md](pair-ledger.md) row 69, row 208.

### 5. `user-invocable: true` frontmatter field (exact duplicate — 5 occurrences, flagged as non-substantive)

Reported as `exact duplicate` by the batch worker because it is a verbatim single-line YAML match, but it is a shared boilerplate frontmatter field independently present on `teacher.agent.md`, `testing.agent.md`, `ticket-refinement.agent.md`, `transcription.agent.md`, and `writing.agent.md` — not a duplicated idea or rule. **Dropped from ranking** per the synthesis phase's rule to drop coincidental/structural matches; retained in [duplicate-passages.md](duplicate-passages.md) for traceability only.

### Dropped as coincidental

- Single overlapping words/phrases inside otherwise-unrelated `no overlap`-verdict pairs (e.g. skill files under `.agents/skills/playwright-*`, `.agents/skills/rust-*`, `.agents/skills/typegpu`, `.agents/skills/webgpu-threejs-tsl` — 19 of 20 skill-file pairs against `audit.agent.md` and 6 of 11 against `brainstorm.agent.md` returned `no overlap`).
- Structural-only matches against `.agents/instructions/commit/*.instructions.md` for the `brainstorm.agent.md` anchor (batch 27: all 7 targets `no overlap` — brainstorming has no natural overlap with commit mechanics).

## Handoff

This review only reports duplicated/similar passages; it does not rewrite, condense, or delete anything in the guidance corpus. Route any consolidation of the ranked ideas above — especially cluster #2 and #3's cross-file verbatim sentences, and the template-conformance question in cluster #1 — to **Simplify Agent** (or, if resumed as part of the full campaign, to **Duplication Consolidation Agent** once the campaign completes and cross-run synthesis runs at the campaign root).
