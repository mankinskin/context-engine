# Scaffold log-viewer-dioxus crate and trunk build

## Problem

The current log-viewer frontend is a Preact/Vite application. Per the Dioxus Viewer Platform epic (`35a6d14b`), all viewer frontends should be ported to Rust/Dioxus 0.7 compiled to WASM via `trunk`. This ticket creates the Dioxus crate scaffold for log-viewer.

## Scope

1. Create `tools/viewer/log-viewer/frontend/dioxus/` with:
   - `Cargo.toml` — depends on `viewer-api-dioxus`, `dioxus`, `gloo-net`, `tracing-wasm`
   - `Dioxus.toml` + `index.html` — trunk config (follow ticket-viewer pattern)
   - `src/main.rs` — app entry point calling `init_tracing_wasm()` and mounting the root component
   - `src/app.rs` — stubbed `App` component (renders "Log Viewer" heading)
   - `src/api.rs` — typed `LogViewerBackend` trait + `HttpLogViewerBackend` impl calling the log-viewer HTTP API
   - `src/types.rs` — mirror of `LogEntry`, `LogFile`, `LogAnalysis` from `tools/viewer/log-viewer/src/types.rs`
2. Add to `viewer-ctl.toml` under the `log-viewer` server's `frontend` entry: point `build_output` to `tools/viewer/log-viewer/frontend/dioxus/dist`.
3. Add `tools/viewer/log-viewer/frontend/dioxus` to workspace `Cargo.toml` members.
4. Verify `trunk serve` starts in `tools/viewer/log-viewer/frontend/dioxus/` with hot reload.

## Acceptance Criteria

- `cargo check --target wasm32-unknown-unknown -p log-viewer-dioxus` passes.
- `trunk serve` starts and renders the stub "Log Viewer" page.
- `viewer-ctl prepare log-viewer` builds the Dioxus frontend and installs it to `~/.context-engine/static/log-viewer/`.
- The existing Preact frontend still builds (no regression).

## Files (new)

- `tools/viewer/log-viewer/frontend/dioxus/Cargo.toml`
- `tools/viewer/log-viewer/frontend/dioxus/Dioxus.toml`
- `tools/viewer/log-viewer/frontend/dioxus/index.html`
- `tools/viewer/log-viewer/frontend/dioxus/src/main.rs`
- `tools/viewer/log-viewer/frontend/dioxus/src/app.rs`
- `tools/viewer/log-viewer/frontend/dioxus/src/api.rs`
- `tools/viewer/log-viewer/frontend/dioxus/src/types.rs`

## Depends on

- `35a6d14b` Epic: Dioxus Viewer Platform (viewer-api-dioxus foundation tickets)
- [LOG-1a] (log files need to exist for the frontend to show anything)
