## Problem

`session_handoff` persists `objective`, `target_tickets`, `target_files`, `decisions`, `validation`, `non_goals`, `context_anchors`, `open_escalations`, `risk_notes`, `predecessor_handoff`. None of these encode the next steps. The most important part of a handoff — the plan the next session should execute — is missing, so each session restarts planning from scratch.

The durable workflow graph (`session_workflow_add_node` / `add_edge` / `set_status`) already has sufficient primitives: `kind="task"` nodes, `category` labels, `order` / `depends-on` edges, per-node status. What is missing is the link and the materialization path.

## Decisions (interview-resolved)

- Store an **embedded snapshot of nodes + edges** in the handoff record, so the handoff is self-contained and readable without the session store.
- The step graph is **required** for a package to be implementation-ready.
- Nodes and target tickets are **interchangeable and independent**: a caller may supply workflow nodes, target ticket ids, or both. There can be nodes without tickets and tickets without nodes. The system validates that supplied ids exist and materializes both into the handoff.
- A node may carry content distinct from a ticket id, and may qualify the operation on a ticket id (e.g. "review", "implement").
- Ordering / "what comes next" is expressed via the existing `order` and `depends-on` edges. No new `next_step_node_id` field.
- The step graph lives in the session store only. Do NOT mirror it onto target tickets.

## Spec

Update the existing reviewed spec **5e52039d Handoff Package Schema** (`.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/`, linked ticket d3af78d7). Do not create a new spec.