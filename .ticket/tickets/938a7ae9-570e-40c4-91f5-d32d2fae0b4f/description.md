FULL metrics rollup: session count, status, aggregate tool/cost metrics, duration, token cost, per-agent breakdown.

**Code anchors**: memory-api/crates/session-api/src/subagent_rollup.rs, memory-api/crates/session-api/src/tool_metrics.rs

**Acceptance**: Rollup returns session count, status distribution, tool/cost aggregates, duration, per-agent breakdown.

**Batch 1 validation bar** (shared with 185a00a2, 9cd9440e): End-to-end session lifecycle test covering check-in → handoff write → finish → reload, asserting on-disk layout and track fields, PLUS MCP and CLI round-trip through new track query surface. Base commands: `cargo test -p session-api && cargo test -p session-mcp`, plus memory-kernel board tests.