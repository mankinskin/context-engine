# Roadmap — Stage 4 Spec-System Workflow-Cycle Route

## Outcome

Plan the remaining route to a verifiable component-oriented specification system without executing it. The v2 schema/model foundation already exists; the remaining route is the explicit migration map, journaled/idempotent migration runner, W6 ticket closure, spec-system dogfood migration, documentation, guidance, and final evidence-backed response.

## Verified Planning Facts

- The v2 schema/model and tests exist in `workflow-tools/spec/crates/spec-api`.
- No authoritative legacy UUID-to-`component_id` mapping artifact or canonical journaled, idempotent migration runner exists.
- W6.x ticket creation is complete in the `.workflow-tools` store; ticket closure and migration tooling are pending.
- [30bbbace W6.1 — Implement v2 model, storage, and explicit migration](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml) is open and blocked by authoritative open dependency [73b2cd22 — Shared tracing and log-api runtime diagnostics](../../../.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml). No dependency mutation is planned.
- [f1b8f01a — Component-Oriented Specification System](../../../.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml) and [fa6e85c8 — Worktree Control Component Pilot](../../../.workflow-tools/spec/specs/fa6e85c8-866c-4d53-bb50-b78bd651e8ce/spec.toml) remain `draft`; approval is planning evidence only.

## Work-Package Route

1. [Work Package 1 — Migration Tooling](01-migration-tooling.md): blocked on `73b2cd22` through W6.1; execution did not start.
2. [Work Package 2 — W6 Ticket Closure and Readiness](02-w6-closure-readiness.md): blocked by the same unsatisfied W6.1 prerequisite; execution did not start.
3. [Work Package 3 — Spec-System Dogfood Migration](03-spec-system-dogfood-migration.md): pending; follows proven tooling and W6 closure.
4. [Work Package 4 — Documentation and Guidance](04-documentation-and-guidance.md): pending; follows verified implementation and migration evidence.
5. [Work Package 5 — Final Validation and Response](05-final-validation.md): pending; produces the evidence-backed response and final decision gate.

## Dependencies And Gate

No work package has been executed by this ingest run. W6.1 cannot begin until `73b2cd22` reaches its required terminal state. The roadmap does not authorize code, ticket, spec, store, or workflow-state changes.

## Active Blockers

- **2026-09-11 — Work Packages 1-2:** ticket [30bbbace W6.1](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml) is open and depends on open ticket [73b2cd22](../../../.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml). The live dependency graph reports unresolved frontier tickets [35cd05c1](../../../.workflow-tools/ticket/tickets/35cd05c1-45f7-4d65-b943-7c000570928f/ticket.toml) and [aa94d02e](../../../.workflow-tools/ticket/tickets/aa94d02e-9620-4db6-9974-36699cd56537/ticket.toml), so W6.1 and the downstream W6 closure package are not dispatchable. Impact: migration tooling, dogfood migration, documentation, and final validation cannot advance. Decision needed: complete the declared dependency chain before resuming this roadmap.

After this planning pass, the requester must choose exactly one outcome:

- `approve`: authorize execution of the exact roadmap, beginning with the next package whose dependencies are satisfied. Approval does not bypass the W6.1 dependency.
- `replan`: return to the planning loop without executing any implementation waypoint.

A planning verdict, ticket existence, or passing validation never substitutes for the explicit `approve` decision.

## Validation Notes

- Existing foundation: `cargo test -p spec-api --test schema_test` and `cargo test -p spec-api --lib`.
- Existing health validation: `./target/debug/spec.exe health --all` and its JSON form, using the correct `.workflow-tools` workspace.
- Future migration validation: `./target/debug/spec.exe migrate --dry-run --all` or equivalent. The command does not exist yet; it becomes runnable only after the mapping and runner are implemented under W6.1.
- Final dogfood validation must read the migrated spec-system spec and rerun health/traceability checks.

## Historical Planning Context

The prior roadmap iterations remain in `ROADMAP.v1.md` through `ROADMAP.v6.md`; `ROADMAP.v7.md` preserves the immediately superseded Stage 4 summary. The current execution route is defined only by this file and the five linked work packages.
