<!-- aligned-structure:v1 -->

# Summary

Migrate the manual README trees in `context-engine` and `context-stack` onto the same rule-backed generation flow and parent/child navigation contract already used in the generated nested workspaces.

## Behavior Story

Migrate the manual README trees in `context-engine` and `context-stack` onto the same rule-backed generation flow and parent/child navigation contract already used in the generated nested workspaces.

## Provided Surface Contracts

- README surfaces in scope follow an explicit rule-backed contract instead of one-off rollout prose.
- Parent and child README navigation stays repo-internal and mechanically derivable.
- README completeness and rollout status are verified by mechanical validation rather than manual review.

## Required Validation

- Contract clause validation: The migrated spec names the intended README structure and navigation behavior as explicit contract properties.
- Contract clause validation: The migrated spec names the validation path that checks the README contract mechanically.
- Contract clause validation: The migrated spec records enough evidence to tell whether the README contract is satisfied or blocked.
- The authored spec body documents the README contract, scope boundaries, and navigation expectations for this migration slice.
- The authored spec body documents the mechanical validation path required to prove this migration slice.
- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

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
