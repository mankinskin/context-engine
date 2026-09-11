# Spec-System Workflow-Cycle Dossier

## Reading Order

1. [ROADMAP.md](ROADMAP.md) defines the current Stage 4 route and its final approval gate.
2. [06-prerequisite-wave-1.md](06-prerequisite-wave-1.md) defines the parallel actionable frontier: `35cd05c1` and `aa94d02e`.
3. [07-prerequisite-wave-2.md](07-prerequisite-wave-2.md), [08-prerequisite-wave-3.md](08-prerequisite-wave-3.md), and [09-prerequisite-wave-4.md](09-prerequisite-wave-4.md) define the ordered completion of `bce26d30`, `73b2cd22`, and W6.1 `30bbbace`.
4. [01-migration-tooling.md](01-migration-tooling.md) defines W6.1's implementation boundary; [02-w6-closure-readiness.md](02-w6-closure-readiness.md) through [05-final-validation.md](05-final-validation.md) define the remaining migration route.
5. [ARTIFACTS.md](ARTIFACTS.md), [REVIEW.v2.md](REVIEW.v2.md), [REVIEW.md](REVIEW.md), [INTERVIEW-1.md](INTERVIEW-1.md), and [INTERVIEW-2.md](INTERVIEW-2.md) provide the source evidence and decisions.
6. [ROADMAP.v1.md](ROADMAP.v1.md) through [ROADMAP.v8.md](ROADMAP.v8.md) and [README.v1.md](README.v1.md) retain superseded planning history.

## Scope

Stage 4 converts the completed planning work into bounded, dependency-aware
work packages. The route first closes every open prerequisite of W6.1 in four
explicit waves, then covers migration tooling, closure of the existing W6.x
ticket set, migration of the spec system's own specification, verified
technical documentation and guidance, and an evidence-backed final response.

## Decisions

- [30bbbace W6.1](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml) remains open behind [73b2cd22](../../../.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml). The complete declared route is `35cd05c1` and `aa94d02e` in parallel, then `bce26d30`, then `73b2cd22`, then W6.1; no substitute dependency or edge mutation is planned.
- [f1b8f01a Component-Oriented Specification System](../../../.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml) and [fa6e85c8 Worktree Control Component Pilot](../../../.workflow-tools/spec/specs/fa6e85c8-866c-4d53-bb50-b78bd651e8ce/spec.toml) remain `draft`. Their approval is planning-level evidence only.
- The first migration target is the specification system itself, not the Presentation System.
- Provider components own their criteria; consumer components reference provider criteria rather than restating them.

## No-Execution Boundary

This dossier authorizes planning artifacts only. It does not modify code,
tickets, specs, or any store/workflow state. The implementation route begins
only after the requester explicitly selects `approve`; selecting `replan`
returns the route to planning. Passing validation alone never implies approval.
