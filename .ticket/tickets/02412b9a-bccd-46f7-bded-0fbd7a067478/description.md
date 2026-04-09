# [AOH][Refinement] Reconcile AOH Architecture with Existing Phase 2 Execution Tickets

## Objective

Normalize the AOH planning tree so there is **one canonical implementation decomposition**, not a second parallel tree beside the existing Phase 2 execution-layer tickets.

This ticket exists because the current AOH planning set has introduced valuable design/refinement work, but the repository already contains implementation tickets under `d5ced7e2` for sandbox manager, assignment runner, notifier, TUI, review coordinator, and E2E execution. Starting implementation without reconciliation would create duplicate ownership and inconsistent done conditions.

## Concrete Problems to Resolve

### 1. Duplicate decomposition risk
The AOH design ticket (`34bc4938`) currently describes new Phase A–E implementation tracks, while `d5ced7e2` already has implementation children:
- `8c185de3` — execution provider contracts + Copilot API auth client
- `51471c3e` — sandbox manager
- `a8632357` — assignment runner
- `d0cc3c8b` — review coordinator
- `8db8ef2f` — notifier adapters
- `5af54f6c` — terminal UI
- `0135d961` — E2E integration / fault injection

### 2. Inconsistent naming/contracts across tickets
Examples already present:
- Branch naming differs between tickets (`aoh/{agent-id}/{ticket-slug}` vs `agent/{agent-id}/{ticket-id}/{slug}`)
- Some AOH research/design tickets still embed implementation acceptance criteria instead of design/research outputs
- Epic/design tickets and Phase 2 plan use different wording for the same components

### 3. Missing canonical mapping
There is not yet a single place that answers:
- Which existing implementation ticket owns which AOH component?
- Which AOH design/research ticket feeds which implementation ticket?
- Which tickets should be updated vs superseded vs closed as planning-only?

## Deliverables

### Canonical mapping matrix
Produce a table mapping:
- AOH design component
- Existing implementation ticket owner
- Required input tickets (research/design prerequisites)
- Remaining gaps needing new implementation tickets

### Naming and scope normalization
Decide and document one canonical form for:
- Branch naming
- Local PR identifier format
- Session/run naming
- Which tickets are planning-only vs implementation-owning

### Ticket hygiene pass
For the affected AOH tickets:
- remove stale contradictions to final ADRs
- rewrite planning tickets whose acceptance criteria currently require implementation work
- enrich the thin Phase 2 implementation tickets with AOH-specific descriptions and clear acceptance criteria

## Recommended Resolution Strategy

1. **Reuse existing Phase 2 implementation tickets** as the canonical implementation tree.
2. Treat current AOH tickets as research/design/refinement inputs that feed those implementation tickets.
3. Add or keep only genuinely new tickets where AOH introduces scope not covered by Phase 2.
4. Avoid creating a second Phase A–E implementation subtree.

## Acceptance Criteria

- [ ] Canonical component→implementation ticket mapping is documented
- [ ] One branch naming convention is selected and applied consistently across AOH tickets
- [ ] All AOH planning tickets are classified as research, design, refinement, or implementation-owning
- [ ] Thin Phase 2 implementation tickets are either enriched with descriptions or explicitly superseded
- [ ] No duplicate implementation ownership remains between AOH tickets and the `d5ced7e2` subtree
- [ ] Epic (`4e28bf38`) and architecture ticket (`34bc4938`) reference the reconciled implementation decomposition