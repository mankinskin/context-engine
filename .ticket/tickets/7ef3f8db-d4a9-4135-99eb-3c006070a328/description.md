## Objective
Implement the shared directed schema engine required by [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml).

## Requirements
- Add strict linear zero-or-one-parent inheritance with atomic full-load/reload validation for missing parents and cycles.
- Resolve child lifecycle graphs as category-contained refinements of universal `plan`, `act`, and `verify` nodes.
- Require exactly one plan-category root per resolved schema and a valid path through all categories for each concrete work-item type.
- Enforce directed lifecycle edges: same category, `plan→act`, `act→verify`, `verify→act`, and `act→plan`; reject skips.
- Keep lifecycle graphs distinct from relation graphs; ticket/spec/rule registries reuse primitives but keep separate schema-kind namespaces and local graphs.
- Make `cancelled` a derived verify terminal leaf; terminal nodes have no outgoing edge.
- Explicit reload atomically swaps registry generation, resolved caches, manifest/catalog index, and client cache version, or retains the prior valid generation.

## Acceptance Criteria
Focused tests prove directionality, inheritance resolution, category containment, terminal validation, atomic reload rollback, and cache invalidation across ancestor changes.


## Decision-Complete Validation Contract
- Define explicit resolved-schema entry and terminal semantics, including each category-refinement boundary.
- Require reachability of every resolved lifecycle node from the one global plan entry and a valid terminating path through `plan`, `act`, and `verify`.
- Treat category refinement as a contained tunnel: validate permitted boundary edges and reject bypasses, skipped categories, and illegal category escapes.
- Validate only declared rework/replan loops; test permitted `verify→act` and `act→plan` loops separately from forbidden loops.
- Preserve five distinct model concepts: schema type, concrete lifecycle state, lifecycle category, ticket relation/dependency edge, and validation gate. Tests must reject cross-namespace or graph-semantic conflation.

## Additional Acceptance Criteria
Focused tests cover entry/terminal boundaries, all-node reachability, contained refinement tunnels, allowed and disallowed loops, illegal escape rejection, and five-way concept separation.


## Recovered Interview Requirements
- Model `cancelled` as a derived `verify` terminal leaf with no outgoing edge; allow direct entry to `cancelled` from `plan`, `act`, or `verify` only as the sole cross-category exception.
- Reuse directed-lifecycle primitives for ticket, spec, and existing rule schemas while keeping independent local graphs and per-kind type-ID namespaces.
- Relation-edge validation checks declared relation-kind rules and endpoint existence, but never applies lifecycle categories or lifecycle directionality.

## Additional Acceptance Criteria
Focused tests prove cancellation direct-entry/terminal behavior, separate ticket/spec/rule local graphs and namespaces, and relation-edge validation independence from lifecycle rules.


## Governing Policy Rule
Create or link the governing policy rule that introduces [e9c38d24 Schema modernization lifecycle and migration](.spec/specs/e9c38d24-42cc-4044-8b2c-6811b918530f/spec.toml) and its lifecycle-validation guards. The rule must apply the shared directed-lifecycle primitives to existing rule schemas without creating a new rule entity.

## Additional Acceptance Criteria
Rule validation proves the governing rule resolves, links the owning spec and validation guards, and preserves separate ticket/spec/rule local graphs and type-ID namespaces.