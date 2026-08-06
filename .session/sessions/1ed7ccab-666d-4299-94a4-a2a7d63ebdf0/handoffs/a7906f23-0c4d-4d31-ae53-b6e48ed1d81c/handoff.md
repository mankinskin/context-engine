# Handoff: a7906f23-0c4d-4d31-ae53-b6e48ed1d81c

## Summary
- **Workspace Session**: `1ed7ccab-666d-4299-94a4-a2a7d63ebdf0`
- **Outgoing Run**: `33389a76-566f-429e-a743-273af0d97871`
- **Created**: 2026-08-05T22:37:41.910029700+00:00
- **Objective**: Implement the directed inherited schema lifecycle engine for ticket 7ef3f8db, including strict single-parent resolution, category-contained plan/act/verify refinement, directed lifecycle validation, atomic generation reload, and focused invariant tests.
- **Implementation Ready**: true

## Resume Command
```bash
session-cli resume --workspace-session-id 1ed7ccab-666d-4299-94a4-a2a7d63ebdf0 --predecessor-run-id 33389a76-566f-429e-a743-273af0d97871
```

## Target Tickets
- `7ef3f8db-d4a9-4135-99eb-3c006070a328`

## Target Files
- `memory-api/crates/memory-api/src/model/schema.rs`
- `memory-api/crates/ticket-api/src/model/schema_registry.rs`
- `memory-api/crates/ticket-api/src/model/default_schema.rs`

## Decisions
- Use strict zero-or-one-parent inheritance; missing parents and parent cycles reject the complete reload atomically.
- Model lifecycle nodes with universal plan, act, and verify categories. Derived schema types refine categories only through contained tunnels with explicit permitted boundary transitions.
- Keep the directed lifecycle graph separate from ticket relation/dependency graphs; lifecycle category and direction rules never apply to relation edges.
- Validate one global plan entry, terminal behavior, resolved-node reachability, containment, skipped categories, illegal escapes, and only explicitly declared verify-to-act or act-to-plan rework loops.
- Treat schema type, concrete lifecycle state, lifecycle category, ticket relation, and validation gate as separate concepts with tests rejecting conflation.
- Atomically swap a new registry generation and invalidate resolved caches and client catalog versions after ancestor changes; retain the prior valid generation after a failed reload.

## Non-Goals
- Dual-format loading and JSON built-in conversion belong to 1f8e6e6d and abd3f280.
- Catalog-driven CLI and VS Code integration belongs to 9e7a5f1a.
- Legacy-ticket migration and release repair belong to 7df984eb and 3bb41fb2.
- Do not change ticket relation graph semantics or implement downstream client/migration workflows in the engine slice.

## Context Anchors
- Spec e9c38d24-42cc-4044-8b2c-6811b918530f documents the complete ten-decision contract and maps each decision to an implementation track.
- Epic 8fdfe135-e3b1-4876-b638-24154edcd78d is healthy with zero scoped health findings.
- Independent dry-run review passed after amendments, with no open decisions or planning findings.
- The transcript comparison source is c:/Users/linus/AppData/Roaming/Code/User/workspaceStorage/8c555ca5a1e8e713994831dc7a55167b/GitHub.copilot-chat/transcripts/1ed7ccab-666d-4299-94a4-a2a7d63ebdf0.jsonl.

## Risk Notes
The engine slice changes a shared schema model and registry boundary. Preserve additive compatibility where feasible, isolate migration/client changes to downstream tickets, and validate declared rework loops rather than banning every cycle.

## Workflow
- **Nodes**: 0
- **Edges**: 0
- **Not Done**: 0
