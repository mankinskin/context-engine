- `warnings[]` — board-derived warnings such as WIP-limit or stale-entry alerts
- `excluded_by_board[]` — candidate tickets excluded because an active/stale board entry covers
  them; fields: `ticket_id`, `agent_id`, `status`, `intent`

When WIP or stale board conditions matter, resolve them through `board_show` / board lifecycle tools.
