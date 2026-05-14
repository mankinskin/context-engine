# Problem

The context-stack-related tools already exist in the standalone `../context-stack` repository under `tools/**`, and the standalone repo has an in-progress integration slice that makes those tools build there. But `context-engine` still treats the original `tools/**/context-*` trees as the source of truth.

That means the actual migration handoff is still incomplete:

- the mounted `crates/context-stack` submodule in `context-engine` still points at an older commit that does not contain the migrated tools;
- the root workspace still references the original `tools/**` locations;
- tasks, docs, and other references can still assume the pre-migration source tree;
- ownership remains ambiguous until the gitlink and source-repo references are cut over together.

# Scope

Complete the actual handoff of the migrated context-stack-related tools from `context-engine/tools/**/context-*` to the standalone `context-stack` repository under `tools/`.

The work should cover:

- committing the validated standalone `../context-stack` integration slice for the migrated tools;
- advancing the `crates/context-stack` submodule pointer in `context-engine` to a commit that contains those tools;
- retargeting `context-engine` workspace members, scripts, tasks, and references from `tools/**` to the chosen post-migration ownership model;
- removing or otherwise quarantining the original source trees in `context-engine` once the new source-of-truth path is live;
- documenting any intentionally deferred follow-up boundaries.

# Acceptance Criteria

- The standalone `context-stack` repository contains the migrated tools under `tools/**` in a committed state.
- `context-engine` points at a `crates/context-stack` gitlink commit that includes the migrated tools.
- The root workspace and supporting repo references no longer depend on the original `tools/**/context-*` trees as the live source of truth.
- The ownership model for each migrated tool is explicit after the cutover.
- Focused validation passes for the affected tooling surfaces in both repositories.
