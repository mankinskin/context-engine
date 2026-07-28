## Problem

`update_body(&self, id_or_slug, content: &str)` in memory-api/crates/spec-api/src/store.rs (~L692-699) calls `write_body(&indexed.path, content)` with no guard and returns `Ok(())` whether `content` is empty or byte-identical to the existing body. In complex workflows this forces agents to re-read the spec just to confirm anything actually changed.

## Decisions (interview-resolved)

- Empty content: reject, but allow it through an explicit force flag for the rare intentional case.
- Byte-identical content: reject as a no-op error. A successful call must mean something changed.

## Notes

Q5 was answered "no": do NOT update .agents/instructions/spec/spec-system.instructions.md as part of this ticket.

## Implementation (done)

- Added `SpecError::EmptyBody(String)` and `SpecError::NoOpUpdate(String)` variants (memory-api/crates/spec-api/src/error.rs).
- `SpecStore::update_body` now takes a `force: bool` param, rejects empty content unless `force`, reads the existing body via `read_body` and rejects byte-identical content as a no-op (memory-api/crates/spec-api/src/store.rs).
- Threaded a `force_body` flag through all three transports and their one downstream caller:
  - CLI: `UpdateArgs.force_body` (`--force-body`) in memory-api/tools/cli/spec-cli/src/cli/args.rs; used in memory-api/tools/cli/spec-cli/src/cli/commands/crud.rs.
  - HTTP: `UpdateSpecRequest.force_body` in memory-api/tools/http/spec-http/src/handlers/specs.rs; new BAD_REQUEST arms (`spec.empty_body`, `spec.noop_update`) in memory-api/tools/http/spec-http/src/error.rs.
  - MCP: `UpdateSpecInput.force_body` in memory-api/tools/mcp/spec-mcp/src/server/types.rs; used in memory-api/tools/mcp/spec-mcp/src/server/query.rs; new `invalid_params` arms in memory-api/tools/mcp/spec-mcp/src/server.rs.
  - Fixed a downstream caller in memory-api/crates/memory-matrix/src/mcp/mcp_spec.rs that constructs `UpdateSpecInput` directly (added `force_body: false`).
- Updated existing test call site to pass `force: false` and added 4 new tests in memory-api/crates/spec-api/src/store/tests.rs: `update_body_rejects_empty_content_without_force`, `update_body_allows_empty_content_with_force`, `update_body_rejects_noop_content`, `update_body_succeeds_on_genuine_change`.
- `RuleStore::update_body` (memory-api/crates/rule-api) is a separate/analogous type and was intentionally left untouched — out of scope for this spec-api ticket.

## Validation

- `rtk cargo test -p spec-api` → 79 passed (4 suites), including the 4 new tests.
- `cargo build --workspace` inside memory-api → 0 errors.