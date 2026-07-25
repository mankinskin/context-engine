Research/design precursor for ticket 2cc7680c-7f19-4ad7-8658-29920e60ce1c, created from the 2026-07-25 review (verdict: Needs changes — "do more research and design before implementing").

## Purpose

Before replacing the trivial transport reference with realistic domain wiring, produce a concrete design that resolves the open questions the reviewer flagged. No implementation of the reference wiring may begin until this design is accepted.

## Design decisions already locked by review

- Durable proof home: the reference proof moves into memory-kernel integration tests (not the context-engine reference workspace). The context-engine reference, if retained, becomes a thin consumer.
- Transport coverage: a single realistic domain operation must be exposed through CLI, MCP, and HTTP (same op, three transports).
- Assertions: tests must assert both success output AND the harness error envelope + HTTP status mapping (the error path is mandatory, since it is the mechanic that most justifies the shared harness).
- Feature slimness: must not regress the accepted parent criterion default = [] with independently selectable CLI/MCP/HTTP.

## Open questions to resolve in this design ticket

- What is the smallest realistic domain operation that is not placeholder product code?
- Exact success output shape and exact error envelope/status codes to assert per transport.
- Directory/crate layout for memory-kernel integration tests and how they resolve the harness.
- Whether/how the context-engine reference is retained, slimmed, or removed once the proof moves.

## Dependencies

Depends on 9451f439 (submodule discoverability) and f10f52e4 (canonical spec in memory-kernel), because the proof home and dependency paths resolve through those decisions.

## Definition of done

A written, reviewed design (spec section or ticket description) that unambiguously specifies the domain op, transport surfaces, output/error assertions, and memory-kernel test layout, ready to hand to 2cc7680c implementation.

## Design (2026-07-25): ACCEPTED

The full design is recorded durably as section "Reference-proof integration tests (design)" on the canonical memory-kernel spec e5294ae5-6bff-44dc-81a9-24a44615b775 (memory-kernel/.spec/specs/e5294ae5-6bff-44dc-81a9-24a44615b775). Summary:

- Domain op: `describe(id) -> Item` over a tiny fixed in-test registry `{ "harness" => "Shared transport harness" }`. Unknown id -> `DomainError::NotFound(id)`. Smallest op exercising input parsing, a structured success payload, and a per-transport-normalized domain error.
- Success shape: `{"id":"harness","summary":"Shared transport harness"}`. CLI via `Output::json` + one trailing newline; MCP returns same item; HTTP `200 OK` with that body.
- Error per transport: domain message `unknown item: <id>`. CLI -> `HarnessError::Domain` (args errors -> `HarnessError::Arguments`); MCP -> harness domain error with same message; HTTP -> `404 NOT_FOUND` with `HttpError` envelope `{"code":"not_found","message":"unknown item: <id>"}` (assert BOTH status and envelope).
- Test layout: `memory-kernel/crates/transport-harness/tests/reference_proof.rs`, an integration test in the harness crate; run gate `cargo test -p transport-harness --all-features`; per-transport runs use single features. Fixture domain inline in the test module (dev-only, never in library surface, `default = []` untouched). CLI via `cli::run_from` (no spawn); MCP via in-process handler routing; HTTP via ephemeral `127.0.0.1:0` + real requests.
- context-engine reference: retained as a thin compile-only consumer proving the branch-pinned git dep resolves and all three gated binaries build/run; no behavioral assertions duplicated there.

All four open questions resolved; all locked decisions honored. Ready to hand to 2cc7680c.

### State move blocker

`update_ticket to_state` returns `store error: no schema for type 'task'` — the known schema defect. Ticket remains in `new`; state not falsified.