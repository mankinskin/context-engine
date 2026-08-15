## Problem

`test-api` currently owns two artifact traits that are entirely domain-neutral:

- `IdentifiableArtifact` (an `id()` accessor over an `AsRef<str>` id type)
- `TraceableArtifact` (domain/operation/run-id/traceability-link accessors, super-trait `InteroperableArtifact`)

Both live in `memory-api/crates/test-api/src/interoperability.rs`, which already
does nothing but `pub use memory_kernel::InteroperableArtifact;` plus these two
definitions. `log-api` imports all three names *through* `test-api`
(`crates/log-api/src/lib.rs`, `crates/log-api/src/store.rs`), which is one of the
two reasons `log-api` depends on `test-api` at all.

Because the traits are neutral, their placement in `test-api` is a violation of
the neutrality rule already established for `memory-kernel`: neutral concepts
belong in the kernel, domain behavior belongs in the domain API. Leaving them in
`test-api` forces every future consumer of the neutral contract to take a
dependency on the test domain.

## Scope

Move the two trait definitions into `memory-kernel` alongside `InteroperableArtifact`,
and keep `test-api` compiling unchanged for its own consumers via re-export.

This ticket does **not** touch the `ValidationExecution` / `ValidationLinks`
coupling — that is the second half of the cycle and is owned by its own ticket.

## Non-goals

- No behavior change to any trait method.
- No change to `ValidationExecution`, `ValidationLinks`, or the log adapters.
- No repository extraction.
