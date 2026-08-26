<!-- aligned-structure:v2 -->

# Consolidated Workflow-Tool Entity Stores

## Target Code Location

- [memory-kernel discovery](../../../../workflow-tools/memory-kernel/src/discovery.rs)
- [memory-kernel workspace resolution](../../../../workflow-tools/memory-kernel/src/workspace.rs)
- [domain store adapters](../../../../workflow-tools/ticket/crates/ticket-api/src)

## Naming Conventions

- Canonical store root: `<consumer-workspace>/.workflow-tools/<domain>/`.
- Domain names: `ticket`, `spec`, `test`, `session`, `feedback`, `log`, `doc`, `rule`, and `audit`.
- Legacy root: `<consumer-workspace>/.<domain>/`.

## Requester Input

> Move tool stores into `.workflow-tools/<domain>/` folders rather than the current `.<domain>/` pattern so all entity stores are collapsed behind one folder.

## Reading Order

1. [memory-kernel discovery](../../../../workflow-tools/memory-kernel/src/discovery.rs) - shared recursive store detection.
2. [memory-kernel workspace resolution](../../../../workflow-tools/memory-kernel/src/workspace.rs) - workspace-to-store resolution.

## Responsibility

Define one stable consumer-workspace namespace for every workflow-tools entity store and a migration rule that avoids split-brain writes during adoption.

## Interfaces And Dependencies

Every CLI, MCP server, viewer, and domain API resolves stores through the shared namespace contract. The contract is independent of the workflow-tools source checkout location.

## Behavior

- A canonical store resolves to `.workflow-tools/<domain>`.
- When the canonical store is absent and the legacy `.<domain>` exists, reads discover the legacy store and return a deprecation diagnostic naming the canonical destination.
- Entity creation, updates, indexes, and generated sidecars always target the canonical path.
- A checked migration command moves a legacy store atomically where possible, verifies the destination, and leaves the source untouched on failure.

## Boundaries And Failure Cases

- A workspace containing both canonical and legacy stores is rejected as ambiguous until the legacy store is migrated or removed.
- A migration never merges two populated stores implicitly.
- Test fixtures exercise legacy read compatibility and canonical writes without mutating tracked fixture inputs.

## Provider/Consumer Contract

If this specification is implemented, every workflow-tools consumer can rely on a compact, domain-addressable store layout and on writes never returning to legacy hidden directories.

## Examples

A ticket API write for a consumer workspace at `/repo` targets `/repo/.workflow-tools/ticket/`; a read-only query may inspect `/repo/.ticket/` only when `/repo/.workflow-tools/ticket/` is absent and reports the legacy-path diagnostic.

## Evidence

- Required guards: focused memory-kernel discovery/resolution tests; domain API and CLI/MCP write-path tests; migration command preflight/apply/rollback tests; fixture read-back assertions.
- Positions: [memory-kernel discovery](../../../../workflow-tools/memory-kernel/src/discovery.rs) is partial because it recognizes only flat hidden markers; domain adapters are partial because they embed legacy names.

## Scope

In scope: namespace resolution, compatibility reads, checked migration, tracked store relocation, fixtures, generated configuration, and documentation. Out of scope: changing entity schemas or cross-workspace transfer semantics.