```bash
# Add / remove files from an active entry
./target/debug/ticket.exe board update-files <ticket-id> \
  --agent-id <agent-id> --add "new.rs" --remove "old.rs" --json