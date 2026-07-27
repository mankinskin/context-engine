## Story

A roast of the instruction corpus, commissioned during the review of epic `79c4ac3e`, found that the guidance layer is itself a material per-turn cost and that it demonstrably failed to change behaviour in the two sessions analysed there.

Measured: `.agents/instructions/` holds **40 files totalling ~107 KB**; `.agents/skills/` holds **11 files totalling ~109 KB**; `AGENTS.md` is 12.8 KB. The instruction and skill indexes are injected into every agent's context on every turn.

### Finding 1 — the guidance exists, is clear, and was ignored

`.agents/instructions/orchestration/file-inspection.instructions.md` states:

> "Use this suite instead of unbounded built-in file reads."
> "Never pull an entire file when only a targeted slice is needed."
> "Full-file reads should become the exception. The `--all` flag is intentionally named to make the cost visible."

`.agents/instructions/orchestration/compact-output.instructions.md` states:

> "Prefer `rtk <cmd>` over bare `<cmd>` — rtk filters/compresses output automatically."

The two sessions analysed under `79c4ac3e` ran **83 unbounded `grep`/`find`/`ls` calls** and made heavy bare-CLI use anyway. The guidance is not unclear — it is unenforced and not surfaced at decision time. Writing it more clearly will not change the outcome.

### Finding 2 — three files define three different delegation triggers

| file | trigger |
|---|---|
| `orchestrator-delegation.instructions.md` | cost threshold — delegation mandatory when `output_mtok` exceeds X |
| `model-routing.instructions.md` | tool availability — section is inert if no cheaper-model tool is loadable |
| `phase-separation.instructions.md` | agent role — structurally enforced by withholding tools |

An agent reading all three cannot resolve precedence. `AGENTS.md` defines a precedence table by *source class* (system > developer > AGENTS.md > path-scoped > prompt) but not between path-scoped files at the same level.

### Finding 3 — unmeasurable rules

Rules phrased so that no violation is detectable:

- `file-inspection.instructions.md`: *"Never pull an entire file when only a targeted slice is needed"* — the agent defines "needed".
- `escalation-gate.instructions.md`: *"If the handoff package is missing implementation-ready context... stop and escalate"* — "implementation-ready" is undefined, and the sessions show agents proceeding with partial context repeatedly.

## Scope

- Inventory every file in `.agents/instructions/` and `.agents/skills/`: size, injection surface, and last evidence of influencing a decision.
- Identify duplicate coverage. `orchestrator-delegation.instructions.md` and `model-routing.instructions.md` overlap substantially; determine whether they merge.
- Resolve the delegation-trigger contradiction with an explicit precedence ladder, and extend the `AGENTS.md` precedence table to cover conflicts between path-scoped files at equal specificity.
- For each unmeasurable rule: operationalize it with a threshold, a required-field list, or a concrete check — or delete it.
- Distinguish guidance that must be resident from guidance that can be loaded on demand. The skills mechanism already supports on-demand loading; instructions largely do not.
- Reduce resident corpus size, with the reduction target set from the inventory rather than assumed up front.

## Acceptance Criteria

1. An inventory exists covering every instruction and skill file with its size and injection surface.
2. Duplicate coverage is identified and resolved by merge or deletion, with the decision recorded per file.
3. The delegation-trigger contradiction is resolved by a stated precedence ladder, and `AGENTS.md` covers equal-specificity path-scoped conflicts.
4. Every retained rule is measurable: a reader can state what observation would prove it violated.
5. Resident corpus size is reduced against the measured baseline of ~107 KB instructions + ~109 KB skills, with the achieved reduction reported.
6. Guidance retained on the grounds that it is followed cites at least one session where it demonstrably changed a decision.

## Relationship to other work

- `79c4ac3e` (delegation cost) owns the **tool schema** half of the per-turn fixed prefix. This epic owns the **guidance text** half. They are the same cost term measured on different payloads; keep them separate so neither blocks the other.
- `cd19fed4` scopes MCP tool grants. That is tool schemas, not instruction text — no overlap.
- `77eb143b` measures substitutable shell commands. Its finding that guidance-based enforcement failed is the empirical basis for Finding 1 here.
- `45ff05c9` is a retrospective grouping tracker for historical rules/specs/agent-guidance work. It does not own forward work, so this epic stands separately and links to it for lineage.

## Evidence

- Corpus: `.agents/instructions/` (40 files, ~107 KB), `.agents/skills/` (11 files, ~109 KB), `AGENTS.md` (12.8 KB)
- Ignored-guidance evidence: `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json`, `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json`
- Command classification: `tmp/subagent_cost_probe.py`