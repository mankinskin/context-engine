## Acceptance Criteria

1. Focused API tests for every supported domain prove creation and index/sidecar writes target the canonical directory.
2. CLI and MCP tests cover a legacy-only workspace read with the deprecation diagnostic and a canonical write.
3. Explicit concrete store-path inputs remain supported and do not silently create a shadow store.
4. Viewer, hook, and generated configuration path consumers no longer encode a flat legacy write destination.
5. User-facing help and documentation identify the canonical layout and the temporary legacy-read rule.