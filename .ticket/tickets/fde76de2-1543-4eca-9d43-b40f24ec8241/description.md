## Objective
Eliminate the global frontend asset cache so builds and served assets are isolated per checkout/worktree.

## Binding decision
Rebuild cost is acceptable. Frontend assets MUST be checkout-local; do not retain a global, content-hash, or worktree-name cache under `~/.context-engine`.

## Required behavior
- `install-ctl prepare <viewer>` writes assets only beneath the caller's checkout.
- Starting a viewer resolves assets from that same checkout.
- Preparing the same viewer in two checkouts cannot delete, replace, or serve the other checkout's bundle.
- Existing global cache contents are not used as fallback assets.

## Owning paths
- `viewer-ctl.toml`
- `tools/install/install-ctl/src/config.rs`
- `tools/install/install-ctl/src/commands/frontend.rs`
- `tools/install/install-ctl/src/commands/server.rs`

## Validation
Add a two-checkout regression that prepares distinct viewer bundles and proves each checkout retains and serves its own assets.

## Parent
Implements one slice of ticket `3a624bf6`.