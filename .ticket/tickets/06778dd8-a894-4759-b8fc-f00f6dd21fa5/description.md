# Summary

Rewrite the documentation validation spec so it targets `doc-api` and a future `doc-cli` instead of a separate wrapper documentation command path.

# Why

The current spec routes documentation validation through the workflow prototype. The intended architecture is that documentation validation and its workflow metadata are first-class behavior of the memory-system doc tooling.

# Scope

- rewrite `.spec/specs/cf5e2942-1a47-43cc-a0ee-14e5774680a6`
- move the target architecture from wrapper-oriented documentation commands to `doc-api` plus a thin `doc-cli`
- define documentation validation metadata as native workflow state owned by the memory-system doc layer
- keep manual and partial coverage explicit, but store it in the default doc workflow model rather than a separate wrapper artifact store
- document the current prototype path only as temporary migration context

# Acceptance criteria

- The rewritten spec no longer treats a separate wrapper documentation command path as the target surface.
- The rewritten spec defines `doc-api` ownership of documentation validation metadata and `doc-cli` as the primary CLI surface.
- The rewritten spec describes how generated-guidance checks and manual doc verification are captured in native workflow metadata.
- The rewritten spec treats the current workflow wrapper path as prototype-only or migration-only.

# Implementation status

- Rewrote `.spec/specs/cf5e2942-1a47-43cc-a0ee-14e5774680a6` around `doc-api` ownership and a future `doc-cli` surface.
- Updated the spec title to reflect the new target architecture.

# Validation status

- `./target/debug/spec.exe scan --force --index-root .spec --json` passed after the rewrite.

# Documentation status

- The rewritten spec now treats generated-guidance checks and manual documentation validation as native `doc-api` workflow metadata rather than wrapper-owned artifacts.
