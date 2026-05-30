## Debug Workflow

When a test fails:
1. Run targeted tests first.
2. Inspect `target/test-logs/` for full trace output.
3. Use log-viewer MCP tools (`query_logs`, `search_all_logs`) with jq filters instead of parsing logs manually.
4. Re-run the nearest required validation after each local fix until it passes or the failure repeats without new signal.
5. If the failure remains a blocker, record the failing command, log path, and current diagnosis in the ticket/spec status summary instead of dropping the validation step.