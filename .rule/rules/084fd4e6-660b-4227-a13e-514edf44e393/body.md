---
description: "Gather the current implementation track into a compact, session-aware handoff prompt that carries findings, entity references, and next actions into a new session."
name: "handoff"
argument-hint: "[ticket-id|track|current]"
agent: "agent"
---

# Handoff

Create a compact handoff prompt that a new session can use to resume a specific implementation track quickly. Carry over the current session's hard-won context: decisions, findings, blockers, suggested next steps, and entity references that would be expensive or error-prone to rediscover.

Reference [AGENTS](../../AGENTS.md), [ticket-next](./ticket-next.prompt.md), [next](./next.prompt.md), [ticket-system instructions](../instructions/ticket-system.instructions.md), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), [ticket-mcp](../../memory-api/tools/mcp/ticket-mcp/README.md), and [spec-cli](../../memory-api/tools/cli/spec-cli/README.md).

Act as a session summarizer and agent orchestrator: summarize the current session's useful state, then shape it into the first prompt the next agent should receive.

## Workflow

1. Read the slash-command text and infer the implementation track, ticket, finding set, or current workstream to hand off.
2. Inspect only enough current ticket, board, spec, validation, code, and persisted session history to identify what is session-specific and valuable to carry forward. Leave generic workflow procedure for the next session to retrieve from the referenced instructions.
3. Always carry forward the high-value session context when it exists:
- findings, decisions, and acceptance judgments from the current session
- suggested next steps, including the first concrete action and first validation check
- entity references: tickets, specs, rule ids, generated files, source files, symbols, commands, logs, validation evidence, and blockers
- board ownership, expected check-in or check-out actions, or stale-entry concerns that affect safe restart
- persisted `session-api` history captured by the Stop hook when it materially improves restart speed
4. Prefer authoritative references over summaries, but briefly explain why each referenced item matters so the next agent can act without reconstructing the conversation.
5. Keep reusable workflow noise out of the handoff paragraph. Do not restate ordinary ticket/spec workflow, tool manuals, or generic validation doctrine unless a specific exception, blocker, or required command from this session matters.
6. Produce one compact paragraph optimized for reuse as the first prompt in a new session.
7. If the current track is unclear, say what is missing and name the single best source to inspect next.

## Response

Return:
- a short handoff prompt in one paragraph
- findings, decisions, blockers, and suggestions that should survive into the next session
- key entity references folded into the paragraph with a short reason each matters
- any board or persisted-session note that materially affects the restart path
- the single next action and first validation check for the new session
