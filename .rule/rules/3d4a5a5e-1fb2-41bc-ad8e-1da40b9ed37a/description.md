- `board.active_count` / `board.stale_count` — current load
- `board.wip_limit_reached` — true when new check-in would be blocked
- `board.warnings[]` — stale-entry alert strings
- `excluded_by_board[]` — candidate tickets excluded because an active/stale board entry covers
  them; fields: `ticket_id`, `agent_id`, `status`, `intent`