#### File Ownership

Owned files block other agents from checking in with overlapping paths.
Keep owned file lists narrow and release them (via check-out or update-files)
when no longer needed.

Use the short flag forms shown below as the canonical CLI shape. The board
parser keeps the older `--agent-id`, `--files`, `--old-path`, and `--new-path`
spellings as compatibility aliases, but help text and docs should use the same
flag names as the rest of `ticket-cli`.