## Problem

`.github/copilot-instructions.md` lines 25-27 say each domain crate exposes transports at `tools/cli/*`, `tools/mcp/*`, and sometimes `tools/http/*`, with business logic in a `-api` crate. The passage describes only the legacy layout.

The target architecture is an internal `{domain}-api` crate plus a public `{domain}` crate that owns feature-gated `cli`, `mcp`, and `http` binaries. `memory-api/crates/ticket` already implements the target architecture. The remaining `tools/{cli,mcp,http}/*` directories still exist only because their tools have not migrated, so treating the legacy layout as canonical gives incorrect direction to new work.

## Required State

Rewrite the passage so the `{domain}-api` plus public `{domain}` crate architecture is canonical, name `memory-api/crates/ticket` as the reference implementation, and identify `tools/{cli,mcp,http}/*` explicitly as the legacy layout for unmigrated tools. Link the migration context to ticket `0da6894c`.

Related migration tickets: `ba4aaa9c`, `0da6894c`, and `1b7e0c3d`.
