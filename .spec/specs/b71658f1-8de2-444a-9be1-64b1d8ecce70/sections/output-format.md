## Inline Output Format

The Iteration Agent's chat summary uses a **fixed 7-field bold-label bullet block** (NOT a table), in this exact order:

- **Track:** ticket id(s) or implementation scope iterated
- **Phase outcomes:** one line each for Review (pass/fail), Interview (escalations resolved), Commit (committed / skipped / declined by user), Handoff (forward or re-packaged inline)
- **Review findings:** nested list of `criterion → verdict`, one per acceptance criterion
- **Ticket transitions:** state before → state after, per ticket
- **Commits:** commit sha(s) produced this iteration, or `none`
- **Handoff package:** clickable link + one-line objective (never the full eight-field package body)
- **Next actions:** immediate next steps for human or next agent

Every field is always rendered; empty fields render as `none`.

**Handoff persistence rule:** The full eight-field handoff package is persisted via `session_handoff` but NEVER printed in the chat message. The summary block reports only a clickable link to the persisted handoff plus a one-line restatement of its `objective`.

## Design Decision (D7): Dropped Summary Fields

The `Blockers` field was deliberately dropped from the summary block because it is confusing for an unblocked track. Unresolved escalations are reported under `Next actions` instead, since an escalation IS the next action. The former `Gates enforced` field was likewise dropped because it conflates two concerns (which gates applied vs. which gate failed), which is already captured more clearly in the `Phase outcomes` field.