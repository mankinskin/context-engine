# Extract memory-matrix + rewire in-tree consumers to external memory-fixtures

Follow-up to T-SHAREDLIBS (1c452ff1). The memory-fixtures leaf was extracted to
its own repo this session (github.com/mankinskin/memory-fixtures, main). This
ticket captures the deferred remainder.

## Scope

1. Rewire the in-tree consumers of memory-fixtures to the external git
   dependency instead of the path dependency:
   - `memory-api/crates/spec-api/Cargo.toml` (`[dev-dependencies]`)
   - `memory-api/crates/ticket-api/Cargo.toml` (`[dev-dependencies]`)
   - `memory-api/crates/memory-matrix/Cargo.toml` (`[dependencies]`)
   Use `{ git = "https://github.com/mankinskin/memory-fixtures", branch = "main" }`
   per the convention documented in the memory-fixtures README. This touches the
   `memory-api` submodule and must be coordinated with any concurrent agent work
   there.
2. Extract `memory-matrix` into a standalone crate/repo. Blocked: memory-matrix
   depends on the entire domain-crate set (memory-api, ticket-api, spec-api,
   rule-api, audit-api, session-api, test-api, log-api) plus their cli/http/mcp
   tools, so it can only build standalone once those domain crates are
   themselves extracted/publishable. It is a cross-domain consumer, not a shared
   substrate leaf.

## Ordering

- memory-fixtures extraction: DONE (this session).
- Consumer rewiring: actionable once memory-api submodule edits can be claimed
  without conflict.
- memory-matrix extraction: blocked on domain-crate extraction (per-tool phase).

## First validation

- After rewiring: `cargo test -p spec-api -p ticket-api` resolves memory-fixtures
  from the git remote and passes.
- After extraction: `memory-matrix` builds standalone against externally
  resolved domain crates.
