# Duplicate Passages — anchor slice [4,5] (`bug-report.agent.md`, `cleanup.agent.md`)

Per [Coverage and Efficiency Rules](../../../.agents/instructions/orchestration/duplication-review.instructions.md#coverage-and-efficiency-rules), every `exact duplicate` and `near-duplicate` finding is quoted below in full and re-verified against the current file text before being recorded (per this run's verification mandate). The 175 `thematic overlap` and 70 `no overlap` findings (one per pair, largely the anchor-template sections recurring against each target's equivalent sections) are not individually transcribed here for size; their per-pair verdicts are recorded in [pair-ledger.md](pair-ledger.md).

## Verification corrections made this run

Several batch-worker `near-duplicate` findings did not reproduce against the actual cited line ranges when independently re-read, and were downgraded to `thematic overlap` (their pair verdict adjusted accordingly) instead of being recorded here:

- `bug-report.agent.md` vs `instructions/orchestration/subagent-return-contract.instructions.md` — cited L21 did not contain the claimed sentence (actual text concerns "Blocked Items").
- `bug-report.agent.md` vs `instructions/session/session-identity-and-handoff.instructions.md` — cited L70 did not contain the claimed sentence (actual text is the Closing Traceability Footer section).
- `bug-report.agent.md` vs `instructions/session/session-workflow.instructions.md` — cited L31-32 did not contain the claimed sentence (actual text concerns `ticket_urn`/`spec_urn` entity references).
- `cleanup.agent.md` vs `agents/merge.agent.md` — the cited merge.agent.md L24-27 excerpt was actually `cleanup.agent.md`'s own text, not present verbatim in `merge.agent.md` (real but weaker thematic relationship: both files divide merge/cleanup responsibility, worded independently).
- `cleanup.agent.md` vs `instructions/commit/workflow.instructions.md` — the cited excerpt is a paraphrase-level thematic match (branch/staging discipline vs. not touching others' in-progress work), not the same statement reworded; downgraded.
- `cleanup.agent.md` vs `instructions/testing/split-responsibility-testing.instructions.md` — cited L11 did not contain the claimed sentence (actual text concerns Worker-tier test-file edit restrictions).
- `cleanup.agent.md` vs `instructions/ticket/board.instructions.md` — cited L45-48 and L71-72 did not contain the claimed sentences (actual text concerns board-claim scope and stale-entry cleanup commands).

## Exact duplicates (21 findings — shared agent-template section headings)

All 21 are the same structural pattern already established in this campaign's anchor 1 and anchor 2-3 runs: every `.agents/agents/*.agent.md` file shares the five-section template skeleton (`## Input Contract`, `## Scope`, `## Constraints`, `## Required Workflow`, `## Output Format`), so heading-only matches recur verbatim across unrelated agent files. Verified against current file text.

### `bug-report.agent.md` (anchor) vs 7 agent-template targets

| # | Target | Input Contract | Scope | Constraints | Required Workflow | Output Format |
|---|---|---|---|---|---|---|
| 1 | `agents/implement.agent.md` | L13/L15 | L28/L30 | L32/L37 | L38/L51 | L47/L63 |
| 2 | `agents/installer.agent.md` | L13/L13 | L28/L28 | L32/L33 | L38/L39 | L47/L48 |
| 3 | `agents/interview.agent.md` | — | L28/L15 | L32/L25 | L38/L99 | L47/L118 |
| 4 | `agents/iteration.agent.md` | — | — | L32/L61 | L38/L69 | L47/L84 |
| 5 | `agents/live-validation.agent.md` | L13/L11 | L28/L17 | L32/L24 | L38/L57 | L47/L65 |
| 6 | `agents/merge.agent.md` | L13/L13 | L28/L20 | L32/L31 | L38/L41 | L47/L55 |
| 7 | `agents/mission-planning.agent.md` | — | L28/L14 | L32/L21 | L38/L28 | L47/L36 |

Excerpt (identical across all rows above, anchor side): `## Input Contract`, `## Scope`, `## Constraints`, `## Required Workflow`, `## Output Format` (each its own heading line).

### `bug-report.agent.md` (anchor) vs 7 more agent-template targets

| # | Target | Input Contract | Scope | Constraints | Required Workflow | Output Format |
|---|---|---|---|---|---|---|
| 8 | `agents/session-bootstrap.agent.md` | L13/L13 | L28/L19 | L32/L28 | L38/L39 | L47/L51 |
| 9 | `agents/session-learning.agent.md` | L13/L13 | L28/L28 | L32/L32 | L38/L38 | L47/L47 |
| 10 | `agents/simplify.agent.md` | — | L28/L15 | L32/L63 | L38/L75 | L47/L85 |
| 11 | `agents/spec.agent.md` | — | L28/L15 | L32/L23 | L38/L32 | L47/L51 |
| 12 | `agents/structured-research.agent.md` | L13/L14 | L28/L20 | L32/L27 | L38/L35 | L47/L47 |
| 13 | `agents/surface-design.agent.md` | L13/L11 | L28/L17 | L32/L24 | L38/L37 | L47/L45 |
| 14 | `agents/teacher.agent.md` | L13/L17 | L28/L25 | L32/L36 | L38/L47 | L47/L65 |

### `bug-report.agent.md` (anchor) vs 7 targets — `user-invocable: true` frontmatter field

| # | Target | Anchor line | Target line | Excerpt |
|---|---|---|---|---|
| 15 | `agents/online-research.agent.md` | L6 | L6 | `user-invocable: true` |
| 16 | `agents/orchestrator.agent.md` | L6 | L6 | `user-invocable: true` |
| 17 | `agents/refactoring.agent.md` | L6 | L6 | `user-invocable: true` |
| 18 | `agents/research.agent.md` | L6 | L6 | `user-invocable: true` |
| 19 | `agents/review.agent.md` | L6 | L6 | `user-invocable: true` |
| 20 | `agents/roast.agent.md` | L6 | L6 | `user-invocable: true` |
| 21 | `agents/scoping.agent.md` | L6 | L6 | `user-invocable: true` |

**Caveat (consistent with anchor 2-3's precedent for the same field):** this is a single shared YAML frontmatter boolean field, not a duplicated prose rule — every `.agent.md` template carries it independently. Retained here for traceability but excluded from the synthesis ranking as non-substantive.

## Near-duplicates (9 findings, all verified)

| # | Anchor | Target | Anchor lines | Target lines | Excerpt |
|---|---|---|---|---|---|
| 1 | `bug-report.agent.md` | `agents/ticket-refinement.agent.md` | L43 | L36 | Anchor: "4. Search the ticket store for a duplicate before any ticket creation." / Target: "- Search for related tickets before creating new ones." |
| 2 | `bug-report.agent.md` | `instructions/orchestration/model-routing.instructions.md` | L7 | L155 | Anchor: `model: "GPT-5.4 mini"` / Target catalog row: `` | `bug-report.agent.md` | GPT-5.4 mini | T3 | Captures and structures bug reports | `` |
| 3 | `bug-report.agent.md` | `instructions/ticket/workflow.instructions.md` | L43 | L106-107 | Anchor: "Search the ticket store for a duplicate before any ticket creation." / Target: "Always search for existing tickets before creating new ones. Duplicate tickets degrade store quality." |
| 4 | `bug-report.agent.md` | `prompts/handoff-tickets.prompt.md` | L43 | L17 | Anchor: "Search the ticket store for a duplicate before any ticket creation." / Target: "2. Search existing tickets first so the handoff flow reuses or updates the authoritative ticket set instead of creating duplicates." |
| 5 | `bug-report.agent.md` | `prompts/ticket.prompt.md` | L36 | L28 | Anchor: "Search for an existing duplicate before creating a ticket. When a duplicate exists, report its id and evidence instead of creating another ticket." / Target: "8. If a matching ticket already exists, do not create a duplicate. Return the existing ticket instead." |
| 6 | `bug-report.agent.md` | `prompts/ticket.prompt.md` | L43 | L22 | Anchor: "Search the ticket store for a duplicate before any ticket creation." / Target: "2. Search existing tickets first with `list_tickets`, `get_ticket_description`, `ticket search`, or `ticket list` so you do not create duplicates." |
| 7 | `bug-report.agent.md` | `prompts/tickets.prompt.md` | L43 | L22 | Anchor: "Search the ticket store for a duplicate before any ticket creation." / Target: "2. Search existing tickets first with `list_tickets`, `get_ticket_description`, `ticket search`, or `ticket list` so you do not duplicate existing work." |
| 8 | `bug-report.agent.md` | `prompts/tickets.prompt.md` | L36 | L29 | Anchor: "Search for an existing duplicate before creating a ticket..." / Target: "9. If some of the needed tickets already exist, reuse them and create only the missing ones." |
| 9 | `bug-report.agent.md` | `prompts/user-training.prompt.md` | L36 | L18 | Anchor: "Search for an existing duplicate before creating a ticket. When a duplicate exists, report its id and evidence instead of creating another ticket." / Target: "3. Search for existing tickets, specs, prompts, or code that already cover the same work." |

**Note:** findings 1, 3-9 above are all independent occurrences of the same "search for an existing ticket duplicate before creating one" rule, verbatim or paraphrased, recurring across `bug-report.agent.md` and six `prompts/`-tier files plus `instructions/ticket/workflow.instructions.md` and `agents/ticket-refinement.agent.md`. Finding 2 is a separate agent-catalog-consistency match (same pattern as anchor 2-3's cluster #4).

| # | Anchor | Target | Anchor lines | Target lines | Excerpt |
|---|---|---|---|---|---|
| 10 | `cleanup.agent.md` | `agents/duplication-consolidation.agent.md` | L23-24 | L32 | Anchor: "You do not integrate branches, delete worktrees with unmerged commits, or revert, stage, or commit another agent's in-progress work." / Target: "Never edit a file another agent actively owns; check board ownership before the first edit, per Mechanical Execution step 5." |
| 11 | `cleanup.agent.md` | `instructions/orchestration/duplication-consolidation.instructions.md` | L23-24 | L53 | Anchor: same as above / Target: "5. Never edit a file another agent actively owns — check `board_show` per [board.instructions.md](../ticket/board.instructions.md) before the first edit to any file in the changeset." |

**Note:** findings 10-11 are the same "protect another agent's in-progress/owned work" rule recurring in both the `duplication-consolidation.agent.md` template and its governing `duplication-consolidation.instructions.md` (the agent quoting/restating its own instructions file), plus independently in `cleanup.agent.md`'s own constraint.
