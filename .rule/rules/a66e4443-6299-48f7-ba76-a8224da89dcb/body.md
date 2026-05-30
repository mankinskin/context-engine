## Integration Boundaries

- Treat `context-api` and `ticket-api` as system-of-record contracts.
- Keep business logic in API crates/services; viewers should focus on transport, presentation, and interaction.
- Preserve request/response compatibility unless the task explicitly requires an API change.
- If API behavior changes, update dependent viewer routes and docs in the same change.