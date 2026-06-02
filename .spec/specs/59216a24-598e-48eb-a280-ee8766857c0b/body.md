# Summary

Migrate the manual README trees in `context-engine` and `context-stack` onto the same rule-backed generation flow and parent/child navigation contract already used in the generated nested workspaces.

## Problem

The root workspace and `context-stack` are still the manual outliers. That blocks a consistent README contract because the most visible repo roots and their first-level child surfaces sit outside the generation pipeline.

## Scope

This spec covers:

- root-owned README targets in `context-engine`
- a repo-local `.rule` store and README targets in `context-stack`
- generated parent/child navigation blocks for the first-level child README surfaces owned by those repos

## Intended Behavior

- `context-engine` generates its root README and root-owned child README surfaces from the root `.rule` store.
- `context-stack` owns a repo-local `.rule` store and local README targets.
- First-level child READMEs in both repos expose repo-internal parent links.
- Repo roots expose child blocks without assuming any external git-submodule parent.

## Assumptions To Prove

- The root `.rule` store can own README targets without violating child repo ownership.
- `context-stack` can host a local rule store alongside its current nested structure without breaking workspace resolution.
- First-level child README generation can be delivered without immediately rewriting deeper docs trees.

## Test Strategy

1. Add the root workspace-doc targets and validate the root README tree.
2. Bootstrap `context-stack` locally and validate its root README target.
3. Extend `context-stack` to its first-level child README tree with parent-link validation.

## Acceptance Criteria

- The manual repo roots no longer rely on hand-edited README maintenance.
- First-level child README surfaces in the manual repos participate in the generated navigation tree.
- Each affected workspace can regenerate its README outputs locally and from an ancestor checkout where applicable.

## Traceability

- [95a12f97 manual repo rollout tracker](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/95a12f97-dc32-4835-a87a-5e24574be951/ticket.toml)
