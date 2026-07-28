Implemented per spec eeaf5479 (memory-api/traceability/structured-ticket-spec-links).

Summary of implementation (built on and completed a partially-uncommitted prior attempt found in the working tree; verified and extended, not discarded):
- `SpecRef` (memory-api/crates/memory-api/src/model/entity.rs) and `TicketRef` (memory-api/crates/spec-api/src/ticket_ref.rs) each carry {id, workspace, store_root}; `store_root` is repo-root-relative and resolved directly against the workspace root, eliminating relative-path-from-referencing-file resolution (the nested-store bug).
- `EntityManifest::related_specs()/set_related_specs()` and `SpecManifest::related_tickets()/set_related_tickets()` are typed accessors backed by the existing `extra`/TOML-flatten map; empty vecs remove the key. `legacy_*_link_entries()` detect old untyped string arrays for migration tooling.
- `ticket validate-links` and `spec validate-links` (both `--json`) detect: dangling refs, wrong-store refs (ref not found at claimed store_root but found under the workspace's canonical store), and bidirectional inconsistencies (one side links, the other doesn't link back).
- Migration guide and test matrix are documented directly in spec eeaf5479's body (authored this session).

Validation:
- cargo test -p spec-api -p ticket-api --lib: 186 passed, 0 failed.
- cargo test -p memory-api --lib: 150 passed, 2 failed (pre-existing, unrelated `workspace::tests::discover_workspace_scan_roots_*` failures caused by a stray `.rule` dir under the OS temp root on this machine — not touched by this change).
- cargo test -p ticket-cli --lib: 31 passed, 0 failed (added 4 new validate-links tests incl. nested-store-bug reproduction).
- cargo test -p spec-cli --lib: 32 passed, 1 failed (pre-existing, unrelated `sync_generated_uses_owning_workspace_and_updates_searchable_body` search-index flake; added 3 new validate-links tests incl. nested-store-bug reproduction, all passing).
- ./target/debug/ticket.exe --json validate-links --workspace default: checked=0, valid=true (no existing manifests use the typed fields yet; migration is documented but not required by this ticket's scope).
- ./target/debug/spec.exe --json validate-links --workspace default: checked=0, valid=true.

Deferred: bulk migration of existing prose related_tickets/ticket_ids across the repo to structured refs (guide is written; execution is a separate follow-up).