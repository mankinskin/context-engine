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