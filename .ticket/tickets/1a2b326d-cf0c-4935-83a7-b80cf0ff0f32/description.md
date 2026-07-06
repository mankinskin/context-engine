# Goal
Eliminate ticket_graph findings by repairing dependency graph hygiene, orphan handling, and lifecycle consistency across all ticket stores.

# Planning Scope
Total findings in class: 258
Batch sequence:
1. context-engine store (125)
2. memory-api store (54)
3. viewer-api store (50)
4. memory-viewers store (23)
5. context-stack and context-editor plus misc stores (6)

# Implementation Strategy
- For each store batch, list failing ticket IDs and classify them by finding subtype.
- Apply the smallest valid correction: add missing dependency links, fix stale lifecycle states, or archive/cancel obsolete tickets.
- Keep tracker semantics: parent trackers depend on children and close last.

# Validation Plan
- ticket health for affected workspace store.
- ticket next sanity check for newly unblocked work.
- audit summary by category from repo root to verify ticket_graph reduction.

# Done Criteria
- ticket_graph findings reach zero or a clearly documented residual set with explicit blocker tickets.
- No dependency inversion introduced while fixing orphans.

# Workspace-policy prerequisite (handoff)
- Ticket-graph closure is additionally blocked on workspace policy implementation so fixture/test stores can be explicitly excluded from discovery/scan/query.
- Dependency gates:
  - `1a2b326d depends_on 65d5885b`
  - `edde88d6 depends_on 65d5885b`
- Required outcome before close: rerun ticket_graph audit with policy applied and confirm fixture-root leakage is not contributing to residual counts.

## Handoff Context For Implementation (authoritative)
- Implementation tracker: `65d5885b-ec09-450e-b6c8-1607ec3e51c3`.
- Child slices in order: `51d53f8f` (policy parser), `6312c5c4` (discovery filter), `eecbcee9` (scan-root metadata+scan enforcement), `42094bd4` (query guard), `c5ff717e` (CLI), `25677720` (regression tests).
- Current leakage source:
  - descendant + ancestor store auto-discovery in `discover_workspace_scan_roots`,
  - only hardcoded skip dirs, no explicit workspace allow/deny policy,
  - query roots based on stored scan roots unless filtered.
- Required design contract:
  - `.ticket/workspace-policy.toml` with `include_descendants`, `include_ancestors`, `ignore_workspaces`, `include_overrides`, `deny_external_paths`, `ignore_markers`.
  - enforce policy at discovery, scan, and query.
  - persist scan-root metadata (`source`, `policy_decision`, `workspace_root`) for auditability.
- Validation minimum before closing this prerequisite:
  - child included by default,
  - child ignored via marker,
  - child ignored via glob,
  - include override restores inclusion,
  - external path denied when `deny_external_paths=true`.