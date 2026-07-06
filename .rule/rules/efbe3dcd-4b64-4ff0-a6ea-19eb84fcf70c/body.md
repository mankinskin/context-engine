```bash
# Find unblocked ready tickets you can work on now (priority-ordered)
ticket next --toon

# For a blocked tracker/epic, find immediate actionable leaf blockers under it
ticket next <ticket-id> --toon

# Optional MCP equivalent when using ticket-mcp
# next_tickets {"workspace":"default","root":"<ticket-id>"}