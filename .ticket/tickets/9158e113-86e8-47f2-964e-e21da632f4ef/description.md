## Implementation plan: audit extraction

### Established baseline
`memory-api/crates/memory-api` is deleted. `audit` must commit `memory_kernel = { git = "https://github.com/mankinskin/memory-kernel", branch = "main" }`; use a relative path only in a root development `[patch]`. Ticket `1b7e0c3d` proved patch-free remote resolution.

Create public crate `audit` re-exporting internal `audit-api`, with no default features and feature-gated bare `audit` and `audit-mcp` bins on `transport-harness` (confirm exact transport surface during planning — an `audit-http` bin is added only if a legacy HTTP transport exists). Keep audit-domain typed manifest accessors as `audit-api` extension traits with unchanged `extra` keys, never in the neutral kernel.

### Verified source surface
- API: `memory-api/crates/audit-api` (consumes `rule-api` per the `rule` extraction ticket `21893f5f`).
- Legacy transports to consolidate first: any `audit-cli`/`audit-mcp` under `memory-api/tools/` (confirm exact legacy paths during planning).
- No known viewer for audit.

### Consumers to repoint
Any crate consuming `audit-api` outside audit's own transports must be repointed to the extracted git dependency before legacy in-tree paths are deleted.

### Execution
1. Fold legacy audit transports into feature-gated bins in the `audit` crate; preserve binary names and tests.
2. Move `audit-api` and the consolidated crate into the standalone audit repository.
3. Repoint all identified consumers to the git dependency before deleting old workspace paths.
4. Independent build + test.

### Acceptance criteria
```bash
cargo test -p audit-api -p audit-cli -p audit-mcp
cargo build --manifest-path "$TOOL_REPO/Cargo.toml" --no-default-features
cargo test --manifest-path "$TOOL_REPO/Cargo.toml" --all-features
cargo test --workspace
```

### Risk
`audit-api` depends on `rule-api`; sequence audit extraction after `rule` (`21893f5f`) is published so the audit repo consumes a real git dependency rather than an in-tree path.
