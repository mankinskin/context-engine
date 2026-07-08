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
2. Always carry forward the high-value session context as relative file references:
- target working directory to start from
- findings, decisions, and motivations from the current session
- concrete long-horizon goal, suggested next steps, including the first concrete action and first validation check
- entity references: tickets, specs, rule ids, sessions, audits, generated files, source files, logs, validation evidence, and blockers
- board ownership, expected check-in or check-out actions, related active or previous sessions
- persisted `session-api` history captured by the Stop hook
3. Prefer authoritative references over summaries, but briefly explain why each referenced item matters so the next agent can act without reconstructing the conversation.
4. Keep reusable workflow noise out of the handoff paragraph. Do not restate ordinary ticket/spec workflow, tool manuals, or generic validation doctrine unless a specific exception, blocker, or required command from this session matters.
5. Keep reusable workflow noise out of the handoff paragraph. Do not restate ordinary ticket/spec workflow, tool manuals, or generic validation doctrine unless a specific exception, blocker, or required command from this session matters.
6. If the current track is unclear, abort the handoff and ask for clarifying questions.

## Response

Return:
- clear sections with small paragraph headings
- a short introduction/overview in one paragraph
- findings, decisions, blockers, and suggested goals in structured lists
- key entities (tickets, specs, logs, sessions, rules, ...) as relative links with a short reason each matters
- one epic ticket if avaliable or alternatively a long-horizon goal we are working towards
- the next actions in execution order
- a definition of done
- any board or persisted-session note that materially affects the restart path
- the instruction to start with the first next action
