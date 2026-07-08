<!-- aligned-structure:v1 -->

# Summary

Define a reusable `peek-api` layer that owns token-bounded file inspection and structural skeleton rendering so CLI and MCP transports share one contract and one error model.

## Behavior Story

Define a reusable `peek-api` layer that owns token-bounded file inspection and structural skeleton rendering so CLI and MCP transports share one contract and one error model.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# agent-tooling/peek-api

## Goal

Define a reusable `peek-api` layer that owns token-bounded file inspection and structural skeleton rendering so CLI and MCP transports share one contract and one error model.

## Problem

The current `peek` behavior is concentrated in `tools/cli/peek-cli/src/main.rs`. That keeps the bounded-read, grep, count, skeleton, and repo-map-adjacent logic tied to one transport and makes the next adapter surface duplicate parsing, validation, and filesystem behavior.

The repository's standard shape is for `*-api` crates to own request handling, validation, and domain behavior while CLI and MCP crates stay thin. `peek` does not follow that shape yet.

## Scope

This spec covers the shared behavior contract for:

- bounded file reads by explicit mode (`count`, `grep`, `window`, `head`, `tail`, `all`)
- skeleton rendering for supported file types
- shared request and response models used by `peek-cli` and `peek-mcp`
- transport-independent validation and filesystem error mapping

The contract requires `peek-cli` to remain a presentation adapter and `peek-mcp` to remain a transport adapter over the same API behavior.

## Non-goals

- defining repository-specific folder-tree and repo-map generation behavior beyond the shared primitives it depends on
- changing the user-facing semantics of existing `peek-cli` modes unless a separately tracked bug or spec requires it
- introducing a second command or validation model that exists only for MCP

## Acceptance Criteria

1. A dedicated `peek-api` crate exists and owns the current bounded-read and skeletonization behavior that was previously embedded in `peek-cli`.
2. `peek-api` exposes stable request and response types that both CLI and MCP transports can call without duplicating validation or file-loading logic.
3. `peek-cli` remains behaviorally compatible for existing inspection modes while delegating execution to `peek-api`.
4. `peek-mcp` exposes named tools for the same core operations and delegates to `peek-api` rather than reimplementing inspection logic.
5. Missing files, invalid line ranges, invalid regex input, and unsupported skeleton targets produce transport-appropriate responses from one shared API error model.
6. The resulting crate layout is clear enough that follow-on folder-skeleton and repo-map work can extend `peek-api` instead of reopening transport-specific code paths.

## Traceability

Primary implementation ticket:

- `C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/06cfe998-c2e1-48a4-83e9-11e85e7c40f4`

Closely related tickets:

- `C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/f93e5db5-4f20-4e23-8832-498c4591938f`
- `C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/9b9df133-d809-4900-b56a-afae4efcdd08`

The repo-map tree work is a follow-on contract slice that depends on this spec's shared API boundary instead of extending `peek-cli` directly.

## Validation Evidence

Minimum validation expected before review:

- `cargo test -p peek-api`
- `cargo test -p peek-cli`
- `cargo test -p peek-mcp`
- `cargo build -p peek-cli -p peek-mcp`

Focused evidence should demonstrate:

- bounded range validation parity after extraction
- grep result stability after transport refactoring
- skeleton output parity before and after extraction
- MCP success and invalid-input cases exercising the same API behavior

Current implementation evidence:

- passed `cargo test -p peek-api`
- passed `cargo test -p peek-cli`
- passed `cargo test -p peek-mcp`
