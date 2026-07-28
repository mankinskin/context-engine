## Problem

The repo needs an agent capable of online search. `.agents/agents/research.agent.md` (line 4 tools list) and `.agents/agents/explore.agent.md` are both repo-scoped: neither carries a `web` grant. `web` is already granted on `default`, `iteration`, `review`, and `ticket-refinement`.

## Decisions (interview-resolved)

- Add `web` to the existing `research.agent.md`. Do NOT create a separate web-research agent template.
- `explore.agent.md` stays web-free by design — it is the fast, cheap, bounded read-only probe.
- Do NOT audit the existing `web` grants on default/iteration/review/ticket-refinement.

## Notes

Add an MCP/tool grant justification note matching the pattern every other template uses, and update the Research Agent description so its online-search capability is discoverable at dispatch time.