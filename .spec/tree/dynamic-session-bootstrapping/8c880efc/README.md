<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=8c880efc-7083-4e1d-bf06-96b8254be913 slug=memory-api/session-api/dynamic-session-bootstrapping digest=00540fe5ba6d -->

# Dynamic session bootstrapping and just-in-time context routing

- slug: `memory-api/session-api/dynamic-session-bootstrapping`
- component: session-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/8c880efc-7083-4e1d-bf06-96b8254be913/spec.toml`

## Summary

Turn `session-api` from a capture/archive-only store into a runtime "cognitive workspace" that lets an agent bootstrap every session, proactively gather selective context (rules, specs, tickets) acro…

## Acceptance Criteria Excerpt

1. A frozen `session_context.json` schema (URN refs, no `current_mode`) is referenced by every child spec. 2. `session_init`/`pin`/`unpin`/`view` signatures are defined headers-only and implemented verbatim by CLI and MCP. 3. Pinning records a usage event and supports a per-enti…

## Navigation

- Parent: _(root)_
- Children: [memory-api/curation/entity-usage-and-feedback](entity-usage-and-feedback/71b81a55/README.md), [memory-api/session-api/cascade-context-gathering](cascade-context-gathering/fda5c915/README.md), [memory-api/session-api/minimal-bootstrapper-selective-loading](minimal-bootstrapper-selective-loading/a28a88db/README.md), [memory-api/session-api/runtime-session-context](runtime-session-context/709f067a/README.md)
