# Extract `ticket` as the first per-tool {domain} crate (pilot)

Action 3 of the workflow-tools extraction epic (69eb4118). Reviewer selected
`ticket` as the FIRST per-tool extraction target on 2026-07-25. Gates cleared:
T-PROVISION done; memory-fixtures substrate stable + externally consumable
(T-SHAREDLIBS in-review).

## Goal

Stand up a public `ticket` domain crate with an internal `ticket-api`
re-export and feature-gated CLI/MCP/HTTP binaries built on the shared
transport-harness, consuming the stabilized shared libs. Extract this ONE tool
end-to-end before scaling to other domains.

## Crate shape (per WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md + SPEC-HARNESS-KERNEL e5294ae5)

- Public `ticket` crate; internal `ticket-api` re-exported.
- `default = []` (slim); `cli`, `mcp`, `http` independently selectable features.
- CLI/MCP/HTTP binaries built on `transport-harness` (branch-pinned git dep to
  memory-kernel, main).
- Consume `memory-fixtures` as a branch-pinned git dev-dependency
  (github.com/mankinskin/memory-fixtures, main) per its README convention.
- transport-harness stays transport-neutral; the ticket domain owns command
  dispatch, MCP tool/handler registration, and HTTP router registration.

## First validation

- `cargo build -p ticket` (slim) builds.
- `cargo build -p ticket --features cli,mcp,http` builds all three binaries.
- The reference-proof pattern (one op across three transports + error envelope +
  HTTP status) holds for a representative ticket operation.

## Dependencies

- depends_on T-SHAREDLIBS (1c452ff1) — shared substrate.
- part of EPIC 69eb4118.
