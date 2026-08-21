# Duplication Report — anchor slice [4,5] (`bug-report.agent.md`, `cleanup.agent.md`)

Scope note: this report's synthesis is scoped to this run's own [duplicate-passages.md](duplicate-passages.md) only, per [Anchor-Subset Scope for Large Corpora](../../../.agents/instructions/orchestration/duplication-review.instructions.md#anchor-subset-scope-for-large-corpora) — anchors remain open beyond anchor 5, so no cross-run/campaign-level synthesis is attempted here.

## Coverage

- Pairs evaluated this run: **275** (anchor `F_4` = `bug-report.agent.md` × 138 targets, anchor `F_5` = `cleanup.agent.md` × 137 targets), matching [pair-ledger.md](pair-ledger.md) row count exactly.
- Verdicts: 21 exact duplicate / 9 near-duplicate / 175 thematic overlap / 70 no overlap.
- Verification pass: every `exact duplicate` and `near-duplicate` finding returned by batch workers was independently re-read against current file text before being recorded. Seven batch-worker findings did not reproduce at their cited line ranges and were downgraded to `thematic overlap` — see [duplicate-passages.md](duplicate-passages.md#verification-corrections-made-this-run) for the full list. This confirms the prior slice's flagged risk (fabricated near-duplicate citations) recurs and must be checked every run.

## Ranked duplicated ideas (surviving clusters)

### 1. Shared agent-template skeleton (thematic + exact "duplicate" headings — recurs across ~150+ occurrences)

Both `bug-report.agent.md` and `cleanup.agent.md` share the same five-section agent template (frontmatter → persona line → Input Contract/Scope → Constraints → Required Workflow → Output Format) as every other file in `.agents/agents/*.agent.md`. The batch workers for `bug-report.agent.md`'s agent-vs-agent pairs (batches 3-5) literally quoted the matching section headings as `exact duplicate` findings; this is the same structural pattern anchors 1 and 2-3 already identified as expected template-level similarity, not a duplicated rule. Consistent with precedent, these are retained in [duplicate-passages.md](duplicate-passages.md) for traceability but not treated as a novel finding.

**Occurrences:** all 21 `exact duplicate` findings in [duplicate-passages.md](duplicate-passages.md), plus the bulk of the 175 `thematic overlap` verdicts against other `.agent.md` files; see [pair-ledger.md](pair-ledger.md) rows 1-42 (bug-report vs agents) and 139-166 (cleanup vs agents).

### 2. "Search for an existing ticket duplicate before creating one" (near-duplicate — 7 occurrences)

A recurring rule requiring ticket-creation flows to search the store for an existing duplicate first, expressed with closely related wording across `bug-report.agent.md`, `agents/ticket-refinement.agent.md`, `instructions/ticket/workflow.instructions.md`, `prompts/handoff-tickets.prompt.md`, `prompts/ticket.prompt.md`, `prompts/tickets.prompt.md`, and `prompts/user-training.prompt.md`. This is a genuine cross-file duplicated rule (ticket-store hygiene), matching the pattern already surfaced for the `prompts/` corpus in anchor 2-3's cluster #3.

**Occurrences:** [duplicate-passages.md](duplicate-passages.md#near-duplicates-9-findings-all-verified) findings 1, 3-9; [pair-ledger.md](pair-ledger.md) rows 37, 98, 109, 123, 124, 127.

### 3. Agent catalog description/model duplicated verbatim in `model-routing.instructions.md` (near-duplicate — 1 occurrence)

`bug-report.agent.md`'s frontmatter `model: "GPT-5.4 mini"` reappears verbatim inside `instructions/orchestration/model-routing.instructions.md`'s agent/model catalog table row for the same agent. Same catalog-consistency pattern already identified in anchor 2-3's cluster #4 — expected if the catalog is kept in sync with each agent's frontmatter, not an accidental duplication.

**Occurrences:** [duplicate-passages.md](duplicate-passages.md) near-duplicate finding 2; [pair-ledger.md](pair-ledger.md) row 67.

### 4. "Protect another agent's in-progress/owned work" (near-duplicate — 2 occurrences)

`cleanup.agent.md`'s constraint against integrating branches, deleting worktrees with unmerged commits, or committing another agent's in-progress work recurs — both in `agents/duplication-consolidation.agent.md`'s own constraint and in its governing `instructions/orchestration/duplication-consolidation.instructions.md` (the agent template restating the instructions file's Mechanical Execution step 5). A genuine repeated ownership-safety rule, not coincidental phrasing.

**Occurrences:** [duplicate-passages.md](duplicate-passages.md) near-duplicate findings 10-11; [pair-ledger.md](pair-ledger.md) rows 145, 194.

### 5. `user-invocable: true` frontmatter field (exact duplicate — 7 occurrences, flagged as non-substantive)

Reported as `exact duplicate` because it is a verbatim single-line YAML match, but it is shared boilerplate frontmatter independently present on every `.agent.md` template (`online-research`, `orchestrator`, `refactoring`, `research`, `review`, `roast`, `scoping`). **Dropped from ranking** per the synthesis phase's rule to drop coincidental/structural matches, consistent with anchor 2-3's identical treatment of this field.

### Dropped as coincidental or unverifiable

- All 21 "exact duplicate" section-heading matches (cluster #1) — structural, not a content duplication.
- Seven near-duplicate findings that failed line-range verification against current file text (`subagent-return-contract.instructions.md`, `session-identity-and-handoff.instructions.md`, `session-workflow.instructions.md`, `merge.agent.md`, `commit/workflow.instructions.md`, `split-responsibility-testing.instructions.md`, `ticket/board.instructions.md`) — downgraded to `thematic overlap`; see [duplicate-passages.md](duplicate-passages.md#verification-corrections-made-this-run) for the specific citation errors caught.
- 70 `no overlap` pairs, concentrated in `.agents/skills/**` (rust/typegpu/webgpu/playwright-cli topics unrelated to bug intake or workspace cleanup) and several `prompts/` files with narrow, unrelated scopes (`sync-model-prices`, `tool-grant-regression-probe`, `transform-transcript`).

## Handoff

This review only reports duplicated/similar passages; it does not rewrite, condense, or delete anything in the guidance corpus. Route any consolidation of the ranked ideas above — especially cluster #2's cross-file "search for duplicate ticket" rule and cluster #4's ownership-safety rule — to **Simplify Agent** or **Duplication Consolidation Agent** once the campaign completes and cross-run synthesis runs at the campaign root.
