## Implementation plan: feedback extraction

### Established baseline
`memory-api/crates/memory-api` is deleted. `feedback` must commit `memory_kernel = { git = "https://github.com/mankinskin/memory-kernel", branch = "main" }`; use a relative path only in a root development `[patch]`. Ticket `1b7e0c3d` proved patch-free remote resolution.

Create public crate `feedback` re-exporting internal `feedback-api`, with no default features and feature-gated bare `feedback`, `feedback-mcp`, and `feedback-http` bins on `transport-harness` (per a prior survey, feedback has cli/mcp/http tools — confirm exact legacy paths during planning). Keep feedback-domain typed manifest accessors as `feedback-api` extension traits with unchanged `extra` keys, never in the neutral kernel.

### Verified source surface
- API: `memory-api/crates/feedback-api`.
- Legacy transports to consolidate first: feedback's cli, mcp, and http tool crates under `memory-api/tools/` (confirm exact legacy paths during planning).
- No known viewer for feedback.

### Consumers to repoint
`rule-api` consumes `feedback-api` (per ticket `21893f5f`'s finding that rule extraction must publish `feedback` before rule's own standalone build). Any other crate consuming `feedback-api` outside feedback's own transports must be repointed to the extracted git dependency before legacy in-tree paths are deleted.

### Execution
1. Fold legacy feedback transports into feature-gated bins in the `feedback` crate; preserve binary names and tests.
2. Move `feedback-api` and the consolidated crate into the standalone feedback repository.
3. Repoint all identified consumers (including `rule-api`) to the git dependency before deleting old workspace paths.
4. Independent build + test.

### Acceptance criteria
```bash
cargo test -p feedback-api -p feedback-cli -p feedback-mcp -p feedback-http
cargo build --manifest-path "$TOOL_REPO/Cargo.toml" --no-default-features
cargo test --manifest-path "$TOOL_REPO/Cargo.toml" --all-features
cargo test --workspace
```

### Risk
`feedback` must be published before `rule` (`21893f5f`) can complete its own standalone build, since `rule-api` consumes `feedback-api`. Sequence feedback extraction early relative to rule.
