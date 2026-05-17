# Summary

Rewrite the current workflow validation spec so it describes embedded memory-system behavior instead of a separate wrapper validation path.

The replacement spec must define workflow validation as default behavior embedded in the shared memory tool stack: `ticket-api`, `spec-api`, `doc-api`, and future `test-api` / `log-api` surfaces.

# Why

The current spec hard-codes a separate workflow wrapper, a separate artifact root, and opt-in guidance. That conflicts with the desired architecture: workflow updates should happen through the normal behavior of the memory-system tools and metadata.

# Scope

- rewrite `.spec/specs/a4f48d84-50ed-4769-a42f-38321ea9600c`
- remove separate wrapper validation CLI and wrapper-owned artifact-store assumptions from the target architecture
- redefine validation capture as native metadata owned by the shared libraries and surfaced through existing tool surfaces
- introduce the role of first-class `test-api` and `log-api` entities in validation specifications and results
- describe existing wrapper-oriented prototype behavior only as migration context

# Acceptance criteria

- The rewritten spec no longer depends on a separate wrapper validation CLI or wrapper-owned artifact store.
- The rewritten spec defines validation workflow updates as default behavior in the shared memory APIs and their CLI/MCP/HTTP surfaces.
- The rewritten spec references first-class `test-api` and `log-api` responsibilities for validation specifications, results, and linked logs.
- The rewritten spec explicitly treats wrapper-only prototype behavior as migration context rather than target architecture.

# Implementation status

- Rewrote `.spec/specs/a4f48d84-50ed-4769-a42f-38321ea9600c` around shared-library workflow metadata instead of a standalone CLI.
- Updated the spec title to align with the embedded memory-api architecture.

# Validation status

- `./target/debug/spec.exe scan --force --index-root .spec --json` passed after the rewrite.

# Documentation status

- The rewritten spec now points future implementation toward `ticket-api`, `spec-api`, `doc-api`, `test-api`, and `log-api` ownership without relying on a separate wrapper validation path.
