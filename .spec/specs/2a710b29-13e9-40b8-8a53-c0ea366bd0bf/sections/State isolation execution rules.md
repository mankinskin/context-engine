### Direct CLI and proxied MCP boundary

Direct CLI commands run in their caller-controlled current working directory. A CLI selector is authoritative: `--index-root` and `--workspace` MUST resolve only inside the explicitly selected or containing checkout and MUST NOT be redirected through session routing or discovery.

MCP is the only session-routed interface. Because an agent's MCP working directory is not reliable, the MCP proxy MUST resolve each call only to the calling session's assigned worktree. The proxy MUST NOT discover, read, or persist sibling-worktree paths or stores.

### Local stores, discovery, and artifacts

Nested stores inside the same checkout, including submodules, are permitted. Discovery and aggregation MUST exclude `.worktrees/**`. A persisted path outside the owning checkout is invalid state and MUST be pruned during open or reconciliation without reading the foreign worktree.

All build outputs, frontend assets, and persistent stores MUST be checkout-local. Rebuild cost is explicitly acceptable; global or shared cache fallback is prohibited. Preparing the same viewer in two worktrees MUST NOT replace either worktree's served assets.

### Acceptance evidence

The worktree-isolation contract is complete only when a two-worktree regression creates independent state, removes one worktree, and proves the survivor's direct CLI operations, session-routed MCP operations, board snapshot, local feedback/spec/test stores, and frontend assets remain functional without any sibling path dependency.

Tracking ticket: `3a624bf6`.