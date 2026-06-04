## Goal
Introduce a proper `peek-api` library crate and move the current `peek-cli` logic behind the repository’s standard `*-api` layering so `peek-cli` and a new `peek-mcp` become thin transport adapters.

## Why
The current `peek` implementation is monolithic in `tools/cli/peek-cli/src/main.rs`, which makes the bounded-read and skeletonization logic hard to reuse from MCP or future HTTP/editor surfaces. This ticket establishes the same layering already used elsewhere in the repo: `<x>-api` owns the logic, `<x>-cli` owns clap/output, and `<x>-mcp` owns MCP tool translation.

## References
- Current monolithic CLI: `tools/cli/peek-cli/src/main.rs`
- Current CLI docs: `tools/cli/peek-cli/README.md`
- Current workspace member wiring: `Cargo.toml`
- Existing adapter pattern reference: `memory-viewers/memory-api/.rule/rules/4c0b0086-4846-4688-ba01-6e905b842185/body.md`
- MCP implementation reference: `memory-viewers/memory-api/tools/mcp/audit-mcp/src/server.rs`
- Rich named-tool MCP reference: `memory-viewers/memory-api/tools/mcp/rule-mcp/src/server.rs`
- Single execute-tool MCP reference: `context-stack/tools/mcp/context-mcp/src/server.rs`

## Scope
Create a reusable `peek-api` crate that owns:
- bounded file reads (`count`, `grep`, `window`, `head`, `tail`, `all`)
- skeletonization for supported file types
- request/response models for transport adapters
- filesystem validation and error mapping at the API layer

Then refactor adapters:
- `peek-cli` becomes clap parsing + human/text rendering only
- `peek-mcp` becomes named MCP tools delegating to `peek-api`

## Proposed crate layout
- `tools/peek-api/` or `crates/peek-api/` library crate
- `tools/cli/peek-cli/` remains binary adapter
- `tools/mcp/peek-mcp/` new stdio MCP server

Preferred internal modules:
- `peek-api/src/lib.rs`
- `peek-api/src/error.rs`
- `peek-api/src/types.rs`
- `peek-api/src/read.rs`
- `peek-api/src/skeleton.rs`
- `peek-api/src/fs.rs`

## Implementation plan
1. Extract current pure logic from `tools/cli/peek-cli/src/main.rs` into `peek-api` functions with structured inputs/outputs.
2. Define stable request/response types for:
   - bounded read
   - grep-only search
   - count-only inspection
   - skeleton render
3. Move file opening, line loading, and argument validation into `peek-api` so transports share behavior.
4. Refactor `peek-cli` to:
   - keep clap parsing
   - translate CLI args into `peek-api` requests
   - render text output from `peek-api` responses
5. Add a new `peek-mcp` crate exposing named MCP tools such as:
   - `peek_read`
   - `peek_grep`
   - `peek_count`
   - `peek_skeleton`
6. Keep the transport layer thin:
   - no duplicated parsing/inspection logic in MCP
   - MCP only maps JSON inputs to `peek-api` and serializes results
7. Update workspace membership and docs.

## Acceptance criteria
- `peek-api` owns all bounded-read and skeletonization logic currently embedded in `peek-cli`.
- `peek-cli` becomes a thin adapter and preserves current user-facing behavior.
- `peek-mcp` exists and exposes a stable MCP tool surface for the same core operations.
- Error behavior is consistent across CLI and MCP for missing files, invalid ranges, and unsupported modes.
- The crate structure and docs clearly follow the repo’s standard `*-api` layering.

## Validation notes
Required validation before moving beyond implementation:
- `cargo test -p peek-api`
- `cargo test -p peek-cli`
- `cargo test -p peek-mcp`
- `cargo build -p peek-cli -p peek-mcp`

Recommended focused tests:
- range validation parity between CLI and API
- grep result line-number stability
- skeleton output parity before/after extraction
- MCP tool contract tests for invalid path / invalid line ranges / success cases

## Risks / design notes
- Do not let MCP invent a second command model separate from the API request/response types.
- Keep file-system and formatting concerns separated so future HTTP/editor adapters can reuse the same API crate.
- Preserve bounded-read defaults; do not regress toward whole-file reads by default.