<!-- aligned-structure:v2 -->

# Validation Observation

## Responsibility And Interface

Record optional evidence result for a criterion. An observation requires `id`,
`criterion_id`, `evidence_reference_id`, and `status`; time and detail are
optional.

## Behavior And Contract

- `observation-required-fields` and `observation-optional-detail` define shape.
- `observation-reference-integrity` resolves criterion and evidence in one root.
- `observation-does-not-gate-health` accepts omitted observations and unsatisfied evidence.
- Consume Criterion identity, Evidence identity, and Validation Store status output.

## Boundaries And Failure Cases

An observation is not required evidence, fulfillment summary, or criterion
owner. Missing references or cross-root targets are invalid; absence is valid.

## Acceptance Evidence And Position

Replace fulfillment-summary tests in `src/manifest/tests.rs` and `src/store/tests.rs`.
Run focused tests plus `spec.exe --workspace . health --all` after implementation.
