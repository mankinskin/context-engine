## Problem

Three tooling files still target the deleted `ticket-cli` package, causing real build/install failures:

- `Makefile.toml` line 50 describes `ticket-cli`; line 52 passes `-p ticket-cli` to `cargo build`.
- `tools/install/artifacts.toml` line 31 declares `id = "ticket-cli"`; line 34 points at `memory-api/tools/cli/ticket-cli`, a missing path.
- `install-tools.sh` lines 40, 107, and 108 reference the `ticket-cli` tool id.

The package and directory no longer exist. `memory-api/crates/ticket` is the public unified ticket crate; its `Cargo.toml` feature-gates the `ticket`, `ticket-mcp`, and `ticket-http` binaries. The CLI has the bare binary name `ticket` and must be built from `memory-api/crates/ticket` with feature `cli`.

## Required State

Replace all three tooling references with the `memory-api/crates/ticket` package/source and the `ticket` binary using feature `cli`. Preserve the existing layout for other unmigrated tools. The change must make the affected build and install workflows execute successfully rather than only removing stale text.

Related migration tickets: `ba4aaa9c`, `0da6894c`, and `1b7e0c3d`.

Validation: commit 717c3329 on branch agent/b9020ba2-df5d-426a-b1b9-228ef159cad1/guidance-learnings-impl — `./install-tools.sh --dry-run --tool ticket --tool ticket-mcp` -> pass; recorded validation spec `ticket-07a3eb2d-install-tools-dryrun` and execution `exec-ticket-07a3eb2d-install-tools-20260814-01`.