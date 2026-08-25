## Objective

Implement [Meta-workspace workflow-tools consumer topology](../../../.spec/specs/72b641b1-6620-4043-b956-102d826ce8ea/body.md): make `meta-workspace` the superproject with one top-level `workflow-tools` source checkout and independent top-level consumers, beginning with `minimal-demo`.

## Requirements

- Register `minimal-demo` as a top-level consumer submodule beside `workflow-tools` and `context-engine`.
- Remove the nested `context-engine/workflow-tools` submodule only through [context-engine consumer cutover](../92741a14-d718-4f49-8843-040432a3d8da/ticket.toml); do not delete it as an untracked filesystem change.
- Define one explicit consumer-root selector for installed CLI, MCP, hook, and session operations.
- Reject an omitted, nonexistent, or ambiguous selector from the meta-workspace root before any consumer store read or write.
- Keep all consumer stores isolated; a `minimal-demo` command cannot access `context-engine` stores and vice versa.

## Acceptance Criteria

- Meta-workspace has one top-level `workflow-tools` submodule and a top-level `minimal-demo` consumer submodule.
- The tutorial installs and runs tools against `minimal-demo` without using the `workflow-tools` source checkout as its consumer root.
- An operation launched at meta-workspace root with explicit `minimal-demo` targets only `minimal-demo` artifacts, confirmed by read-back.
- The same unqualified operation fails with no artifact mutation.
- context-engine no longer declares `workflow-tools` in `.gitmodules` once ticket `92741a14` completes.

## Validation

- Meta-workspace integration test with both consumers initialized.
- Selected-store read-back and unqualified-command negative test.
- `bash workflow-tools/fixtures/minimal-demo/run-tutorial.sh`.
- `cargo metadata --format-version 1 --no-deps` from context-engine after cutover.