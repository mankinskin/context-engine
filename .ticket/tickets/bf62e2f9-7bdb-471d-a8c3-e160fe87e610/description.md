## Problem

`apply_manifest_update` in memory-api/crates/ticket-api/src/storage/store.rs (~L548-578) unconditionally calls `TicketFs::write_description(ticket_path, desc)` whenever `description` is `Some(..)`. There is no prior read, no merge, no warning. Agents that believe they are amending a description silently destroy the previous content.

## Decisions (interview-resolved)

- Add an explicit `mode` parameter with values `replace` and `append`, defaulting to `replace`.
- Independently of `mode`, ALWAYS capture the pre-overwrite description in ticket history so any overwrite is recoverable. `ticket-api` already calls `append_history` elsewhere; reuse it.

## Notes

Applies to the `update` / `update_with_options` paths (~L396-470) which both funnel through `apply_manifest_update`. Surface `mode` through the MCP `update_ticket` tool and the `ticket` CLI.

## Implementation notes (2026-07-28)

Added `DescriptionUpdateMode` enum (`Replace` default / `Append`) to `ticket-api`. `TicketStore::update_with_options` now takes `description_mode: DescriptionUpdateMode`; `apply_manifest_update` applies replace-or-append and returns the pre-update description text. `update_with_options` always inserts that pre-update text under the new public `DESCRIPTION_HISTORY_KEY` ("__previous_description__") into the history-revision fields whenever `description` is `Some(..)`, regardless of mode — so it's captured in `history.ndjson` on every description change. `TicketStore::apply_revert` (the undo primitive) restores `description.md` from that key when present, and strips the key from the manifest patch before writing `ticket.toml`. The simple `update()` wrapper is unchanged (still `Option<&str>` description, always mode=Replace internally) since none of its callers ever pass a description.

Files touched:
- memory-api/crates/ticket-api/src/storage/store.rs — `DescriptionUpdateMode` enum, `DESCRIPTION_HISTORY_KEY` const, `update_with_options`/`apply_manifest_update` signature + logic changes.
- memory-api/crates/ticket-api/src/storage/store/lifecycle.rs — `apply_revert` restores description from history key.
- memory-api/crates/ticket-api/src/storage/mod.rs, memory-api/crates/ticket-api/src/lib.rs — re-export `DescriptionUpdateMode`, `DESCRIPTION_HISTORY_KEY`.
- memory-api/crates/ticket-api/src/storage/tests/mod.rs — import new exports for tests.
- memory-api/crates/ticket-api/src/storage/tests/update_regression_tests.rs — 2 existing `update_with_options` calls updated with the new arg; 5 new tests added (see below).
- memory-api/tools/cli/ticket-cli/src/cli/args/operations.rs — `--description-mode` flag (default "replace") on `UpdateArgs`.
- memory-api/tools/cli/ticket-cli/src/cli/commands/crud.rs — `cmd_update` parses `--description-mode`, forwards mode; undo branch merges the description-history key from the newest revision into the fields passed to `apply_revert`.
- memory-api/tools/http/ticket-http/src/serve/handlers/tickets/types.rs — `description_mode: Option<String>` on `UpdateTicketBody`.
- memory-api/tools/http/ticket-http/src/serve/handlers/tickets/mutations.rs — parses `description_mode` (400 on invalid value), forwards mode.
- memory-api/tools/mcp/ticket-mcp/src/server/types.rs — `description_mode: Option<String>` on `UpdateTicketInput`.
- memory-api/tools/mcp/ticket-mcp/src/server/mutations.rs — `update_ticket_tool` parses `description_mode` (McpError::invalid_params on invalid value), forwards mode; `undo_ticket_update` merges the description-history key the same way as the CLI.
- memory-api/tools/mcp/ticket-mcp/src/server/workflow.rs — added `description_mode` to `update_ticket`'s documented optional params.

Tests added (memory-api/crates/ticket-api/src/storage/tests/update_regression_tests.rs):
- `update_without_description_preserves_existing_description` — regression test: a field-only update with `description: None` leaves `description.md` untouched.
- `update_with_replace_mode_overwrites_description` — explicit `Replace` overwrites.
- `update_with_append_mode_concatenates_description` — explicit `Append` concatenates old + new with a newline separator.
- `update_captures_previous_description_in_history_regardless_of_mode` — asserts the pre-update text is written to the history revision under `DESCRIPTION_HISTORY_KEY`.
- `undo_restores_previous_description` — asserts `apply_revert` restores the pre-overwrite description text.

Validation: `rtk cargo test -p ticket-api` → `cargo test: 133 passed (5 suites, 241.58s)`. Also `rtk cargo build -p ticket-cli -p ticket-http -p ticket-mcp` → `cargo build: 0 errors, 2 warnings (4 crates)` (both warnings pre-existing/unrelated: `EdgeBody::reason` dead code, `parse_uuid_field` unused).

Spec: recon bundle reported "none found" for a spec covering ticket update/description semantics; none was created or cited, per instruction not to fabricate one.