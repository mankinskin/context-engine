---
description: "Create a short, reference-centric session handoff prompt and create or match the ticket or tracker follow-up items needed for the handoff track."
name: "handoff-tickets"
argument-hint: "[ticket-id|track|current]"
agent: "agent"
---

# Handoff Tickets

Create a compact handoff prompt for a new session and formalize the handoff track through the ticket workflow when needed.

Reference [AGENTS](../../AGENTS.md), [session-optimization instructions](../instructions/session-optimization.instructions.md), [ticket](./ticket.prompt.md), [tickets](./tickets.prompt.md), [ticket-next](./ticket-next.prompt.md), [ticket-system instructions](../instructions/ticket-system.instructions.md), [rule-target](./rule-target.prompt.md), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), and [ticket-mcp](../../memory-api/tools/mcp/ticket-mcp/README.md).

## Workflow

1. Read the slash-command text and determine the handoff track, current implementation slice, and whether new ticketing is needed.
2. Search existing tickets first so the handoff flow reuses or updates the authoritative ticket set instead of creating duplicates.
3. Inspect the current board, spec, validation, implementation references, and any persisted cross-session history needed to describe the track accurately.
4. Produce a short, paragraph-style, reference-centric handoff prompt for a new session.
5. If the current track is not already represented well enough in the ticket graph, create or refine the needed ticket or tracker ticket items:
- create one actionable ticket when the handoff points to one concrete next slice
- create a tracker ticket when the handoff needs a parent work package spanning multiple follow-up slices
- link or reference existing tickets when they already cover the work
6. Keep the ticketing workflow minimal and explicit:
- avoid duplicates
- keep titles scoped to the handoff track
- preserve canonical ticket paths and references
7. In the handoff paragraph, mention the ticket or tracker references that the next session should open first, plus any board check-in, check-out, or stale-entry issue that must be resolved before implementation resumes.
8. When persisted `session-api` history captured by the Stop hook materially improves the restart path, reference that history alongside the ticket and board pointers rather than restating the whole prior conversation.
9. Treat transcript history as diagnostic evidence for future prompt quality rather than as a prompt payload to replay:
- prefer concise findings about repeated tool chatter, oversized outputs, and routine-action reasoning
- keep the next-session handoff focused on the durable work state and the next concrete action
10. Do not implement the work in this prompt; stop after producing the handoff and the ticketing setup.

## Response

Return:
- the short handoff prompt in one paragraph
- created or matched tickets, rendered as canonical ticket links when available
- whether a tracker ticket was needed and why
- any board or persisted-session note that affects how the next session should resume
- the single next action for the new session
