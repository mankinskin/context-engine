```bash
# Add / remove files from an active entry
./target/debug/ticket.exe board update-files <ticket-id> \
  --agent <agent-id> --add "new.rs" --remove "old.rs" --toon