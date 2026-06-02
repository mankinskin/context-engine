# Problem

`context-stack` still lacks a repo-local `.rule` store and local README targets, so its root README cannot participate in the same local-generation workflow as the memory-viewers workspaces.

## Scope

Create a repo-local `context-stack/.rule` store, add a local `rule-targets.yaml` shim plus themed workspace-doc fragments, and generate the `context-stack/README.md` root surface from local rules.

## Assumptions To Prove

- `context-stack` can host a repo-local `.rule` store alongside its existing nested tooling without breaking workspace resolution.
- The local target config can be generated both from inside `context-stack` and from the ancestor checkout using `--workspace-root context-stack`.
- The generated root README can use the shared schema while explicitly stating that the repo root has no installable binary surface.

## Test-Driven Plan

1. Add the local rule workspace and target shim.
2. Create a failing or empty target explanation for `context-stack-readme`.
3. Add the minimum local rules required to make the root README target render cleanly.

## Acceptance Criteria

- `context-stack` contains a repo-local `.rule` store and README target config.
- `context-stack/README.md` is generated from local rules.
- The target can be explained and synchronized from both local and ancestor workspaces.

## Validation

- `./target/debug/rule.exe --workspace-root context-stack explain-target --config context-stack/rule-targets.yaml --target context-stack-readme`
- `./target/debug/rule.exe --workspace-root context-stack sync-targets --config context-stack/rule-targets.yaml`
- `./target/debug/rule.exe --workspace-root context-stack sync-targets --check --config context-stack/rule-targets.yaml`
