## Goal

Author a new **Iteration Agent** template (`.agents/agents/iteration.agent.md`) and matching prompt (`.agents/prompts/iteration.prompt.md`). It is a **thin orchestrator** for the post-implementation transition phase.

## Canonical sequence

**Review → Interview → Commit → Handoff** (only approved work is committed).

1. **Review** — delegate to the Review Agent to verify acceptance criteria against the finished, validated implementation. Findings become follow-up tickets; unmet criteria return the ticket to `in-implementation`.
2. **Interview** — delegate to the Interview Agent to resolve any remaining open questions / escalations with the user.
3. **Commit** — delegate to the Commit Agent to commit only the approved work (pre-commit hooks, rule sync, generated files, submodule pointers, conventional messages).
4. **Handoff** — delegate to the Handoff Agent to (re)define the next self-contained handoff package for the next implementation session.

## Behavior

- Orchestrator only: it sequences and gates, and authors the next-handoff decision; it does not implement code, run research, or clarify with the user directly (that is delegated to Interview).
- A failed review returns the ticket and **immediately re-packages that returned work into the next handoff** (see epic).
- Advances three axes: the repo (commit), the tickets (close/return), and user content/feedback (review + interview).
- Binds a durable session and persists progress as a workflow graph; ends with `session_handoff`.

## Acceptance criteria

- `.agents/agents/iteration.agent.md` exists with tool grants limited to orchestration + delegation (agent, session-mcp, ticket-mcp, spec-mcp, read; no direct edit/search-heavy surface beyond what gating needs).
- `.agents/prompts/iteration.prompt.md` exists and documents the Review→Interview→Commit→Handoff sequence and gates.
- The template references the existing Review, Interview, Commit, and Handoff agents by their `.agents/agents/*.agent.md` names.
- The agent is listed in the workspace agent catalog after a rule scan.

## Confirmed Decisions (2026-07-27)

**D1. Interview runs on BOTH paths.** Previously the Interview phase was skipped when review failed. Now: Review → Interview (always) → escalation gate → review gate → Commit → Handoff. Rationale: a returned handoff package must carry an empty `open_escalations` list, so review findings that raise open questions must be interviewed before the package is written. Skipping Interview on failure was a contradiction with the handoff-package schema.

**D2. Iteration Agent owns ALL ticket state transitions.** Sub-agents report verdicts and findings only. The Review Agent is now strictly verdict-only: it must never call `close_ticket`, never pass `to_state` to `update_ticket`, and never move a spec to `reviewed`. This applies ALWAYS, not just when the Review Agent is invoked by the Iteration Agent.

**D3. WIP commit on failed review is user-gated.** When a review fails, the Iteration Agent must ASK THE USER whether to commit the partial work as WIP before stopping. If approved, delegate to the Commit Agent. If declined, leave the worktree dirty and report that fact in the summary. It is neither always-commit nor never-commit.

**D4. Handoff is persist-only in chat.** The full eight-field handoff package is NEVER printed in the chat message. The summary block reports only a clickable link to the persisted handoff plus a one-line restatement of its `objective`.

**D5. Model tiering is non-uniform.** The Review phase gets one tier ABOVE the cheap threshold (prefer "Claude Sonnet 4.5 (copilot)"). The Interview, Commit, and Handoff phases stay AT the cheap threshold (prefer "Claude Haiku 4.5 (copilot)", "GPT-5 mini (copilot)", "Gemini Flash 2.0 (copilot)"). When models are equal in cost, prefer the latest generation.

**D6. Inline output format is a fixed 7-field bold-label bullet block** (NOT a table), in this exact order:
- **Track:** ticket id(s) or implementation scope iterated
- **Phase outcomes:** one line each for Review (pass/fail), Interview (escalations resolved), Commit (committed / skipped / declined by user), Handoff (forward or re-packaged inline)
- **Review findings:** nested list of `criterion → verdict`, one per acceptance criterion
- **Ticket transitions:** state before → state after, per ticket
- **Commits:** commit sha(s) produced this iteration, or `none`
- **Handoff package:** clickable link + one-line objective (per D4)
- **Next actions:** immediate next steps for human or next agent
Every field is always rendered; empty fields render as `none`.

**D7. Dropped fields.** The former `Gates enforced` field is removed as noise. The former `Blockers` field is removed as confusing on an unblocked track — unresolved escalations are now reported under **Next actions** instead.

## Implementation Status

The following files have been updated to match the decisions above:
- `.agents/agents/iteration.agent.md`
- `.agents/prompts/iteration.prompt.md`
- `.agents/agents/review.agent.md`

Re-review is needed against the new policy.