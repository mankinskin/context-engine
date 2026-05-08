<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=0c1e147b-a899-4225-b06b-a49d6e96af45 slug=shared/instructions/core-crates/core-crates-instructions/l1 -->
---
description: "Use when editing context-engine core crates (trace/search/insert/read/api). Covers layering, common API gotchas, and edit-time rules."
applyTo: "crates/context-*/**"
---

<!-- rule-api:entry id=4fdc8a48-9b12-421f-b3a8-15bcc745d566 slug=shared/instructions/core-crates/core-crates-guidance/architecture-order/l8 -->
## Architecture Order

The workspace layers build in this order:
1. `context-trace`
2. `context-search`
3. `context-insert`
4. `context-read`

<!-- rule-api:entry id=bffb5203-fc67-4560-ad80-ca9815d6e1f8 slug=shared/instructions/core-crates/core-crates-guidance/architecture-order/l16 -->
When changing upper layers, check assumptions in lower layers first.

<!-- rule-api:entry id=18059044-0fbd-4b8e-9d78-8f8473fe4c49 slug=shared/instructions/core-crates/core-crates-guidance/edit-rules/l18 -->
## Edit Rules

- Keep public APIs stable unless the task explicitly requires changes.
- Preserve existing type and naming conventions within the crate.
- Prefer minimal, local changes over broad refactors.

<!-- rule-api:entry id=03241234-afd2-40fa-9dc9-2857687a8833 slug=shared/instructions/core-crates/core-crates-guidance/discovery-checklist/l24 -->
## Discovery Checklist

Before editing:
1. Read crate-level `README.md` and `HIGH_LEVEL_GUIDE.md` when available.
2. Check `CHEAT_SHEET.md` for known type gotchas.
3. Read existing tests for expected behavior.

<!-- rule-api:entry id=7607216f-4dc6-47c1-9e0e-edd66889d9b5 slug=shared/instructions/core-crates/core-crates-guidance/testing-and-validation/l31 -->
## Testing and Validation

- Run targeted crate tests first: `cargo test -p <crate> <test_name> -- --nocapture`
- For trace-driven tests, initialize tracing with:

<!-- rule-api:entry id=be8814e3-08ec-4e8b-b3bd-d1315fb510c5 slug=shared/instructions/core-crates/core-crates-guidance/testing-and-validation/l36 -->
```rust
let _tracing = init_test_tracing!(&graph);
```

<!-- rule-api:entry id=8745d344-d72b-42e0-9064-9acb8ef0c0b2 slug=shared/instructions/core-crates/core-crates-guidance/testing-and-validation/l40 -->
- Use `target/test-logs/` for full debug output when tests fail.
