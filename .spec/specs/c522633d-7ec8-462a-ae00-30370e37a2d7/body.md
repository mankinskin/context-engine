<!-- aligned-structure:v2 -->

# Tickets

## Responsibility And Interface

Turn a reviewed governing spec into executable slices and dependency edges in
`.ticket/`. Consume Specification's three criteria; use `ticket.exe search`,
`create`, `get --view plan`, and graph operations from
`.agents/instructions/ticket/workflow.instructions.md`; provide two criteria to Tests.

## Behavior And Contract

- `tickets-spec-reference`: each implementation ticket references its governing spec.
- `tickets-executable-slices`: each ticket bounds scope, acceptance work, and dependencies.
- Planning happens after the spec is ready; tickets do not author its contract.

## Boundaries And Failure Cases

Do not create a ticket below the repository's ticket threshold or recreate an
intentionally absent one. A ticket cannot replace a missing spec. Search for
duplicates first; incomplete requirements/dependencies return to spec or
interview work instead of receiving an invented plan.

## Acceptance Evidence And Position

`ticket.exe get <id> --view plan --json` shows reference/scope/dependencies and
`ticket.exe health --all --toon` checks the graph. This change has no related
ticket and no `validated_by`. `.ticket/` and `ticket.exe` are implemented.
