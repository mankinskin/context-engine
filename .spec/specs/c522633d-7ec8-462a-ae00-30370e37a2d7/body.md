duplicates first; incomplete requirements/dependencies return to spec or
ticket and no `validated_by`. `.ticket/` and `ticket.exe` are implemented.
<!-- aligned-structure:v2 -->

# Production Workflow: Tickets

## Target Code Location

[.agents/instructions/ticket/workflow.instructions.md](.agents/instructions/ticket/workflow.instructions.md) owns ticket workflow; [workflow-tools/ticket/](workflow-tools/ticket/) owns ticket storage and CLI behavior.

## Naming Conventions

Use `ticket.toml`, a governing `spec` reference, and dependency edges. This
component owns `tickets-spec-reference` and `tickets-executable-slices`.

## Requester Input

> The implementation ticket is created second, referencing the spec, and plans how to reach that goal.

## Reading Order

1. [.agents/instructions/ticket/workflow.instructions.md](.agents/instructions/ticket/workflow.instructions.md) — ticket authoring and graph owner.
2. [d6b4d989 Production Workflow: Specification](.spec/specs/d6b4d989-f9ac-428a-9dbc-68400006fc96/body.md) — governing criteria provider.
3. [18c1b04d Production Workflow: Tests](.spec/specs/18c1b04d-d23f-4047-9793-5c2af0ee04c1/body.md) — executable-slice consumer.

## Responsibility

If implemented, Tests can rely on a ticket that links its governing reviewed
specification and bounds the executable scope, acceptance work, and dependencies.

## Interfaces And Dependencies

Consumes `spec-goal`, `spec-owned-criteria`, and `spec-traceability`; stores
eligible work in `.ticket/` via `ticket.exe create`, `get --view plan`, and graph operations.

## Behavior

- `tickets-spec-reference` records the governing spec for each implementation ticket.
- `tickets-executable-slices` names scope, validation work, and dependencies.
- Ticket creation happens only after an adequate reviewed spec and only when
	the task crosses the repository ticket threshold.

## Boundaries And Failure Cases

Do not create tickets for small self-contained work, recreate intentionally absent
tickets, or use a ticket to author a missing specification. Duplicate search,
unknown dependencies, and incomplete requirements return to Specification or Request.

## Provider/Consumer Contract

Consumes the three `spec-*` criteria from [d6b4d989 Production Workflow: Specification](.spec/specs/d6b4d989-f9ac-428a-9dbc-68400006fc96/body.md); provides `tickets-spec-reference` and `tickets-executable-slices` to [18c1b04d Production Workflow: Tests](.spec/specs/18c1b04d-d23f-4047-9793-5c2af0ee04c1/body.md).

## Examples

`./target/debug/ticket.exe get <id> --view plan --json` exposes a ticket's spec reference, scope, and dependencies; a one-file fix is recorded as direct work and has no ticket.

## Evidence

Run `./target/debug/ticket.exe health --all --toon` after creating a ticket.
Position: `implemented` ticket store and guidance; this draft has no ticket.

## Scope

Owns thresholded implementation planning, not requirements or test execution.
