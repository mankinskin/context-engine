---
description: "Gather the current implementation track into a compact, session-aware handoff prompt that carries findings, entity references, and next actions into a new session."
name: "handoff"
argument-hint: "[ticket-id|track|current]"
agent: "agent"
---

# Handoff

Create a compact handoff prompt that a new session can use to resume a specific implementation track quickly. Carry over the current session's hard-won context: decisions, findings, blockers, suggested next steps, and entity references that would be expensive or error-prone to rediscover.

Reference [AGENTS](../../AGENTS.md), [session-optimization instructions](../instructions/session-optimization.instructions.md), [ticket-next](./ticket-next.prompt.md), [next](./next.prompt.md), [ticket-system instructions](../instructions/ticket-system.instructions.md), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), [ticket-mcp](../../memory-api/tools/mcp/ticket-mcp/README.md), [spec-cli](../../memory-api/tools/cli/spec-cli/README.md), and [audit-cli](../../memory-api/tools/cli/audit-cli/README.md).

Act as a session summarizer and agent orchestrator: summarize the current session's useful state, then shape it into the first prompt the next agent should receive.

## Workflow

1. Read the slash-command text and infer the implementation track, ticket, finding set, or current workstream to hand off.
2. Always carry forward the high-value session context as relative file references:
- target working directory to start from
- findings, decisions, and motivations from the current session
- concrete long-horizon goal, suggested next steps, including the first concrete action and first validation check
	- entity references: tickets, specs, rule ids, sessions, audits, generated files, source files, logs, validation evidence, and blockers
	- session-audit selectors, schema-versioned session artifacts, and report fields when the handoff track depends on persisted session evidence
- board ownership, expected check-in or check-out actions, related active or previous sessions
- persisted `session-api` history captured by the Stop hook
3. Prefer authoritative references over summaries, but briefly explain why each referenced item matters so the next agent can act without reconstructing the conversation.
4. Treat persisted session history as diagnostic evidence, not default prompt material:
- prefer durable findings, blockers, and next actions over raw transcript replay
- mention upstream request-shaping issues such as repeated state checks, oversized tool output, or routine-action reasoning when they matter to the restart path
- keep large transcript or log artifacts as pointers with a short reason they matter
5. Keep reusable workflow noise out of the handoff paragraph. Do not restate ordinary ticket/spec workflow, tool manuals, or generic validation doctrine unless a specific exception, blocker, or required command from this session matters.
6. Ticket references are strict:
- use exact full ticket UUIDs for all ticket mentions in the handoff
- do not invent shorthand-only identifiers without resolvable canonical ids
- if shorthand labels are used for readability (for example `CH1`, `T2`, `epic`, `child-a`), include an explicit ticket legend that maps each shorthand label to the exact full UUID and canonical title
7. Shorthand and placeholder declarations must appear at the top of the handoff output before the overview, if any are used:
- include a `Shorthand And Placeholder Legend` section immediately after the opening paragraph heading, if any shorthands are used
- list every shorthand token or placeholder used anywhere in the handoff (for example `T1`, `EPIC`, `SPEC-A`, `SESSION-X`, `<workspace>`)
- map each shorthand to its authoritative entity id/title/path
8. If the current track is unclear, abort the handoff and ask for clarifying questions.

## Response

Return:
- the full handoff response inside a fenced plain-text block using `~~~text` and closing `~~~` so copied output preserves literal markdown links
- render all entity references per the Clickable Reference Policy in `AGENTS.md`
- clear sections with small paragraph headings
- a short introduction/overview in one paragraph
- findings, decisions, blockers, and suggested goals in structured lists
- key entities (tickets, specs, logs, sessions, rules, ...) as markdown links with a short reason each matters
- all file references must use markdown links with forward slashes only
- for files in the current directory, use `./`-prefixed links (for example `[./AGENTS.md](./AGENTS.md)`)
- do not emit bare file paths or Windows-style backslashes
- strict ticket references using full UUIDs
- a `Shorthand And Placeholder Legend` section near the top that defines all shorthand and placeholders used later in the handoff, if any
- a ticket legend section mapping any shorthand labels used in the handoff to exact full UUID + canonical ticket title
- one epic ticket if available or alternatively a long-horizon goal we are working towards
- the next actions in execution order
- a completion status section for follow-up handoffs covering each next action (`not-started`, `in-progress`, `completed`, or `blocked`) with a short note
- a definition of done
- any board or persisted-session note that materially affects the restart path
- any transcript artifact pointer only when the artifact cannot be reduced to a durable finding in the handoff itself
- no unresolved references: every shorthand, ticket id, spec id, session id, rule id, or file mentioned in findings/decisions/blockers/next actions is defined in the legend sections or linked in key entities
- the instruction to execute the next actions in order and continue until all are completed or explicitly blocked
