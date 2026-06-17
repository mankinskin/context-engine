# [ticket-vscode] Integrate Rust/WASM core into TS hosts and remove replaced legacy logic

The Rust/WASM core (`ticket-vscode-core`) is built, tested, and packaged, but it is **not yet wired into the live extension code path**. Today only `core_version()` is called as a smoke check in `src/extension.browser.ts`; none of the ported domain functions are invoked from TypeScript, and the equivalent TS logic is still the active implementation in both the desktop (`main`) and browser hosts. The earlier tickets stopped short of swapping the call sites, so the "port" is not actually in effect and there is no legacy code that has been replaced yet.

This ticket closes that gap: route the live hosts through the WASM core and delete the TypeScript that the core supersedes, so the replacement is real and observable at every endpoint.

## Scope

Wire the WASM core into both extension entrypoints and replace the duplicated TS logic with calls into the core:

- Filtering — `ticket_matches` replaces the TS substring/state filter logic.
- Dependency maps — `DependencyMaps::build` replaces `_depsOf` / `_parentOf` / `_hasParent` construction in `ticketProvider.ts`.
- State grouping / root detection — `build_state_groups` replaces `buildStateGroups` in `ticketProvider.ts`.
- Host-kind capability gates — `supports_server_control` / `supports_browser_bridge` / `supports_file_browsing` (+ `HostKind`) replace the ad-hoc gating, including command contribution/runtime gating.
- URL / label derivation — `ticket_viewer_url` and `ticket_display_label` replace the inline TS equivalents.

Then remove the now-dead TypeScript that the core supersedes (no parallel TS implementation left behind). Keep host-only seams (`vscode`/Node I/O, capability adapters, tree-item rendering) in TS.

## Dependency links

- Tracker: [6d07d610 Rust/WASM port track](../6d07d610-75c1-448a-afd5-6ae15098ca21/ticket.toml)
- Depends on: [011563c2 Extract portable Rust core](../011563c2-59e7-48f1-a61f-d8fdc80d2f6e/ticket.toml)
- Depends on: [bfafde19 Replace Node-bound behaviors with host capability adapters](../bfafde19-ddf7-47ef-966e-a1135be4efd6/ticket.toml)
- Depends on: [362448d4 Add dual-host packaging, bundling, and extension test harnesses](../362448d4-ccf1-4b9d-90f3-d4577da83a65/ticket.toml)
- Blocks: [6de424b0 Validate Rust/WASM parity across desktop, web, and remote hosts](../6de424b0-68ec-43c7-9d70-eb8d17305ab3/ticket.toml) — validation must run against the replaced code path, not the legacy TS.

## Acceptance criteria

- [ ] Both hosts load the WASM core via the shared loader and call into it for filtering, dependency-map construction, state grouping, host-kind gates, and URL/label derivation (no remaining smoke-check-only usage).
- [ ] The TypeScript logic superseded by the core is deleted, not left as a parallel implementation.
- [ ] Command contribution/runtime gating for capability-absent commands (`startServer`, `bridge*`) is driven by the core host-kind gates.
- [ ] `cargo test -p ticket-vscode-core`, `cargo check --target wasm32-unknown-unknown -p ticket-vscode-core`, `npm run build`, and `npm run test:unit` all pass after the swap.
- [ ] Spec `ticket-vscode/rust-wasm-port` (a592900c) traceability is updated to reflect that the core is the live implementation, and the validation ticket 6de424b0 validates the replaced code at all endpoints.

## Frozen architecture boundary

The Rust/WASM architecture is frozen in spec `ticket-vscode/rust-wasm-port` (a592900c, state `reviewed`). The Module Portability Matrix and Host Capability Contract define which logic moves into the core and which stays as a host seam; this ticket implements that crossing, it does not change the boundary.