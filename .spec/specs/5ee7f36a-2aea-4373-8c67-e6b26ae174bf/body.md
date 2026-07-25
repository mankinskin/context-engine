# Summary

Each extracted workflow domain is consumed through one public Rust crate named after the domain. The crate library is the sole dependency handle, re-exporting its internal `{domain}-api` crate. CLI, MCP, and HTTP entry points remain executable interfaces, but are feature-gated `[[bin]]` targets of that same crate.

# Layout

A domain repository contains an internal `{domain}-api` crate, a public `{domain}` crate, and separate frontend crates where applicable. The public crate has `src/lib.rs`, which re-exports the internal API crate and contains only transport-agnostic composition. Each transport has a dedicated source file in `src/bin/`.

# Manifest Contract

The domain crate declares a `[lib]` target and features `cli`, `mcp`, and `http`. Each `[[bin]]` has `required-features` for its one transport feature and keeps the established binary name (`{domain}-cli`, `{domain}-mcp`, or `{domain}-http`). The default feature set is empty, so `cargo build -p {domain}` builds the library without transport dependencies.

# Harness Contract

Transport binaries depend on `transport-harness`; they provide domain-specific command, server, or router wiring only. Parsing, MCP setup, HTTP error mapping, and shared output mechanics belong in the harness. The initial harness lives beside `memory-kernel`, subject to the foundation ticket's placement decision.

# Consumption

`workflow-tools` and target repositories depend on the public domain crate for library use. Installers build the required bin features, for example `cargo install --path crates/ticket --features cli,mcp,http`. Frontends remain separate packages depending on the public crate library and do not become binary targets of the domain crate.

# Verification

The reference workspace must compile with no features and with `cli,mcp,http` enabled. The public library test verifies the internal API re-export; each binary emits a transport-specific proof message through the shared harness.

# Traceability

Implements ticket `0da6894c-dcbb-4196-8ac7-b6fae7c40ec9`; the required harness is tracked by ticket `dbe0e955-c1b4-414d-820c-10c3fbbb5d3d`.