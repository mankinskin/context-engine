# Problem

The already-generated README surfaces in `memory-api`, `viewer-api`, and `memory-viewers` still use bespoke target layouts. They need to adopt the shared schema and fill the missing parent or child navigation blocks consistently.

## Scope

Adopt the shared README schema across the `memory-viewers` family and normalize parent or child navigation blocks for the repo roots and their first-level generated child READMEs.

## Assumptions To Prove

- Existing local rule stores in the `memory-viewers` family can adopt the shared schema without losing current ownership boundaries.
- Child README parent links can be added without changing repo ownership or imported target resolution.
- The aggregate `memory-viewers` root can normalize child blocks after the child repos adopt the shared schema.

## Test-Driven Plan

1. Move one generated repo at a time onto the shared schema.
2. Add parent blocks to child README targets in the child repos.
3. Normalize the `memory-viewers` aggregate README after the child repos expose the expected blocks.

## Acceptance Criteria

- The child tickets in this branch are closed.
- `memory-api`, `viewer-api`, and `memory-viewers` use the shared README schema.
- Parent and child navigation blocks are consistent across the generated README family.
