<!-- aligned-structure:v2 -->

# Production Workflow Cycle

## Motivation

This root defines the traceable path from a user request to that user's next
judgment. It keeps planning, validation, execution, and follow-up connected
without making a ticket or an implementation agent redefine the requirement.

## Dependent Expectation

If this specification is implemented, dependents can rely on every production
unit progressing through Request -> Spec -> Tickets -> Tests -> Implementation
-> Validated response -> Next iteration, with a durable artifact at each handoff.

## Shared Invariants

- Each child owns its outward-facing acceptance criteria; a consumer names the
  upstream criteria it requires without copying them.
- A draft cycle contract is documentation, not a claim that an implementation or
  executable guard exists. These specs remain draft pending user review.
- The implementation phase receives a complete handoff or escalates; it does
  not discover missing requirements while changing code.
- The cycle closes only when the user's judgment is recorded as satisfied or as
  a new request.

## Component Relationship Map

| Consumer | Provider | Provider criteria referenced by the consumer |
| --- | --- | --- |
| Specification | Request | `request-outcome`, `request-open-questions` |
| Tickets | Specification | `spec-goal`, `spec-owned-criteria`, `spec-traceability` |
| Tests | Tickets | `tickets-spec-reference`, `tickets-executable-slices` |
| Implementation | Tests | `tests-measurable-criteria`, `tests-recorded-evidence`, `tests-documented-manual-criteria` |
| Validated response | Implementation | `implementation-planned-scope`, `implementation-review-evidence`, `implementation-escalation` |
| Next iteration | Validated response | `response-evidence`, `response-user-judgment`, `response-review-gate` |
| Request | Next iteration | `iteration-recorded-judgment`, `iteration-follow-up-transition` |

## Positions And Evidence

- `implemented`: `.agents/instructions/orchestration/core-cycle.instructions.md`
  defines the documented seven-stage workflow and handoff artifacts.
- `implemented`: `.agents/instructions/orchestration/phase-separation.instructions.md`
  and `escalation-gate.instructions.md` define the implementation boundary.
- `partial`: store-backed validation evidence is available through `.test/` and
  `./target/debug/test.exe`, but this draft has no recorded guard execution.
- Governing rule: `.agents/instructions/spec/spec-system.instructions.md` must
  introduce this hierarchy before an agent relies on it.

## Scope

This root owns only the shared cycle and directed relationship map. The seven
child specs own stage behavior, acceptance evidence, and boundaries. It neither
