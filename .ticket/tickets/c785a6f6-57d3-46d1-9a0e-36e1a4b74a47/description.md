# Problem

Even once the `context-stack` root README is generated, the internal README tree still fails because its first-level child READMEs do not link back to the parent and are not managed as a coherent repo-local navigation surface.

## Scope

Generate the first-level `context-stack` child README targets needed for repo-internal navigation, covering `context-api`, `context-trace`, `context-search`, `context-insert`, `context-read`, `context-trace-macros`, `ngrams`, and `packages/context-types`.

## Assumptions To Prove

- First-level child READMEs can adopt the shared schema without forcing deeper agent or docs trees into the same ticket.
- Missing repository-level README surfaces for `context-trace-macros`, `ngrams`, and `packages/context-types` can be introduced as part of the same first-level rollout.
- Each child README can include a parent block that links to `context-stack/README.md` only.
- Command-doc coverage can be supplied through local child docs or explicit external command references where no local README exists.

## Test-Driven Plan

1. Start with one representative child README target and parent-link block.
2. Extend the target set across the remaining first-level children.
3. Regenerate the full `context-stack` README tree and verify parent-link coverage.

## Acceptance Criteria

- The in-scope first-level `context-stack` child READMEs are generated from local rules.
- Each generated child README includes a parent block linking to `context-stack/README.md`.
- The resulting tree is internally navigable without assuming any external submodule parent.

## Validation

- `./target/debug/rule.exe --workspace-root context-stack sync-targets --config context-stack/rule-targets.yaml`
- `./target/debug/rule.exe --workspace-root context-stack sync-targets --check --config context-stack/rule-targets.yaml`
