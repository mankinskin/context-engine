## Problem
During the context-engine -> meta-workspace/.workflow-tools migration (2026-09-01), 10 of 253 spec.toml manifests were found storing `related_tickets` as store-relative or submodule-relative paths (e.g. `.ticket/tickets/<uuid>/ticket.toml`, `memory-api/.ticket/tickets/<uuid>/ticket.toml`) instead of bare ticket UUIDs. Once the `.ticket` store physically moved, every one of these references was already broken/stale, because the field's persisted form coupled identity to a moment-in-time physical location instead of a portable ID.

All 10 were repaired manually as part of the migration (see transcripts/30-08-2026_spec-system-improvement-planning/migration-execution.md) by extracting the bare UUID and discarding the path prefix and suffix.

## Root cause
Nothing in spec-api's write path validates that `related_tickets` entries are bare IDs. A path-shaped string round-trips silently through create/update until the underlying store moves and the embedded path stops resolving to anything at all.

## Proposed fix
- Add write-time validation in spec-api (spec_create / spec_update) that rejects any `related_tickets` (and similarly-shaped relation fields, if any exist) entry that is not a bare UUID (or short-id, if short-ids are accepted elsewhere).
- Add a regression test asserting a path-shaped value (`.ticket/tickets/<uuid>/ticket.toml`) is rejected with a clear error, and a bare UUID is accepted.
- Consider a one-time spec_health-style check across the corpus to catch any other structured field with the same defect.

## Non-goals
- Does not cover markdown prose links inside spec body.md/ticket description content — those are historical narrative, not structured/relied-upon fields, and are out of scope for this ticket.
- Does not require rendering-layer changes; this is a write-time validation fix only.

## Acceptance Criteria
- spec-api write path rejects a path-shaped related_tickets entry with a clear, actionable error message.
- A regression test exists covering both the rejection case and the bare-UUID acceptance case.
- The crate that owns this validation builds and its tests pass.
