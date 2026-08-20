# 01. Conceptual Input Contract

## Outcome

Extend the planned `presentation-api` model with a versioned, persisted
contract for conceptual deck inputs and generator ownership. A generated claim
must be reproducible from authoritative specification content and safe to
replace without overwriting human-owned material.

## Inputs

- [Presentation System](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/body.md), especially R8, R9, R11, R12, R14, and R15.
- [Phase 2 presentation API plan](../../.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/description.md).
- [Legacy deck manifest](../../.presentation/deck.toml) and [composition rules](../../.presentation/README.md).

## Required Contract

- A source lock records selected specification paths and sections, their
  content hashes, Git base, transform version, and theme/preset version.
- A claim is either `quoted` or `synthesized`, carries source selectors and
  citations, and cannot be published without resolved source references.
- A disagreement sidecar records category, enum severity, owner, resolution
  state, and source locations. Material unresolved entries block publication
  or visibly qualify their affected slides.
- A managed-output declaration separates generated paths from human overlays;
  writes reject traversal and symlink escapes and require explicit replacement
  when managed output has unexpected modifications.
- Legacy singleton discovery and cross-repository imports resolve
  deterministically before a multi-deck registry is canonical.

## Non-Goals

- Render Slidev output or implement topology extraction.
- Reconcile contradictions automatically.
- Treat implementation, documentation, or telemetry as authoritative.

## Validation

Implement fixture tests for stale locks, claim/citation classification, all
sidecar fields, material-contradiction publication behavior, explicit
replacement, path and symlink containment, legacy singleton discovery, and
deterministic imports. The target command is
`cargo test -p presentation-api --test conceptual_input_contract` plus a
managed-output-boundary test once the Phase 2 crate exists.

## Tracking

Ticket `1500a9e6-293f-4803-969d-0dcabeaa470a` depends on [3cdcaf3b Phase 2](../../.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/ticket.toml). Resolve the new DB-backed ticket through `mcp_ticket_get_ticket`; it is intentionally ticket-sized because it changes persisted authoritative semantics across the deck model.
