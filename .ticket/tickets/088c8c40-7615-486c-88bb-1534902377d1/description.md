# Problem

`memory-api` is already generated, but its README targets still encode structure locally and its first-level tool READMEs do not provide the parent-link blocks required for a navigable repo tree.

## Scope

Adopt the shared README schema in `memory-api` and add parent-linked README targets for the in-scope CLI, MCP, and HTTP tool surfaces.

## Assumptions To Prove

- The existing `memory-api` workspace-doc targets can be migrated to the shared schema without losing local ownership.
- The generated tool READMEs under `tools/cli`, `tools/mcp`, and `tools/http` can include parent links back to `memory-api/README.md`.
- Command-doc coverage can remain direct and explicit after the schema migration.
- Shared schema fragments can be consumed through the workspace target layout without duplicate-registration or unknown-schema failures during `sync-targets --check`.

## Test-Driven Plan

1. Migrate the `memory-api` root README target to the shared schema.
2. Add one representative tool README parent block and validate the generated output.
3. Roll the same pattern across the remaining in-scope tool READMEs.

## Acceptance Criteria

- `memory-viewers/memory-api/README.md` uses the shared README schema.
- The in-scope generated tool READMEs include parent links to `memory-api/README.md`.
- `sync-targets --check` passes from the `memory-api` workspace root.
- The workspace no longer fails due to shared-schema loader behavior.

## Validation

- `./target/debug/rule.exe --workspace-root memory-viewers/memory-api explain-target --config memory-viewers/memory-api/rule-targets.yaml --target memory-api-readme`
- `./target/debug/rule.exe --workspace-root memory-viewers/memory-api sync-targets --config memory-viewers/memory-api/rule-targets.yaml`
- `./target/debug/rule.exe --workspace-root memory-viewers/memory-api sync-targets --check --config memory-viewers/memory-api/rule-targets.yaml`
