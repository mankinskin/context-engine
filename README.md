# context-engine

context-engine is the top-level workspace that coordinates the graph crates in `context-stack`, the viewer and operator tooling in `memory-viewers`, and the repo-local configuration and install surfaces that bind them together.

## Getting Started

Before step 1, install Git, a Rust toolchain with `cargo` and `rustup`, and a
Bash shell (Git Bash on Windows). Node/npm are not required for this bootstrap
or the Trunk-based viewer frontends. `install-deps.sh` installs `ripgrep`,
`rtk`, `trunk`, `cargo-llvm-cov` (and `llvm-tools-preview`), and `cargo-make`.

From a fresh clone, run the bootstrap commands in order:

```bash
bash setup_git.sh
./install-deps.sh
./install-tools.sh --mcp
bash init.sh
bash tools/verify-bootstrap.sh
```

## Repository Map

The root-level `repo_map.toon` file is the compact structural index for this repository.
Refresh it with:

```bash
cargo run -p peek-cli -- . --repo-map --output repo_map.toon
```

Use `peek repo_map.toon --grep "crates"` for bounded inspection or decode/query it
as TOON when automation needs machine-readable structure.

| Child repo or folder | What it contains | Direct README |
| --- | --- | --- |
| [context-stack](context-stack/README.md) | Graph, search, insert, and read crates plus extracted support dependencies. | [context-stack/README.md](context-stack/README.md) |
| [memory-viewers](memory-viewers/README.md) | Viewer binaries, CLI/MCP/HTTP tooling, and shared viewer runtime packages. | [memory-viewers/README.md](memory-viewers/README.md) |
| [config](config/README.md) | Shared tracing and repository configuration. | [config/README.md](config/README.md) |

context-stack child READMEs:

- [context-stack/context-api/README.md](context-stack/context-api/README.md)
- [context-stack/context-trace/README.md](context-stack/context-trace/README.md)
- [context-stack/context-search/README.md](context-stack/context-search/README.md)
- [context-stack/context-insert/README.md](context-stack/context-insert/README.md)
- [context-stack/context-read/README.md](context-stack/context-read/README.md)

memory-viewers child READMEs:

- [memory-api/README.md](memory-api/README.md)
- [viewer-api/README.md](viewer-api/README.md)

## Installable Tools

The shared installer in [install-tools.sh](install-tools.sh) refreshes the executable Rust binaries and installable tooling surfaced by this repository.

Install the complete MCP server set used by VS Code with:

```bash
./install-tools.sh --mcp
```

The MCP configuration invokes these installed binaries directly rather than
running Cargo for each request. Ensure Cargo's binary directory (`$HOME/.cargo/bin`
on Unix or `%USERPROFILE%\\.cargo\\bin` on Windows) is on the `PATH` inherited by
VS Code, then run `Developer: Reload Window` to restart and rediscover the servers.

The companion installer in [install-deps.sh](install-deps.sh) installs the
repo-wide developer dependencies listed in [Getting Started](#getting-started).

The companion installer in [install-extensions.sh](install-extensions.sh) packages and installs the workspace's VS Code extensions into your local VS Code profile:

- [memory-api/tools/ticket-vscode/package.json](memory-api/tools/ticket-vscode/package.json) exposes the `ticket-vscode` extension package and the repo-local VSIX install workflow.

- [memory-viewers/README.md](memory-viewers/README.md) covers the top-level viewer workflows and the `spec-viewer` and `ticket-viewer` binaries.
- [viewer-api/README.md](viewer-api/README.md) covers the `install-ctl` binary and the `trunk`-backed frontend toolchain.
- [memory-api/README.md](memory-api/README.md) covers the `rule`, `spec`, `ticket`, and `audit` CLIs, the `cargo llvm-cov` coverage collector used by `audit`, and the MCP and HTTP surfaces behind them.
- [memory-viewers/doc-viewer/README.md](memory-viewers/doc-viewer/README.md) covers the `doc-viewer` binary.
- [memory-viewers/log-viewer/README.md](memory-viewers/log-viewer/README.md) covers the `log-viewer` binary.

## Working With Submodules

[Getting Started](#getting-started) initializes submodules and attaches them to
their configured tracking branches. Once initialization completes, continue from
[context-stack/README.md](context-stack/README.md) and [memory-viewers/README.md](memory-viewers/README.md) for repository-local commands.

### Development-Only Submodules

The `memory-kernel/` submodule is **development-only**. It provides a local checkout of the production [memory-kernel](https://github.com/mankinskin/memory-kernel) repository so contributors can browse and edit the shared `transport-harness` crate alongside the [workflow-tools-contract-reference](workflow-tools-contract-reference/) example.

- Ordinary consumers do not need to initialize `memory-kernel/`. The reference resolves `transport-harness` through a branch-pinned Git dependency (see [workflow-tools-contract-reference/crates/example/Cargo.toml](workflow-tools-contract-reference/crates/example/Cargo.toml)), so Cargo fetches the harness from the pinned `main` branch without the submodule checkout.
- Contributors editing the harness should initialize it explicitly:

  ```bash
  git submodule update --init memory-kernel
  ```

## Workspace Validation

```bash
cargo test --workspace
cargo doc --workspace --open
```

- `cargo test --workspace` is documented in [The Cargo Book: cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
- `cargo doc --workspace --open` is documented in [The Cargo Book: cargo doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html).
- Crate-specific validation entry points live in [context-stack/README.md](context-stack/README.md) and the child READMEs linked above.