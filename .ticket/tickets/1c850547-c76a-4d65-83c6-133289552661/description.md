## Problem
17 agent templates + 24 prompts have 15 colliding pairs and 10 orphan files. Need 8 merged templates plus one new standalone Telemetry template, each declaring an explicit MCP tool list or a wildcard + one-line role-justification per spec ec3b13f1.

## Scope
Author new template files (do not yet delete superseded ones — that's C3):
1. Explorer (R1,R2,R3,R4) — merges explore.agent.md, audit.agent.md, roast.agent.md, research.agent.md, audit.prompt.md. Include new R2 state-overview mode (deferred to C4 for full implementation, but template must reserve the mode).
2. Refinement (R5,R9) — merges spec.agent.md, ticket-refinement.agent.md, interview.agent.md, spec.prompt.md, interview.prompt.md, ticket.prompt.md, tickets.prompt.md.
3. Implementer (R10) — merges implement.agent.md, implement.prompt.md, next.prompt.md, ticket-next.prompt.md.
4. Test & Harness (R6,R7) — merges testing.agent.md, debug-test.prompt.md, tdd.prompt.md.
5. Reviewer (R11) — merges review.agent.md, reviews.prompt.md.
6. Orchestrator (R14) — reauthor from orchestrator.agent.md ONLY; must remain pure-delegation (no direct file/search/execute/MCP tool grants beyond dispatch).
7. Closer (R5 transitions) — merges iteration.agent.md, iteration.prompt.md, handoff.agent.md, handoff.prompt.md, handoff-tickets.prompt.md, commit.agent.md, commit.prompt.md.
8. Strategy (R12,R13) — merges simplify.agent.md, rule.prompt.md, rule-target.prompt.md.
9. NEW Telemetry (R8) — standalone; grants session_tool_metrics, session_query-class tooling for cost/latency/outage measurement.

## Coordination
- Ticket 3df54f79 (open) is currently editing `.agents/agents/research.agent.md` to grant web tooling; research.agent.md folds into Explorer here. Confirm 3df54f79's change lands in Explorer's merged tool grant, or sequence with that ticket's owner to avoid clobbering.
- Every template must comply with spec ec3b13f1 (explicit tool list or wildcard + justification) and resolve tool categories against draft spec 7c9757a7.

## Affected paths
- .agents/agents/explorer.agent.md (new)
- .agents/agents/refinement.agent.md (new)
- .agents/agents/implementer.agent.md (new)
- .agents/agents/test-harness.agent.md (new)
- .agents/agents/reviewer.agent.md (new)
- .agents/agents/orchestrator.agent.md (rewritten)
- .agents/agents/closer.agent.md (new)
- .agents/agents/strategy.agent.md (new)
- .agents/agents/telemetry.agent.md (new)

## Acceptance criteria
- [ ] All 9 templates exist with explicit tool grants or wildcard+justification per ec3b13f1
- [ ] Orchestrator template contains no direct file/search/execute grants — dispatch-only
- [ ] Each template's role coverage matches the approved merge map exactly (no missing/extra source file folded in)
- [ ] Explorer template documents the reserved R2 state-overview mode slot
- [ ] Cross-reference note added linking to ticket 3df54f79 and specs ec3b13f1, 7c9757a7
