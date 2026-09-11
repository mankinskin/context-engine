# Work Package 2 — W6 Ticket Closure and Readiness

## Outcome

Move the existing W6.x ticket set through its declared dependency order and
standard review lifecycle, collecting ticket-scoped validation evidence before
each ticket reaches `done`.

## Non-Goal

Do not create replacement W6 tickets, alter ticket dependencies, or treat open
ticket records as implementation completion.

## Dependencies

- W6.1 is blocked by [73b2cd22](../../../.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml).
- [30bbbace W6.1](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml) precedes [c61d8b27 W6.2](../../../.workflow-tools/ticket/tickets/c61d8b27-ccea-43ae-91bd-a8627ab6f400/ticket.toml), [17aa8220 W6.3](../../../.workflow-tools/ticket/tickets/17aa8220-48ed-47f1-ae66-7500279417cf/ticket.toml), and [b3626b87 W6.4](../../../.workflow-tools/ticket/tickets/b3626b87-6162-4857-860d-ba4b1c3bc8bd/ticket.toml).
- [50dc45df W6.5](../../../.workflow-tools/ticket/tickets/50dc45df-0c22-496f-af02-adf4de8e124e/ticket.toml) follows the W6.1-W6.4 prerequisites.

## Owned Paths

- `.workflow-tools/ticket/tickets/30bbbace-*/`
- `.workflow-tools/ticket/tickets/c61d8b27-*/`
- `.workflow-tools/ticket/tickets/17aa8220-*/`
- `.workflow-tools/ticket/tickets/b3626b87-*/`
- `.workflow-tools/ticket/tickets/50dc45df-*/`
- Implementation paths declared by each approved ticket.

## Executable Validation

```text
cargo test -p spec-api --test schema_test
cargo test -p spec-api --lib
./target/debug/spec.exe health --all
```

Each ticket also requires its own declared test evidence and the ticket lifecycle
review gate before closure.

## Planning Status

Pending. The W6.x records remain open, and W6.1 is not ready until 73b2cd22 closes.
