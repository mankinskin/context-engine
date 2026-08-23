the owner for refinement rather than becoming a fabricated pass.
<!-- aligned-structure:v2 -->

# Production Workflow: Tests

## Target Code Location

[memory-api/](memory-api/) owns test-api/test-mcp evidence surfaces; [AGENTS.md](AGENTS.md) owns validation and test-log requirements.

## Naming Conventions

Use `.test/<workspace>/specs/` and `executions/`; execution records include
`ticket_ids`, `spec_ids`, criterion ids, outcome, command, and detail. This
component owns `tests-measurable-criteria`, `tests-recorded-evidence`, and `tests-documented-manual-criteria`.

## Requester Input

> Fold in the test-evidence cross-link (test-api already links `spec_ids`/`ticket_ids`/`acceptance_criterion_ids`).

## Reading Order

1. [AGENTS.md](AGENTS.md) — validation and log-reading owner.
2. [c522633d Production Workflow: Tickets](.spec/specs/c522633d-7ec8-462a-ae00-30370e37a2d7/body.md) — ticket-slice provider.
3. [44b32c98 Production Workflow: Implementation](.spec/specs/44b32c98-a4cf-4d83-b114-6e5db65b6212/body.md) — evidence consumer.

## Responsibility

If implemented, Implementation can rely on measurable criterion checks and
queryable execution records instead of a claimed or copied test result.

## Interfaces And Dependencies

Consumes ticket spec references and executable slices. test-api/test-mcp writes
test definitions and executions; `ticket_ids` is the durable link from an
execution to ticket work, but no current ticket lifecycle gate consumes it.

## Behavior

- `tests-measurable-criteria` maps feasible criteria to a command or manual check.
- `tests-recorded-evidence` records identifiers, outcome, command, and detail.
- `tests-documented-manual-criteria` records method and limitation when automation is unavailable.

## Boundaries And Failure Cases

Tests do not redefine a criterion or bury an outcome in ticket prose. Failed or
blocked commands retain their reason; missing criteria return to the owner. Ticket
completion reconciliation from `ticket_ids` is specified-but-not-built.

## Provider/Consumer Contract

Consumes `tickets-spec-reference` and `tickets-executable-slices` from [c522633d Production Workflow: Tickets](.spec/specs/c522633d-7ec8-462a-ae00-30370e37a2d7/body.md); provides all three `tests-*` criteria to [44b32c98 Production Workflow: Implementation](.spec/specs/44b32c98-a4cf-4d83-b114-6e5db65b6212/body.md).

## Examples

`./target/debug/test.exe --store-root "$PWD/.test" list --ticket <id>` must return the execution whose `ticket_ids` includes `<id>`; a manual record includes the criterion, method, and limitation.

## Evidence

Position: `partial`; the evidence store contract exists. Record execution evidence
through test-api/test-mcp before review; no run is recorded for this draft.

## Scope

Owns validation evidence, not ticket-state transitions or implementation changes.
