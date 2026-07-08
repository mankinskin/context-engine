<!-- aligned-structure:v1 -->

# Summary

Relocate the context-stack submodule from crates/context-stack to a top-level context-stack directory and remove deprecated repository folders that should no longer participate in the active workspace layout.

## Behavior Story

Relocate the context-stack submodule from crates/context-stack to a top-level context-stack directory and remove deprecated repository folders that should no longer participate in the active workspace layout.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Summary
Relocate the context-stack submodule from crates/context-stack to a top-level context-stack directory and remove deprecated repository folders that should no longer participate in the active workspace layout.

# Goals
- Simplify the repository root layout by promoting context-stack to a first-class top-level component.
- Remove deprecated humans/, agents/, scripts/, and tools/http/ surfaces from the live repository.
- Keep workspace manifests and configuration consistent with the new paths.

# Validation
- Cargo workspace metadata/check resolves against the updated paths.
