# Design Review Checklist v0.1

Status: review handoff matrix for implementation tickets
Parent ticket: 91e22471-4895-4fcf-bab2-63efd7d9262d

## Gate rule
Implementation tickets should not move beyond `in-progress` until all required design artifacts in their row are accepted.

## Artifact index
- API contract: `../21a1b9ca-c053-4709-8785-e41fb0661c31/assets/design/api-contract-v0.1.md`
- SSE schema: `../09a32876-665c-476c-9587-8dcb3acd6e6a/assets/design/sse-schema-v1.md`
- SSE contract tests: `../09a32876-665c-476c-9587-8dcb3acd6e6a/assets/design/sse-contract-tests-v0.1.md`
- UX wireframes: `../08e3f042-f690-4d0e-907a-b4ffb9508e50/assets/design/wireframes-v0.1.md`
- Auth lifecycle: `../68dfc679-9eb7-48cd-ade5-a452fdc0f01d/assets/design/auth-lifecycle-v0.1.md`
- Hook contract: `../24aa7e5e-1d62-4f35-a4f7-b056a0b8abce/assets/design/hook-contract-v0.1.md`
- Subgraph API semantics: `../e79fdc1f-2bfb-410f-931c-dbb744cd209e/assets/design/subgraph-api-v0.1.md`

## Implementation mapping

### 43dedd9b-46cd-46c7-96f8-6683ded2cc4d
Impl: ticket serve mode (HTTP + auth + workspace-aware ticket endpoints)
Required design inputs:
- API contract
- Auth lifecycle

### 5e68c2e1-e93e-415f-a3c3-c1a396f36395
Impl: live ticket graph stream pipeline (SSE + hooks + conflict events)
Required design inputs:
- API contract
- SSE schema
- SSE contract tests
- Hook contract

### a1259318-f992-44e3-9cdf-0ea4c224f6f3
Impl: viewer-api extraction for shared tree/file/graph server primitives
Required design inputs:
- API contract
- Hook contract
- Subgraph API semantics

### 02dea1fa-828e-4173-aed3-7a0e654e9d81
Impl: ticket-viewer shell reusing doc-viewer tree and file display
Required design inputs:
- UX wireframes
- API contract

### 2772fe5d-3f29-4116-82fe-bf611ea54c58
Impl: hypergraph dependency view reusing log-viewer graph patterns
Required design inputs:
- UX wireframes
- SSE schema
- Subgraph API semantics

### b594864a-008c-423d-bf86-df940ed9dc54
Impl: state styling baseline and per-workspace UI state persistence
Required design inputs:
- UX wireframes
- SSE schema

### 00ee9f46-7d24-4c3e-8961-00ed760e7ca2
Impl: auth token reload and runtime reconfiguration for ticket serve
Required design inputs:
- Auth lifecycle
- API contract

### 6d4d9a66-ed28-45e1-93f6-a6595c4593b3
Validation: ticket-viewer + ticket-serve E2E, scale envelope, and regression suite
Required design inputs:
- All above artifacts finalized

## Review checklist
- [x] Artifact links are valid
- [x] Each implementation ticket has explicit required design inputs
- [x] No implementation ticket depends on undefined contract details
- [ ] Validation ticket scope covers every contract area

## Wave 1 kickoff (completed 2026-03-21)
All Wave 1 impl tickets moved to `in-progress` with concrete kickoff descriptions:
- `43dedd9b` — ticket serve mode: module structure, Axum routes, auth middleware
- `5e68c2e1` — SSE pipeline: HookEmitter, StreamBroker, ReconcileLoop, event types
- `a1259318` — viewer-api extraction: request_id, pagination, error envelope, SSE helpers, BearerAuthLayer
- `00ee9f46` — auth token reload: ArcSwap TokenSet, reload endpoint, SIGHUP, diagnostics
