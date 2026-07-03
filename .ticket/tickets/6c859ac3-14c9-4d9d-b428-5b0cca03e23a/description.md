# Goal

Generalize the move kernel's journaling concept into a reusable operation journal contract for memory stores.

## Scope

- Define an `OperationJournal` envelope with ids, schema version, operation kind, preflight inputs, blockers, ordered steps, phases, touched entities/files, inverse operations, recovery instructions, and links.
- Represent reversibility explicitly: replayable, rollbackable, manual_recovery.
- Define read-only preflight, apply, resume, rollback, and replay semantics.
- Decide storage layout and index ownership for journal metadata.

## Acceptance criteria

- The schema can model existing `MoveJournal` without losing recovery information.
- Non-mutating graph/search journals and rollbackable store mutations are both representable.
- The schema explicitly avoids treating trace logs as rollback journals.