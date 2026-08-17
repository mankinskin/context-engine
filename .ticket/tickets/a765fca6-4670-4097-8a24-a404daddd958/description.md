## Implementation plan: session extraction

### Established baseline
`memory-api/crates/memory-api` is deleted. `session` must commit `memory_kernel = { git = "https://github.com/mankinskin/memory-kernel", branch = "main" }`; use a relative path only in a root development `[patch]`. Ticket `1b7e0c3d` proved patch-free remote resolution.

Create public crate `session` re-exporting internal `session-api`, with no default features and feature-gated bare `session` and `session-mcp` bins on `transport-harness`. Keep session-domain typed manifest accessors as `session-api` extension traits with unchanged `extra` keys, never in the neutral kernel.

### Verified source surface
- API: `memory-api/crates/session-api`.
- Legacy transports to consolidate first: `memory-api/tools/cli/session-cli`, `memory-api/tools/mcp/session-mcp` (confirm exact legacy paths during planning).
- Also in scope: `memory-api/crates/session-capture-hook` and `memory-api/crates/session-worktree-provision`, both of which also consume `ticket-api`.
- No known viewer for session.

### Consumers / cross-domain dependency (explicit)
Per a prior review pass, `session-api`, `session-capture-hook`, and `session-worktree-provision` all depend on `ticket-api` (for board/session-worktree/ticket linkage). This ticket's extraction **depends on the consumer-repoint step of ticket `ba4aaa9c-d270-4cfc-a1e2-395634608371`** (the `ticket` domain-crate extraction) being complete: session cannot cut over to a published `ticket` git dependency until `ticket-api` is available as an external git dependency rather than an in-tree workspace path. Session extraction should be sequenced after `ba4aaa9c`'s consumer repointing lands.

### Execution
1. Confirm `ba4aaa9c` has repointed `session-api`/`session-capture-hook`/`session-worktree-provision` (or their equivalents) to the published `ticket` git dependency.
2. Fold legacy session transports into feature-gated bins in the `session` crate; preserve binary names and tests.
3. Move `session-api` (and `session-capture-hook`/`session-worktree-provision` if in scope) into the standalone session repository.
4. Repoint remaining consumers to the git dependency before deleting old workspace paths.
5. Independent build + test.

### Acceptance criteria
```bash
cargo test -p session-api -p session-cli -p session-mcp
cargo build --manifest-path "$TOOL_REPO/Cargo.toml" --no-default-features
cargo test --manifest-path "$TOOL_REPO/Cargo.toml" --all-features
cargo test --workspace
```

### Risk
Session extraction is blocked on `ticket` being consumable as an external dependency; attempting session extraction first would require an in-tree path dependency on ticket, which contradicts the domain-crate contract (`0da6894c`).
