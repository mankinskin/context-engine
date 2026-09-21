Follow-up migration (successor run b4deeb1e) — the last two agent-customization surfaces are now hand-owned, closing the migration half of the epic.

PROMPTS (ticket 76d0ace3) — DONE:
- Stripped `rule-api:file generated=true` + `rule-api:entry` markers from all 17 marker-carrying `.agents/prompts/*.prompt.md` files (the other 4 prompt files were already marker-free).
- Parity proven: `git diff` (normalized, autocrlf=input) shows exactly 4 deletions per file (2 marker lines + 2 blank lines), 68 deletions total, 0 insertions; substantive diff under `--ignore-all-space` (excluding markers/blank lines) is empty — zero guidance changed.
- No stale "generated from a .rule entry" self-reference notes existed to remove (remaining `rule-api`/`generated file` mentions are substantive workflow guidance in rule/rule-target/commit prompts, left intact for parity).
- All prompts retain accurate `description` frontmatter.
- Deleted generator source `rule-targets/30-agents-prompts.yaml` via `git rm`.

AGENTS-MIG (ticket 16cfd19f) — DONE:
- Stripped markers from all 10 marker-carrying `.agents/agents/*.agent.md` files (11th, `ticket-refinement.agent.md`, was already marker-free). This includes the newly-added `review.agent.md` from concurrent Review Agent work.
- Parity proven: 4 deletions per file, 40 deletions total, 0 insertions; substantive diff empty; all agents retain `name`/`description` frontmatter; no stale generator notes.
- Deleted generator source `rule-targets/45-agents-agents.yaml` via `git rm -f` (it carried a benign pre-existing background edit adding the review-agent target — moot on deletion).

No-regeneration verification:
- `grep` confirms no remaining `rule-targets/*.yaml` references `.agents/prompts`, `.agents/agents`, `prompt.md`, or `agent.md` — no generator target covers either surface.
- `grep -rl "rule-api:" .agents/prompts .agents/agents` returns nothing.
- `rule-targets/00-imports.yaml` left unchanged: it imports sibling submodule directories, never the two deleted files individually (the root `rule-targets.yaml` aggregates the `rule-targets/` dir by glob), so deleting the files removes them from generation. This matches the CH-D precedent, which likewise did not edit `00-imports.yaml`.
- `.rule/entities/**` is gitignored; its stale generated-target records point to the now-deleted config yamls and have no regeneration path (no committed-state impact).

Ticket bookkeeping: state transitions to `in-review`/`done` remain blocked by the pre-existing `no schema for type 'task'` store loader issue (unchanged from CH-D). PROMPTS and AGENTS-MIG stay in `new` with work complete; completion recorded here and via board check-out.