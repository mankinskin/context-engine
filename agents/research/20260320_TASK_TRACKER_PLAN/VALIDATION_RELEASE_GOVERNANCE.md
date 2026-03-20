# Validation, Bug Tracking, and Stable Release Governance

## Objective

Define how coordinators, workers, and validator agents collaborate so ticket
progress is auditable, defects are tracked deterministically, and stable releases
ship quickly without regressions.

## Role Model

- Coordinator agent:
  - schedules work
  - assigns worker and validator agents
  - enforces validation and release gates
- Worker agent:
  - implements ticket changes
  - cannot self-validate final acceptance for the same ticket
- Validation agent:
  - independently verifies acceptance criteria and risk-specific checks
  - can approve, reject, or request changes with evidence
- Release agent:
  - assembles release candidates from validated tickets
  - enforces release gates and rollback readiness

## Ticket Validation Contract

Every delivery-class ticket must include:

- `risk_level`: `low | medium | high`
- `validation_plan`: explicit checks to run
- `validation_status`: `pending | in-progress | passed | failed`
- `validator_id`: assigned validation agent
- `evidence_refs`: command/test/log references
- `release_target`: release train or milestone

## State Model

Default lifecycle for delivery tickets:

`open -> in-progress -> review -> validating -> validated -> release-candidate -> released -> monitoring -> done`

Exception states:

- `blocked`
- `cancelled`

Failure path:

- `validating -> review` on failed validation
- `released -> blocked` if post-release regression is found

## Validation Rules

- Separation of duties: worker and validator must be different identities.
- High-risk tickets require two validation passes:
  - functional validator
  - reliability/regression validator
- Validation failures require structured rejection details and at least one linked
  bug ticket when a product defect is found.
- Tickets cannot enter `release-candidate` unless `validation_status=passed`.

## Bug Tracking Policy

When validation or monitoring finds a defect:

1. Create linked bug ticket with:
   - severity (`sev0`, `sev1`, `sev2`, `sev3`)
   - impacted release version
   - reproduction evidence
2. Link relation:
   - bug `caused_by` source ticket
   - source ticket `has_bug` bug ID
3. Release blocking:
   - `sev0`/`sev1` block release candidate promotion
4. Closure:
   - bug fix requires independent re-validation before release resumes

## Coordinator Scheduling Policy

- Dispatch model with independent workers:
  - coordinator assigns tickets
  - workers claim and update tickets directly through the ticket protocol
  - coordinator is not the sole writer for worker progress
- Validation queue is explicit:
  - coordinator moves ticket to `validating`
  - assigns validator based on component + risk
  - waits for pass/fail result event

## Stable Release Gates

Gate R1: Scope readiness
- all included tickets are `validated`
- no unresolved dependency blockers

Gate R2: Defect safety
- no open `sev0`/`sev1` bugs in release scope

Gate R3: Operational safety
- migration notes present when schema/state/history behavior changed
- rollback command path verified (`history`/`revert` checks)

Gate R4: Verification
- release smoke suite passes on release artifact

Gate R5: Monitoring
- post-release observation window has no critical incident

## Release Train Flow

1. Build candidate from `release-candidate` tickets.
2. Run release validation suite.
3. Promote to `released`.
4. Observe in `monitoring` window.
5. Mark `done` if stable; otherwise open bug and rollback as required.

## Metrics

- Validation lead time (review -> validated)
- Rejection rate by component and risk level
- Bug escape rate (bugs found after release)
- Mean time to recover (MTTR) for release regressions
- Release success rate per train

## Integration Points

- Phase 1: schema fields and state machine include validation + release states.
- Phase 1.5: lease metadata supports validator ownership; coordinator enforces separation of duties.
- Phase 4: dogfooding gates include validator throughput and bug/release gates.
- Phase 5: messenger/visualization publish validation and release status dashboards.
