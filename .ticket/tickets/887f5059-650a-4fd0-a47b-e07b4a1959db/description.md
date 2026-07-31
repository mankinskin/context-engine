## Symptom

- `serve::routes::tests::ancestor_graph_ref_from_child_workspace_is_followable` ([memory-api/tools/http/ticket-http/src/serve/routes/tests.rs](memory-api/tools/http/ticket-http/src/serve/routes/tests.rs#L439))
- `serve::routes::tests::workspace_graph_includes_isolated_local_and_cross_workspace_nodes` ([memory-api/tools/http/ticket-http/src/serve/routes/tests.rs](memory-api/tools/http/ticket-http/src/serve/routes/tests.rs#L541))

Both panic at the edge-add step with `NotFound(<uuid>)`:

```text
add mixed-workspace edge: NotFound(048c6f0e-e976-4ec4-a1fe-6cab8f72ea21)
add mixed-workspace edge: NotFound(42384740-5677-4275-b05e-2ea94d26274f)
```

## Root cause evidence

- The failure originates in `TicketStore::add_edge` at [memory-api/crates/ticket-api/src/storage/store/query.rs](memory-api/crates/ticket-api/src/storage/ticket-api/src/storage/store/query.rs#L136) line 144, where `self.get_indexed(&edge.to)?.ok_or(StorageError::NotFound(edge.to))?` rejects the target UUID before any edge write happens.
- `get_indexed()` only reads the current store index: [memory-api/crates/ticket-api/src/storage/store.rs](memory-api/crates/ticket-api/src/storage/store.rs#L445).
- The route test fixture opens two independent stores: `open_workspace_store(dir)` and `open_workspace_store(&child_dir)`, and each only adds its own `dir.join("tickets")` scan root ([memory-api/tools/http/ticket-http/src/serve/routes/tests.rs](memory-api/tools/http/ticket-http/src/serve/routes/tests.rs#L294)). The parent ticket is created in `parent_store`, not in `child_store`, so the child store has no indexed copy of the parent UUID.
- `visible_scan_roots()` in the store only admits scan roots owned by the current store, so the parent UUID is outside the lookup domain even before the visibility check.

## Verdict

- This is a fixture/product-boundary issue, not workspace-id serialization. The test is constructing the parent in a different `TicketStore` instance than the one used for `add_edge`, so the target genuinely is not present in the lookup domain the store consults.
- If the intended product contract is cross-workspace edge creation across separately registered workspaces, then the product gap is that `TicketStore::add_edge` is store-local and does not resolve endpoints through the workspace registry.

## History

- `git log --oneline -- memory-api/tools/http/ticket-http/src/serve/routes/tests.rs` shows these tests were introduced in `d285297` (`refactor(ticket-http): extract routes test module`).
- `git show 3103297` and `git show 3471427` do not touch `TicketStore::add_edge`; they are not the direct cause of this NotFound.
- The behavior change that introduced the rejecting guard is `7a92a11` (`test(ticket-api): add test for edge addition rejection under ignored scan roots`), which added the `visible_scan_roots` check in [memory-api/crates/ticket-api/src/storage/store/query.rs](memory-api/crates/ticket-api/src/storage/store/query.rs#L140).

## Minimal fix direction

- Either adjust the test fixture to use a shared workspace/store setup that actually contains the ancestor ticket in the active lookup domain, or move cross-workspace edge creation to a layer that resolves both endpoints through the registry before calling the store.
