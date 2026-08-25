# 01 - Publish One Minimal Consumer Contract

## Outcome

Define and implement one commit-pinned, patch-free installation contract for a minimal consumer. The contract covers a GitHub-hosted `install.sh`, local-workspace installation of `install-ctl`, interactive tool and installation-home selection, prebuilt binary reuse, Cargo dependency resolution, and a selected installed transport binary.

## Existing Owners

[workflow-tools umbrella](../../.ticket/tickets/b525a7fa-f59d-4a14-b234-2ec7b8a42e95/ticket.toml) owns the aggregate install surface. [workflow-skill](../../.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml) owns skill publication and bootstrap behavior.

## Requirements

- The consumer's `Cargo.toml` resolves public domain crates from canonical version-pinned Git sources.
- The consumer uses no local workflow-tools path or Cargo patch override.
- A single `curl | bash` command fetches `install.sh` from the workflow-tools GitHub repository at an explicit commit; `install.sh` uses the same commit as its installation source.
- `install.sh` installs `install-ctl` into the consumer's local workspace rather than a global executable directory.
- `install-ctl` presents an interactive configuration TUI that selects the required workflow tools, agent client tools, instructions, hooks, and either the user directory or current folder as the installation home.
- Every selected binary is installed beneath `<installation-home>/.workflow-tools/bin/`.
- `install-ctl` accepts a documented prebuilt-binary source or cache, allowing the installation pipeline test to reuse delivered binaries without compiling workflow-tools from source.
- The workflow-skill installs or selects exactly the tool bundle and guidance necessary for the tutorial.
- A documented command exposes the selected CLI or MCP binary from the configured installation home.

## Non-Goal

Do not make the umbrella a monolithic Rust API dependency or require every domain tool for the minimal fixture.

## Validation

Run the tutorial in a fresh temporary directory and inspect `cargo metadata --format-version 1 --no-deps` to confirm external source identities. Run the commit-pinned `curl | bash` entry point, drive the `install-ctl` TUI with `ratatui-testlib`, assert the selected binaries appear below `<installation-home>/.workflow-tools/bin/`, and run the installed transport with `--help` and the documented fixture command. The installation scenario verifies installation and environment wiring, not source builds of delivered workflow-tools binaries.