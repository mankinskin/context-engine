# 01 - Publish One Minimal Consumer Contract

## Outcome

Define and implement one version-pinned, patch-free installation contract for a minimal consumer, including workflow-skill bootstrap, Cargo dependency resolution, and a selected installed transport binary.

## Existing Owners

[workflow-tools umbrella](../../.ticket/tickets/b525a7fa-f59d-4a14-b234-2ec7b8a42e95/ticket.toml) owns the aggregate install surface. [workflow-skill](../../.ticket/tickets/b9a52b79-2beb-4710-958d-25582ed79dcf/ticket.toml) owns skill publication and bootstrap behavior.

## Requirements

- The consumer's `Cargo.toml` resolves public domain crates from canonical version-pinned Git sources.
- The consumer uses no local workflow-tools path or Cargo patch override.
- The workflow-skill installs or selects exactly the tool bundle and guidance necessary for the tutorial.
- A documented command installs the selected CLI or MCP binary into a caller-controlled location.

## Non-Goal

Do not make the umbrella a monolithic Rust API dependency or require every domain tool for the minimal fixture.

## Validation

Run the tutorial in a fresh temporary directory and inspect `cargo metadata --format-version 1 --no-deps` to confirm external source identities. Run the installed transport with `--help` and the documented fixture command.