# Problem

`viewer-api` is already generated, but its README targets still use a bespoke structure and its first-level generated child READMEs do not currently provide the repo-internal parent-link chain required by the rollout.

## Scope

Adopt the shared README schema in `viewer-api` and add parent-linked README targets for `viewer-ctl`, `viewer-api`, and `viewer-api/frontend/dioxus`.

## Assumptions To Prove

- The existing `viewer-api` workspace-doc targets can migrate to the shared schema without losing screenshots or tool-use sections.
- The generated child READMEs can include parent links back to `viewer-api/README.md`.
- Frontend and lifecycle command-doc references can remain direct after the schema migration.

## Test-Driven Plan

1. Migrate the `viewer-api` root README target to the shared schema.
2. Add one representative child README parent block and validate the generated output.
3. Extend the pattern across the remaining in-scope child README targets.

## Acceptance Criteria

- `memory-viewers/viewer-api/README.md` uses the shared README schema.
- The in-scope generated child READMEs include parent links to `viewer-api/README.md`.
- `sync-targets --check` passes from the `viewer-api` workspace root.

## Validation

- `./target/debug/rule.exe --workspace-root memory-viewers/viewer-api explain-target --config memory-viewers/viewer-api/rule-targets.yaml --target viewer-api-readme`
- `./target/debug/rule.exe --workspace-root memory-viewers/viewer-api sync-targets --config memory-viewers/viewer-api/rule-targets.yaml`
- `./target/debug/rule.exe --workspace-root memory-viewers/viewer-api sync-targets --check --config memory-viewers/viewer-api/rule-targets.yaml`
