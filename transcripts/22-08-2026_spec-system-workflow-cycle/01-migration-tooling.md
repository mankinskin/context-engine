# Work Package 1 — Migration Tooling

## Outcome

Implement the W6.1 migration boundary: an authoritative legacy UUID to immutable
`component_id` mapping artifact and a journaled, idempotent `spec migrate`
runner with dry-run, resume, rollback, and health verification.

## Non-Goal

Do not broaden W6.1 into criteria, provider edges, templates, document
projections, annotations, health hooks, or ticket gating.

## Dependencies

- [30bbbace W6.1](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml) is the owning ticket.
- [73b2cd22 Shared tracing and log-api runtime diagnostics](../../../.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml) must reach the dependency's required terminal state before W6.1 begins.
- [f1b8f01a Component-Oriented Specification System](../../../.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml) remains the governing draft contract.

## Owned Paths

- `workflow-tools/spec/crates/spec-api/`
- The new canonical mapping artifact and migration journal location selected by W6.1.

## Executable Validation

```text
cargo test -p spec-api --test schema_test
cargo test -p spec-api --lib
# Future gate: `spec migrate` does not exist yet; add and run this only after W6.1 implements the mapping and runner.
./target/debug/spec.exe migrate --dry-run --all
./target/debug/spec.exe health --all
```

## Planning Status

Pending. No implementation is authorized by this work package.
