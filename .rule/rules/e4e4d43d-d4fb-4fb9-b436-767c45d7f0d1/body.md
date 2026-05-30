## Acceptance criteria

- This spec no longer depends on a dedicated wrapper validation CLI or wrapper-owned artifact store.
- Validation workflow state is defined as default shared-library behavior across `ticket-api`, `spec-api`, `doc-api`, and future `test-api` / `log-api`.
- First-class responsibilities for `test-api` and `log-api` are part of the design, including native identifiers and cross-store links.
- Any wrapper-only prototype implementation is explicitly described as migration context rather than target architecture.
- Existing CLI, MCP, and HTTP surfaces are treated as the default way users interact with workflow metadata once the shared-library behavior lands.