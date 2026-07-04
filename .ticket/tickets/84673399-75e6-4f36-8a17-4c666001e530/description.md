# Goal

Resolve the core architecture decisions for unified logging, operation journaling, and replayable visualization before implementation proceeds.

## Scope

- Decide owning crate boundaries for shared tracing runtime, log-api metadata, and generalized operation journals.
- Decide whether journals live under `.log`, domain stores, or a dedicated `.journal` layout.
- Define schema-versioning strategy for logs, journals, and graph replay events.
- Classify operations as replayable, rollbackable, or manual-recovery only.
- Document dependency-direction constraints so `log-api`, `memory-api`, context-stack, and viewer tooling stay DRY without cycles.

## Placement decision

Keep this ticket in the root `context-engine` workspace as the lowest common ancestor. Its output governs specialized follow-on work in lower crates; do not move it into `memory-api`, `log-api`, or `context-stack`.

## Implementation tracks (completed via child scope split)

- `529844ac`: cross-store correlation-id contract.
- `1c56033e`: canonical profiling/tracing phase taxonomy.
- `8b1eab26`: deterministic replay vs profiling evidence boundary.
- `72b3545c`: profiling metadata retention and redaction governance.

All child dependencies are now complete and integrated into the owning observability architecture spec.

## Evidence

Published in:

- `.spec/specs/aa769a27-2721-4b9d-880c-5c4e2f8136a7/body.md`
  - Cross-store correlation-id contract
  - Profiling metadata retention and redaction policy
  - Deterministic replay versus profiling evidence boundary
  - Canonical profiling and tracing phase taxonomy

These outputs satisfy architecture-level boundary decisions and provide explicit contract guidance for downstream implementation tickets and `ff6637f5` validation checklist linkage.

## Acceptance criteria status

- Open decisions from `aa769a27` were resolved in this tracker's child outputs or narrowed with explicit policy/contract sections.
- Artifact boundaries (trace log vs operation journal vs replay event) are now concretely specified and linked to contract sections.
- Tracker roadmap can proceed without ambiguous ownership for the completed contract slices.

## Acceptance criteria addendum status

- Correlation-id contract is published.
- Canonical phase taxonomy is published.
- Deterministic replay payload boundary vs profiling evidence is explicitly documented.
- Retention/redaction governance decision is documented.