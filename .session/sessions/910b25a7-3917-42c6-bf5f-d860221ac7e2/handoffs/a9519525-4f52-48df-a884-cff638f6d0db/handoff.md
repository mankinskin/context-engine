# Handoff: a9519525-4f52-48df-a884-cff638f6d0db

## The High-Level Goal (why this session exists)

This session advances epic [1b58aaf5 Test coverage and contract hardening program](../../../../.ticket/tickets/1b58aaf5/ticket.toml). That epic is a **phased program (P1..P7)** to raise test coverage and harden cross-domain contracts across a large codebase: an audit found ~352k Rust LOC, ~39 tool surfaces, and 380+ operations with uneven test density, 104 of 122 specs still in draft, and mostly synthetic fixtures. The program cannot safely add coverage until the underlying contracts are unambiguous — so it starts by refining specs, then builds a replay harness and generative fixtures, then hardens reliability and process.

The epic is gated behind two open prerequisite defects that must not be forgotten:
- [e507818f ticket-cli batch hangs indefinitely during link creation](../../../../.ticket/tickets/e507818f/ticket.toml) — `ticket batch` can hang on multi-edge link creation and must complete or fail atomically.
- [70019883 next_tickets --root does not traverse containment edges to reach actionable leaves](../../../../.ticket/tickets/70019883/ticket.toml) — root-scoped next-ticket discovery misses real leaves, so Phase 1 leaves must be scoped explicitly until this is fixed.

## Where This Handoff Fits

Epic [1b58aaf5](../../../../.ticket/tickets/1b58aaf5/ticket.toml) → **Phase 1** [e9109814 Phase 1: Refine coverage-critical contracts](../../../../.ticket/tickets/e9109814/ticket.toml) → the leaf tickets below. Phase 1's job is narrow and specific: **resolve the contract ambiguities that block reliable test design**, by editing the underlying spec files during each leaf ticket. No harness or coverage work happens yet — that is Phase 2 onward.

## Summary
- **Workspace Session**: `910b25a7-3917-42c6-bf5f-d860221ac7e2`
- **Outgoing Run**: `fba96337-5ffa-449f-a233-baf543934d62`
- **Created**: 2026-08-05T17:01:48.134928600+00:00
- **Objective**: Begin **Phase 1** by implementing its leaf tickets in dependency order, applying the spec-file edits inside each ticket. Phase 1 refines the coverage-critical contracts so later phases can build tests on stable definitions.
- **Implementation Ready**: true

## Resume Command
```bash
session-cli resume --workspace-session-id 910b25a7-3917-42c6-bf5f-d860221ac7e2 --predecessor-run-id fba96337-5ffa-449f-a233-baf543934d62
```

## Target Tickets — what each one is and why it matters

Implement these in dependency order. Each is a **Phase 1 leaf**; the phase ticket itself is the container, not a unit of work.

| Ticket | What it does | Why it is in Phase 1 |
|---|---|---|
| [e9109814 Phase 1: Refine coverage-critical contracts](../../../../.ticket/tickets/e9109814/ticket.toml) | The phase container. Tracks the spec-refinement leaves below. | Resolving these contracts is the precondition for all later coverage work. |
| [b6b58573 Define default output and transport negotiation for spec 1d62442b](../../../../.ticket/tickets/b6b58573/ticket.toml) | Pin TOON as the default output; HTTP negotiates to JSON, MCP to TOON; explicit selectors, stable field ordering, no byte cap. | Output/transport behavior must be deterministic before it can be asserted in tests. |
| [3824c5da Define transport parity matrix for spec 9074b2ef](../../../../.ticket/tickets/3824c5da/ticket.toml) | Author a strict machine-readable CLI/MCP/HTTP field matrix. | Turns cross-transport parity into a **generated** test instead of prose. |
| [5a4c2e4d [test-api] Add first-class validation spec and result storage](../../../../.ticket/tickets/5a4c2e4d/ticket.toml) | Stand up a new `test-api` crate: validation plans/executions/outcomes (passed/failed/blocked) with native ids and first-class links to tickets/specs/docs/logs. | Gives the program a real store for validation evidence (folded into Phase 1; risk=high). |
| [65ea4528 [architecture][contracts] Core shared contract crate](../../../../.ticket/tickets/65ea4528/ticket.toml) | A shared trait/model primitive crate with ownership/versioning boundaries and dependency-DAG checks; compiles with no domain logic. | Establishes the contract baseline the other domains build on (folded into Phase 1). |
| [e26373a3 Build multi-store sandbox and replay harness](../../../../.ticket/tickets/e26373a3/ticket.toml) | A reusable multi-store sandbox + deterministic replay harness (temp-store isolation, cwd control, child-process concurrency). | **Listed for visibility only** — gated behind Phase 1 (`depends_on e9109814`), NOT part of this slice. |

## Target Files — the specs Phase 1 edits
- [.spec/specs/1d62442b-61dc-4eeb-9b7c-e933f84470f2/spec.toml](../../../../.spec/specs/1d62442b-61dc-4eeb-9b7c-e933f84470f2/spec.toml) — default output & transport-negotiation spec, refined by [b6b58573](../../../../.ticket/tickets/b6b58573/ticket.toml).
- [.spec/specs/39983ddf-1f7e-4081-a060-6b8258eb4c41/spec.toml](../../../../.spec/specs/39983ddf-1f7e-4081-a060-6b8258eb4c41/spec.toml) — Phase 1 contract spec touched during refinement.
- [.spec/specs/347b6f97-5ebf-46c6-a0e1-cc8afc600319/spec.toml](../../../../.spec/specs/347b6f97-5ebf-46c6-a0e1-cc8afc600319/spec.toml) — spec whose **retirement is deferred** to leaf ticket 9515b7db; do not retire it here.

## Decisions (settled — do not re-litigate)
- **Phase-gate ordering**: each phase `depends_on` its predecessor (P2→P1, P3→P2, … P7→P6) so the program advances one phase at a time.
- **Phase→leaf edges**: each phase ticket `depends_on` its own leaves, so `next_tickets` surfaces the actionable leaf, not the phase container.
- **Spec edits deferred to implementation**: the reconciliation run only wired the dependency graph; the actual `spec.toml` changes are made inside the leaf tickets (b6b58573, 3824c5da, …).
- **Spec 347b6f97 retirement deferred** to leaf ticket 9515b7db, not done inline.
- **Two defects relocated** to be epic-level prerequisites: [e507818f](../../../../.ticket/tickets/e507818f/ticket.toml) and [70019883](../../../../.ticket/tickets/70019883/ticket.toml).
- **20 external testing tickets folded** into their mapped phases rather than tracked separately.

## Non-Goals
- Do not re-open the 6 settled decisions in the epic's decision log.
- Do not fix the 11 `off_schema_state` warnings in `memory-viewers/.ticket` — separate cleanup pass (see Risk Notes).
- Do not close epic [1b58aaf5](../../../../.ticket/tickets/1b58aaf5/ticket.toml); it stays open until all seven phases land.
- Do not implement [e26373a3](../../../../.ticket/tickets/e26373a3/ticket.toml) (sandbox/replay harness) in this slice — it is gated behind Phase 1.

## Context Anchors
- **Commit** `2ffc9c903eda48c5abe7b13decaf3b6ae189dba9` — the reconciled graph state this handoff builds on.
- **Epic** [1b58aaf5 Test coverage and contract hardening program](../../../../.ticket/tickets/1b58aaf5/ticket.toml) — the high-level goal.
- **Phase 1** [e9109814 Phase 1: Refine coverage-critical contracts](../../../../.ticket/tickets/e9109814/ticket.toml) — the ticket to begin.
- **Defect** [e507818f](../../../../.ticket/tickets/e507818f/ticket.toml) and **defect** [70019883](../../../../.ticket/tickets/70019883/ticket.toml) — open epic prerequisites.

## Risk Notes
Separate forward cleanup (**not** part of Phase 1): 11 `off_schema_state` warnings in `memory-viewers/.ticket`, rooted at ticket 956485ad and transitively linked to 0556ed59, 09bef250, and 26a73130, need their own reconciliation pass. Also: the two epic prerequisites are still open — [70019883](../../../../.ticket/tickets/70019883/ticket.toml) breaks `next_tickets --root` traversal, so scope Phase 1 leaves **explicitly** rather than relying on root-scoped discovery until it is fixed.

## Workflow
- **Nodes**: 0
- **Edges**: 0
- **Not Done**: 0
