# read_sequence

`WorkspaceManager::read_sequence` reads text through the graph and returns a
decomposition-oriented view of the resulting token.

## Contract

- The empty string is rejected with `ReadError::SequenceTooShort { len: 0 }`.
- A single character is handled as a special case: the atom is ensured to exist
  and the returned `PatternReadResult` describes that atom as a leaf node.
- Multi-character input is delegated to `context_read::pipeline::ReadCtx`.
- On success, the command returns `PatternReadResult` containing:
  - the root token info,
  - the concatenated leaf text,
  - a recursive decomposition tree.
- The decomposition tree is deterministic for a given graph snapshot because the
  command's helper path follows the first child pattern at each vertex after
  sorting by pattern id.

## Confidence boundary

This spec freezes the command boundary and result shape.

It does not yet freeze every internal decomposition preference inside complex
overlap-collapse cases in `context-read`. Those lower-level algorithm details are
still evolving and are better tracked by the parent graph-induction spec and the
linked `context-read` regression corpus.