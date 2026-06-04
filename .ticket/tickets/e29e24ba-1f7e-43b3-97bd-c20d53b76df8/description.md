## Goal
Make MCP update tools accept sparse payloads that include only the keys being changed, and return minimal response payloads that include only changed or directly relevant fields.

## Why
In the previous turn, MCP update calls failed or became awkward because the tool surface effectively encouraged sending placeholder values for fields that were not being updated. That is the opposite of token-efficient behavior.

For token-efficient agent workflows:
- request payloads should include only the keys actually being changed
- responses should avoid echoing unchanged, unrelated, or default fields
- transport adapters should not require callers to send no-op values just to satisfy shape expectations

## Problem statement
Current MCP update flows are too verbose and too rigid:
1. callers can be pushed toward sending placeholder values for untouched fields
2. update responses may include unrelated fields or full field maps even when only one field changed
3. this inflates token usage and makes the effective patch semantics less clear than they should be

## References
- Last-turn failure mode came from MCP ticket update usage where unchanged state/field placeholders were sent unnecessarily.
- Existing minimal-output normalization ticket: `20b6a09a` — omit default workspace/schema metadata from ticket outputs
- Ticket MCP update surface: `memory-viewers/memory-api/tools/mcp/ticket-mcp/**`
- Spec MCP update surface: `memory-viewers/memory-api/tools/mcp/spec-mcp/**`
- Rule MCP update surface: `memory-viewers/memory-api/tools/mcp/rule-mcp/**`
- Existing MCP tool contract patterns: `memory-viewers/memory-api/tools/mcp/audit-mcp/src/server.rs`, `memory-viewers/memory-api/tools/mcp/rule-mcp/src/server.rs`
- Token-efficiency guidance: `.agents/instructions/token-efficiency.instructions.md`

## Scope
Apply sparse-update and minimal-response semantics across existing MCP update tools.

Primary targets:
- ticket MCP update tools
- spec MCP update tools
- rule MCP update tools

Required outcomes:
- MCP input schemas allow omitted keys for unchanged values
- update handlers distinguish absent fields from explicit updates
- update responses return only changed/relevant fields, not full unrelated field maps
- transport-level docs/examples show sparse request payloads as the canonical usage

## Implementation plan
1. Inventory current update-style MCP tools and their input schemas.
   - identify which fields are truly required
   - identify where optional fields are treated as if they must be sent
2. Review update handler behavior in the backing API/store layer.
   - confirm whether absent fields already mean “no change”
   - identify adapter code that forces full-field envelopes or echoes too much state
3. Redesign MCP input shapes so callers can omit untouched fields entirely.
   - no placeholder `"."`, empty arrays, or synthetic no-op values needed
   - preserve explicit clearing semantics where needed by distinguishing `absent` from `present but empty/null`
4. Redesign MCP update responses to return only:
   - identifier/path metadata needed for follow-up
   - changed fields
   - state transition info when applicable
   - any directly relevant audit/revision metadata
5. Keep backward compatibility where practical.
   - if full payload mode is required for compatibility, make sparse/minimal mode the default and full mode opt-in
6. Update docs and examples for MCP update calls to show sparse request bodies.
7. Add focused regression tests for sparse request handling and minimal response envelopes.

## Acceptance criteria
- MCP update tools accept payloads containing only changed keys.
- Callers do not need to send unchanged fields or placeholder values.
- Responses omit unchanged, unrelated, or default fields unless explicitly requested.
- Sparse update semantics are consistent across ticket/spec/rule MCP surfaces where applicable.
- Documentation examples use sparse update payloads by default.

## Validation notes
Required validation:
- focused tests for `ticket-mcp` update behavior
- focused tests for `spec-mcp` update behavior
- focused tests for `rule-mcp` update behavior
- verify one-field update requests succeed when all other fields are omitted
- verify responses exclude unrelated unchanged fields

Suggested commands:
- `cargo test -p ticket-mcp`
- `cargo test -p spec-mcp`
- `cargo test -p rule-mcp`
- if API-layer changes are involved, also run relevant `ticket-api` / `spec-api` / `rule-api` tests

## Design notes / risks
- Distinguish clearly between:
  - field omitted → no change
  - field present with empty/null value → explicit clear/reset
- Avoid adapter-only hacks; the contract should be explicit in schemas and handler logic.
- Keep this aligned with `20b6a09a`: minimal requests and minimal responses should reinforce each other.
- If one store surface cannot support sparse semantics cleanly yet, document that gap explicitly instead of forcing noisy placeholder payloads.