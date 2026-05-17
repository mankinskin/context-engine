# Summary
Relocate the context-stack submodule from crates/context-stack to a top-level context-stack directory and remove deprecated repository folders that should no longer participate in the active workspace layout.

# Goals
- Simplify the repository root layout by promoting context-stack to a first-class top-level component.
- Remove deprecated humans/, agents/, scripts/, and tools/http/ surfaces from the live repository.
- Keep workspace manifests and configuration consistent with the new paths.

# Validation
- Cargo workspace metadata/check resolves against the updated paths.
