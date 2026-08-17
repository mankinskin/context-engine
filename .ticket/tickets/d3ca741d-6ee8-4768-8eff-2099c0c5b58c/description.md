## Implementation plan: spec extraction and viewer migration

### Established baseline
`memory-api/crates/memory-api` is deleted. `spec` must commit `memory_kernel = { git = "https://github.com/mankinskin/memory-kernel", branch = "main" }`; use a relative path only in a root development `[patch]`. Ticket `1b7e0c3d` proved patch-free remote resolution.

Create public crate `spec` re-exporting internal `spec-api`, with no default features and feature-gated bare `spec`, `spec-mcp`, and `spec-http` bins on `transport-harness`. Keep spec-domain typed manifest accessors as `spec-api` extension traits with unchanged `extra` keys, never in the neutral kernel.

### Verified source surface
- API: `memory-api/crates/spec-api`.
- Legacy transports to consolidate first: `memory-api/tools/cli/spec-cli`, `memory-api/tools/mcp/spec-mcp`, `memory-api/tools/http/spec-http` (confirm exact legacy paths during planning).
- Viewer and frontend: `memory-viewers/spec-viewer`, including its Dioxus frontend.

### Consumers to repoint
Any crate consuming `spec-api` outside spec's own transports must be identified and repointed to the extracted git dependency before the legacy in-tree paths are deleted (repeat the consumer survey done for `rule`/`log`, e.g. `memory-api/tools/cli/spec-cli` if retained as a legacy consumer during transition).

### Execution
1. Fold legacy `spec-cli`/`spec-mcp`/`spec-http` into feature-gated bins in the `spec` crate; preserve binary names and tests.
2. Move `spec-api` and the consolidated crate into the standalone spec repository; move `spec-viewer` alongside it.
3. Repoint all identified consumers to the git dependency before deleting old workspace paths.
4. Independent build + test, then browser-verify the spec viewer.

### Acceptance criteria
```bash
cargo test -p spec-api -p spec-cli -p spec-mcp -p spec-http
cargo build --manifest-path "$TOOL_REPO/Cargo.toml" --no-default-features
cargo test --manifest-path "$TOOL_REPO/Cargo.toml" --all-features
cargo test --workspace
```
Validate the spec viewer in an external fullscreen Chromium-family browser and capture Playwright screenshots per `AGENTS.md`.

### Risk
Standalone proof needs a published `memory-kernel` and any domain crates spec depends on (per contract `0da6894c`). The viewer migration must preserve existing HTTP/SSE integration with `viewer-api`.
