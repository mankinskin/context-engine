1. `audit-api` owns audit logic, models, config loading, indexing, and trials.
2. `audit-cli` owns argument parsing and human/json rendering.
3. `audit-mcp` only translates MCP inputs into `audit-api` calls and serializes the result.