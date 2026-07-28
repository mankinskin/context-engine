# Handoff: ticket description/spec body mutability guard fixes

## Scope
Pairwise implementation unit for the silent-data-loss defects in ticket description updates and spec body updates. This session hands off the next implementation slice only; the prior mcp-cost-gate work is background and complete.

## Durable Session Identity
- workspace_session_id: `6a51a1af-6812-4dfc-80d7-0e4f56b4af4f`
- outgoing_run_id: `22633ff8-ab1e-43e6-9940-7ebdb09f50ee`
- handoff_id: `0b14eae5-208b-4e9a-8dd9-6ed31239f03f`
- resume_command: `session-cli resume --workspace-session-id 6a51a1af-6812-4dfc-80d7-0e4f56b4af4f --predecessor-run-id 22633ff8-ab1e-43e6-9940-7ebdb09f50ee`
- predecessor_handoff: `ea1e2d33-99a7-484a-9833-8de72595dbc3`

## Epic
- [2558a279-8819-4682-8db5-c2a4aa30aa0e]([.ticket/tickets/2558a279-8819-4682-8db5-c2a4aa30aa0e/ticket.toml](.ticket/tickets/2558a279-8819-4682-8db5-c2a4aa30aa0e/ticket.toml)) — workflow and tooling reliability umbrella. This unit is one of its frontier leaves.

## Done
- Collected the authoritative ticket/spec anchors and verified the concrete file paths for the next implementation slice.
- Confirmed the current workspace session has no open escalations for this unit.
- Persisted this handoff record under the session store layout for cold-start resumption.

## Decisions
- Treat the ticket description fix and the spec body fix as the same defect class: silent data loss through mutation APIs that do not make overwrite semantics explicit.
- Default ticket description updates to replace mode, with append as an explicit opt-in that concatenates instead of overwriting.
- Always write the pre-update ticket description into history before the overwrite/append mutation, so undo and history inspection can recover the previous text.
- Reject empty spec body updates unless force is explicitly set, and reject byte-identical no-op updates so a successful spec write guarantees a real change.
- Expose the new semantics through both the underlying crates and the CLI/MCP surfaces, not only in storage helpers.

## Blockers
- None. `open_escalations` is intentionally empty.
- The only execution caveat is that the next session should keep unrelated dirty files out of scope.

## Remaining
1. Implement bf62e2f9 in the ticket-api storage path and expose `mode=replace|append` through the ticket MCP/CLI surfaces.
2. Implement f986e666 in spec-api storage and expose the empty/no-op rejection behavior through the spec MCP/CLI surfaces.
3. Add or update the focused regression tests that prove replace, append, history capture, empty rejection, force override, no-op rejection, and normal-update behavior.
4. Run the validation commands below and report the results back before any ticket transition.

## Target Tickets
- [bf62e2f9-7bdb-471d-a8c3-e160fe87e610 [ticket-api] Add explicit replace/append mode to description update and always capture pre-overwrite description in history](.ticket/tickets/bf62e2f9-7bdb-471d-a8c3-e160fe87e610/ticket.toml) — state new. Acceptance criteria: update_ticket accepts mode=replace|append, defaulting to replace; append concatenates rather than overwriting; the pre-update description is written to ticket history on every description change regardless of mode; an overwritten description is recoverable via history/undo; unit tests cover replace, append, and history capture; MCP tool and CLI both expose mode
- [f986e666-d8db-4845-ba86-eb4bb89484ce [spec-api] Reject empty and no-op spec body updates so a successful update guarantees a change](.ticket/tickets/f986e666-d8db-4845-ba86-eb4bb89484ce/ticket.toml) — state new. Acceptance criteria: spec update_body rejects empty content unless an explicit force flag is set; update_body rejects content byte-identical to the existing body as a no-op error; both rejections return actionable error messages distinguishing empty from no-op; a successful update_body guarantees the stored body changed; unit tests cover empty-rejected, empty-forced, no-op-rejected, and normal-update paths; rejection surfaces through the MCP spec update tool and CLI

## Target Files
- [memory-api/crates/ticket-api/src/storage/store.rs](memory-api/crates/ticket-api/src/storage/store.rs)
- [memory-api/crates/ticket-api/src/storage/ticket_fs.rs](memory-api/crates/ticket-api/src/storage/ticket_fs.rs)
- [memory-api/crates/ticket-api/src/storage/tests/update_regression_tests.rs](memory-api/crates/ticket-api/src/storage/tests/update_regression_tests.rs)
- [memory-api/tools/mcp/ticket-mcp/src/server/mutations.rs](memory-api/tools/mcp/ticket-mcp/src/server/mutations.rs)
- [memory-api/tools/cli/ticket-cli/src/cli/commands/crud.rs](memory-api/tools/cli/ticket-cli/src/cli/commands/crud.rs)
- [memory-api/crates/spec-api/src/store.rs](memory-api/crates/spec-api/src/store.rs)
- [memory-api/crates/spec-api/src/error.rs](memory-api/crates/spec-api/src/error.rs)
- [memory-api/crates/spec-api/src/store/tests.rs](memory-api/crates/spec-api/src/store/tests.rs)
- [memory-api/tools/mcp/spec-mcp/src/server/sections.rs](memory-api/tools/mcp/spec-mcp/src/server/sections.rs)
- [memory-api/tools/cli/spec-cli/src/cli/commands/crud.rs](memory-api/tools/cli/spec-cli/src/cli/commands/crud.rs)

## Context Anchors
- Epic [2558a279-8819-4682-8db5-c2a4aa30aa0e]([.ticket/tickets/2558a279-8819-4682-8db5-c2a4aa30aa0e/ticket.toml](.ticket/tickets/2558a279-8819-4682-8db5-c2a4aa30aa0e/ticket.toml)) is the parent workflow-and-tooling reliability track; both tickets are frontier leaves on that epic.
- The current spec reference for caller-model tolerance, [9f0b9e30-e32c-4092-b2a2-68179141cfc4]([.spec/specs/9f0b9e30-e32c-4092-b2a2-68179141cfc4/spec.toml](.spec/specs/9f0b9e30-e32c-4092-b2a2-68179141cfc4/spec.toml)), is background only for the prior unit and should not be modified by this next slice.
- Ticket-api update path: [memory-api/crates/ticket-api/src/storage/store.rs](memory-api/crates/ticket-api/src/storage/store.rs) applies manifest updates and currently writes descriptions unconditionally when present; [memory-api/crates/ticket-api/src/storage/ticket_fs.rs](memory-api/crates/ticket-api/src/storage/ticket_fs.rs) owns the on-disk description/history helpers.
- Spec-api update path: [memory-api/crates/spec-api/src/store.rs](memory-api/crates/spec-api/src/store.rs) currently accepts empty and no-op body updates; [memory-api/crates/spec-api/src/error.rs](memory-api/crates/spec-api/src/error.rs) already defines the no-op/empty error shape.
- CLI and MCP surfaces that expose the same semantics live in [memory-api/tools/cli/ticket-cli/src/cli/commands/crud.rs](memory-api/tools/cli/ticket-cli/src/cli/commands/crud.rs), [memory-api/tools/mcp/ticket-mcp/src/server/mutations.rs](memory-api/tools/mcp/ticket-mcp/src/server/mutations.rs), [memory-api/tools/cli/spec-cli/src/cli/commands/crud.rs](memory-api/tools/cli/spec-cli/src/cli/commands/crud.rs), and [memory-api/tools/mcp/spec-mcp/src/server/sections.rs](memory-api/tools/mcp/spec-mcp/src/server/sections.rs).
- Review of the current workspace indicates no open escalations for this next unit and no need to revisit the prior mcp-cost-gate changes.

## Validation
- `rtk cargo test -p ticket-api`
- `rtk cargo test -p spec-api`
- `rtk cargo test -p ticket-cli`
- `rtk cargo test -p spec-cli`

## Delegated
- None yet. The next session should dispatch an Implement Agent once the package is loaded.

## Completion Status
- bf62e2f9: not-started — description update path still needs the replace/append split and history capture.
- f986e666: not-started — spec body update still needs empty/no-op rejection plus force handling.
- MCP/CLI surface updates: not-started — the same semantics must be surfaced through both control planes.
- Validation: not-started — run the four crate tests after the edits land.

## Definition of Done
- Ticket descriptions no longer silently overwrite previous text without an explicit mode.
- Spec body updates no longer succeed without changing the stored body unless force is explicitly requested for the empty case.
- Both MCP and CLI surfaces expose the new semantics.
- Focused regression tests pass and the next session can move the tickets forward with confidence.

## Remaining Risk
- Unrelated dirty files are still present in the worktree; they must not be swept into this unit's commit.
