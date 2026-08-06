# Handoff: 276acf70-5af5-45de-8154-5ef9b58357f7

Make the upward-context and ticket-narrative contract reproducible for newly created handoffs.

## Upward Context
[25b5f3e7 Make upward context and ticket narrative reproducible](.ticket/tickets/25b5f3e7-cace-4822-a955-bc2e3202be77/ticket.toml) (parent) -> Handoff Package Schema (phase) -> [742dbc65 [session-api][handoff] Model and enforce upward context for implementation-ready handoffs](.ticket/tickets/742dbc65-a100-4278-9274-7d99a3e2afc4/ticket.toml), [ba8f5528 [session-api][handoff] Render resolved ticket narrative and upward context in handoff markdown](.ticket/tickets/ba8f5528-5af3-4de2-8904-442a4691854a/ticket.toml)

## Summary
- **Workspace Session**: `470fb360-8426-4f79-bdda-8ba89a81cf4b`
- **Outgoing Run**: `8552a51e-5188-4a78-b4d6-fff4e995d76c`
- **Created**: 2026-08-06T01:02:21.184726800+00:00
- **Objective**: Implement the remaining upward-context and ticket-narrative work for newly created handoffs.
- **Implementation Ready**: true

## Resume Command
```bash
session-cli resume --workspace-session-id 470fb360-8426-4f79-bdda-8ba89a81cf4b --predecessor-run-id 8552a51e-5188-4a78-b4d6-fff4e995d76c
```

## Target Tickets
| Ticket | What it does | Why |
| --- | --- | --- |
| [742dbc65 [session-api][handoff] Model and enforce upward context for implementation-ready handoffs](.ticket/tickets/742dbc65-a100-4278-9274-7d99a3e2afc4/ticket.toml) | ## Problem<br>`SessionHandoffPackage` has no durable higher-level objective or structured upward-context data. `create_handoff_record` therefore cannot distinguish an implementation-ready handoff with missing program context from a well-formed handoff.<br><br>## Goal<br>Add durable, backward-compatible package data and conditional creation-time validation for the already-decided upward-context contract.<br><br>## Authoritative Spec<br>`5e52039d` (`agent-workflow/handoff-package-schema`, Handoff Package Schema). A separate agent owns the spec update; this task must remain compatible with that update.<br><br>## Target Paths<br>- `memory-api/crates/session-api/src/model/handoff.rs`<br>- `memory-api/crates/session-api/src/store/config/handoff_finish.rs`<br>- `memory-api/crates/session-api/tests/handoff_folder_storage.rs`<br>- `memory-api/crates/session-api/tests/handoff_roundtrip.rs`<br><br>## Acceptance Criteria<br>1. `SessionHandoffPackage` persists a structured upward-context ancestor chain whose entries contain an entity URN, a human title, and a role (for example epic, phase, or parent), plus a prose `higher_level_objective` field that explains why the current phase exists now. Round-trip tests prove the fields survive JSON persistence.<br>2. Creation determines “claims implementation-ready” from the existing derived readiness rule: objective is non-empty and `open_escalations` is empty. When that condition is true, absent or empty `higher_level_objective` or ancestor chain rejects creation with `SessionError::HandoffPackageIncomplete` before handoff artifacts are written.<br>3. When the derived readiness condition is false, absent upward-context fields emit warnings but creation persists the exploratory handoff. Tests prove the warning path does not change the existing readiness derivation.<br>4. Existing on-disk `handoff.json` data that lacks the new optional-on-wire fields remains deserializable. The task does not change the existing path existence validation for `target_files` or path-shaped `context_anchors`.<br><br>## Related Work<br>`7bb007e9` is an adjacent generic conformance-gate effort that may later share mechanism but is not a blocking dependency. `f77e35d8`, `a6f17580`, `6431985e`, and `0d3fdba6` are adjacent handoff work with different field sets or scope. `e4f84414` is a markdown-renderer precedent, not scope for this task.<br><br><br>## Validation Evidence<br>- Merge commit: `26c7ce7c`<br>- Validation spec: `session-api-tests` (`cargo test -p session-api`)<br>- Validation execution: `exec-session-api-handoff-upward-context-20260806` (`passed`)<br>- Supporting checks: `cargo build --workspace`, `cargo check -p session-mcp`, `cargo check -p session-cli`<br>- Coverage: legacy `target_tickets` deserialization, readiness-gated upward-context enforcement, warning-path persistence, breadcrumb/table rendering, fallback rendering for unresolved tickets, and regeneration-vs-exemplar proof with no manual hand edit required for the proof run.<br>- Acceptance verdicts: AC1 met; AC2 met; AC3 met; AC4 met. | Model and enforce the structured upward-context fields so newly created implementation-ready handoffs retain the contract. |
| [ba8f5528 [session-api][handoff] Render resolved ticket narrative and upward context in handoff markdown](.ticket/tickets/ba8f5528-5af3-4de2-8904-442a4691854a/ticket.toml) | ## Problem<br>`render_handoff_record_markdown` renders target tickets as bare short IDs and omits both a higher-level goal and upward program context. Manual edits to the generated exemplar are destroyed by the next handoff generation.<br><br>## Goal<br>Generate legible, reproducible handoff markdown from durable package data, resolving ticket metadata through existing `ticket-api` integration and never requiring post-generation edits.<br><br>## Authoritative Spec<br>`5e52039d` (`agent-workflow/handoff-package-schema`, Handoff Package Schema). A separate agent owns the spec update.<br><br>## Target Paths<br>- `memory-api/crates/session-api/src/model/handoff.rs`<br>- `memory-api/crates/session-api/src/store.rs`<br>- `memory-api/crates/session-api/src/store/config/handoff_finish.rs`<br>- `memory-api/crates/session-api/tests/handoff_folder_storage.rs`<br>- `memory-api/crates/session-api/tests/handoff_roundtrip.rs`<br>- `.session/sessions/910b25a7-3917-42c6-bf5f-d860221ac7e2/handoffs/a9519525-4f52-48df-a884-cff638f6d0db/handoff.md`<br><br>## Acceptance Criteria<br>1. `target_tickets` evolves from bare IDs to structured entries with at least a ticket ID and author-supplied `why` text. Serde accepts legacy JSON arrays of strings so existing stored handoffs remain readable.<br>2. `render_handoff_record_markdown` emits the prose higher-level objective near the beginning and an upward-context breadcrumb that presents ancestor role, human title, and clickable entity reference in epic-to-phase-to-leaf order.<br>3. Generated markdown includes a per-ticket table. Each row contains a clickable ticket reference and title plus auto-resolved “what it does” from ticket-api, and preserves the author-supplied “why it belongs in this handoff.”<br>4. A missing or unresolvable referenced ticket produces a clear fallback row using available authored/ID data; rendering neither panics nor fails handoff creation. Tests cover the fallback behavior.<br>5. Regenerate handoff `a9519525-4f52-48df-a884-cff638f6d0db` entirely through the generator and diff it with the hand-authored exemplar. The generated artifact demonstrably has: a stated high-level goal up front, an epic-to-phase-to-leaf breadcrumb, real titles replacing every bare ticket ID, and clickable references following repository policy.<br><br>## Related Work<br>`e4f84414` is the closest markdown-renderer precedent. `f77e35d8` changes the terminal renderer for different narrative fields; no dependency is intended. `7bb007e9`, `a6f17580`, `6431985e`, and `0d3fdba6` are adjacent but non-blocking work.<br><br><br>## Validation Evidence<br>- Merge commit: `26c7ce7c`<br>- Validation spec: `session-api-tests` (`cargo test -p session-api`)<br>- Validation execution: `exec-session-api-handoff-upward-context-20260806` (`passed`)<br>- Supporting checks: `cargo build --workspace`, `cargo check -p session-mcp`, `cargo check -p session-cli`<br>- Coverage: structured `target_tickets`, resolver-backed title/what-it-does rendering, upward-context breadcrumb rendering, fallback row behavior for unresolved tickets, no double-linking in existing links or code spans, and regenerated exemplar proof with all four quality checks passing.<br>- Reviewed deviation: the exemplar at `.session/sessions/910b25a7-3917-42c6-bf5f-d860221ac7e2/handoffs/a9519525-4f52-48df-a884-cff638f6d0db/handoff.md` was regenerated and diffed for proof, but the stored file was not overwritten in the isolated worktree because that path was untracked there.<br>- Acceptance verdicts: AC1 met; AC2 met; AC3 met; AC4 met; AC5 met with reviewed deviation. | Render resolved ticket narrative and upward context from structured handoff data for newly created records. |

## Target Files
- `memory-api/crates/session-api/src/model/handoff.rs`
- `memory-api/crates/session-api/src/store/config/handoff_finish.rs`
- `memory-api/crates/session-api/src/store.rs`
- `memory-api/crates/session-api/tests/handoff_roundtrip.rs`

## Decisions
- Historical handoff a9519525 remains immutable and is not migrated or regenerated.
- Validate only newly created handoffs under the current schema.
- Account for ticket-mirror side effects during implementation validation.

## Non-Goals
- Legacy handoff migration or backfill.
- Overwriting or modifying historical handoff a9519525.

## Context Anchors
- ce://default/ticket/[25b5f3e7 [session-api][handoff] Make upward context and ticket narrative reproducible in handoff markdown](.ticket/tickets/25b5f3e7-cace-4822-a955-bc2e3202be77/ticket.toml)
- ce://default/spec/5e52039d-aabc-434d-bdf3-eca63e312476

## Workflow
- **Nodes**: 0
- **Edges**: 0
- **Not Done**: 0

## Validation
- `session-api-handoff-isolated-create-render`: - (required)
- `session-api-handoff-roundtrip`: - (required)
