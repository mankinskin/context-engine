## Filesystem and Source Access

- Constrain filesystem reads to configured roots (workspace/log/static roots).
- Normalize and validate paths before access; prevent path traversal.
- Prefer existing viewer-api helpers/utilities before adding new path logic.
- When local source is unavailable (remote/deployed mode), use configured remote source resolution paths.