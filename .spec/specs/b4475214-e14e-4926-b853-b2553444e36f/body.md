<!-- aligned-structure:v2 -->

# Specification Health Check

## Responsibility And Interface

Report structural target-model validity through `SpecStore::health` and
`SpecStore::health_all` in `src/store.rs`, exposed by `spec.exe health`.

## Behavior And Contract

- `health-validates-references`: check required fields, root membership,
	ownership, uniqueness, and artifact references.
- `health-allows-unvalidated-criteria` and `health-no-fulfillment-gate` accept
	missing validation and never require satisfied evidence.
- `health-hierarchy-integrity`: reject missing parents, orphaned specs, and
	parent cycles across the spec hierarchy.
- Consume persisted artifacts from Specification Store.

## Boundaries And Failure Cases

Health reports structural findings; it does not mark a target fulfilled. Current
`children`, `ancestors`, and `subtree` traversal in `src/store/hierarchy.rs`
does not detect missing parents or cycles, so this is a newly required gap.

## Acceptance Evidence And Position

Add hierarchy orphan/cycle/missing-parent tests in `src/store/tests.rs`; run
`spec.exe --workspace . health --all`. Today it must report only the known three
unrelated findings on `9f0b9e30`.
