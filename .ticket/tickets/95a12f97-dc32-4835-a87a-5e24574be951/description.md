# Problem

`context-engine` and `context-stack` are still the manual outliers in the repository README family. They need to move onto rule-backed generation and the same parent/child README navigation contract as the already-generated workspaces.

## Scope

Migrate the manual repo roots and their first-level child README surfaces to rule-backed generation using the shared README schema.

## Assumptions To Prove

- The root workspace can add README targets to its existing `.rule` store without duplicating child repo ownership.
- `context-stack` can gain a repo-local `.rule` store and local target config without confusing nested workspace resolution.
- First-level child READMEs in manual repos can include parent blocks while staying repo-internal.

## Test-Driven Plan

1. Add the shared schema primitives first.
2. Generate the root README tree and validate from the root store.
3. Bootstrap `context-stack`, then add child README generation with repo-internal parent links.

## Acceptance Criteria

- The child tickets in this branch are closed.
- `context-engine` and `context-stack` no longer rely on manual repo-root README maintenance.
- First-level child READMEs in both repos participate in the parent/child navigation chain.
