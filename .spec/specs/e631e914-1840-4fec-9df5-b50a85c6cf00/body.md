# insert_sequence

`WorkspaceManager::insert_sequence` induces graph structure from a text string.

## Contract

- The empty string is rejected with `InsertError::QueryTooShort`.
- A single character is accepted and returns the atom token directly.
- Multi-character inputs are delegated to `context-read::pipeline::ReadCtx`, so
  text induction uses the current segmentation, expansion, and overlap logic of
  the read pipeline.
- Missing atoms are auto-created as part of the induction flow.
- The returned `InsertResult.token` reports the induced or reused token label and
  width.
- `InsertResult.already_existed` is `true` when reinserting text that does not
  require any new graph vertices.
- When new vertices are created, the workspace is marked dirty.

## Structural expectations

The current test corpus supports these additional expectations:

- reinserting the same text is idempotent at the token-identity level;
- overlapping, supersequence, and subsequence insertions preserve graph validity;
- repeated characters survive induction and round-trip through `read_as_text`;
- existing searchable structure remains available after related insertions.

## Historical note

Older markdown in the repository still describes this command as a minimum-two-
character operation. The live implementation and tests no longer match that
historical contract. This spec records the current behavior instead: only the
empty string is rejected.