## Implementation plan: doc extraction and viewer migration

### Established baseline
`memory-api/crates/memory-api` is deleted. `doc` must commit `memory_kernel = { git = "https://github.com/mankinskin/memory-kernel", branch = "main" }`; use a relative path only in a root development `[patch]`. Ticket `1b7e0c3d` proved patch-free remote resolution.

Create public crate `doc` re-exporting internal `doc-api`, with no default features. Feature-gate transport bins on `transport-harness` for whichever transports actually exist (see open scope question below).

### Verified source surface
- API: `memory-api/crates/doc-api`.
- Viewer and frontend: `memory-viewers/doc-viewer`, including its frontend and e2e suite.
- An `http` tool for doc was found in a prior survey; no `doc-cli` or `doc-mcp` crate was found in that same pass.

### Open scope question (resolve during planning, not blocking creation)
Confirm whether a standalone `doc-cli`/`doc-mcp` genuinely does not exist (i.e. doc is HTTP + viewer only, like the log precedent where "no separate log-cli/log-mcp/log-http crate exists"), or whether cli/mcp access to doc is only exposed indirectly (e.g. via doc-viewer's internal MCP server, or via another tool). If no cli/mcp transport exists, the extracted `doc` crate should expose only the `doc` lib, an optional `doc-http` bin, and the viewer — do not fabricate cli/mcp bins that have no legacy source.

### Consumers to repoint
Any crate consuming `doc-api` outside doc's own transports/viewer must be identified and repointed to the extracted git dependency before legacy in-tree paths are deleted.

### Execution
1. Resolve the open scope question above (cli/mcp existence) before finalizing the transport bin list.
2. Fold whatever legacy doc transport(s) exist into feature-gated bins in the `doc` crate; preserve binary names and tests.
3. Move `doc-api` and the consolidated crate into the standalone doc repository; move `doc-viewer` alongside it.
4. Repoint all identified consumers to the git dependency before deleting old workspace paths.
5. Independent build + test, then browser-verify the doc viewer.

### Acceptance criteria
```bash
cargo test -p doc-api
cargo build --manifest-path "$TOOL_REPO/Cargo.toml" --no-default-features
cargo test --manifest-path "$TOOL_REPO/Cargo.toml" --all-features
cargo test --workspace
```
Validate the doc viewer in an external fullscreen Chromium-family browser and capture Playwright screenshots per `AGENTS.md`.

### Risk
The doc transport surface (cli/mcp existence) is unconfirmed; do not commit to a transport bin list until the open scope question is resolved during planning.
