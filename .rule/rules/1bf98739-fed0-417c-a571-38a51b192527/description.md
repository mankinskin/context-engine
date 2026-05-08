**Rules:**
- Each line is parsed identically to a top-level `ticket <subcommand>` call.
- `serve`, `watch`, nested `batch`, `scan`, lease commands, and
  config commands (`add-root`, `workspace`, `export-command-schema`) are
  rejected with a clear error — they cannot be used inside a batch.
- On rollback: `create` → deleted, `update` → state/fields restored, `link`
  → edge removed. Read-only commands (`get`, `list`, `search`, `health`, etc.)
  produce no undo entry and are not affected by rollback.
- No `--index-root` requirement — uses normal workspace resolution like any
  other CLI command.