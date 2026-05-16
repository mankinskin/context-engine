---
description: "Create new tickets from the slash-command text using the ticket-api flow. Prefer ticket-mcp tools and fall back to ticket.exe when needed."
name: "tickets"
argument-hint: "<your content>"
agent: "agent"
---

# Create Ticket Set

Create one or more new tickets from the user's current slash-command request using the ticket-api flow.

Reference [ticket-cli](../../memory-viewers/memory-api/tools/cli/ticket-cli/README.md) and [ticket-mcp](../../memory-viewers/memory-api/tools/mcp/ticket-mcp/README.md).

Install or build the ticket tools when needed:
- Build the CLI in this workspace with `cargo build -p ticket-cli --bin ticket` and use `./target/debug/ticket.exe`.
- Install the CLI onto your Cargo bin path with `cargo install --path memory-viewers/memory-api/tools/cli/ticket-cli --bin ticket`.
- Run the MCP server with `cargo run -p ticket-mcp` when MCP access needs to be configured locally.

Workflow:
1. Treat the text typed after `/tickets` as the source request.
2. Search existing tickets first with `list_tickets`, `get_ticket_description`, `ticket search`, or `ticket list` so you do not duplicate existing work.
3. Prefer `ticket-mcp` tools such as `list_tickets`, `get_ticket_description`, `create_ticket`, `add_edge`, and `workflow` when they are available.
4. If `ticket-mcp` is unavailable, fall back to `./target/debug/ticket.exe search`, `./target/debug/ticket.exe list`, `./target/debug/ticket.exe create`, and `./target/debug/ticket.exe link`; use `--index-root` when the intended `.ticket` store is not the nearest one.
5. Infer the smallest useful set of tickets that captures the request. Split only where the prompt implies distinct deliverables, dependencies, or workstreams.
6. Give each ticket a clear title plus reasonable type, priority, and initial state. Add descriptions when the prompt provides enough detail.
7. Add dependency edges only when the ordering is explicit or strongly implied. Do not invent extra structure.
8. If some of the needed tickets already exist, reuse them and create only the missing ones.
9. Ask one concise clarification if the target store or ticket breakdown is still ambiguous after a focused search.
10. Do not implement code or change unrelated tickets unless the user explicitly asks.

Response:
- target store and number of tickets created or matched
- ticket ids and titles created or reused
- dependency edges added, if any
- assumptions or follow-up gaps
