## Problem

`apply_manifest_update` in memory-api/crates/ticket-api/src/storage/store.rs (~L548-578) unconditionally calls `TicketFs::write_description(ticket_path, desc)` whenever `description` is `Some(..)`. There is no prior read, no merge, no warning. Agents that believe they are amending a description silently destroy the previous content.

## Decisions (interview-resolved)

- Add an explicit `mode` parameter with values `replace` and `append`, defaulting to `replace`.
- Independently of `mode`, ALWAYS capture the pre-overwrite description in ticket history so any overwrite is recoverable. `ticket-api` already calls `append_history` elsewhere; reuse it.

## Notes

Applies to the `update` / `update_with_options` paths (~L396-470) which both funnel through `apply_manifest_update`. Surface `mode` through the MCP `update_ticket` tool and the `ticket` CLI.