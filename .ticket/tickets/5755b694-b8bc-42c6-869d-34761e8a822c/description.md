## Goal

Author the **iteration-loop workflow spec**: the phase model, canonical ordering, and gates that govern how a finished implementation becomes the next handoff.

## Content

- **Phase model**: implementation (self-contained) → transition (Iteration Agent) → next implementation.
- **Canonical order**: Review → Interview → Commit → Handoff (only approved work committed).
- **Gates**:
  - Review gate: acceptance criteria verified before commit; failures return ticket to `in-implementation`.
  - Escalation gate: no ticket reaches `done` with an unresolved user escalation.
  - Loop-closure gate: every finished implementation terminates in a durable handoff package + a ticket transition (closed or returned).
- **Re-packaging rule**: a returned (failed-review) ticket is immediately re-packaged into the next handoff.
- **Roles**: which existing agent owns each phase (Review, Interview, Commit, Handoff) and what the Iteration Agent adds (sequencing + next-handoff authoring).

## Acceptance criteria

- Spec exists describing phases, ordering, and the three gates.
- Spec cross-links the Iteration Agent ticket (T1), the handoff-package schema spec (T2), and the durable rules (T5).
- Spec is linked to this ticket and to the epic.

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