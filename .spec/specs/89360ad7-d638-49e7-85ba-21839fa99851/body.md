<!-- aligned-structure:v2 -->

# Validation Store Evidence Integration

## Responsibility And Interface

Supply executable or recorded evidence to Validation Observation without
requiring automation for every criterion. It exposes a criterion target, status,
and optional time/detail to the observation boundary.

## Behavior And Contract

- `validation-criterion-link`: evidence identifies applicable spec/criterion targets.
- `validation-observation-source`: outcomes expose status and optional time/detail.
- `validation-best-effort`: missing executable validation remains documented and reviewable.
- Validation Observation consumes all three criteria through the root map.

## Boundaries And Failure Cases

The store does not own criteria, declare fulfillment, or make health fail when
automation is absent. An invalid target/status is rejected; no result is valid.

## Acceptance Evidence And Position

Add test-api cases for targeted evidence and absent automation under reviewed
implementation work. `90e4fb79-2c60-42a6-ab10-91d243693150` supplies the
existing workflow rule for recording validation evidence.
