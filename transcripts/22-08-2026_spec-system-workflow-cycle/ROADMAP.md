# Roadmap - Stage 4 Spec-System Workflow-Cycle Route

## Outcome Summary

This route reaches a verifiable component-oriented specification system by first completing every unresolved prerequisite that blocks W6.1. The route then executes W6.1 migration tooling, the remaining W6.x closure, dogfood migration, documentation and guidance, and an evidence-backed response. The roadmap remains planning-only until the requester explicitly approves it.

## Relevant Artifact IDs

- Ticket store: `.workflow-tools/ticket/tickets/`.
- Spec store: `.workflow-tools/spec/specs/`.
- `30bbbace` W6.1: `.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml`.
- `73b2cd22`: `.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml`.
- `35cd05c1`: `.workflow-tools/ticket/tickets/35cd05c1-45f7-4d65-b943-7c000570928f/ticket.toml`.
- `aa94d02e`: `.workflow-tools/ticket/tickets/aa94d02e-9620-4db6-9974-36699cd56537/ticket.toml`.
- `bce26d30`: `.workflow-tools/ticket/tickets/bce26d30-0a79-40b4-812a-c14b4a246de5/ticket.toml`.
- `f1b8f01a`: `.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml`.
- W6 downstream tickets: `c61d8b27`, `17aa8220`, `b3626b87`, and `50dc45df` under `.workflow-tools/ticket/tickets/`.
- Implementation surface: `workflow-tools/spec/crates/spec-api/`.
- Work packages: [06](06-prerequisite-wave-1.md), [07](07-prerequisite-wave-2.md), [08](08-prerequisite-wave-3.md), [09](09-prerequisite-wave-4.md), and [01](01-migration-tooling.md) through [05](05-final-validation.md).

## Active Blockers

- **2026-09-11 — W1 and W2:** tickets `35cd05c1` and `aa94d02e` are dependency-actionable, but neither ticket has a governing spec link. The implementation readiness gate requires spec coverage before a ticket-backed code change can start. Impact: the two wave-1 tickets cannot enter implementation; every later waypoint remains blocked. Required resolution: replan the route to author or identify governing specs for the memory-kernel journal-envelope and log-api live-indexing scopes, then link those specs to the tickets before execution resumes.

## Validation Gates

- `cargo test -p spec-api --test schema_test`
- `cargo test -p spec-api --lib`
- `./target/debug/spec.exe health --all`
- `./target/debug/spec.exe get f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json`

`./target/debug/spec.exe migrate --dry-run --all` is a future W6.1 gate, not an available command. Each of `35cd05c1`, `aa94d02e`, `bce26d30`, and `73b2cd22` requires a pre-execution validation-discovery gate: read the canonical ticket manifest and its validation parts, record the declared command(s), then run the ticket lifecycle review gate before closure.

## Roadmap Waypoints

### W1. Complete prerequisite ticket 35cd05c1

Status: blocked
Scope: ticket 35cd05c1
Session package: prerequisite-wave-1-35cd05c1
Prompt: Complete ticket `35cd05c1` from `.workflow-tools/ticket/tickets/35cd05c1-45f7-4d65-b943-7c000570928f/ticket.toml`; first discover and run only ticket-declared validation, collect review evidence, and close the ticket. The only prerequisite, `6c859ac3`, is satisfied. Do not start `bce26d30` or alter dependencies.
Validate: spec-coverage and validation-discovery gate from the canonical `35cd05c1` ticket manifest and validation parts
Commit checkpoint: after passing ticket-declared validation and lifecycle review; use the approved ticket's conventional commit scope.

### W2. Complete prerequisite ticket aa94d02e

Status: blocked
Scope: ticket aa94d02e
Session package: prerequisite-wave-1-aa94d02e
Prompt: Complete ticket `aa94d02e` from `.workflow-tools/ticket/tickets/aa94d02e-9620-4db6-9974-36699cd56537/ticket.toml`; first discover and run only ticket-declared validation, collect review evidence, and close the ticket. All direct prerequisites are satisfied. Do not start `bce26d30` or alter dependencies.
Validate: spec-coverage and validation-discovery gate from the canonical `aa94d02e` ticket manifest and validation parts
Commit checkpoint: after passing ticket-declared validation and lifecycle review; use the approved ticket's conventional commit scope.

W1 and W2 form wave 1 and may run in parallel. See [Work Package 6](06-prerequisite-wave-1.md).

### W3. Complete prerequisite ticket bce26d30

Status: pending
Scope: ticket bce26d30
Depends: W1, W2
Session package: prerequisite-wave-2-bce26d30
Prompt: Complete ticket `bce26d30` from `.workflow-tools/ticket/tickets/bce26d30-0a79-40b4-812a-c14b4a246de5/ticket.toml`; first discover and run only ticket-declared validation, collect review evidence, and close the ticket. `35cd05c1` and `aa94d02e` are the only remaining open prerequisites; `2e41c96d`, `3041d7e3`, `cc78d33d`, and `ff6637f5` are satisfied. Do not start `73b2cd22` or alter dependencies.
Validate: pre-execution validation-discovery gate from the canonical `bce26d30` ticket manifest and validation parts
Commit checkpoint: after passing ticket-declared validation and lifecycle review; use the approved ticket's conventional commit scope.

See [Work Package 7](07-prerequisite-wave-2.md).

### W4. Complete prerequisite ticket 73b2cd22

Status: pending
Scope: ticket 73b2cd22
Depends: W3
Session package: prerequisite-wave-3-73b2cd22
Prompt: Complete ticket `73b2cd22` from `.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml`; first discover and run only ticket-declared validation, collect review evidence, and close the aggregator ticket. The 19 direct prerequisites are represented in the prerequisite appendix. Do not begin W6.1 or change dependencies.
Validate: pre-execution validation-discovery gate from the canonical `73b2cd22` ticket manifest and validation parts
Commit checkpoint: after passing ticket-declared validation and lifecycle review; use the approved ticket's conventional commit scope.

See [Work Package 8](08-prerequisite-wave-3.md).

### W5. Complete W6.1 migration tooling

Status: pending
Scope: ticket 30bbbace
Depends: W4
Session package: prerequisite-wave-4-w6-1
Prompt: Complete W6.1 ticket `30bbbace` using [Work Package 9](09-prerequisite-wave-4.md) and [Work Package 1](01-migration-tooling.md). Implement only the authoritative legacy UUID-to-immutable-`component_id` mapping and journaled, idempotent migration runner in `workflow-tools/spec/crates/spec-api/`; validate the existing spec-api tests and health command, then run migration dry-run only after the command exists. Do not begin downstream W6.x closure or dogfood migration.
Validate: cargo test -p spec-api --test schema_test
Validate: cargo test -p spec-api --lib
Validate: ./target/debug/spec.exe health --all
Commit checkpoint: after W6.1 validation and lifecycle review; use the approved ticket's conventional commit scope.

### W6. Close downstream W6.x tickets

Status: pending
Scope: single-session
Depends: W5
Session package: w6-closure-readiness
Prompt: Execute [Work Package 2](02-w6-closure-readiness.md) in the declared dependency order for `c61d8b27`, `17aa8220`, `b3626b87`, and `50dc45df`; discover and run each ticket's own validation, apply review gates, and retain W6.1 evidence. Do not create replacement tickets or modify declared edges.
Validate: cargo test -p spec-api --test schema_test
Validate: cargo test -p spec-api --lib
Validate: ./target/debug/spec.exe health --all
Commit checkpoint: after each independently reviewable ticket closure; use each approved ticket's conventional commit scope.

### W7. Dogfood the spec-system migration

Status: pending
Scope: single-session
Depends: W6
Session package: spec-system-dogfood-migration
Prompt: Execute [Work Package 3](03-spec-system-dogfood-migration.md) against `.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/`, retaining the legacy source until health and traceability checks pass. Do not use the Presentation System first or change a spec lifecycle state outside approved work.
Validate: ./target/debug/spec.exe migrate --dry-run --all
Validate: ./target/debug/spec.exe get f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json
Validate: ./target/debug/spec.exe health --all
Commit checkpoint: after passing migration and review evidence; use the approved migration commit scope.

### W8. Publish documentation and guidance

Status: pending
Scope: single-session
Depends: W7
Session package: documentation-and-guidance
Prompt: Execute [Work Package 4](04-documentation-and-guidance.md) for verified v2 behavior only, using `workflow-tools/spec/crates/spec-api/`, `AGENTS.md`, and relevant `.agents/instructions/` paths. Resolve the doc-viewer index command before relying on it. Do not document planned behavior as available.
Validate: cargo test -p spec-api --lib
Validate: ./target/debug/spec.exe health --all
Commit checkpoint: after documentation review and validation; use the approved documentation commit scope.

### W9. Produce final evidence and request a decision

Status: pending
Scope: single-session
Depends: W8
Session package: final-validation-response
Prompt: Execute [Work Package 5](05-final-validation.md): collect the completed W6, migration, dogfood, and documentation evidence; run final validation; provide the evidence-backed response; then ask the requester for exactly `approve` or `replan`. Do not infer approval from any command result.
Validate: cargo test -p spec-api --test schema_test
Validate: cargo test -p spec-api --lib
Validate: ./target/debug/spec.exe health --all
Validate: ./target/debug/spec.exe get f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json
Commit checkpoint: none (the package produces the final evidence and explicit decision gate).

Final explicit gate: `approve` authorizes the next dependency-satisfied execution waypoint; `replan` returns to Stage 4 planning. Neither planning evidence nor passing validation substitutes for the requester decision.

## Prerequisite Appendix

`30bbbace` depends on `73b2cd22`. `73b2cd22` has 19 direct prerequisites; 16 are satisfied evidence and three remain open. The satisfied entries are not execution waypoints.

| Ticket | State | Disposition |
|---|---|---|
| `15ce7eab`, `1dffcf23`, `2e41c96d`, `3041d7e3` | done | Satisfied evidence |
| `363e26d6`, `6c859ac3`, `756fed27`, `84673399` | done | Satisfied evidence |
| `8ad77570`, `cc78d33d`, `d3349747`, `d760a9bb` | done | Satisfied evidence |
| `dda04f91`, `e813f958`, `efdcbb83`, `ff6637f5` | done | Satisfied evidence |
| `35cd05c1` | open | W1; depends only on done `6c859ac3` |
| `aa94d02e` | open | W2; all direct prerequisites are done |
| `bce26d30` | open | W3; waits for W1 and W2; four other dependencies are done |

Completion waves are: wave 1, W1 and W2 in parallel; wave 2, W3; wave 3, W4; wave 4, W5. Only then may W6 through W9 proceed.

## Heads-up Notes

- `f1b8f01a` and `fa6e85c8` remain draft; their planning status is not a lifecycle transition.
- `spec migrate` is not currently available and must never be claimed as a completed check before W6.1 implements it.
- The ticket store's `depends_on` edges are authoritative. The route does not authorize dependency mutation.
- [ROADMAP.v8.md](ROADMAP.v8.md) preserves the superseded active roadmap; [ROADMAP.v1.md](ROADMAP.v1.md) through [ROADMAP.v7.md](ROADMAP.v7.md) retain earlier planning history.
