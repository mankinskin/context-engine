# Work Package 9 — Prerequisite Wave 4

## Outcome

Complete [30bbbace W6.1](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml) after `73b2cd22` reaches its required terminal state, producing the migration tooling required by Work Packages 1-5.

## Non-Goal

Do not start downstream W6.x closure, dogfood migration, documentation, or the
final response until W6.1 has passed its own validation and lifecycle gate.

## Dependencies

- Work Package 8 must have passing completion evidence for `73b2cd22`.
- [f1b8f01a Component-Oriented Specification System](../../../.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml) remains the governing draft contract.

## Owned Paths

- `.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/`
- `workflow-tools/spec/crates/spec-api/`
- The mapping and migration-journal paths selected by W6.1.

## Validation

The following W6.1 validation commands are verified by Work Package 1:

```text
cargo test -p spec-api --test schema_test
cargo test -p spec-api --lib
./target/debug/spec.exe health --all
```

Before relying on `./target/debug/spec.exe migrate --dry-run --all`, confirm
that W6.1 has implemented that command and record the command result. Satisfy
the ticket lifecycle review gate before marking W6.1 done.

## Planning Status

Pending explicit roadmap approval and Work Package 8 completion evidence.
