## Viewer-API Shared Patterns

- Initialize server apps via `ServerConfig::new(name, port)` and prefer `.with_host()` / `.with_static_dir()` helpers.
- Use `init_tracing_full()` with `TracingConfig::from_env(...)` instead of custom tracing setup.
- Reuse `default_cors()` and `with_static_files()` from viewer-api for consistent HTTP behavior.
- For stable error responses in viewer-style APIs, prefer the shared API error envelope patterns from viewer-api or tool-local wrappers.
- For SSE output, use viewer-api SSE helpers instead of ad hoc event formatting.
- For JQ/filter behavior, reuse viewer-api query primitives; avoid introducing parallel query stacks.