# Problem

Repository README generation is split between manual repo roots and generated nested workspaces. Shared structure is duplicated, parent/child navigation is inconsistent, and there is no single tracker tree that coordinates schema work, rollout order, and final verification.

## Scope

Coordinate the schema work, manual-repo migrations, generated-repo adoption, and final validation needed to make repository README generation consistent across `context-engine`, `context-stack`, `memory-viewers`, `memory-api`, and `viewer-api`.

## Assumptions To Prove

- Existing `rule-targets` imports and themed fragment directories are enough foundation for a shared README rollout.
- A shared README schema can be introduced without breaking current AGENTS or non-README targets.
- Parent README links must remain repo-internal and never infer an external submodule parent.
- Each workspace must still be able to run `rule explain-target` and `rule sync-targets` in isolation.
- The final check must detect missing parent blocks, child blocks, installable-content sections, and command-doc references before review.

## Test-Driven Plan

1. Land failing `rule-api` tests that describe shared schema inheritance and required-block validation.
2. Implement schema support until those tests pass.
3. Migrate manual and generated repos in dependency order, validating each workspace with `explain-target`, `sync-targets`, and `sync-targets --check`.
4. Add an audit or check surface that fails when the README tree drifts.

## Acceptance Criteria

- The child trackers for schema primitives, manual repos, generated repos, and audit enforcement are closed.
- The resulting tree supports generated repo-root and first-level child README surfaces across the in-scope workspaces.
- Validation coverage exists for schema inheritance, rollout generation, and completeness checks.
