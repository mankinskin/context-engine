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