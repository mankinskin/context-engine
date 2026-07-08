<!-- aligned-structure:v1 -->

# Summary

Adopt the shared README schema across the existing generated repos in the `memory-viewers` family so those workspaces stop carrying bespoke README target layouts and gain consistent parent/child navigation blocks.

## Behavior Story

Adopt the shared README schema across the existing generated repos in the `memory-viewers` family so those workspaces stop carrying bespoke README target layouts and gain consistent parent/child navigation blocks.

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

Adopt the shared README schema across the existing generated repos in the `memory-viewers` family so those workspaces stop carrying bespoke README target layouts and gain consistent parent/child navigation blocks.

## Problem

The generated repos already have local rule stores and README targets, but each workspace still hard-codes its own structure. That preserves drift in exactly the area the new shared schema is meant to remove.

## Scope

This spec covers:

- schema adoption in `memory-api`
- schema adoption in `viewer-api`
- normalization of the aggregate `memory-viewers` root README after the child repos settle

## Intended Behavior

- Each generated repo root can opt into the shared README schema.
- First-level generated child READMEs can add repo-internal parent blocks.
- The aggregate `memory-viewers` README can normalize its child blocks without flattening child ownership.

## Assumptions To Prove

- Existing generated README targets can adopt the shared schema incrementally.
- Parent links can be added to generated child READMEs without changing workspace ownership boundaries.
- The aggregate `memory-viewers` README should follow the child repos rather than redefining them.

## Test Strategy

1. Migrate one generated repo at a time.
2. Add parent links to its child README surfaces.
3. Normalize the aggregate README only after the child roots are stable.

## Acceptance Criteria

- The generated `memory-viewers` family adopts the shared README schema.
- Parent/child navigation is consistent across the family.
- Each workspace can validate its README tree independently.

## Traceability

- [7b8d2e81 generated repo rollout tracker](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/7b8d2e81-6f00-486c-a839-ca5eb77dc109/ticket.toml)
