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
