### Tool reference

| Tool | Required | Optional |
|------|----------|----------|
| `board_show` | workspace | agent_id |
| `board_check_in` | workspace, ticket_id, agent_id | intent, files, ttl_secs |
| `board_check_out` | workspace, ticket_id | agent_id, reason |
| `board_heartbeat` | workspace, entry_id | — |
| `board_configure` | workspace | max_wip, stale_after_secs, completed_audit_window_secs |
| `board_clean_preview` | workspace | include_stale |
| `board_clean_apply` | workspace, token | include_stale |
| `board_update_files` | workspace, ticket_id, agent_id | add, remove |
| `board_rename_file` | workspace, ticket_id, agent_id, old_path, new_path | — |