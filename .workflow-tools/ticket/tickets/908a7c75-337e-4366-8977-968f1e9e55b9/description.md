## Problem

`memory-kernel` is the shared, filesystem-backed substrate used by every workflow-tools domain (audit, test, doc, log, feedback, peek, rule, session, spec, ticket), but it is still registered as a root-level `context-engine` submodule (sibling to `workflow-tools`) instead of nested inside `workflow-tools` alongside its consumers. With the 6-domain extraction (audit/test/doc/log/feedback/peek) and the earlier session/rule/ticket/spec extractions complete, `memory-kernel` is the one remaining shared-kernel repo that doesn't follow the `workflow-tools/<domain>` nesting convention.

Additionally, `workflow-tools/contract-reference` (the minimal reference scaffold teaching the domain-crate contract shape) currently only demonstrates the `cli`/`mcp`/`http` transports via `example`/`example-api`. It does not demonstrate a viewer (dioxus frontend + viewer-api integration) or a VS Code extension, both of which are part of the "most complete" shape exhibited by domains like `ticket` (ticket-viewer, ticket-vscode) and `doc`/`log` (doc-viewer, log-viewer).

## Goal

1. Move the `memory-kernel` submodule registration from the `context-engine` root to `workflow-tools/memory-kernel`, matching the nesting convention of every other domain repo. No code changes to memory-kernel's own repository; this is a submodule re-parenting plus dependent-path updates (root `Cargo.toml` `[patch]` block, README references, dev-submodule init instructions).
2. Extend `workflow-tools/contract-reference` with a minimal viewer crate/frontend and a minimal VS Code extension, so the reference demonstrates every transport shape used across the domains in their smallest viable form.

## Acceptance Criteria

- `workflow-tools/memory-kernel` exists as a nested submodule pointed at `github.com/mankinskin/memory-kernel` branch `main`; the root-level `memory-kernel` submodule entry is removed from `.gitmodules`/`.git/config`/git index.
- Root `Cargo.toml` `[patch."https://github.com/mankinskin/memory-kernel"]` block resolves to `workflow-tools/memory-kernel` paths.
- Root `README.md` dev-only-submodule guidance references the new nested path and the `workflow-tools/contract-reference` path (fixing the existing stale `workflow-tools-contract-reference/` reference).
- `cargo build --workspace` / `cargo test -p example*` succeed after the relocation.
- `contract-reference` gains a minimal example viewer (reusing `viewer-api` conventions) and a minimal example VS Code extension, each demonstrating the smallest working shape (no full feature parity with `ticket-viewer`/`ticket-vscode` required).
- `install-tools.sh --all` still passes after any artifact registration changes.
