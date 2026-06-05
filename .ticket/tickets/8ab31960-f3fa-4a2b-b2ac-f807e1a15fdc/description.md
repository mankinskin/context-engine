# Problem

The repository has the pieces of a query engine, but not the complete interface needed for focused ticket discovery.

Current technical state:

- `memory-api` already owns a shared query AST and Tantivy translation layer.
- `ticket-api` already owns workflow ranking and deterministic candidate ordering.
- CLI, MCP, and HTTP adapters still split query concerns across separate flag sets, ad hoc title filtering, equality-only field filters, and transport-specific request structs.

This follow-on work should turn those building blocks into one more complete query interface instead of layering more bespoke filter flags onto each command.

# Scope

1. Extend the shared query model and parser so it can represent the approved expressive query contract:
   - boolean combinators (`and`, `or`, `not`)
   - deep field predicates
   - comparison operators (`contains`, `gt`, `gte`, `lt`, `lte`, range, and any required existence predicates)
   - text-search clauses
2. Apply that shared query model to ticket surfaces:
   - `ticket list`
   - `ticket search`
   - workflow selection surfaces used by `ticket next` and `ticket board show`
   - MCP request types for ticket query / workflow discovery
   - HTTP query types for ticket search and workflow-next discovery
3. Add explicit ordering support where the spec allows it, while preserving specialized workflow ranking where that remains the default contract.
4. Keep ordering deterministic and close to iterator-style semantics: query/scope filtering first, then ordering, then truncation/limit.
5. Add focused regression coverage for parser behavior, storage evaluation, adapter parity, and workflow filtering/ordering interactions.

# Acceptance Criteria

- The shared query layer supports the new operators and deep-field predicates defined by the spec.
- Ticket adapters no longer duplicate incompatible query semantics for list/search/next/board flows.
- Workflow surfaces can consume the expressive query contract for narrowing candidate sets, with ordering behavior matching the spec.
- Explicit ordering clauses work where supported and preserve deterministic tie-break behavior.
- Tests cover combinators, comparisons, text search, deep fields, and ordering interactions.

# Dependencies and Coordination

- Depends on the new expressive-query specification ticket.
- Should align with completed selector and workflow-ordering work rather than replacing their contracts by accident.
- Overlap with existing one-off filter tickets should be resolved by linking or closing duplicates once implementation scope is clear.

# Likely Surfaces

- `memory-viewers/memory-api/crates/memory-api/`
- `memory-viewers/memory-api/crates/ticket-api/`
- `memory-viewers/memory-api/tools/cli/ticket-cli/`
- `memory-viewers/memory-api/tools/mcp/ticket-mcp/`
- `memory-viewers/memory-api/tools/http/ticket-http/`
- related tests under `memory-viewers/memory-api/**/tests/`
