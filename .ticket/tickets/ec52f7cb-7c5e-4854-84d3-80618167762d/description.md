# [Board] ticket-mcp: Board Tool Endpoints for Agent Coordination

## Purpose

Expose the draftboard as MCP tools so that agent sessions can coordinate through the MCP protocol without shelling out to the CLI. This is the primary machine interface for agent-to-board interaction during automated sessions.

## Component Boundaries

### In scope
- New MCP tools in `tools/mcp/ticket-mcp/` registered on the `ToolRouter`
- Tools: `board_show`, `board_check_in`, `board_check_out`, `board_heartbeat`, `board_configure`, `board_clean`
- JSON schema definitions for all tool parameters and return types
- Error mapping from `BoardError` → MCP error responses

### Out of scope
- Board storage logic (owned by `0db86ac1`)
- CLI output formatting (owned by `bcc111c6`)
- HTTP endpoint exposure (future work if ticket-http needs it)

## MCP Tool Definitions

### `board_show`
**Parameters:**
- `workspace` (string, required): Workspace name

**Returns:** `BoardSnapshot` — full board state with entries, config, counts, warnings, file ownership map.

**Usage:** Agent sessions call this at startup to orient: understand current WIP, find stale entries, discover file conflicts before starting work.

### `board_check_in`
**Parameters:**
- `workspace` (string, required)
- `ticket_id` (string, required): UUID or prefix
- `agent_id` (string, required): Caller's session identifier
- `intent` (string, optional): Description of planned work
- `files` (string[], optional): Files the agent will own
- `ttl_secs` (number, optional): Heartbeat TTL in seconds (default: 600)

**Returns:** `BoardEntry` on success. `BoardError` on WIP limit or file conflict.

**Usage:** Agent calls this immediately after selecting a ticket to work on.

### `board_check_out`
**Parameters:**
- `workspace` (string, required)
- `ticket_id` (string, required)
- `agent_id` (string, optional)

**Returns:** Success confirmation or `NotCheckedIn` error.

**Usage:** Agent calls this when finishing work on a ticket (session end, ticket state advanced, or abandoning).

### `board_heartbeat`
**Parameters:**
- `workspace` (string, required)
- `ticket_id` (string, required)
- `agent_id` (string, required)

**Returns:** Updated `BoardEntry` with refreshed TTL.

**Usage:** Long-running sessions call this periodically (recommended: every `ttl_secs / 2`).

### `board_configure`
**Parameters:**
- `workspace` (string, required)
- `max_wip` (number, optional)
- `stale_after_secs` (number, optional)

**Returns:** Current `BoardConfig` (after any updates).

### `board_clean`
**Parameters:**
- `workspace` (string, required)
- `include_stale` (boolean, optional, default: false)

**Returns:** `BoardCleanResult` with counts of removed entries.

## Agent Workflow Integration

Recommended agent session startup sequence:

```
1. board_show → read current state, check WIP budget
2. next_tickets → get unblocked work (filtered by board state)
3. board_check_in → register selected ticket + file ownership
4. ... do work, periodically board_heartbeat ...
5. board_check_out → release ticket on completion
```

## Acceptance Criteria

- [ ] `board_show` tool registered and returns `BoardSnapshot` JSON
- [ ] `board_check_in` validates inputs and returns `BoardEntry` or structured error
- [ ] `board_check_out` releases board entry and lease
- [ ] `board_heartbeat` updates TTL and returns refreshed entry
- [ ] `board_configure` reads/writes board configuration
- [ ] `board_clean` prunes entries and reports results
- [ ] All tools handle workspace resolution consistently with existing ticket MCP tools
- [ ] Error responses from `BoardError` are structured and actionable
- [ ] Tools appear in `mcp_ticket-mcp_help` output
- [ ] Integration test: full MCP tool cycle (show → check-in → heartbeat → check-out → show)
