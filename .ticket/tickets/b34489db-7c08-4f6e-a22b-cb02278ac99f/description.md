## Implementation plan: test extraction

### Established baseline
`memory-api/crates/memory-api` is deleted. `test` must commit `memory_kernel = { git = "https://github.com/mankinskin/memory-kernel", branch = "main" }`; use a relative path only in a root development `[patch]`. Ticket `1b7e0c3d` proved patch-free remote resolution.

Create public crate `test` re-exporting internal `test-api`, with no default features and feature-gated bare `test` and `test-mcp` bins on `transport-harness` (confirm exact legacy transport surface during planning). Keep test-domain typed manifest accessors as `test-api` extension traits with unchanged `extra` keys, never in the neutral kernel.

### Verified source surface
- API: `memory-api/crates/test-api`.
- Legacy transports to consolidate first: `memory-api/tools/cli/test-cli` and any `test-mcp` crate (confirm exact legacy paths during planning).
- No known viewer for test.

### Blocking dependency (explicit)
`memory-api/tools/cli/test-cli` currently has a cross-repo cycle dependency on `log-api` (`test-cli` consumes `log-api`, while `log-api` consumes `test-api`), which is the exact cross-repo dependency cycle blocking log extraction identified in ticket `a6d38372-0df2-437b-b06d-3984c290dbd1` ("Resolve test-cli/log-api/test-api dependency cycle blocking log extraction"). This ticket **depends_on `a6d38372`**: the cycle must be resolved (test-cli's log-api import removed or restructured) before `test-cli` can be folded into the standalone `test` domain crate and published as a git dependency without re-introducing the cycle in the extracted repo.

### Consumers to repoint
Any crate consuming `test-api` outside test's own transports (including `log-api`, per the cycle above) must be repointed to the extracted git dependency, in the order fixed by `a6d38372`.

### Execution
1. Confirm `a6d38372` has removed the test-cli → log-api cycle.
2. Fold legacy test transports into feature-gated bins in the `test` crate; preserve binary names and tests.
3. Move `test-api` and the consolidated crate into the standalone test repository.
4. Repoint all identified consumers to the git dependency before deleting old workspace paths.
5. Independent build + test.

### Acceptance criteria
```bash
cargo test -p test-api -p test-cli -p test-mcp
cargo build --manifest-path "$TOOL_REPO/Cargo.toml" --no-default-features
cargo test --manifest-path "$TOOL_REPO/Cargo.toml" --all-features
cargo test --workspace
```

### Risk
Extraction cannot land while the test/log cycle is open; `a6d38372` is a hard prerequisite, not a soft ordering preference.
