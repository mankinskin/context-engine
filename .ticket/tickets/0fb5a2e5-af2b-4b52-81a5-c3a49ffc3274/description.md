# Summary

Rewrite the workflow traceability spec so cross-store links are modeled as first-class metadata across the memory stores instead of wrapper-owned path artifacts.

# Why

The current spec calls the links authoritative while still anchoring them in wrapper-owned artifact payloads. The desired architecture is reversible, identity-based linkage owned by the ticket/spec/doc/test/log stores themselves.

# Scope

- rewrite `.spec/specs/38e337c2-cdda-4488-9aa7-b47a300563b0`
- replace path-first wrapper artifacts with first-class cross-store metadata identities
- define how tickets, specs, docs, validation specs/results, and logs reference each other through native store metadata
- ensure existing markdown path links remain compatibility output, not source of truth
- document how existing prototype artifacts map into the corrected design, if migration is needed

# Acceptance criteria

- The rewritten spec no longer treats wrapper-owned workflow artifacts as the authoritative link source.
- The rewritten spec defines reversible, first-class identities across ticket/spec/doc/test/log surfaces.
- The rewritten spec requires retrieval through the existing memory-system tools and shared libraries rather than a dedicated workflow wrapper.
- The rewritten spec preserves current markdown/path references only as compatibility presentation or migration output.

# Implementation status

- Rewrote `.spec/specs/38e337c2-cdda-4488-9aa7-b47a300563b0` around first-class cross-store workflow metadata instead of wrapper-owned artifacts.
- Updated the spec title to reflect the metadata-first target architecture.

# Validation status

- `./target/debug/spec.exe scan --force --index-root .spec --json` passed after the rewrite.

# Documentation status

- The rewritten spec now treats markdown path links as compatibility presentation only and requires store-owned identities across ticket/spec/doc/test/log surfaces.
