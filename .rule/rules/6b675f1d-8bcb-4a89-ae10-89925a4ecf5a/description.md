#### Check-In / Check-Out / Heartbeat

```bash
# Register yourself as actively working a ticket
./target/debug/ticket.exe board check-in <ticket-id> \
  --agent-id <agent-id> \
  --intent "brief description of planned work" \
  --files "src/foo.rs,src/bar.rs" \
  --ttl 3600 \
  --json