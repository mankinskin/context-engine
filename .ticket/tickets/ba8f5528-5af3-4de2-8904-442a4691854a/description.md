## Problem
`render_handoff_record_markdown` renders target tickets as bare short IDs and omits both a higher-level goal and upward program context. Manual edits to the generated exemplar are destroyed by the next handoff generation.

## Goal
Generate legible, reproducible handoff markdown from durable package data, resolving ticket metadata through existing `ticket-api` integration and never requiring post-generation edits.

## Authoritative Spec
`5e52039d` (`agent-workflow/handoff-package-schema`, Handoff Package Schema). A separate agent owns the spec update.

## Target Paths
- `memory-api/crates/session-api/src/model/handoff.rs`
- `memory-api/crates/session-api/src/store.rs`
- `memory-api/crates/session-api/src/store/config/handoff_finish.rs`
- `memory-api/crates/session-api/tests/handoff_folder_storage.rs`
- `memory-api/crates/session-api/tests/handoff_roundtrip.rs`
- `.session/sessions/910b25a7-3917-42c6-bf5f-d860221ac7e2/handoffs/a9519525-4f52-48df-a884-cff638f6d0db/handoff.md`

## Acceptance Criteria
1. `target_tickets` evolves from bare IDs to structured entries with at least a ticket ID and author-supplied `why` text. Serde accepts legacy JSON arrays of strings so existing stored handoffs remain readable.
2. `render_handoff_record_markdown` emits the prose higher-level objective near the beginning and an upward-context breadcrumb that presents ancestor role, human title, and clickable entity reference in epic-to-phase-to-leaf order.
3. Generated markdown includes a per-ticket table. Each row contains a clickable ticket reference and title plus auto-resolved “what it does” from ticket-api, and preserves the author-supplied “why it belongs in this handoff.”
4. A missing or unresolvable referenced ticket produces a clear fallback row using available authored/ID data; rendering neither panics nor fails handoff creation. Tests cover the fallback behavior.
5. Regenerate handoff `a9519525-4f52-48df-a884-cff638f6d0db` entirely through the generator and diff it with the hand-authored exemplar. The generated artifact demonstrably has: a stated high-level goal up front, an epic-to-phase-to-leaf breadcrumb, real titles replacing every bare ticket ID, and clickable references following repository policy.

## Related Work
`e4f84414` is the closest markdown-renderer precedent. `f77e35d8` changes the terminal renderer for different narrative fields; no dependency is intended. `7bb007e9`, `a6f17580`, `6431985e`, and `0d3fdba6` are adjacent but non-blocking work.