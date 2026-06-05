#### Check-In / Check-Out / Heartbeat

```bash
# Register yourself as actively working a ticket
./target/debug/ticket.exe board check-in <ticket-id> \
  --agent <agent-id> \
  --intent "brief description of planned work" \
  --file "src/foo.rs" \
  --file "src/bar.rs" \
  --ttl-secs 3600 \
  --toon