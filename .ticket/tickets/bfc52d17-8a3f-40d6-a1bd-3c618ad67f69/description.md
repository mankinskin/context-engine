## Problem

Split out of MATRIX-FOLLOWUP 15e632f1. That ticket carried two criteria: (a) rewire in-tree consumers to the external memory-fixtures git dep, and (b) make memory-matrix build standalone against externally resolved domain crates. (a) is complete and accepted (198 tests passed; memory-fixtures resolved from git in spec-api/ticket-api dev-deps and memory-matrix's normal dep). (b) is structurally blocked, so 15e632f1 was closed and the remaining work lives here.

## Blocker

`cargo build -p memory-matrix` succeeds **in-tree**, but memory-matrix is a cross-domain consumer with path deps on 8 internal api crates plus 6 in-tree transport tool crates:

```
memory-api, ticket-api, spec-api, rule-api, audit-api, session-api, test-api, log-api   (path)
ticket-http, ticket-cli, spec-cli, rule-cli, ticket-mcp, spec-mcp, rule-mcp             (path)
```

It cannot resolve these externally until the corresponding domains are extracted. This unblocks incrementally as domain repos land.

## Unblocking sequence

1. `ticket` consolidation (26da8f59) removes the ticket-cli/ticket-mcp/ticket-http path deps, collapsing them onto the public `ticket` crate.
2. `ticket` repo split (ba4aaa9c) makes `ticket` an external git dep — memory-matrix's ticket-* deps become external.
3. Repeat for spec, rule, and the remaining domains.
4. Re-attempt standalone build after each domain lands.

## Acceptance Criteria

- memory-matrix builds standalone, outside the in-tree domain graph, with every domain dependency resolved externally (git deps, no `path = "../..."` to in-tree domain crates).
- No remaining path dependency on an in-tree domain api or transport tool crate.
- Re-attempt is recorded after each domain extraction, with the current remaining path-dep list captured in implementation notes.

## Non-goals

- Re-reviewing the memory-fixtures rewire; it is accepted and closed under 15e632f1.