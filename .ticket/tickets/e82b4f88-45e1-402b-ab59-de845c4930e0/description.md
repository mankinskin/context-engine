# Problem

Ticket↔spec traceability is currently PROSE-ONLY. Spec body.md files reference their related tickets in narrative markdown text, and tickets reference specs the same way. **There is NO structured field on either ticket.toml or spec.toml that expresses the link.**

## Verified Evidence

1. **Schema inspection** (memory-api/crates/ticket-api/src/model/ticket.rs:1-3):
   ```rust
   pub use memory_api::model::entity::{
       EntityId as TicketId,
       EntityManifest as TicketManifest,
   };
   ```
   TicketManifest is EntityManifest, which has only `id`, `created_at`, and flattened `extra: BTreeMap<String, Value>`. NO structured ticket or spec link field exists.

2. **Schema inspection** (memory-api/crates/spec-api/src/manifest.rs:124-139):
   ```rust
   pub struct SpecManifest {
       pub id: SpecId,
       pub created_at: DateTime<Utc>,
       #[serde(default, skip_serializing_if = "Vec::is_empty")]
       pub code_refs: Vec<CodeRef>,  // ← CODE links only (files/symbols)
       #[serde(flatten)]
       pub extra: BTreeMap<String, Value>,  // ← no ticket_ids or spec_ids
   }
   ```
   SpecManifest has `code_refs` for CODE links (file paths, symbols, line ranges) but NO `ticket_ids` or `related_tickets` field.

3. **Evidence from prior session** (corroborating that prose links rot):
   - 5 of 10 spec→ticket links used inconsistent relative paths and had to be hand-normalized
   - memory-api/.spec/.../body.md linked `../../.ticket/`, resolving to the WRONG store (nested-store bug)
   - A mapping session added 10 ticket→spec links and 8 spec sections, all unenforced prose

## Consequences

- **Validation gap**: Nothing validates these links, so they rot silently
- **Dangling references**: A moved, renamed, or deleted ticket/spec leaves a prose reference nobody detects
- **Coverage blind spot**: "Which requirements have no ticket?" and "Which tickets have no spec?" cannot be answered mechanically
- **Store mismatch hazard**: Prose paths can resolve to the wrong store (nested-store bug reproduced in real session)

This directly undercuts traceability: a prior session mapped 10 tickets to 8 specs, all of which is unenforced prose today.

# Goal

Add structured `ticket_ids: Vec<String>` field to SpecManifest and `spec_ids: Vec<String>` field to TicketManifest (via extra or top-level), plus a validation command that detects dangling links, wrong-store references, and missing cross-references.

# Scope

1. **Schema extension**:
   - Add `ticket_ids` or `related_tickets` field to spec.toml schema (SpecManifest)
   - Add `spec_ids` or `related_specs` field to ticket.toml schema (TicketManifest/EntityManifest)
   - Each link must identify the OWNING STORE (not just an id) — related ticket fb14754e makes the same point for handoff packages

2. **Validation command**:
   - Detect dangling ticket→spec references (ticket exists but spec is missing/moved/deleted)
   - Detect dangling spec→ticket references (spec exists but ticket is missing/moved/deleted)
   - Detect wrong-store references (link resolves to a different workspace/store than intended)
   - Report bidirectional consistency (ticket links spec but spec doesn't link ticket, or vice versa)

3. **Migration support**:
   - Document how to extract existing prose links into structured fields
   - Provide a migration script or manual procedure for the 10 tickets and 8 specs from the prior session

# Acceptance Criteria

- [ ] SpecManifest has a structured field (e.g. `related_tickets: Vec<TicketRef>` where `TicketRef` includes workspace/store identifier)
- [ ] TicketManifest (EntityManifest) has a structured field (e.g. `related_specs: Vec<SpecRef>` where `SpecRef` includes workspace/store identifier)
- [ ] Validation command exists: `ticket validate-links --workspace <ws>` and `spec validate-links --workspace <ws>`
- [ ] Validation detects: dangling ticket refs, dangling spec refs, wrong-store refs, bidirectional inconsistencies
- [ ] Migration guide documents how to convert existing prose links to structured fields
- [ ] Unit tests cover: serde round-trip, validation detection, cross-store scenarios
- [ ] Integration test reproduces the nested-store bug scenario and confirms the fix

# Proposed Approach

1. **Design phase**:
   - Define `TicketRef` struct with `workspace`, `store_root`, and `ticket_id` fields
   - Define `SpecRef` struct with `workspace`, `store_root`, and `spec_id` fields
   - Decide whether to extend EntityManifest with top-level field or use `extra` BTreeMap convention

2. **Implementation phase**:
   - Add structured link fields to both manifests
   - Extend TicketStore and SpecStore with link accessor methods
   - Implement validation command in ticket-cli and spec-cli
   - Add MCP tool: `validate_ticket_spec_links`

3. **Migration phase**:
   - Extract existing prose links from body.md/description.md files (grep + parse)
   - Update the 10 tickets and 8 specs from the prior session with structured links
   - Verify validation command catches no errors after migration

# Dependencies

Related ticket fb14754e-2be8-40a5-a995-488842ba6367 addresses the same store-ownership issue for handoff packages.

# Validation Plan

Fast: `cargo test -p ticket-api`, `cargo test -p spec-api` (manifest serde + validation tests)
Primary: `ticket validate-links --workspace default --json`, `spec validate-links --workspace default --json`
Manual: Create a nested-store scenario, confirm validation catches it
Failure logs: target/test-logs/

# Risk Assessment

Medium risk:
- Schema change affects existing ticket and spec manifests
- Migration requires careful extraction of prose links
- Bidirectional consistency enforcement may surface many existing inconsistencies
