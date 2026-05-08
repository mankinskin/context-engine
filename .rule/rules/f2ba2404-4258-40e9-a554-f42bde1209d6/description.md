## Debug Workflow

When a test fails:
1. Run targeted tests first.
2. Inspect `target/test-logs/` for full trace output.
3. Use log-viewer MCP tools (`query_logs`, `search_all_logs`) with jq filters instead of parsing logs manually.