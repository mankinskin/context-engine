---
description: "Gather the current implementation track into a short, paragraph-style, reference-centric handoff prompt for starting a new session quickly."
name: "handoff"
argument-hint: "[ticket-id|track|current]"
agent: "agent"
---

# Handoff

Create a compact handoff prompt that a new session can use to resume a specific implementation track quickly without rereading the whole conversation.

Reference [AGENTS](../../AGENTS.md), [ticket-next](./ticket-next.prompt.md), [next](./next.prompt.md), [ticket-system instructions](../instructions/ticket-system.instructions.md), [ticket-cli](../../memory-viewers/memory-api/tools/cli/ticket-cli/README.md), [ticket-mcp](../../memory-viewers/memory-api/tools/mcp/ticket-mcp/README.md), and [spec-cli](../../memory-viewers/memory-api/tools/cli/spec-cli/README.md).

## Workflow

1. Read the slash-command text and infer the implementation track, ticket, or current workstream to hand off.
2. Inspect the current ticket, board, spec, validation, code context, and any persisted cross-session history just enough to identify the active track and the next actionable slice.
3. Prefer authoritative references over restating large summaries:
- ticket links and exact ticket folder paths when available
- spec links and ids when relevant
- touched files, symbols, commands, logs, and blockers
- board ownership, expected check-in or check-out actions, or stale-entry concerns when they matter
- persisted `session-api` history captured by the Stop hook when that history materially helps the next session resume faster
4. Produce a short handoff as one compact paragraph that is reference-centric and optimized for reuse as the first prompt in a new session.
5. Keep the handoff concrete:
- what track is active
- what has already been done
- what references matter most
- what the next action should be
- whether the next session needs to resolve board state or can rely on the persisted session trail directly
6. Do not turn the result into a full retrospective summary; keep only the details that help the next session start fast.
7. If the current track is unclear, say what is missing and name the single best source to inspect next.

## Response

Return:
- a short handoff prompt in one paragraph
- the key references folded into that paragraph
- any board or persisted-session note that materially affects the restart path
- the single next action for the new session
