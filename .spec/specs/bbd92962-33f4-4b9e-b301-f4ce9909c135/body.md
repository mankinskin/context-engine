# insert_sequences

`WorkspaceManager::insert_sequences` performs bulk graph induction over an
unordered set of text inputs.

## Contract

- The input is a `HashSet<String>`, so the command does not promise a stable
  processing order.
- Each element is induced independently through `insert_sequence`.
- On success, the returned vector contains one `InsertResult` per processed
  element, in the same order the current set iteration produced them.
- An empty set returns an empty vector.

## Confidence boundary

This spec freezes the bulk-success and empty-input behavior above.

It does not yet freeze stronger ordering or transactional guarantees beyond the
current implementation pattern of iterating the set and delegating each element to
`insert_sequence`.

Because `insert_sequence` is the per-item engine, single-character acceptance,
empty-string rejection, and atom auto-creation inherit from that child contract.