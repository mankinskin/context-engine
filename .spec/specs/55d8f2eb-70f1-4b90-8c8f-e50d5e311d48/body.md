<!-- aligned-structure:v2 -->

# Specification Store

## Responsibility And Interface

Persist roots and their new artifacts while preserving current section and
hierarchy behavior in `src/store/sections.rs` and `src/store/hierarchy.rs`.
`SpecStore` is the persistence and retrieval boundary.

## Behavior And Contract

- `store-persists-artifacts`: round-trip components, criteria, evidence, edges,
  and observations with their root.
- `store-preserves-baselines`: preserve sections, hierarchy, and `TicketRef`.
- `store-removes-retired-model`: retire `contract_mode`, expected properties,
  mandatory evidence requirements, and fulfillment summaries.

## Boundaries And Failure Cases

The store does not decide health policy or invent a migration plan. Failed parse,
missing root, and invalid persisted references return errors; retained baselines
must not silently change semantics.

## Acceptance Evidence And Position

Extend `src/store/tests.rs` and `src/manifest/tests.rs` with round-trips and
baseline regressions. Current `SpecStore` persists the retired manifest fields.
