## Architecture

- `context-http` is a thin transport adapter over `context-api`.
- Keep business/domain behavior in `context-api`; keep adapter behavior in `context-http`.