## Problem
Guidance corpus: 101 files, ~103,750 tokens. `.agents/agents/**` = 24,168 tokens; only 3,641 tokens load unconditionally per session; 5 orchestration instruction files alone = 16,898 tokens. 15 colliding template pairs across 17 templates (research/explore/audit/roast all = read-only investigation; interview/spec/ticket-refinement all = refinement; next.prompt.md / ticket-next.prompt.md / implement.prompt.md all = pick-next-ticket). 10 orphan files map to no role. Routing today is semantic-similarity guessing; AGENTS.md routes by prompt, never by agent template. Session data (226 sessions): median 2 turns, p90 6, max 29, zero >50 turns; 744 subagent dispatches — cost is fan-out rediscovery, not long conversations.

## Goal
Consolidate 17 `.agents/agents/*.agent.md` + 24 `.agents/prompts/*.prompt.md` into ~8 role-based agent templates, and replace vibes-based template selection with a deterministic first-match-wins request->role routing table in AGENTS.md.

## Target role taxonomy (14 roles)
R1 artifact search | R2 global state overview (cross-workspace board, state distribution, highest-priority next) | R3 ticket-track research | R4 deep reading: audit/health-check/subgraph+edge inspection | R5 deep writing: artifact create/modify, multi-session planning, transition review | R6 acceptance testing + tool-gap/UX-friction capture | R7 executable specs + regression harness | R8 telemetry: cost/latency/outage measurement | R9 task refinement: plans, decision records, dependency + DAG shaping | R10 task execution along the DAG | R11 change review: regressions/deviations/failures | R12 strategic planning: benefit + effort estimation, global sequencing | R13 simplification | R14 orchestration + escalation to user

## Approved merge map (8 targets)
1. Explorer (R1,R2,R3,R4) <- explore.agent.md, audit.agent.md, roast.agent.md, research.agent.md, audit.prompt.md. Adds new R2 "state overview" mode calling board_show + next_tickets + list_tickets.
2. Refinement (R5,R9) <- spec.agent.md, ticket-refinement.agent.md, interview.agent.md, spec.prompt.md, interview.prompt.md, ticket.prompt.md, tickets.prompt.md.
3. Implementer (R10) <- implement.agent.md, implement.prompt.md, next.prompt.md, ticket-next.prompt.md.
4. Test & Harness (R6,R7) <- testing.agent.md, debug-test.prompt.md, tdd.prompt.md.
5. Reviewer (R11) <- review.agent.md, reviews.prompt.md.
6. Orchestrator (R14) <- orchestrator.agent.md ONLY. MUST stay pure-delegation.
7. Closer (R5 transitions) <- iteration.agent.md, iteration.prompt.md, handoff.agent.md, handoff.prompt.md, handoff-tickets.prompt.md, commit.agent.md, commit.prompt.md.
8. Strategy (R12,R13) <- simplify.agent.md, rule.prompt.md, rule-target.prompt.md.
NEW STANDALONE: Telemetry (R8) — no existing template covers cost/latency; calls session_tool_metrics, session_query.
KEEP UNTOUCHED: default.agent.md, transcription.agent.md, transform-transcript.prompt.md, memory-setup.prompt.md, user-training.prompt.md, sync-model-prices.prompt.md, build-validate-tools.prompt.md, tool-grant-regression-probe.prompt.md.

## Known risks
(a) merging a .prompt.md into an .agent.md removes a slash-command surface; (b) contaminating Orchestrator's pure-delegation guarantee if commit/handoff fold into it — hence Closer is separate; (c) fold-in roles R2/R7/R12 becoming invisible and inheriting the wrong model tier.

## Binding constraints
- Spec ec3b13f1 (reviewed) "Per-template MCP tool grants": every template in `.agents/agents/` must declare an explicit tool list OR a wildcard plus a one-line role-justification note. Every new template must comply.
- Spec 7c9757a7 (draft) "Default agent tool suite: five token-bounded tool categories": grants resolve against this category contract.
- Ticket 3df54f79 (open) edits `.agents/agents/research.agent.md` (grant web tooling to Research Agent). research.agent.md folds into Explorer — must coordinate to avoid clobbering.
- Ticket 67e254b5 (open, several unstarted deps away) will eventually rewrite the same AGENTS.md entry points to point at an external workflow-skill. Note as long-term interaction, not a blocker.
- Ticket f227c217 (in-implementation) touches AGENTS.md for compact-output style guidance only — different subject, file-level overlap only. Do NOT extend it.

## Acceptance criteria
- [ ] 14-role taxonomy and routing table documented in AGENTS.md
- [ ] 8 consolidated templates + Telemetry template exist, each ec3b13f1-compliant
- [ ] Superseded templates/prompts removed or merged; load-bearing slash-command surfaces preserved
- [ ] Explorer has R2 state-overview mode
- [ ] Orchestration instruction files compressed after C1/C2 land
- [ ] Deterministic routing validated by prompt-replay/routing check
