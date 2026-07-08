<!-- aligned-structure:v1 -->

# Summary

Generate the top-level `context-engine` README tree from the root `.rule` store, including the repo root plus the root-owned first-level child README surfaces that currently break the navigation chain.

## Behavior Story

Generate the top-level `context-engine` README tree from the root `.rule` store, including the repo root plus the root-owned first-level child README surfaces that currently break the navigation chain.

## Provided Surface Contracts

- README surfaces in scope follow an explicit rule-backed contract instead of one-off rollout prose.
- Parent and child README navigation stays repo-internal and mechanically derivable.
- README completeness and rollout status are verified by mechanical validation rather than manual review.

## Required Validation

- Contract clause validation: The migrated spec names the intended README structure and navigation behavior as explicit contract properties.
- Contract clause validation: The migrated spec names the validation path that checks the README contract mechanically.
- Contract clause validation: The migrated spec records enough evidence to tell whether the README contract is satisfied or blocked.
- The authored spec body documents the README contract, scope boundaries, and navigation expectations for this migration slice.
- The authored spec body documents the mechanical validation path required to prove this migration slice.
- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

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
