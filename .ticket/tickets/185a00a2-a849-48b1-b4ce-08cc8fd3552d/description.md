MCP tools for querying sessions by track_id and track-scoped rollup.

**Code anchors**: memory-api/tools/mcp/session-mcp/src/server.rs

**Acceptance**: MCP tools can query by track_id and return rollup metrics.

**Batch 1 validation bar** (shared with 938a7ae9, 9cd9440e): End-to-end session lifecycle test covering check-in → handoff write → finish → reload, asserting on-disk layout and track fields, PLUS MCP and CLI round-trip through new track query surface. Base commands: `cargo test -p session-api && cargo test -p session-mcp`, plus memory-kernel board tests.