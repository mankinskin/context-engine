# Check out when done (records handoff reason in audit trail)
./target/debug/ticket.exe board check-out <ticket-id> \
  --agent-id <agent-id> \
  --reason "implemented and tested" \
  --json
```