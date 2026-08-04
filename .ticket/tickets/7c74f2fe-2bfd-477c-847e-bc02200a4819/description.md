## Objective

Author a new dedicated agent template, `.agents/agents/context-enrichment.agent.md`, whose sole job is to enrich context for an in-review ticket by reconstructing its history via `sessions_for_ticket`, then review and close it (or send it back to open/ready).

## Background

Existing templates do not cover this role:
- `.agents/agents/review.agent.md` explicitly drops the `session-mcp` tool grant, so it cannot reconstruct context from session history.
- `.agents/agents/iteration.agent.md` has `session-mcp` but drives the current session's own Review→Interview→Commit→Handoff loop, not arbitrary-ticket history reconstruction.
- `.agents/agents/handoff.agent.md` has `session-mcp` but summarizes the current session only.

## Depends on

- Ticket bba9b313 (sessions_for_ticket capability) must ship first.
- Ticket for the 06cfe998 dogfood run should inform this template's workflow steps (its findings feed the template's procedure), but this ticket may proceed once the capability exists even if the dogfood ticket is still in progress.

## Locked design decisions (from interview 04-08-2026)

- New dedicated template (`.agents/agents/context-enrichment.agent.md`), not an extension of `review.agent.md`.
- Grants `session-mcp/*` (to call `sessions_for_ticket`) alongside the existing ticket/spec/test tool grants already used by review-style agents.
- Relation-strength usage: the template should default to `linked` tier (per interview: strict+linked, excluding transcript-scan `mentioned`... — actually locked as selectable strength with three tiers; template should expose the strength choice, not hardcode one) and let the operator/agent choose `strict`/`linked`/`mentioned` per ticket based on how much of the trail is missing.
- Closure authority: per the interview, autonomous closure is allowed — the template may transition a ticket to `done`/`accepted` without a separate human-confirmation step when it has recorded sufficient acceptance-criteria evidence, mirroring the dogfood ticket's decision.
- Inverse Ticket API query is out of scope for this template (deferred, see ticket 1ff57502).

## Acceptance Criteria

1. `.agents/agents/context-enrichment.agent.md` exists, declares an explicit `session-mcp` tool grant (not dropped as in review.agent.md), and states its sole objective as enrich→review→close for in-review tickets.
2. The template's documented workflow calls `sessions_for_ticket` (from ticket bba9b313) as its context-reconstruction entry point, and documents how to choose among the `strict`/`linked`/`mentioned` tiers per ticket.
3. The template documents that it may autonomously transition a ticket to `done`/`accepted` when acceptance-criteria evidence is sufficient, and to `open`/`ready` when the decision is not to implement — without requiring a separate human-confirmation step, consistent with the interview's closure-authority decision.
4. The template explicitly states the Ticket API inverse query is out of scope and points to ticket 1ff57502 for that follow-up.
5. The template is added to the routing/model-tier table alongside the other 15 templates per `.agents/instructions/orchestration/model-routing.instructions.md`'s per-template `model:` declaration contract (declares an explicit `model:` value).