---
description: "Run a research-then-review pass on a scope: dispatch Research Agent, then feed its cited findings into Review Agent so the verdict is grounded in the scope's actual, verified state rather than a single agent's unsubstantiated read."
name: "substantiated-review"
argument-hint: "[scope] — ticket, spec, file, crate, or repo area (defaults to the highest-ranked in-review ticket)"
agent: "agent"
---

# Substantiated Review

Run a two-stage evidence-chained pipeline over the requested scope: dispatch
[research.agent.md](../agents/research.agent.md), then feed its cited findings
into [review.agent.md](../agents/review.agent.md) as prior context, so the
recorded verdict is grounded in the scope's actual, verified state instead of
a single agent's unsubstantiated read. Do not research or review the scope
yourself — both stages are dispatched sub-agents.

Reference [orchestrator-delegation.instructions.md](../instructions/orchestration/orchestrator-delegation.instructions.md), [shared-context-bundle.instructions.md](../instructions/orchestration/shared-context-bundle.instructions.md), [model-routing.instructions.md](../instructions/orchestration/model-routing.instructions.md), and [write-and-die.instructions.md](../instructions/orchestration/write-and-die.instructions.md) for the dispatch mechanics used below.

## Scope Resolution

1. Treat the slash-command text as the scope (ticket, spec, file, crate, or
   repo area) and an optional ticket/spec anchor.
2. When no scope is given, discover the in-review queue exactly as
   [review.agent.md](../agents/review.agent.md) does: confirm the `in-review`
   set with `mcp_ticket-mcp_list_tickets` (`{"workspace":"default","state":"in-review"}`)
   or `ticket list --state in-review --toon`, then rank with
   `mcp_ticket-mcp_next_tickets` or `ticket next --toon`.
3. If the scope or first hop is unverified, run the Research/Explore
   pre-dispatch gate from [pre-dispatch-gates.instructions.md](../instructions/orchestration/pre-dispatch-gates.instructions.md)
   before dispatching.

## Research Stage

1. Dispatch `.agents/agents/research.agent.md` at `"GPT-5 mini (copilot)"`
   with a compiled, self-contained prompt naming the scope and anchor — never
   assume the sub-agent inherits this session's context.
2. Require its findings in the template's own `fact | inference |
   stale-or-pending | evidence` shape.
3. Treat the result as an **unverified claim**: spot-check 1-2 of its
   highest-impact findings against ground truth (a real file read or search)
   before trusting them, per "Verify Subagent Output Before Acting" in
   model-routing.instructions.md.

## Review Stage

1. Compile the verified Research findings into the "prior context" section of
   a new compiled prompt.
2. Dispatch `.agents/agents/review.agent.md` at `"GPT-5.6 Terra (copilot)"` to
   walk the scope criterion-by-criterion (or feature-by-feature, for a
   non-ticket scope) using that evidence instead of re-deriving it from
   scratch.
3. Require its criteria table, reviewer verdicts, findings, and recommended
   state transition. **Never apply the transition yourself** — review.agent.md
   never transitions ticket or spec state, and neither does this workflow.

## Reconciliation

1. If the Review stage's questions surface evidence that contradicts a
   Research finding, re-check the disputed evidence directly rather than
   averaging the two claims; do not paper over the contradiction.
2. Record every follow-up finding via `feedback-mcp` or
   `create_ticket`/`add_edge` when an anchor exists. The Review stage's own
   follow-up tickets remain its responsibility.
3. Apply [retry-limit.instructions.md](../instructions/orchestration/retry-limit.instructions.md)
   if a dispatched stage fails its own validation: one self-fix retry inside
   that stage's dispatch, then escalate rather than re-dispatching a third
   time.

## Output Format

Return:
- scope and anchor (ticket/spec under review)
- pipeline stages run, each stage's dispatched agent, model, and one-line
  summary of its return
- evidence trail: every load-bearing finding with its path + line citation,
  carried forward from the Research stage into the Review stage's criteria
  table
- criteria table: each acceptance criterion, the Research evidence backing
  it, the reviewer's verdict, and any finding raised
- **verdict:** the Review stage's recommended state (`pass`/`fail` or
  `reviewed`/`changes-requested`) — explicitly noting that no transition was
  applied
- resume pointer, if the Review stage left criteria pending
- every blocker, including any stage that failed its own dispatch or gate
- all ticket/spec/code/log references rendered per the Clickable Reference
  Policy in `AGENTS.md`
