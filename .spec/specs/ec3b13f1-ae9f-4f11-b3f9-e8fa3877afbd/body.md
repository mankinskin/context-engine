
# agent-tooling/mcp-tool-grants

## Goal

Specify a per-template MCP tool-grant contract so each agent template in
`.agents/agents/` exposes only the MCP tool surface its documented role
requires, and so `tool_search` / deferred-tool-loading availability for
sub-agents is either enabled or explicitly and correctly documented as
unavailable.

## Problem

Every template in `.agents/agents/` currently grants the same wildcard MCP
block (`'audit-mcp/*'`, `'context-mcp/*'`, `'session-mcp/*'`, `'spec-mcp/*'`,
`'ticket-mcp/*'`, etc.) regardless of role. A read-only lookup agent
(`explore.agent.md`) loads the full 35-tool `session-mcp` surface and the
12.9k-char `context-mcp` schema it never calls. Empirical probes recorded in
`.ticket/tickets/cd19fed4-44d5-4ef0-848c-19753f1539b0` measured 164 tools and
an estimated ~24k tokens of fixed MCP schema per sub-agent turn, with
`tool_search` unavailable to sub-agents in the probed sessions. This is paid
on every turn of every delegation regardless of task size.

## Scope

- A tool-grant schema for the agent template format: for each template, an
  explicit MCP tool set (not a blanket `server/*` wildcard) or a wildcard grant
  paired with a one-line role justification recorded in the template.
- A capability-to-tool mapping: which existing MCP servers/tools
  (`ticket-mcp`, `spec-mcp`, `test-mcp`, `rule-mcp`, `session-mcp`,
  `context-mcp`, `peek-mcp`, `feedback-mcp`, `audit-mcp`,
  `compact-terminal-mcp`) each documented agent role legitimately needs, per
  the working baseline already listed in `cd19fed4`'s Scope section.
- Enforcement semantics: how the declared grant is applied at dispatch time
  (which layer filters/validates the tool list handed to a spawned sub-agent
  against its template's declared grant).
- `tool_search` / deferred-tool-loading availability for sub-agents: whether it
  can be enabled, and if not, the documented reason and the fallback (explicit
  per-template grants) that this spec's schema provides instead.
- A regression probe contract: a repeatable self-report check (tool count,
  and/or measured schema payload size) that can be re-run to detect drift back
  toward wildcard grants.

## Non-goals

- New tool categories or new MCP servers for filesystem editing, filesystem
  operations, or repo-wide search. Those are tracked independently by epic
  `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b` and already have
  draft specs (`agent-tooling/file-editing`, `agent-tooling/filesystem-operations`,
  `agent-tooling/repo-wide-search`) that are out of scope here and not a
  prerequisite for this contract.
- Per-template `model:` tier declarations, which are the separate concern of
  `.ticket/tickets/66acb737-71d6-4585-a921-b597f7c88e8e`.
- Redesigning any individual MCP server's tool schema or descriptions.

## Acceptance Criteria

1. The schema for declaring a template's MCP tool grant (explicit tool list,
   or wildcard plus required one-line justification) is defined and applies
   uniformly across all templates in `.agents/agents/`.
2. The capability-to-tool mapping used to decide each template's grant is
   documented, covering at minimum the Explore, Commit, Implement, Spec, and
   Testing roles named in `cd19fed4`.
3. The enforcement point (what validates or filters a spawned sub-agent's
   tool list against its template's declared grant) is identified.
4. `tool_search` availability for sub-agents is resolved to one of: enabled,
   or documented unavailable with the reason, matching `cd19fed4` AC4.
5. A regression-probe method (self-report tool count and/or measured schema
   payload size, using one consistent measurement method for before/after
   comparison) is defined so drift is detectable, matching `cd19fed4` AC1,
   AC2, and AC5.
6. Every acceptance criterion in `cd19fed4` traces to a requirement in this
   spec's Scope or Acceptance Criteria; no `cd19fed4` AC depends on an
   undefined term.

## Traceability

- Ticket (primary): `.ticket/tickets/cd19fed4-44d5-4ef0-848c-19753f1539b0`
  — this spec defines the contract `cd19fed4`'s acceptance criteria assume.
- Ticket (dependent): `.ticket/tickets/66acb737-71d6-4585-a921-b597f7c88e8e`
  — depends on `cd19fed4`; this spec's tool-grant schema is a prerequisite
  input for that ticket's per-template `model:` declaration work, since both
  add fields to the same template format.
- Related but out of scope: epic `.ticket/tickets/e342cc4c-a7a4-42de-81fc-572d0497d12b`
  and its draft specs `agent-tooling/file-editing`, `agent-tooling/filesystem-operations`,
  `agent-tooling/repo-wide-search` — new tool-category design, not a
  prerequisite for this contract.
- Guidance: `.agents/instructions/orchestration/model-routing.instructions.md`,
  `.agents/instructions/orchestration/orchestrator-delegation.instructions.md`.

## Validation Evidence

Expected before review:

- A regression probe run recording the Explore Agent's self-reported tool
  count and measured MCP schema payload size, before and after grants are
  scoped, using the same measurement method both times.
- Manual review confirming each template's grant is either explicit or
  wildcard-plus-justification, with no unexplained wildcard remaining.
