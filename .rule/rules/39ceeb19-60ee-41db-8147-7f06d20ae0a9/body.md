## Single-Process Viewer Pattern

- `ticket-viewer` embeds `ticket-http` routes directly and serves SPA + API on one process.
- For new viewers, prefer importing/mounting existing HTTP routers rather than introducing extra backend daemons.
- Keep static fallback routing behavior consistent with current SPA serving pattern.