# Work Package 4 — Documentation and Guidance

## Outcome

Describe the implemented v2 specification workflow in the spec-api technical
documentation and agent guidance, including migration-map use, journaled
migration behavior, provider-owned criteria, and the full workflow cycle.

## Non-Goal

Do not document planned behavior as available behavior, or change presentation,
code, ticket, or spec state before the implementation route is complete.

## Dependencies

- Work Packages 1-3 must have passing implementation and dogfood-migration evidence.
- [f1b8f01a Component-Oriented Specification System](../../../.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml) supplies the governing terminology.

## Owned Paths

- `workflow-tools/spec/crates/spec-api/README.md`
- `workflow-tools/spec/crates/spec-api/HIGH_LEVEL_GUIDE.md`
- `AGENTS.md`
- Relevant `.agents/instructions/` guidance paths.

## Executable Validation

```text
cargo test -p spec-api --lib
./target/debug/spec.exe health --all
```

The doc-viewer re-index command must be resolved at implementation pickup rather
than invented during planning.

## Planning Status

Pending. Documentation follows verified behavior.
