## Problem

Every sub-agent spawned in sessions `3e9bc20b` and `41966513` carried the full MCP tool surface. Empirical probe from subagent `[2]` in `41966513`:

> **Total tools available:** 164 — **MCP tools (135 total)** ... `tool_search` is not available in my current tool set.

Two sub-agents in `3e9bc20b` independently reasoned *"Tool search is disabled"* after a tool failure, then re-planned around it.

Measured `tools/list` payload sizes (probed directly against the debug binaries):

| server | tools | schema chars |
|---|---|---|
| session-mcp | 35 | 23,896 |
| context-mcp | 3 | 12,898 |
| spec-mcp | 19 | 11,103 |
| rule-mcp | 16 | 8,448 |
| test-mcp | 6 | 8,096 |
| audit-mcp | 6 | 2,843 |
| compact-terminal-mcp | 2 | 2,616 |
| feedback-mcp | 5 | 2,199 |
| peek-mcp | 4 | 1,526 |
| ticket-mcp | 33 (lazy-reports 1) | ~20,000 est. |

Total ~94k characters, an estimated **~24k tokens of MCP schema** (character-derived at an assumed ~4 chars/token; not tokenizer-measured), before built-in tool schemas (~29 tools with verbose descriptions), `AGENTS.md` (12.8 KB), the 40-entry instruction index, and the 11-entry skills index. Fixed prefix lands at **~35-40k tokens, re-sent every turn**.

Every template in `.agents/agents/` grants the same wildcard block regardless of role:

```
tools: [read, search, execute, agent, ..., 'audit-mcp/*', 'compact-terminal-mcp/*',
        'context-mcp/*', 'feedback-mcp/*', 'log-viewer-mcp/*', 'peek-mcp/*',
        'rule-mcp/*', 'session-mcp/*', 'spec-mcp/*', 'test-mcp/*', 'ticket-mcp/*']
```

`explore.agent.md` is a read-only lookup agent and still loads `session-mcp/*` (35 tools, 23.9k chars) and `context-mcp/*` (12.9k chars). `iteration.agent.md` and `orchestrator.agent.md` are the only templates with restrained grants.

## Why this is the dominant lever

A 40-turn sub-agent pays roughly 40 x 37k = **1.5M input tokens on fixed prefix alone**, before any task work. Sub-agents in these sessions ran 3-64 turns (median ~20). Even the 3-turn `Compact the spilled briefing` agent, which made exactly one `read_file` call, paid ~110k tokens of schema. This term alone explains the observed $1-3 per delegation.

## Scope

- Determine whether `tool_search` / deferred tool loading can be enabled for sub-agents. If it can, enable it — this fixes the problem without per-template curation.
- If it cannot, replace every wildcard grant with an explicit, role-justified tool set per template. Working baseline:
  - `explore.agent.md`: `read, search, peek-mcp/*` + read-only ticket/spec getters
  - `commit.agent.md`: `execute, read` + `ticket-mcp` state transitions only
  - `implement.agent.md`: `edit, read, execute, peek-mcp/*, ticket-mcp, spec-mcp, test-mcp`
  - `spec.agent.md`: `spec-mcp/*, ticket-mcp` readers, `read, edit`
  - `testing.agent.md`: `execute, read, test-mcp/*, log-viewer-mcp/*`
- Audit whether `session-mcp` (35 tools) and `context-mcp` (3 tools, 12.9k chars) need to be in any sub-agent at all, or only in the orchestrator/iteration layer.
- Consider trimming individual tool descriptions on the highest-cost servers: `context-mcp` averages **4.3k chars per tool** across only 3 tools.

## Acceptance Criteria

1. A freshly spawned Explore Agent, asked to self-report its tool inventory, returns <=60 tools (down from 164).
2. Measured MCP schema payload for the Explore Agent is <=10k tokens (down from an estimated ~24k). Measure both endpoints with the same method so the comparison holds regardless of the chars/token assumption.
3. No agent template retains a wildcard MCP grant that its documented role does not require; each remaining grant has a one-line justification in the template.
4. `tool_search` availability for sub-agents is either enabled or explicitly documented as unavailable with the reason.
5. A regression probe exists that re-runs the empirical tool-count self-report and records the number, so drift is detectable.

## Evidence

- Probe transcript: `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json` events 220-240
- Analysis: `tmp/subagent_cost_probe.py`
- Templates: `.agents/agents/`
- Registry: `.vscode/mcp.json`, `.github/mcp.json`