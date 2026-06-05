# context-engine

context-engine is the top-level workspace that coordinates the graph crates in `context-stack`, the viewer and operator tooling in `memory-viewers`, and the repo-local configuration and install surfaces that bind them together.

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

- [memory-viewers/memory-api/README.md](memory-viewers/memory-api/README.md)
- [memory-viewers/viewer-api/README.md](memory-viewers/viewer-api/README.md)

## Installable Tools

The shared installer in [install-tools.sh](install-tools.sh) refreshes the executable Rust binaries and installable tooling surfaced by this repository:

- [memory-viewers/README.md](memory-viewers/README.md) covers the top-level viewer workflows and the `spec-viewer` and `ticket-viewer` binaries.
- [memory-viewers/viewer-api/README.md](memory-viewers/viewer-api/README.md) covers the `viewer-ctl` binary and the `trunk`-backed frontend toolchain.
- [memory-viewers/memory-api/README.md](memory-viewers/memory-api/README.md) covers the `rule`, `spec`, `ticket`, and `audit` CLIs, the `cargo llvm-cov` coverage collector used by `audit`, and the MCP and HTTP surfaces behind them.
- [tools/viewer/doc-viewer/README.md](tools/viewer/doc-viewer/README.md) covers the `doc-viewer` binary.
- [tools/viewer/log-viewer/README.md](tools/viewer/log-viewer/README.md) covers the `log-viewer` binary.

## Working With Submodules

```bash
git submodule update --init --recursive
bash tools/checkout-submodule-branches.sh
```

- `git submodule update --init --recursive` follows the Git submodule workflow documented in [Git Tools - Submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules).
- [tools/checkout-submodule-branches.sh](tools/checkout-submodule-branches.sh) attaches initialized submodules to their configured tracking branches when you need to edit them.
- Once the submodules are attached, continue from [context-stack/README.md](context-stack/README.md) and [memory-viewers/README.md](memory-viewers/README.md) for repository-local commands.

## Workspace Validation

```bash
cargo test --workspace
cargo doc --workspace --open
```

- `cargo test --workspace` is documented in [The Cargo Book: cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
- `cargo doc --workspace --open` is documented in [The Cargo Book: cargo doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html).
- Crate-specific validation entry points live in [context-stack/README.md](context-stack/README.md) and the child READMEs linked above.