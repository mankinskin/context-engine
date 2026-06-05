# Problem

The current ticket query contract is not expressive enough for focused discovery.

What exists now:

- the shared `memory-api` query AST only supports `Expr::And`, `Expr::Fts`, and exact/range field predicates
- ticket workflow surfaces still expose narrow bespoke filters instead of a common query object
- implicit ordering exists, but the API surface does not define how explicit ordering composes with default ranking

What is needed now:

- logical combinators such as `and`, `or`, and `not`
- deep field search over structured ticket metadata, not only flat equality predicates
- comparison operators such as `contains`, `gt`, `gte`, `lt`, `lte`, and clear range behavior
- first-class text-search clauses because substring and text search are common operator workflows
- ordering that stays close to Rust iterator/pipeline semantics where practical: selection narrows first, ordering applies after filtering, multi-key ordering composes lexicographically, and deterministic tie-breakers are always defined
- workflow surfaces (`next`, `board`, MCP, HTTP`) that can consume the same query contract without inventing a second language

# Scope

1. Specify a canonical query model for ticket discovery across list, search, board, next, MCP, and HTTP.
2. Define the minimum operator set and the exact semantics for:
   - equality vs contains vs full-text search
   - deep field paths and dynamic field namespaces
   - boolean composition (`and`, `or`, `not`)
   - scalar comparisons and range semantics
   - null / missing-field handling
3. Specify ordering behavior:
   - default ordering for `list`, `search`, `next`, and `board`
   - explicit order clauses and multi-key lexicographic chaining
   - deterministic stable tie-breaks
   - where workflow-specific ranking remains authoritative versus where caller-specified order is allowed
4. Define a transport-safe shape for CLI, MCP, and HTTP so the same query can be expressed without each adapter inventing new field names.
5. Document how the model should preserve a Rust-like iterator mental model while still providing a good operator UX.

# Acceptance Criteria

- Canonical spec and docs define the expressive ticket query language and ordering contract.
- The spec names the supported logical operators, comparison operators, deep-field path rules, and text-search behavior.
- The spec defines explicit ordering semantics and how they compose with current workflow ranking.
- The spec identifies which existing surfaces reuse the shared query model versus retain specialized defaults.
- The spec includes a validation matrix spanning parser tests, storage/search tests, CLI/MCP/HTTP parity tests, and workflow-surface regression cases.

# Likely Surfaces

- `memory-viewers/memory-api/.spec/`
- `memory-viewers/memory-api/crates/memory-api/src/model/query.rs`
- `memory-viewers/memory-api/crates/memory-api/src/storage/search.rs`
- `memory-viewers/memory-api/crates/ticket-api/`
- `memory-viewers/memory-api/tools/cli/ticket-cli/`
- `memory-viewers/memory-api/tools/mcp/ticket-mcp/`
- `memory-viewers/memory-api/tools/http/ticket-http/`
