
# GitHub Copilot Instructions

## Source of Truth

All behavioral and workflow guidance (ticket/spec workflow, worktree/branch protocol,
model routing, escalation rules) lives in [AGENTS.md](../AGENTS.md). This file adds
concrete build/architecture facts AGENTS.md does not restate.

Path-scoped guidance lives in [.agents/instructions/](../.agents/instructions/).
Workflow prompts live in [.agents/prompts/](./prompts/).

## Repository Shape

`context-engine` is a Cargo workspace superproject with git submodules, each also a
buildable workspace:

- `context-stack/` — the graph engine crates, layered bottom-up:
  `context-trace` → `context-search` → `context-insert` → `context-read` →
  `context-api`. Each layer only depends on layers below it; check assumptions in
  lower layers before changing upper ones.
- `memory-api/` — generic entity-store infrastructure (`memory-api` crate) plus
  internal domain APIs such as `ticket-api`, `spec-api`, `rule-api`, `audit-api`,
  `test-api`, `session-api`, `feedback-api`, `doc-api`, and `log-api`.
- **Domain-tool architecture.** Pair each internal `{domain}-api` crate with one
  public `{domain}` crate that re-exports the API and solely owns opt-in `cli`,
  `mcp`, and `http` features plus the corresponding binaries. The public CLI uses
  the bare domain name; MCP and HTTP binaries use `-mcp` and `-http`. The reference
  implementation is `memory-api/crates/ticket`: `ticket` (`cli`), `ticket-mcp`
  (`mcp`), and `ticket-http` (`http`). The remaining `tools/cli/*`, `tools/mcp/*`,
  and `tools/http/*` paths are legacy layouts for unmigrated tools; see
  [0da6894c Single domain crate per tool](../.ticket/tickets/0da6894c-dcbb-4196-8ac7-b6fae7c40ec9/ticket.toml).
- `viewer-api/` — shared viewer runtime (tracing, CORS, static-file serving, SSE,
  dev proxy) reused by every viewer binary.
- `memory-viewers/` — viewer binaries (`ticket-viewer`, `spec-viewer`, `doc-viewer`,
  `log-viewer`), each a single-process server embedding its domain HTTP router plus
  an SPA frontend (Preact/Vite or Dioxus/Trunk depending on the viewer).
- `memory-kernel/` — required neutral shared base layer beneath domain repositories.
  Its `memory_kernel` crate owns generic primitives and its sibling
  `transport-harness` crate owns transport-generic startup; domain-specific
  extension traits, command dispatch, server handlers, and router registration
  remain domain-owned. The submodule tracks `github.com/mankinskin/memory-kernel`
  and has a separate push cycle; the root Cargo patch substitutes the checked-out
  kernel and harness only for this development workspace. See
  [kernel layering guidance](../.agents/instructions/engine/kernel-layering.instructions.md).

All of the above are registered as workspace members in the root [Cargo.toml](../Cargo.toml),
so `cargo build`/`cargo test` from the repo root builds/tests everything unless scoped
with `-p <crate>`.

## Build, Test, Lint

```bash
# Build one crate/binary (fast, preferred over full workspace build)
cargo build -p <crate-name>

# Run one test by name (fastest — use this first)
cargo test -p <crate> <test_name> -- --nocapture

# Run all tests in one crate
cargo test -p <crate>

# Full workspace build/test (slow — only after local crate tests pass)
cargo build --workspace
cargo test --workspace

# cargo-make targets: build every binary + every viewer frontend, or scope narrower
cargo make build-all            # native binaries + all frontends
cargo make build-native-tools   # CLI/HTTP/MCP/viewer binaries only
cargo make build-all-frontends  # Vite (doc/log-viewer) + Trunk/Dioxus (ticket/spec-viewer)
```

Formatting uses a repo-root `rustfmt.toml` (max_width=80, vertical fn args/params,
`context-stack/deps` ignored — that's vendored petgraph, don't reformat it).

For tracing-based tests, initialize tracing so graph/token state is legible in logs:

```rust
let _tracing = init_test_tracing!(&graph);
```

Debug failing tests via `target/test-logs/` (fuller trace output than truncated stdout)
or the log-viewer MCP tools (`get_log`, `search_all_logs`, `query_logs`) rather than
re-running with more verbosity.

### Frontend (viewer) validation

Each viewer frontend is its own npm package (Preact+Vite for doc/log-viewer,
Dioxus+Trunk for ticket/spec-viewer). Run lint/typecheck/unit tests in the affected
package, then browser E2E:

```bash
cargo make test-e2e     # spec-viewer release E2E + doc/log-viewer shared smoke suites
```

Ticket-viewer and spec-viewer release E2E run from their own
`frontend/dioxus` directory via `npm run test:e2e:release`. Shared Playwright suites
live under `viewer-api/viewer-api/frontend/dioxus/e2e/shared/`.

## Key Conventions

- **Repo-local tools over ad hoc scripts.** Ticket, spec, rule, and audit state
  live in filesystem-backed stores (`.ticket/`, `.spec/`, `.rule/`, `.test/`,
  `.session/`) manipulated through their CLIs (`ticket`, `spec`, `rule`, `audit`,
  `session`) or MCP servers — never hand-edit the TOML/JSON records directly.
- **Tickets and specs gate implementation, not just document it.** A ticket
  moves through a one-way state machine (`open → planned → in-implementation →
  in-review → done`); entering `planned` freezes the planning parts
  (objective/requirements/design/examples/acceptance_criteria) — corrections go
  through `write-amendment`, not direct edits.
- **Worktree-per-task isolation.** Implementation work happens in its own git
  worktree (`.worktrees/<session-uuid>/<slug>`) on its own branch
  (`agent/<session-uuid>/<slug>`) cut from `main`; never commit to `main` or merge
  your own branch — see AGENTS.md and `.agents/instructions/commit/branch-worktree.instructions.md`.
- **`--toon` output.** Repo CLIs support a compact `--toon` format in addition to
  `--json`; prefer `--toon` for machine-readable output and the `rtk` proxy
  (`rtk <cmd> --toon ...`) to reduce token cost of shell command output.
- **Generated files aren't hand-edited.** `.github/agents/`, `.github/prompts/`,
  and `.github/instructions/` are generated from `.agents/` canonical sources by
  `bootstrap.sh`/`rule sync-targets`; `packages/context-types/src/generated/` TS
  types are generated from Rust `ts-rs` exports via `viewer-ctl gen-types` (or
  `cargo make gen-types`) — edit the source, then regenerate.
- **Repo map.** `repo_map.toon` is a generated structural index; refresh with
  `cargo run -p peek-cli -- . --repo-map --output repo_map.toon` after layout
  changes.

## Optional Copilot CLI MCP Config

Repository-local Copilot CLI MCP config guidance lives alongside the relevant tool README files:

See [context-mcp README](../context-stack/tools/mcp/context-mcp/README.md) for the current repository-local Copilot CLI MCP config example.

## Hooks

Hook reminders are configured in [.github/hooks/](./hooks/).

# RTK — Token-Optimized CLI

**rtk** is a CLI proxy that filters and compresses command outputs, saving 60-90% tokens.

## Install

```bash
cargo install --git https://github.com/rtk-ai/rtk
```

## Rule

Always prefix shell commands with `rtk`:

```bash
# Instead of:              Use:
git status                 rtk git status
git log -10                rtk git log -10
cargo test                 rtk cargo test
docker ps                  rtk docker ps
kubectl get pods           rtk kubectl pods
```

When a repository CLI supports `--toon`, prefer `rtk <cmd> --toon ...` over `rtk <cmd> --json ...` for compact machine-readable output. Use the `toon-format` / `toon-rust` codec for encoding and decoding TOON instead of hand-rolled text transforms.

`rtk` proxies executables; it cannot exec a shell script directly. On Windows, `rtk ./some-script.sh` fails with `os error 193`. Invoke the interpreter explicitly instead: `rtk bash ./some-script.sh`.

## Meta commands (use directly)

```bash
rtk gain              # Token savings dashboard
rtk gain --history    # Per-command savings history
rtk discover          # Find missed rtk opportunities
rtk proxy <cmd>       # Run raw (no filtering) but track usage
```
