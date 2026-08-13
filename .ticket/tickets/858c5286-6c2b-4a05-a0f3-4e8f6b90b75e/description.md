Phase B parent tracker. Split each of the 11 domain tools into its own bare-named repository under github.com/mankinskin. Each tool repo is built around a single domain crate (see contract `0da6894c`) that unifies the domain api and all Rust transports, with frontends kept as separate crates.

## Single-crate structure (per contract 0da6894c + harness dbe0e955)
- One domain crate `{domain}` is the primary build target: its lib is the public handle and re-exports the internal `{domain}-api` crate (api kept as its own internal crate, not absorbed).
- Each transport (cli, mcp, http) is a FEATURE-GATED binary target (`[[bin]]`) of that crate, built on the shared `transport-harness` crate. The CLI target is always the bare `{domain}` name; MCP and HTTP targets retain `{domain}-mcp` and `{domain}-http` names. No separate transport crates.
- Frontends stay separate crates in the repo: the domain viewer (Dioxus/WASM) and vscode extension, depending on the domain crate lib.

## Common per-tool extraction recipe
1. Assemble the domain crate: depend on the internal `{domain}-api` crate (re-exported) and add feature-gated transport bin targets over `transport-harness`; move its viewer from memory-viewers as a separate crate.
2. Declare `memory-kernel`, `transport-harness`, `viewer-api`, and `memory-fixtures` as external dependencies as applicable.
3. Preserve git history where practical (subtree/filter-repo).
4. Migrate scoped artifacts into the tool repo's own stores using the safe cross-workspace move tooling (memory-api `505b2cd4`) so references are relinked.
5. Independent build + test: `cargo build` produces the lib + selected transport bins; smoke each bin (cli/mcp/http); viewer browser verification.
6. Register the tool repo (its domain crate) as a dependency of `workflow-tools`.

## Children (one per tool)
ticket, spec, rule, doc, test, log, feedback, session, audit, peek, interview.

## Acceptance criteria
- Every child tool repo builds/tests independently: one domain crate lib (primary) re-exporting the internal api crate + feature-gated transport bins over the harness, plus separate frontend crates.
- CLI targets use bare `{domain}` names; MCP and HTTP targets use `{domain}-mcp` and `{domain}-http` names.
- Tool-scoped artifacts moved with reference integrity preserved.
- All child tickets closed and aggregated into workflow-tools.

## Dependencies
- Blocked by foundations (memory-kernel, transport-harness, shared libs) and the domain-crate contract `0da6894c`.
- Artifact moves blocked on cross-store move tooling (memory-api `505b2cd4`).