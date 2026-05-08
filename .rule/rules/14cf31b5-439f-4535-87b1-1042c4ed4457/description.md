## HTTP and Runtime Conventions

- Reuse `viewer-api` server utilities for tracing, CORS, static files, and CLI args.
- Keep router composition explicit and stable; avoid breaking endpoint names without migration.
- Preserve single-process viewer startup assumptions where present (for example ticket-viewer embedding ticket-http).