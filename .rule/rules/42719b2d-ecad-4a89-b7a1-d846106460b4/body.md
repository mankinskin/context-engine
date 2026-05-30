This means the store will **reject** `close` (or `update --to-state done`) if
`in-review` has never been visited. This is enforced at the API layer, so it
applies to CLI, MCP, and HTTP equally.