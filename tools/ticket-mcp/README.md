# ticket-mcp

MCP server that provides a thin wrapper around the ticket HTTP API (`ticket-http`).

The server runs on stdio and forwards requests to a running ticket API instance.

## Tools

- `help` - Shows supported ticket API operations and required parameters.
- `health` - Checks ticket API health.
- `list_workspaces` - Lists workspaces.
- `list_tickets` - Lists tickets in one workspace with optional filters.
- `get_ticket` - Gets a ticket by id.
- `get_ticket_description` - Gets markdown description for a ticket.
- `list_edges` - Lists edge graph rows, optionally filtered by kind.
- `subgraph` - Returns dependency subgraph rooted at a ticket.
- `workflow` - Returns ready-to-run tool call sequences for common tasks.
- `request` - Generic fallback operation router (kept for compatibility and edge cases).

## Supported operations

- `health`
- `list_workspaces`
- `list_tickets`
- `get_ticket`
- `get_ticket_description`
- `list_edges`
- `subgraph`

## Usage

```bash
# Start ticket API first
cargo run -p ticket-http -- --port 4000

# Then start MCP wrapper (defaults to http://127.0.0.1:4000)
cargo run -p ticket-mcp

# Custom API URL
TICKET_API_URL=http://127.0.0.1:4010 cargo run -p ticket-mcp
```

## VS Code MCP configuration

Add `ticket-mcp` to `.github/copilot-mcp-config.json`:

```json
{
  "mcpServers": {
    "ticket-mcp": {
      "type": "stdio",
      "command": "target/release/ticket-mcp.exe",
      "env": {
        "TICKET_API_URL": "http://127.0.0.1:4000"
      }
    }
  }
}
```

Then start Copilot with MCP config:

```bash
copilot --additional-mcp-config @.github/copilot-mcp-config.json
```

## Request tool input

`request` accepts:

- `operation`: one of the supported operations above
- Optional fields depending on operation: `workspace`, `id`, `state`, `query`, `limit`, `kind`, `root`, `direction`, `edge_kind`, `depth`, `limit_nodes`, `limit_edges`
- `base_url` (optional): override default API URL per call

Example:

```json
{
  "operation": "list_tickets",
  "workspace": "default",
  "state": "open",
  "limit": 50
}
```

## Recommended calling pattern

1. Call `help` or `workflow` first.
2. Use named tools (`list_tickets`, `get_ticket`, etc.) for normal usage.
3. Use `request` only when you need generic operation routing.
