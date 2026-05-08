# Rename a file in an active entry (atomic)
./target/debug/ticket.exe board rename-file <ticket-id> \
  --agent-id <agent-id> --old-path "old.rs" --new-path "new.rs" --json
```