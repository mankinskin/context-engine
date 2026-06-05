### Available ticket-mcp tools

| Tool | Required | Optional |
|------|----------|----------|
| `health` | — | — |
| `list_workspaces` | — | — |
| `list_tickets` | workspace | state, query, limit |
| `get_ticket` | workspace, id | — |
| `get_ticket_description` | workspace, id | — |
| `list_edges` | workspace | kind |
| `subgraph` | workspace, root | direction, edge_kind, depth, limit_nodes, limit_edges |
| `topgraph` | workspace, root | direction, edge_kind, depth, limit_nodes, limit_edges |
| `health_check` | workspace | root, all, ids, depth, direction |
| `update_ticket` | workspace, id | to_state, fields, field_map, from_state, undo, description, author |
| `close_ticket` | workspace, id | to_state |
| `cancel_ticket` | workspace, id | — |
| `workflow` | — | name, workspace, id, query |

`update_ticket` accepts sparse payloads: omit untouched optional keys entirely instead of sending placeholder values.