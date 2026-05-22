---
description: "Create a single new ticket from the slash-command text using the ticket-api flow, then update the related spec when requirements or goals change."
name: "ticket"
argument-hint: "<your content>"
agent: "agent"
---

# Create Single Ticket

Create a single new ticket from the user's current slash-command request using the ticket-api flow.
Follow the repository workflow: ticket first, spec second, implementation later.

Reference [ticket-cli](../../memory-viewers/memory-api/tools/cli/ticket-cli/README.md) and [ticket-mcp](../../memory-viewers/memory-api/tools/mcp/ticket-mcp/README.md).

Install or build the ticket tools when needed:
- Build the CLI in this workspace with `cargo build -p ticket-cli --bin ticket` and use `./target/debug/ticket.exe`.
- Install the CLI onto your Cargo bin path with `cargo install --path memory-viewers/memory-api/tools/cli/ticket-cli --bin ticket`.
- Run the MCP server with `cargo run -p ticket-mcp` when MCP access needs to be configured locally.

Workflow:
1. Treat the text typed after `/ticket` as the source request.
2. Search existing tickets first with `list_tickets`, `get_ticket_description`, `ticket search`, or `ticket list` so you do not create duplicates.
3. Search existing specs for the same work so you can update the relevant spec after the ticket is created or matched.
4. Prefer `ticket-mcp` tools such as `list_tickets`, `get_ticket_description`, `create_ticket`, and `workflow` when they are available.
5. If `ticket-mcp` is unavailable, fall back to `./target/debug/ticket.exe search`, `./target/debug/ticket.exe list`, and `./target/debug/ticket.exe create`; use `--index-root` when the intended `.ticket` store is not the nearest one.
6. Infer the best single ticket title, type, priority, and initial state from the request. Keep the result scoped to one actionable work item.
7. When the prompt includes enough detail, add a useful initial description covering motivation, scope, constraints, and acceptance criteria.
8. If a matching ticket already exists, do not create a duplicate. Return the existing ticket instead.
9. For work that introduces new or changed requirements, goals, or behavior, create or update the relevant spec after the ticket is created or matched. Prefer spec-mcp tools when they are available and fall back to `./target/debug/spec.exe` when needed.
10. When linking the ticket in chat output or the spec body, never synthesize the folder path from the UUID, the selected store, or an example path.
11. Extract the exact canonical ticket folder path from ticket-api output. If the first create or match response does not include the folder path, run an immediate follow-up ticket-api command that returns the authoritative path before composing the chat response or updating the spec. Use that exact returned folder path as the link base and append `/ticket.toml` only when rendering the markdown target.
12. Ensure the spec records the request's requirements or goals before implementation begins and links the ticket with a target of the form `<exact ticket folder path>/ticket.toml` when available.
13. Ask one concise clarification if the target store, scope, or ticket shape is still ambiguous after a focused search.
14. Do not split the request into multiple tickets unless the user explicitly asks; `/ticket` should create one ticket.
15. Do not implement code or change unrelated tickets, specs, edges, or board state unless the user explicitly asks.

Response:
- created or matched ticket folder path and title, rendered as a markdown link of the form `[<short-id> <title>](<exact ticket folder path returned by ticket-api output>/ticket.toml)`
- chosen type, priority, and state
- created or updated spec slug and id, or why no spec change was needed
- duplicate candidates considered, if any
- assumptions that still matter
