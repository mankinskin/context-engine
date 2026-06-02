# Summary

Generate the top-level `context-engine` README tree from the root `.rule` store, including the repo root plus the root-owned first-level child README surfaces that currently break the navigation chain.

## Problem

The root workspace already has a `.rule` store, but [README.md](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/README.md) and root-owned child docs like `config`, `doc-viewer`, and `log-viewer` remain manual.

## Scope

This spec covers:

- the root `README.md` target
- root-owned first-level child README targets for `config`, `tools/viewer/doc-viewer`, and `tools/viewer/log-viewer`
- parent and child navigation blocks internal to the root repo
- direct command-doc coverage for the root-owned docs surfaces

## Intended Behavior

- The root README is generated from root-owned rules.
- Root-owned child READMEs include parent links back to the root README.
- The root README exposes child blocks for `context-stack`, `memory-viewers`, and other root-owned doc surfaces.
- Installable content and command-doc links remain explicit at the repo root and in root-owned child docs.

## Assumptions To Prove

- The root store can own README targets without re-declaring imported child workspace targets.
- Root-owned child README targets can use the shared schema cleanly.
- Existing local docs can be migrated to canonical rule entries without losing command coverage.

## Test Strategy

1. Explain the new root README target before sync.
2. Regenerate the root README tree from the root store.
3. Re-run `sync-targets --check` to ensure root-owned README drift is mechanical.

## Acceptance Criteria

- The top-level root README is rule-generated.
- The in-scope root-owned child READMEs are rule-generated and parent-linked.
- Root README generation can be explained and checked from the root workspace.

## Traceability

- [ce0beb35 root README rollout ticket](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/ce0beb35-fc60-45ae-b26b-3cd06a282476/ticket.toml)
