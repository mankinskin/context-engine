## State Management Patterns

- For shared mutable backend state, follow `Arc<Mutex<_>>` app-state patterns used by adapter tools.
- For hot-reloadable runtime config (for example auth tokens), follow arc-swap style patterns used by HTTP tools.
- Keep session-like frontend/backend coordination in shared utilities when available.