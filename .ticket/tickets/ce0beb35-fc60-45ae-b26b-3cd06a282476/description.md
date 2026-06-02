# Problem

The root workspace already has a `.rule` store, but its `README.md` and the root-owned child READMEs are still manual. That leaves the most visible repository entry points outside the generation pipeline.

## Scope

Add root-owned workspace-doc targets that generate the top-level `README.md` plus the first-level child README surfaces owned by the root repository, including `config`, `doc-viewer`, and `log-viewer`.

## Assumptions To Prove

- The root `.rule` store can own README targets without re-declaring child repo target definitions already imported elsewhere.
- Root-owned child READMEs can use repo-internal parent links back to the root `README.md` only.
- Direct command documentation can be authored for `config`, `doc-viewer`, and `log-viewer` inside root-owned rules.

## Test-Driven Plan

1. Add the root README target using the shared schema and validate it with `explain-target`.
2. Add child README targets for the root-owned docs surfaces.
3. Regenerate and check all root-owned README outputs together.

## Acceptance Criteria

- `README.md` is generated from the root rule store.
- Root-owned child READMEs exist for the in-scope surfaces and include parent links to the root README.
- The generated outputs call out installable content and direct command-doc links where applicable.

## Validation

- `./target/debug/rule.exe explain-target --config rule-targets.yaml --target context-engine-readme`
- `./target/debug/rule.exe sync-targets --config rule-targets.yaml --workspace-root .`
- `./target/debug/rule.exe sync-targets --check --config rule-targets.yaml --workspace-root .`
