<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=3ccdde3a-368c-4655-a6c8-20a58822c83d slug=agent-tooling/peek-api digest=3e9fe04aa3df -->

# peek-api transport layering

- slug: `agent-tooling/peek-api`
- component: agent-tooling
- scope: internal
- state: agent-tooling
- index_ref: `.spec/specs/3ccdde3a-368c-4655-a6c8-20a58822c83d/spec.toml`

## Summary

Define a reusable `peek-api` layer that owns token-bounded file inspection and structural skeleton rendering so CLI and MCP transports share one contract and one error model.

## Acceptance Criteria Excerpt

1. A dedicated `peek-api` crate exists and owns the current bounded-read and skeletonization behavior that was previously embedded in `peek-cli`. 2. `peek-api` exposes stable request and response types that both CLI and MCP transports can call without duplicating validation or f…

## Navigation

- Parent: _(root)_
- Children: _(none)_
