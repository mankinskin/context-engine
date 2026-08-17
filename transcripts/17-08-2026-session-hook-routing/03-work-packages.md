# Session Hook Routing Work Packages

## Work Packages

| ID | Title | Objective | Owning paths | Discharges | Prerequisites | Unit-test done-criterion |
| --- | --- | --- | --- | --- | --- | --- |
| WP1 | Reproduce routing incident | Fixture records hook payload, resolver result, proxy rewrite, and sole mutated store for default/absolute selectors. | `memory-api/crates/session-capture-hook/src/main.rs`; `memory-api/crates/session-workspace-resolver/src/lib.rs`; `memory-api/tools/mcp/mcp-toolmon/src/proxy.rs` | F6 | None | Focused unit tests distinguish registered, main-only, and main-store mutation outcomes; non-reproduction passes as diagnosis. |
| WP2 | Decompose capture hook | Extract capture, store resolution, provisioning, spill statistics, feedback, ticket synthesis, and metrics without routing changes. | `memory-api/crates/session-capture-hook/src/main.rs`; `memory-api/crates/session-capture-hook/src/args.rs` | F2, D3 | WP1 | `cargo test -p session-capture-hook` passes preserved behavior tests for every boundary. |
| WP3 | Add main registry | Add ignored UUID-to-path registry and branch-only committed session declaration with typed validation. | `memory-api/crates/session-api/src/model.rs`; `memory-api/crates/session-api/src/store/config/worktree_runtime.rs`; `memory-api/crates/session-workspace-resolver/src/lib.rs` | D1, F3, F7 | WP1 | `cargo test -p session-api -p session-workspace-resolver` passes main-only, registered, malformed, and branch-mismatch cases. |
| WP4 | Add deferred transfer | Add `worktree-ctl transfer-session`: commit/read `transfer-pending`, instantiate/validate, then write/read registry without rollback. | `tools/worktree/worktree-ctl/Cargo.toml`; `tools/worktree/worktree-ctl/src/main.rs`; `memory-api/crates/session-worktree-provision/src/lib.rs`; `memory-api/crates/session-worktree-provision/src/policy.rs` | D2, D4, F4 | WP3 | `cargo test -p worktree-ctl -p session-worktree-provision` passes ordered transfer and post-commit creation-failure cases. |
| WP5 | Route hook and proxy by registry | Replace eager provisioning and positional routing with typed main-only/registered resolution; bad registrations block. | `memory-api/crates/session-capture-hook/src/main.rs`; `memory-api/crates/session-workspace-resolver/src/lib.rs`; `memory-api/tools/mcp/mcp-toolmon/src/proxy.rs` | D1, D2, F6, F7 | WP1-WP4 | `cargo test -p session-capture-hook -p session-workspace-resolver -p mcp-toolmon` passes main-only, registered-only, and no-fallback tests. |
| WP6 | Add hook-free deregistration | Add `worktree-ctl deregister-session` with clean/equivalent/no-lease checks, forward unset commit, read-back, and idempotence. | `tools/worktree/worktree-ctl/src/main.rs`; `memory-api/crates/session-worktree-provision/src/policy.rs`; `memory-api/crates/session-api/src/store/config/worktree_runtime.rs` | D4, F5 | WP3-WP5 | `cargo test -p worktree-ctl -p session-worktree-provision -p session-api` passes precondition, reconciliation, and main-only idempotence tests. |
| WP7 | Migrate legacy sessions | Explicitly migrate nested/legacy records and reject path-only debris from normal routing. | `memory-api/crates/session-workspace-resolver/src/lib.rs`; `tools/worktree/worktree-ctl/src/main.rs`; `memory-api/crates/session-worktree-provision/src/policy.rs` | F3, F7 | WP3, WP4, WP6 | `cargo test -p session-workspace-resolver -p worktree-ctl -p session-worktree-provision` passes nested, legacy, missing-registry, mismatch, and debris cases. |
| WP8 | Update lifecycle guidance | Replace eager/positional instructions with registry, transfer, blocked recovery, deregistration, and migration; reference D3 guidance only. | `.agents/instructions/session/worktree-provisioning.instructions.md`; `.agents/instructions/session/session-identity-and-handoff.instructions.md`; `.agents/instructions/orchestration/code-quality.instructions.md` | D1-D4, F3, F5, F7 | WP4-WP7 | Focused repository documentation validation passes. |

WP2 lands before WP5: F2 requires a reviewable capture-hook boundary, while routing must not dictate module layout.

## Invariant Coverage Map

`02-routing-operating-model.md` has no numbered invariants; the following cover every enforceable rule.

| Invariant | Package | Unit-test assertion |
| --- | --- | --- |
| I1 Main registry is authoritative/local-only. | WP3 | Canonical registry round-trip succeeds; malformed data fails. |
| I2 Committed record contains branch, never path. | WP3 | Serialization lacks canonical path. |
| I3 Main-only captures/resolves in main. | WP5 | Typed main target writes only main store. |
| I4 Registered captures/resolves only registered worktree. | WP5 | Valid entry rewrites hook/proxy target. |
| I5 Bad registration blocks, with no fallback. | WP5 | Error and zero fallback writes. |
| I6 Transfer commit/read precedes instantiation. | WP4 | Ordered transfer trace. |
| I7 Creation failure blocks; later unset is forward-only. | WP4 | Forced failure retains pending record. |
| I8 Deregistration is hook-free and preconditioned. | WP6 | Failed preconditions prevent unset. |
| I9 Idempotence requires both stores main-only. | WP6 | Divergence requires reconciliation. |
| I10 Legacy/positional sessions migrate explicitly; debris never routes. | WP7 | Path-only fixture is migration-required. |

## Ticket Relationships

| Ticket | Relationship | Reason |
| --- | --- | --- |
| [200e9ecc session_check_in rejects a single nested worktree unless a persisted assignment exists](.ticket/tickets/200e9ecc-d61a-4b1a-a3a6-a9dd1e77d915/ticket.toml) | supersede | WP3/WP7 replace positional assignment with D1 registry and migration. |
| [0afe45b5 [ticket-api][session-api] Store resolution enumerates .worktrees/* and mis-anchors the active store](.ticket/tickets/0afe45b5-9ec8-4f4a-af74-f46f06cc7516/ticket.toml) | unaffected | Done prior art and WP1 evidence. |
| [2b657154 Handle unregistered worktree debris during removal](.ticket/tickets/2b657154-df78-4bb3-807a-66c9ff811ceb/ticket.toml) | depend-on | WP7 preserves non-routable debris; removal remains 2b657154 scope. |
| [326bfe38 [workflow][session-worktree] Add worktree-first session guidance and hooks](.ticket/tickets/326bfe38-a34e-4eca-8079-6f66f83bf97f/ticket.toml) | supersede | WP8 replaces eager/worktree-first guidance. |
| [ba4aaa9c [workflow-tools][per-tool] Extract ticket tool as a single `ticket` domain crate (api + transport bins) + viewer/vscode frontends](.ticket/tickets/ba4aaa9c-d270-4cfc-a1e2-395634608371/ticket.toml) | unaffected | In-review extraction artifact is WP1 evidence only and out of scope. |

## Non-Goals

- D5: no full E2E, browser, or Playwright packages.
- No cleanup, revert, or modification of the `ba4aaa9c` artifact; no ticket-domain work or ticket-viewer merge decision.
- No edit or duplication of `.agents/instructions/orchestration/code-quality.instructions.md`.

## Risks

| Package | Failure mode | Cheapest signal |
| --- | --- | --- |
| WP1 | Fixture assumes cause. | Invalid-registry case yields diagnostic, not main write. |
| WP2 | Extraction changes order. | Existing capture-hook unit tests. |
| WP3 | Tracked path leak. | Serialization assertion. |
| WP4 | Rollback or unclear partial state. | Forced-create pending-state test. |
| WP5 | Fallback routing. | Zero nonselected-store writes. |
| WP6 | Unsafe removal. | Per-precondition unit cases. |
| WP7 | Positional routing survives. | Path-only migration-required test. |
| WP8 | Docs name unavailable behavior. | Documentation validation. |
