# Problem

Once the failing tests exist, `rule-api` still needs real schema support and validation behavior. The current model in `targets.rs` only supports explicit node lists, which leaves the README rollout stuck in copy-and-paste mode.

## Scope

Implement shared README schema inheritance, node extension behavior, required-block validation, and shared-fragment loader behavior in `rule-api`, `rule-cli`, and any touched explain or sync paths.

## Assumptions To Prove

- Schema metadata can be added compatibly to the existing target config model.
- `explain-target` can report enough detail for missing required README blocks.
- `sync-targets --check` can fail cleanly when a README target violates the shared schema contract.
- Existing non-README targets continue to render without behavioral changes.
- Shared schema fragments can be loaded through both imports and fragment discovery without duplicate-registration failures.
- Schema references remain resolvable across sibling fragments within one config load.

## Test-Driven Plan

1. Start from the failing tests created in the schema test ticket.
2. Implement the minimum parser, model, validation, and loader-state changes to turn those tests green.
3. Re-run focused rule-api tests, then validate representative workspace targets with `explain-target` and `sync-targets --check`.

## Acceptance Criteria

- Shared README schema inheritance passes the focused rule-api tests.
- Missing required README blocks fail deterministically in target validation.
- Representative workspace README targets can opt into the shared schema without breaking unrelated targets.
- Shared schema fragments remain deduplicated per config load while staying visible to sibling fragments.

## Validation

- `cargo test -p rule-api readme_schema_ -- --nocapture`
- `cargo build -p rule-cli`
- `./target/debug/rule.exe explain-target --config memory-api/rule-targets.yaml --target memory-api-readme`
- `./target/debug/rule.exe sync-targets --check --config memory-api/rule-targets.yaml --workspace-root memory-viewers/memory-api`
