<!-- aligned-structure:v2 -->

# Production Workflow: Next Iteration

## Target Code Location

[.agents/instructions/orchestration/loop-closure.instructions.md](.agents/instructions/orchestration/loop-closure.instructions.md) owns the closure cycle; [AGENTS.md](AGENTS.md) owns feedback-api and `.feedback` recording rules.

## Naming Conventions

Use feedback targets `ce://default/spec/<id>` or `ce://default/ticket/<id>`;
handoffs live under `.session/sessions/<session-id>/handoffs/`. This component
owns `iteration-recorded-judgment` and `iteration-follow-up-transition`.

## Requester Input

> Record the user's judgment of that outcome (satisfied, or follow-up needed); that judgment either closes this roadmap or seeds the next pass through the cycle.

## Reading Order

1. [.agents/instructions/orchestration/loop-closure.instructions.md](.agents/instructions/orchestration/loop-closure.instructions.md) — handoff and closure owner.
2. [0013fe78 Production Workflow: Validated Response](.spec/specs/0013fe78-9279-4bb2-8707-e86b6a3dd3b8/body.md) — response and judgment provider.
3. [e8104080 Production Workflow: Request](.spec/specs/e8104080-df78-46cb-ac64-3bfeb51e583b/body.md) — follow-up consumer.

## Responsibility

If implemented, Request can rely on a durable follow-up outcome and open
questions, while a satisfied judgment closes the cycle without hidden work.

## Interfaces And Dependencies

Consumes the `response-*` criteria. feedback-api stores the received judgment in
`.feedback`; `session.exe handoff` creates the forward package when continuation is ready.

## Behavior

- `iteration-recorded-judgment` persists satisfaction or requested follow-up.
- `iteration-follow-up-transition` closes on satisfaction, otherwise seeds Request.
- A continuation handoff preserves objective, target paths, decisions, non-goals,
  anchors, and empty `open_escalations`.

## Boundaries And Failure Cases

Do not close with unresolved escalation, unmet criteria, missing evidence, or a
missing next handoff. A follow-up is not silently added to old implementation scope;
ambiguous intent returns to discovery. Automatic extraction of chat satisfaction into
feedback-api is specified-but-not-built.

## Provider/Consumer Contract

Consumes all `response-*` criteria from [0013fe78 Production Workflow: Validated Response](.spec/specs/0013fe78-9279-4bb2-8707-e86b6a3dd3b8/body.md); provides `iteration-recorded-judgment` and `iteration-follow-up-transition` to [e8104080 Production Workflow: Request](.spec/specs/e8104080-df78-46cb-ac64-3bfeb51e583b/body.md).

## Examples

`feedback ingest --target ce://default/spec/<id> --note "follow-up: add export filters"` records a new request seed; `./target/debug/session.exe handoff --session-id <uuid> ...` creates its continuation package after review.

## Evidence

Position: `partial`; loop closure and handoff creation are implemented. Verify the
feedback record and, for continuation, inspect the produced handoff package.

## Scope

Owns judgment persistence and re-entry, not response composition or implementation.
