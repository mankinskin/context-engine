# Problem

`memory-viewers` is the aggregate repo root for the generated family, but its README target still has a bespoke structure and its child-block behavior needs to be normalized after the child repos adopt the shared schema.

## Scope

Adopt the shared README schema in `memory-viewers` and normalize the repo-root child blocks so the aggregate README cleanly composes the `memory-api` and `viewer-api` surfaces.

## Assumptions To Prove

- The aggregate `memory-viewers` README can adopt the shared schema without flattening child ownership boundaries.
- Child blocks can remain direct links to child repo roots rather than hard-coded prose summaries.
- The aggregate README can keep its screenshots or dependency graph sections as optional schema extensions.

## Test-Driven Plan

1. Migrate the `memory-viewers` root README target to the shared schema.
2. Normalize the child block definitions after the child repos expose their final root shapes.
3. Validate the aggregate target with `explain-target` and `sync-targets --check`.

## Acceptance Criteria

- `memory-viewers/README.md` uses the shared README schema.
- The child blocks reflect the final `memory-api` and `viewer-api` root surfaces cleanly.
- `sync-targets --check` passes from the `memory-viewers` workspace root.

## Validation

- `./target/debug/rule.exe --workspace-root memory-viewers explain-target --config memory-viewers/rule-targets.yaml --target memory-viewers-readme`
- `./target/debug/rule.exe --workspace-root memory-viewers sync-targets --config memory-viewers/rule-targets.yaml`
- `./target/debug/rule.exe --workspace-root memory-viewers sync-targets --check --config memory-viewers/rule-targets.yaml`
