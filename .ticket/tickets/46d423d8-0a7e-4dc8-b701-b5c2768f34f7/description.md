## Problem
R2 (global state overview: cross-workspace board, state distribution, highest-priority next) has no template today. It is folded into Explorer per the merge map but needs its own concrete workflow, not just a reserved slot.

## Scope
- Implement the R2 mode inside the Explorer template authored in C2: a request pattern that triggers a fixed call sequence of `board_show` -> `next_tickets` -> `list_tickets` (state distribution) and returns a compact cross-workspace summary.
- Wire this mode into the C1 routing table row for R2.

## Affected paths
- .agents/agents/explorer.agent.md

## Acceptance criteria
- [ ] Explorer template's R2 mode documents the exact tool call sequence (board_show, next_tickets, list_tickets)
- [ ] R2 mode output format is a compact state summary, not raw dumps
- [ ] Routing table row for R2 (from C1) points at this mode
