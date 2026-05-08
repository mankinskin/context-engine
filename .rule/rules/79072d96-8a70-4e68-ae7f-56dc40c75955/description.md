```bash
# Find what a completed ticket blocks
./target/debug/ticket.exe topgraph <id> --json \
  | jq -r '.payload.nodes[] | select(.state=="new" or .state=="ready") | .id'
```