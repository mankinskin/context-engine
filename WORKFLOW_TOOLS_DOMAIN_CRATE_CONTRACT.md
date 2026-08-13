# Workflow-Tools Domain Crate Contract

Each extracted workflow domain is consumed through one public Rust crate named
after the domain. Its library is the sole library dependency handle; it
re-exports the public surface of the internal `{domain}-api` crate and provides
only transport-agnostic composition.

## Repository Layout

```text
{domain}/
  crates/
    {domain}-api/              # internal domain API
    {domain}/                  # public library and transport binaries
      src/lib.rs
      src/bin/{domain}.rs
      src/bin/{domain}-mcp.rs
      src/bin/{domain}-http.rs
  frontend/                    # optional, separate consumer crates
```

The public crate must declare `[lib]` and a `[[bin]]` target for every supported
transport. It depends on `{domain}-api`, re-exports that API from `src/lib.rs`,
and uses the generic executable naming rule: the CLI binary is the bare
`{domain}` name, while MCP and HTTP binaries remain `{domain}-mcp` and
`{domain}-http`.

```toml
[features]
default = []
cli = ["dep:transport-harness"]
mcp = ["dep:transport-harness"]
http = ["dep:transport-harness"]

[[bin]]
name = "{domain}"
path = "src/bin/{domain}.rs"
required-features = ["cli"]
```

Repeat the `[[bin]]` pattern for MCP and HTTP. Features gate transport binaries
and their harness dependency, so `cargo build -p {domain}` builds a slim library
and `cargo build -p {domain} --features cli,mcp,http` builds every interface.

## Harness And Frontends

Transport binaries use `transport-harness` for shared CLI, MCP, and HTTP
scaffolding. A binary owns only its domain-specific command, server, or router
wiring.

The harness is owned by the memory-kernel repository at
`memory-kernel/crates/transport-harness` (discoverable through the
`memory-kernel/` submodule). Its normative responsibilities, non-goals,
features, public API boundaries, and guards are defined by the canonical spec in
memory-kernel (`memory-kernel/.spec/specs/e5294ae5-6bff-44dc-81a9-24a44615b775/spec.toml`,
slug `transport-harness`). This document does not restate those requirements;
consult that spec for the harness contract.

Domain viewers and VS Code extensions remain independent crates/packages. They
depend on the public domain library and never become transport binaries.

## Consumers

`workflow-tools` and target repositories depend on the public `{domain}` crate
for library use. Installers select executable interfaces explicitly, for example:

```bash
cargo install --path crates/ticket --features cli,mcp,http
```

The executable names stay stable even though their build targets move into the
public domain crate.

## Reference Workspace

[`workflow-tools-contract-reference`](workflow-tools-contract-reference) is the
compiling reference implementation. It uses an `example-api` internal crate,
an `example` facade crate, and gated CLI, MCP, and HTTP binaries that consume
the production harness from the sibling `memory-kernel` repository.