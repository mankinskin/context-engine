## Objective

`ticket-api`'s `SchemaRegistry::with_builtins()` (memory-api/crates/ticket-api/src/model/default_schema.rs#L12-L17) registers exactly 5 built-in types: `tracker-improvement`, `bug`, `task`, `epic`, `feature`. There is no schema for the type string `"ticket"`.

## Impact (verified 2026-08-04)

Any ticket with `type = "ticket"` cannot be transitioned at all: `update --to-state <x>` and `close` fail with `store error: no schema for type 'ticket'`, and `ticket.exe transitions <id>` fails identically, so the store cannot even enumerate legal next states for that type.

On 2026-08-04, 4 tickets store-wide had `type = "ticket"` (verified via `grep -l 'type *= *"ticket"' .ticket/tickets/*/ticket.toml | wc -l`). All 4 were retyped to `"task"` as an immediate workaround (see resolution below) — 0 tickets currently have `type = "ticket"` in the store, but the root cause (no schema registered for that type string) is unfixed and will recur the next time any agent or workflow creates a ticket with `type = "ticket"`.

## A second latent defect discovered during the workaround

None of the 5 registered schemas (`tracker-improvement`, `bug`, `task`, `epic`, `feature`) define a `"ready"` state, even though `"ready"` is a state value already in active use across the ticket store (it's referenced in `.agents/instructions` and was the actual on-disk state of the 3 tickets this incident affected). Retyping a ticket sitting in `state = "ready"` to any of the 5 registered types leaves it unable to transition anywhere (`allowed_next_states: []`), because `"ready"` isn't a valid state in any registered schema's state list. This ticket's resolution had to route through a temporary `--schema-dir` override adding `ready -> in-review` to the `task` schema (not committed, deleted after use) to unblock the 3 affected tickets.

## Acceptance Criteria

1. `ticket-api` registers a built-in schema for type `"ticket"` (or an equivalent decision is recorded to retire the type string entirely and document that it must never be used), modeled on an existing registered schema's state machine.
2. A regression test asserts that a ticket with `type = "ticket"` can be transitioned through its schema's states, and that `ticket.exe transitions` succeeds for that type.
3. Decide whether `"ready"` should be added as a first-class state (with appropriate transitions) to the shared state vocabulary used by the built-in schemas, given it is already in active use store-wide; if not, document why and where `"ready"`-stated tickets should route instead.
4. `SchemaRegistry::with_builtins()` and `ticket health` continue to flag any other type strings present in the store without a registered schema (the `unknown_type` health check already does this — confirm it still fires correctly after the fix).

## Evidence

- Root cause: memory-api/crates/ticket-api/src/model/default_schema.rs#L12-L17 (5-entry `DELIVERED_SCHEMA_TOML` list, no `"ticket"` entry)
- `unknown_type` health check message: memory-api/crates/ticket-api/src/health.rs#L166
- Confirmed via `mcp_ticket-mcp_health_check` on ticket `bba9b313-ff13-4fd1-91d4-6485a6c2f4de` before its retype: `"check":"unknown_type","severity":"error","message":"Ticket type 'ticket' has no registered schema; transitions and validation will fail."`
- Tickets affected and worked around: `bba9b313-ff13-4fd1-91d4-6485a6c2f4de`, `33463861-ffba-4ead-905e-5d867b707936`, `7c74f2fe-2bfd-477c-847e-bc02200a4819`, `1ff57502-ad4e-4c40-a852-18752c18f44c` — all retyped `ticket` -> `task` on 2026-08-04.
