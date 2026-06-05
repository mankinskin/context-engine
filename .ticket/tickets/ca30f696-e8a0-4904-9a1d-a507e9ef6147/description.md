# Problem

The shared README schema rollout now has a concrete loader-contract gap: shared schema fragments can be reached through both explicit imports and ambient fragment discovery, and the rollout depends on that path behaving deterministically across isolated workspace `sync-targets --check` runs.

## Scope

Track the schema-loader contract updates, the current `rule-api` implementation work, and the first workspace rollout validations needed to carry the shared README schema from `rule-api` through representative workspace adoption.

## Assumptions To Prove

- Shared schema fragments register once per canonical config file during a single config load, even when reached through both imports and fragment discovery.
- Schema visibility remains global for the active config load so sibling fragments can reference shared schemas without re-registering them.
- Representative workspace validations can move from schema-loader failures to real rollout/output issues without regressing unrelated targets.

## Plan

1. Update the schema tickets to capture the shared-fragment loader contract explicitly.
2. Complete the active `rule-api` implementation ticket and validate `memory-api` representative targets.
3. Progress the generated-repo rollout tickets once the representative workspace path is green.
4. Close this tracker once the schema contract is reflected in ticket/spec guidance and the dependent rollout tickets are complete.

## Acceptance Criteria

- The child tickets for schema implementation and representative workspace rollout are closed.
- The ticket descriptions capture the shared schema loader contract explicitly.
- The representative workspace validation path fails only on real rollout/output drift, not duplicate-schema loader behavior.
