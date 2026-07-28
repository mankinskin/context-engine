## Validation Evidence

Recorded at e82b4f88 review (2026-07-28):

- `cargo test -p spec-api -p ticket-api --lib` — 186 passed / 0 failed.
- `cargo test -p ticket-cli --lib` — 31 passed / 0 failed.
- `cargo test -p spec-cli --lib` — 32 passed / 1 failed; the failure
  (`sync_generated_uses_owning_workspace_and_updates_searchable_body`)
  exercises rule-generated body/section sync + full-text search, does not
  touch `related_tickets`/`related_specs` code paths, and was confirmed
  pre-existing/unrelated.
- `cargo test -p memory-api --lib` — 150 passed / 2 failed; both failures
  reproduced and confirmed caused by a stray ambient `.rule` directory
  directly under the machine's temp root being picked up by
  ancestor-scan-root discovery — unrelated to this spec. Follow-up:
  [818e894a](.ticket/tickets/818e894a/ticket.toml).
- Nested-store-bug regression test confirmed present:
  `detects_wrong_store_ref_for_nested_store_bug_scenario` in
  `memory-api/tools/cli/ticket-cli/src/cli/commands/ops.rs`.
- Migration guide confirmed present in this spec's own body (section
  "Migration Guide (Prose → Structured)").

All 7 acceptance criteria confirmed met by the [e82b4f88](.ticket/tickets/e82b4f88-45e1-402b-ab59-de845c4930e0/ticket.toml) review; ticket state is `done`.
