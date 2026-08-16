<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=c737328d-a97e-4250-bf9a-390224ab57fd slug=memory-api/session-api/handoff-provenance-and-tracks digest=0fe57951fe9b -->

# Session merge and pickup: handoff-edge provenance graph and first-class tracks

- slug: `memory-api/session-api/handoff-provenance-and-tracks`
- component: session-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml`

## Summary

Promote the session handoff record to be the literal provenance edge of the cross-session

## Acceptance Criteria Excerpt

AC1 (unit) — Given a set of persisted `SessionRecord` and handoff records covering a fan_out (1 source → N target-less handoffs, some claimed, some still open) and a merge (1 session with N entries in `picked_up_handoff_ids`), a graph-reconstruction routine built only from those…

## Navigation

- Parent: [memory-api/session-api/durable-session-workflow](../../README.md)
- Children: _(none)_
