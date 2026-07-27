## Problem

`tool_cost()` in memory-api/tools/mcp/mcp-cost-gate/src/gate.rs (lines 197-200) uses bidirectional substring matching between the requested tool name and rollup tool names:

```rust
name_low.contains(&tool_low) || tool_low.contains(&name_low)
```

This causes short rollup entries like `read` to match both `read_file` and `thread_safe_reader`, leading to false positive cost assignments when the rollup contains similarly-named tools.

## Pre-existing Behavior

This substring matching was present before ticket 9185d8f2 and is preserved in the current implementation. It is not a regression; it is an existing issue in the tool name resolution logic.

## Proposed Fix

Implement exact-match-first fallback strategy:
1. First, try exact match (case-insensitive) on tool names.
2. If no exact match, fall back to substring matching only if needed.
3. Consider namespace-aware matching to distinguish `peek_read` from `read_file` when both are present.

## Scope

- memory-api/tools/mcp/mcp-cost-gate/src/gate.rs, `tool_cost()` function.
- Add test cases covering exact-match precedence and namespace disambiguation.
- Update or add unit tests to verify the fix prevents the short-name collision.

## Non-goals

- Retuning the graded-cost calibration constants.
- Changing the rollup aggregation window.
