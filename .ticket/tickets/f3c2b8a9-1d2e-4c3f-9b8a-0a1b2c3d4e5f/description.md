Problem

The repository's `install-tools.sh` is failing to overwrite binaries when those binaries are held open by running processes (notably on Windows where an .exe is locked by the process). Root cause verified: `install-tools.sh` (line ~324) runs `cargo install --path ... --bin ... --force` with no mechanism to detect and stop a running process holding the executable open, so the install fails when the process does not exit and the OS prevents overwrite.

Decisions (already made)

1. The new binary is named `install-ctl`.
2. Organization: generalize `viewer-ctl` into `install-ctl`. This is one tool; viewer entries become ordinary managed-artifact entries alongside general tool binaries. This is NOT a shared-crate extraction and NOT a copy.
3. The managed-artifact registry will be a committed TOML file mirroring the shape of the existing `viewer-ctl.toml`.
4. The shell surface collapses to a single bootstrap script: `install-tools.sh` and `install-extensions.sh` are removed; the remaining script only builds/installs `install-ctl` and forwards args to it.

Research facts (verified)

- `install-tools.sh` (369 lines, repo root) installs 24 artifacts across helper tools, CLI tools, MCP tools, and four viewers. Flags: positional names, `--tool`, `--tools`, `--mcp`, `--all`, `--list`, `--dry-run`, `--no-force`, `-h/--help`; env `INSTALL_TOOLS` used only when the CLI selects nothing.
- `install-extensions.sh` (351 lines, repo root) installs `ticket-vscode` (from `memory-api/tools/ticket-vscode`) and exposes similar CLI flags; VS Code extension install logic is already implemented in Rust under `viewer-ctl`.
- `viewer-api/viewer-ctl/src/process.rs` already exposes the needed process-management helpers: `pids_on_port`, `pids_by_image_name`, `kill_process`, `process_exists`, and `print_process_info`. `pids_by_image_name` is sufficient for non-server binaries like `mcp-toolmon` and `copilot-capture-hook`. `kill_process` uses `taskkill` on Windows.
- `viewer-api/viewer-ctl/src/commands/extension.rs` and `server.rs` already cover extension installation and build/install workflow patterns (cargo build, copy release binaries, fallback to `cargo install --path`), so much of `install-extensions.sh` and `install-tools.sh` behavior is already present in Rust.
- `viewer-ctl` has a committed TOML registry (`viewer-ctl.toml`) and a `viewer-api/viewer-ctl.toml` mirror; the same model will be used for `install-ctl`.

Approach / Generalization

- Implement `install-ctl` as a Rust CLI (relocating and adapting `viewer-ctl` functionality) that:
  - Reads a committed TOML registry (repo-root `install-ctl.toml` or re-using `viewer-ctl.toml` shape) describing managed artifact kinds: managed server, general tool binary, VS Code extension, frontend, and task.
  - For binary installs, when the destination executable is held by a running process, `install-ctl` discovers the culprit using `pids_by_image_name` and stops it (graceful stop, then forced kill if configured), reporting the stopped PID(s) in the CLI output.
  - Preserves `--dry-run`, `--list`, and selection semantics from `install-tools.sh` and `install-extensions.sh`.
  - Provides `install`, `uninstall`, `list`, `prepare`, and lifecycle commands similar to `viewer-ctl` but generalized for tools (viewers become just one artifact-kind).
  - Keeps registry additions editable without Rust recompile: registry is a committed TOML file and `install-ctl` reads it at runtime.
  - Replace the two shell scripts with a single bootstrap installer script that only builds/installs `install-ctl` and forwards args to it.

Blast Radius (files referencing viewer-ctl / install-tools / install-extensions)

- Makefile.toml (28 hits)
- install-tools.sh (15)
- viewer-api/viewer-ctl/src/main.rs (14)
- viewer-ctl.toml (11)
- viewer-api/viewer-ctl.toml (11)
- viewer-api/README.md (11)
- viewer-api/viewer-ctl/README.md (10)
- memory-api/tools/cli/rule-cli/tests/install_contract_sync.rs (8)
- viewer-api/viewer-ctl/src/commands/server.rs (7)
- install-extensions.sh (6)
- viewer-api/viewer-ctl/src/commands/mod.rs (5)
- viewer-api/viewer-ctl/src/config.rs (5)
- viewer-api/viewer-ctl/src/cli.rs (4)
- repo_map.toon (3)
- README.md (3)
- viewer-api/viewer-api/src/client_log.rs (1)
- memory-api/crates/peek-api/src/lib.rs (1)
- memory-viewers Playwright configs (4 managed Playwright config files)
- Cargo.toml (1)
- memory-api/README.md (1)
- memory-viewers/README.md (1)
- .vscode/tasks.json (8 tasks invoking `viewer-ctl`)

Acceptance Criteria (concrete & verifiable)

1. Installing a tool binary whose process is currently running succeeds: `install-ctl` discovers the holder via `pids_by_image_name`, stops it (graceful then forced as needed), installs the new binary, and reports which PID(s) were stopped. Demonstration: start `mcp-toolmon` locally, run `install-ctl install mcp-toolmon`, observe `install-ctl` stops the running process and the install completes (cargo can replace the executable).

2. All artifacts previously installable by `install-tools.sh` (the 24 listed artifacts) and `install-extensions.sh` (`ticket-vscode`) are installable via `install-ctl` and appear in `install-ctl --list`.

3. The artifact registry is a committed TOML file in the repo; adding an artifact entry requires no Rust recompile and is reflected in `install-ctl --list` after editing the TOML.

4. `--dry-run`, selection semantics, and `--list` are preserved: `install-ctl --dry-run` shows the planned actions without performing them; `install-ctl --list` enumerates available artifacts and categories.

5. The eight `.vscode/tasks.json` viewer tasks that currently call `viewer-ctl start/prepare` continue to work (either by `install-ctl` providing compatible `start/prepare` shims for managed viewers, or by keeping `viewer-ctl` as a thin compatibility shim that forwards into `install-ctl`).

6. `memory-api/tools/cli/rule-cli/tests/install_contract_sync.rs` passes against the new install contract (update tests if they assert `viewer-ctl` specifically — validation should show parity).

7. `cargo build --release` for the whole workspace succeeds after any crate relocation and workspace-member update (e.g., moving `viewer-ctl` crate to `install-ctl` or adding `install-ctl` as a workspace member).

Implementation notes / non-goals

- Do NOT attempt to make `install-ctl` a shared crate copy of `viewer-ctl`; the intent is to generalize and relocate logic, not duplicate code.
- Tests that assert `viewer-ctl` binary name should be updated to accept `install-ctl` or a compatibility shim unless a thin wrapper remains.

Validation plan

- Add `install-ctl` workspace member and run `cargo build --release` to verify workspace compiles.
- Run `./install-tools.sh --tool mcp-toolmon` (or replaced bootstrap script that invokes `install-ctl`) with a live `mcp-toolmon` process to prove AC#1.
- Run `install-ctl --list` and confirm the 25 artifacts (24 binaries + ticket-vscode) are listed.
- Edit the committed registry TOML to add a dummy artifact, run `install-ctl --list`, and confirm it appears without recompiling Rust.
- Run the `install_contract_sync.rs` test and update it if necessary.

