# 02. Specification and Structural Projection Extractors

## Outcome

Implement adapters that normalize authoritative specification facts and two
separate structural projections: Git repository/submodule containment and
Cargo workspace/crate membership or dependency. Every emitted fact has a type,
projection, stable identifier, and source location.

## Inputs

- [Conceptual input contract](01-conceptual-input-contract.md).
- [Git submodule declaration](../../.gitmodules), [root Cargo workspace](../../Cargo.toml), and [nested context-stack workspace](../../context-stack/Cargo.toml).
- [spec-api implementation conventions](../../workflow-tools/spec/crates/spec-api/src/) and [Peek API boundary](../../.ticket/tickets/06cfe998-c2e1-48a4-83e9-11e85e7c40f4/description.md).

## Required Contract

- Git containment, Cargo workspace/crate membership, and Cargo dependency are
  distinct projections with separately typed edges and source evidence.
- Invalid or absent source manifests produce typed diagnostics instead of
  silently omitted facts.
- Declarative workflow sources may yield normative workflow claims.
- Telemetry is only a labeled illustrative example and cannot create a
  normative claim.

## Non-Goals

- Merge these projections into an unlabeled tree.
- Parse TypeScript, JavaScript, Python, documentation, CLI/MCP surfaces, or
  tests.
- Turn `Peek` into a universal repository graph or consume live telemetry.

## Validation

Use fixture repositories to independently prove submodule containment,
workspace membership, crate membership, and dependency edges. Assert that a
dependency cannot be rendered or queried as containment; assert diagnostics
for invalid manifests; assert telemetry cannot yield a normative workflow
claim. The target commands are `cargo test -p presentation-api --test
extraction_adapters` and `cargo test -p presentation-api --test
typed_projections` once the API crate exists.

## Tracking

Ticket `693763fc-e4c1-4c93-b39f-5e0958b57d19` depends on DB-backed ticket `1500a9e6-293f-4803-969d-0dcabeaa470a`; resolve both through `mcp_ticket_get_ticket`. Before a cold session reads the cited source, run `git -C workflow-tools submodule update --init spec` and preflight `test -f workflow-tools/spec/crates/spec-api/src/lib.rs`.
