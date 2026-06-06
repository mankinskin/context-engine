# Spec store: memory-api submodule has its own .spec

- The `spec-mcp` MCP tools index the context-engine root `.spec` (~652 entities)
  and do NOT surface specs in `memory-viewers/memory-api/.spec/`. Searches there
  return 0 even after `spec_scan` / `spec_add_root`.
- For ticket-query / ticket-api / memory-api specs, use the built CLI from the
  memory-api dir:
  `cd memory-viewers/memory-api && ../../target/debug/spec.exe <cmd>`
  (the `spec.exe` binary lives at the workspace root `target/debug/`).
- Spec format on disk: `<.spec>/specs/<uuid>/{spec.toml, body.md, history.ndjson}`.
- Create via `spec.exe create --title --slug --component --scope --parent --body-file --json`.
- Ticket-query expressive query+ordering contract:
  spec `08aa283e-34ee-47d4-83bc-4c4311a9c85f`
  slug `ticket-query/expressive-query-and-ordering`.

## ticket-query impl progress (ticket 8ab31960, in-implementation)

- Slice 1 DONE: shared parser layer. `query.rs` now has `CompareOp`
  (Eq/Contains/Gt/Gte/Lt/Lte/Range/Exists), `ValueExpr::Empty`, and
  `Expr::Compare`. `Expr::Field` kept as the Eq/Range backward-compat form.
  Tokens: `key:~`/`key:*v*` (Contains), `key:>`/`>=`/`<`/`<=`, `key:[a TO b]`,
  `key:?` (Exists). Dotted `x.<type>.<field>` normalizes to flat
  `x_<type>_<field>`. tokenize() is now bracket-aware (unquoted ranges work).
  `search.rs expr_to_query` has a `Compare` arm (Contains=substring,
  Exists=regex `.+`, ordering ops degrade to AllQuery pending fast fields).
  Tests: `contracts_query_parser.rs` 16/16 pass.
- NEXT slices: fast-field eval of Gt/Gte/Lt/Lte/Range; explicit `order:f:dir`
  clause + composition with workflow comparator; expose via adapters one at a
  time (list/search/MCP/HTTP); workflow-surface narrowing for next/board.
