# Campaign Coverage — default scope (`.agents/` + root `AGENTS.md`)

## Campaign parameters

- Comparison scope: every file under `.agents/agents/*.agent.md`, `.agents/instructions/**/*.instructions.md`, `.agents/prompts/*.prompt.md`, `.agents/skills/**/SKILL.md`, plus repository-root [AGENTS.md](../../AGENTS.md) — per Scope Resolution in [duplication-review.instructions.md](../../.agents/instructions/orchestration/duplication-review.instructions.md#scope-resolution).
  - **Note:** the caller's estimate of `n=260` files was based on every `.md` file under `.agents/` (including catalogs, `README.md`, `INDEX.md`, `PROVENANCE.md`, and skill `references/`/`docs/` subfiles). Scope Resolution's default scope is narrower — only the four glob patterns above plus root `AGENTS.md`. The actual comparison-scope file count is **n = 142**.
- `n = 142` files, stable sort order `F_1 .. F_142` (directory-path then filename; `AGENTS.md` = `F_1`, then `.agents/agents/*.agent.md` alphabetically = `F_2..F_43`, then `.agents/instructions/**` directory-then-filename = `F_44..F_103`, then `.agents/prompts/*.prompt.md` alphabetically = `F_104..F_131`, then `.agents/skills/**/SKILL.md` directory-alphabetically = `F_132..F_142`).
- Total pairs in campaign scope: `n × (n-1) / 2 = 142 × 141 / 2 = 10,011`.
- `MAX_FILES_PER_BATCH = 8`, `MAX_BATCH_CHARS = 800,000` (GPT-5 mini, default worker model), `PHASE_WIDTH = 6`, `MAX_BATCHES_PER_RUN = 40`.
- `estimated_total_batches ≈ 10,011 / 7 ≈ 1,430` — far exceeds `MAX_BATCHES_PER_RUN`, so this campaign runs in Anchor-Subset mode per [duplication-review.instructions.md](../../.agents/instructions/orchestration/duplication-review.instructions.md#anchor-subset-scope-for-large-corpora).
- Full sorted file list `F_1..F_142` is recorded in this run's [pair-ledger.md](20-08-2026_anchor-001-agents-md/pair-ledger.md) header for reference.

## Closed anchor slices

| Run folder | Anchor index range `[a,b]` | Anchor file(s) | Target range | Pairs resolved | Batches | Phases | Verdict counts (exact / near / thematic / no-overlap) |
|---|---|---|---|---|---|---|---|
| [20-08-2026_anchor-001-agents-md](20-08-2026_anchor-001-agents-md/duplication-report.md) | [1,1] | `AGENTS.md` (F_1) | F_2..F_142 | 141 | 21 | 4 | 0 / 31 / 91 / 19 |
| [21-08-2026_anchor-002-003-audit-brainstorm](21-08-2026_anchor-002-003-audit-brainstorm/duplication-report.md) | [2,3] | `audit.agent.md` (F_2), `brainstorm.agent.md` (F_3) | F_2: F_3..F_142; F_3: F_4..F_142 | 279 | 40 | 7 | 5 / 21 / 205 / 48 |
| [21-08-2026_anchor-004-005-bugreport-cleanup](21-08-2026_anchor-004-005-bugreport-cleanup/duplication-report.md) | [4,5] | `bug-report.agent.md` (F_4), `cleanup.agent.md` (F_5) | F_4: F_5..F_142; F_5: F_6..F_142 | 275 | 40 | 7 | 21 / 9 / 175 / 70 |

## Remaining

- Next anchor to resolve: index `6` (`.agents/agents/code-architect.agent.md`).
- Anchors closed: 5 of 141 (anchors 1..141 own targets; anchor 142 owns none).
- Pairs closed: 695 of 10,011 (141 + 279 + 275).
- Pairs remaining: 9,316 (open anchors 6..141, each vs its full remaining target range `F_(i+1)..F_142`).
- Cross-run synthesis (per [duplication-review.instructions.md](../../.agents/instructions/orchestration/duplication-review.instructions.md#anchor-subset-scope-for-large-corpora)) must wait until anchor 141 closes; do not run it yet.

