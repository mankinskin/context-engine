## Problem

All 24 delegations across sessions `3e9bc20b` and `41966513` passed `model: "Claude Sonnet 4.5 (copilot)"`. Not one used a cheaper tier.

The cause is structural: **no template in `.agents/agents/` declares a `model:` field**. Routing is entirely the orchestrator's per-call choice at dispatch time, and it defaulted to the same tier every time. `.agents/instructions/orchestration/model-routing.instructions.md` describes a tiered ladder that nothing enforces or defaults to.

Sonnet 4.5 was used for work that plainly did not need it:

| delegation | turns | actual work |
|---|---|---|
| `Compact the spilled briefing` | 3 | 1 `read_file`, then summarize |
| `Empirical subagent tool-list probe` | 3 | zero tool calls, pure self-report |
| `Locate canonical handoff artifact` | 8 | file lookup, no edits |
| `Audit commit file sweep` | 15 | `git show --stat` + grep |
| `Commit compact-terminal-api extraction` | 28 | 26 of 27 tool calls were `git status` / `git diff` / `git commit` |

Roughly half of the 24 delegations were mechanical: lookup, summarization, git mechanics, or evidence gathering with no design judgement.

## Interaction with existing price awareness

`445a2d76` (done) already enforces orchestrator mode for expensive models and gates individual tools by cost — during the analysis of this epic, `mcp_ticket-mcp_subgraph` was refused for an Opus caller with *"Delegate via runSubagent(model=<cheaper>, ...)"*. So the gate exists at the **caller** level. What is missing is the **callee** default: nothing declares what tier each agent class should run at.

## Scope

- Add a `model:` field to the agent template schema and populate it per class:
  - cheap tier: Explore, Research, Transcription, Handoff-compaction, Commit
  - mid tier: Testing, Spec, Ticket Refinement, Audit
  - strong tier: Implement, Review, Interview, Iteration
  - orchestrator tier: Orchestrator only
- Make the declared model the default when `runSubagent` omits `model`, so routing is correct even when the dispatcher does not think about it.
- Allow the orchestrator to override upward with a stated reason, and record the override in the session so escalation is auditable.
- Define the tier ladder concretely in `model-routing.instructions.md` with named models, not abstract tiers.
- Add a cheap-tier escalation path: if a cheap agent fails or reports low confidence, re-dispatch once at the next tier rather than defaulting expensive.

## Acceptance Criteria

1. Every template in `.agents/agents/` declares a `model:` tier.
2. `runSubagent` without an explicit `model` resolves to the template's declared tier.
3. At least the Explore, Research, and Commit classes route to a tier cheaper than Sonnet 4.5.
4. Orchestrator overrides above the declared tier are recorded with a reason in the session record.
5. Measured against the benchmark in `10d21210` — whose scenario includes a mechanical delegation that should route cheap — model distribution across delegations is no longer uniform, and the mechanical delegation does not run on the strong tier.

## Evidence

- `runSubagent` arguments for all 24 delegations across both sessions, extracted from `events.json` — every one carried `Claude Sonnet 4.5 (copilot)`
- Per-delegation turn counts and tool profiles: `tmp/subagent_cost_probe.py`
- Existing but unenforced guidance: `.agents/instructions/orchestration/model-routing.instructions.md`
- Related: `445a2d76` model price awareness (caller-side gate), `6737a239` budget-offset grants

## Scope boundary vs `373072a9` — reviewed and revised

Reviewed 2026-07-27. Two reviewers disagreed: one judged this a clean mechanism/policy split from `373072a9`, the other judged it a 100% duplicate on the grounds that "the mechanism **is** the policy".

**Resolution: this ticket stays, but now `depends_on` `373072a9`.**

`373072a9` (Delegation decision policy: case -> cost-class mapping and allocation strategy) decides **which class maps to which tier**. This ticket implements the **default binding** that makes such a mapping take effect: each agent template declares its tier, so a `runSubagent` call omitting `model` still routes correctly. Both reviewers were partly right — the split is real, but implementing a binding before knowing what it binds to would hard-code an unreviewed mapping into 17 templates.

Sequencing: `373072a9` decides the mapping, then this ticket binds it into the templates and makes it the dispatch default. The concrete class-to-tier proposal in the Scope section above is **input to `373072a9`, not a decision made here**.