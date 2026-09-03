---
description: "Run a research-then-review-then-roast pass on a scope: substantiate the roast with dispatched Research and Review Agent evidence before Roast Agent critiques it."
name: "substantiated-roast"
argument-hint: "[scope] — file, crate, feature, ticket, or repo area to roast"
agent: "agent"
---

# Substantiated Roast

Run a three-stage evidence-chained pipeline over the requested scope: run the
same research-then-review pass as
[substantiated-review.prompt.md](substantiated-review.prompt.md), then
dispatch [roast.agent.md](../agents/roast.agent.md) fed the prior two stages'
cited evidence, so the roast is grounded in the scope's actual, verified state
rather than a single agent's unsubstantiated opinion. Do not research,
review, or roast the scope yourself — every stage is a dispatched sub-agent.
For a critique without a roast, run
[substantiated-review.prompt.md](substantiated-review.prompt.md) directly
instead of this workflow.

Reference [orchestrator-delegation.instructions.md](../instructions/orchestration/orchestrator-delegation.instructions.md), [shared-context-bundle.instructions.md](../instructions/orchestration/shared-context-bundle.instructions.md), [model-routing.instructions.md](../instructions/orchestration/model-routing.instructions.md), and [write-and-die.instructions.md](../instructions/orchestration/write-and-die.instructions.md) for the dispatch mechanics used below.

## Research + Review Stages

Run these two stages exactly as [substantiated-review.prompt.md](substantiated-review.prompt.md)
defines: resolve the scope, dispatch `.agents/agents/research.agent.md` at
`"GPT-5 mini (copilot)"`, spot-check its highest-impact findings, then
dispatch `.agents/agents/review.agent.md` at `"GPT-5.6 Terra (copilot)"` fed
the verified Research findings. Require the full output that workflow
produces: evidence trail, criteria table, reviewer verdicts, and findings.
Never apply the Review stage's recommended state transition yourself.

## Roast Stage

1. Compile the Research stage's evidence trail and the Review stage's criteria
   table and findings into a new, self-contained prompt.
2. Dispatch `.agents/agents/roast.agent.md` at `"GPT-5.6 Terra (copilot)"`,
   instructing it to roast only findings substantiated by the prior two
   stages' cited evidence, plus any new flaw it independently verifies per
   its own Evidence Contract.
3. Treat the result as an **unverified claim**: spot-check its highest-impact
   citation with a real file read before trusting it, per "Verify Subagent
   Output Before Acting" in model-routing.instructions.md.

## Reconciliation

1. Reconcile any conflicting claims between the Research/Review pass and the
   Roast stage by re-checking the disputed evidence directly; do not average,
   and do not paper over a material contradiction.
2. Record every follow-up finding via `feedback-mcp` or
   `create_ticket`/`add_edge` when an anchor exists.
3. Apply [retry-limit.instructions.md](../instructions/orchestration/retry-limit.instructions.md)
   if a dispatched stage fails its own validation: one self-fix retry inside
   that stage's dispatch, then escalate rather than re-dispatching a third
   time.

## Output Format

Return:
- scope and anchor (ticket/spec, if any)
- pipeline stages run (Research, Review, Roast), each stage's model and
  one-line summary of its return
- evidence trail: every load-bearing finding with its path + line citation,
  carried forward from the stage that produced it
- reconciled findings ordered by severity, each labeled with the stage(s) that
  substantiate it
- final verdict: the Review stage's recommended state transition (not
  applied) and the Roast stage's closing verdict and highest-leverage fix
- every blocker, including any stage that failed its own dispatch or gate
- all ticket/spec/code/log references rendered per the Clickable Reference
  Policy in `AGENTS.md`
