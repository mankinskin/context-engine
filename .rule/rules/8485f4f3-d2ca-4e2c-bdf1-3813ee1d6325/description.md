## State and Concurrency

- Use `AppState` with `Arc<Mutex<WorkspaceManager>>` patterns for shared mutable access.
- Do not bypass manager locking patterns for command execution paths.
- Keep capture-config resolution (`capture_config_for`) aligned with workspace log-directory behavior.