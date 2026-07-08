<!-- aligned-structure:v1 -->

# Summary

Standardize repository README generation across `context-engine`, `context-stack`, `memory-viewers`, `memory-api`, and `viewer-api` so the repo-root and first-level child README trees share one rule-backed structure, one parent/child navigation contract, and one validation story.

## Behavior Story

Standardize repository README generation across `context-engine`, `context-stack`, `memory-viewers`, `memory-api`, and `viewer-api` so the repo-root and first-level child README trees share one rule-backed structure, one parent/child navigation contract, and one validation story.

## Provided Surface Contracts

- All in-scope repositories use one rule-backed README schema for repo roots and first-level child trees.
- Parent and child README navigation remains repo-internal and is generated mechanically.
- README completeness is enforced by mechanical validation instead of manual review.

## Required Validation

- Contract clause validation: The root, manual, generated, and audit branches form one coherent README generation program.
- Contract clause validation: Parent and child README blocks are generated with repo-internal navigation semantics only.
- Contract clause validation: The rollout uses rule explain-target, rule sync-targets, and rule sync-targets --check as the validation path.
- The scope, intended behavior, and acceptance criteria define the shared README generation contract and rollout boundaries.
- The rollout documents a workspace-scoped mechanical validation path for README completeness.
- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Summary

Standardize repository README generation across `context-engine`, `context-stack`, `memory-viewers`, `memory-api`, and `viewer-api` so the repo-root and first-level child README trees share one rule-backed structure, one parent/child navigation contract, and one validation story.

## Motivation

The current README surface is split between rule-generated nested workspaces and manual repo roots. That leaves three kinds of drift in place:

- repo roots and child README trees do not follow the same generation workflow
- parent and child navigation blocks are inconsistent or absent
- shared README structure is duplicated in per-workspace target definitions instead of being modeled once

## Scope

This rollout spec coordinates four delivery branches:

- shared README schema and validation primitives in `rule-api`
- migration of the manual `context-engine` and `context-stack` README trees
- schema adoption across the generated `memory-viewers` family
- a final completeness check that makes the README contract mechanically enforceable

## Intended Behavior

- Every in-scope repo root can generate its `README.md` from canonical rule entries.
- First-level child READMEs in the in-scope repos participate in the same generated tree.
- Repo-root READMEs expose child blocks.
- Child READMEs expose parent blocks that stay internal to the owning repository.
- Generated README surfaces explicitly highlight installable content or the absence of a root binary surface.
- Commands referenced in generated README sections resolve to direct local docs or explicit external command documentation.

## Assumptions To Prove

- Existing imported `rule-targets` configs are sufficient foundation for the rollout; no new cross-workspace composition model is required.
- A shared README schema can be added without breaking existing AGENTS or non-README generation flows.
- Parent README relationships must remain repo-internal and cannot infer an external git-submodule parent.
- The rollout can be validated workspace by workspace with `rule explain-target`, `rule sync-targets`, and `rule sync-targets --check`.
- A mechanical completeness check can detect missing parent blocks, child blocks, installable-content sections, and command-doc references.

## Relationship To Existing Specs

This rollout builds on the nested import and repo-local generation work already captured in:

- [nested rule-target imports and themed fragments](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.spec/specs/47465a64-0c5f-4ddc-8d38-018048090af2/body.md)
- [memory-api local README generation](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/memory-api/.spec/specs/3b96ec1c-4e99-48f4-86e5-a36ba24b827a/body.md)

It does not require the larger `doc-api` family to ship first.

## Test Strategy

1. Add failing `rule-api` tests that define shared README schema behavior and required-block validation.
2. Implement the schema primitives until those tests pass.
3. Roll out repo generation in dependency order, validating each workspace independently.
4. Add a final completeness check so README drift fails mechanically.

## Acceptance Criteria

- The child specs under this rollout are implemented and linked to their tickets.
- The root, manual, generated, and audit branches define one coherent README-generation program.
- The rollout is explicitly test-driven rather than relying on manual README review alone.

## Traceability

- [ef50db70 README schema rollout tracker](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/ef50db70-90e6-4de4-bcb0-fa364664a6cf/ticket.toml)
