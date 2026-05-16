---
description: "Create a single new ticket from the slash-command text. Prefer ticket-mcp tools and fall back to ticket.exe when needed."
name: "ticket"
argument-hint: "<your content>"
agent: "agent"
---

# Create Single Ticket

Create a single new ticket from the user's current slash-command request using the ticket-api flow.

Reference [ticket-cli](../../memory-viewers/memory-api/tools/cli/ticket-cli/README.md) and [ticket-mcp](../../memory-viewers/memory-api/tools/mcp/ticket-mcp/README.md).

Install or build the ticket tools when needed:
- Build the CLI in this workspace with `cargo build -p ticket-cli --bin ticket` and use `./target/debug/ticket.exe`.
- Install the CLI onto your Cargo bin path with `cargo install --path memory-viewers/memory-api/tools/cli/ticket-cli --bin ticket`.
- Run the MCP server with `cargo run -p ticket-mcp` when MCP access needs to be configured locally.

Workflow:
1. Treat the text typed after `/ticket` as the source request.
2. Search existing tickets first with `list_tickets`, `get_ticket_description`, `ticket search`, or `ticket list` so you do not create duplicates.
3. Prefer `ticket-mcp` tools such as `list_tickets`, `get_ticket_description`, `create_ticket`, and `workflow` when they are available.
4. If `ticket-mcp` is unavailable, fall back to `./target/debug/ticket.exe search`, `./target/debug/ticket.exe list`, and `./target/debug/ticket.exe create`; use `--index-root` when the intended `.ticket` store is not the nearest one.
5. Infer the best single ticket title, type, priority, and initial state from the request. Keep the result scoped to one actionable work item.
6. When the prompt includes enough detail, add a useful initial description covering motivation, scope, constraints, and acceptance criteria.
7. If a matching ticket already exists, do not create a duplicate. Return the existing ticket instead.
8. Ask one concise clarification if the target store, scope, or ticket shape is still ambiguous after a focused search.
9. Do not split the request into multiple tickets unless the user explicitly asks; `/ticket` should create one ticket.
10. Do not change unrelated tickets, edges, or board state unless the user explicitly asks.
11. In markdown outputs, reference the ticket by its folder path instead of a bare id. Use the folder path as the markdown link text and target; for example, `[.ticket/tickets/<full-uuid>/](.ticket/tickets/<full-uuid>/) - Ticket title` when the default store is used.

Response:
- created or matched ticket folder path and title, rendered as a markdown link named by the folder path instead of a raw id
- chosen type, priority, and state
- duplicate candidates considered, if any
- assumptions that still matter
