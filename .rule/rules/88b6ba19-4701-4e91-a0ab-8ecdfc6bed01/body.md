### Available ticket-mcp tools

| Tool | Required | Optional |
|------|----------|----------|
| `health` | — | — |
| `list_workspaces` | — | — |
| `list_tickets` | — | workspace, state, query, limit |
| `get_ticket` | id | workspace |
| `get_ticket_description` | id | workspace |
| `list_edges` | — | workspace, kind |
| `subgraph` | root | workspace, direction, edge_kind, depth, limit_nodes, limit_edges |
| `topgraph` | root | workspace, direction, edge_kind, depth, limit_nodes, limit_edges |
| `health_check` | — | workspace, root, all, ids, depth, direction |
| `update_ticket` | id | workspace, to_state, fields, field_map, from_state, undo, description, author |
| `close_ticket` | id | workspace, to_state |
| `cancel_ticket` | id | workspace |
| `workflow` | — | name, workspace, id, query |

When omitted, `workspace` defaults to the active `default` workspace.

`update_ticket` accepts sparse payloads: omit untouched optional keys entirely instead of sending placeholder values.
