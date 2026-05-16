---
description: "Use when editing context-engine core crates (trace/search/insert/read/api). Covers layering, common API gotchas, and edit-time rules."
applyTo: "crates/context-*/**"
---

## Architecture Order

The workspace layers build in this order:
1. `context-trace`
2. `context-search`
3. `context-insert`
4. `context-read`

When changing upper layers, check assumptions in lower layers first.

## Edit Rules

- Keep public APIs stable unless the task explicitly requires changes.
- Preserve existing type and naming conventions within the crate.
- Prefer minimal, local changes over broad refactors.

## Discovery Checklist

Before editing:
1. Read crate-level `README.md` and `HIGH_LEVEL_GUIDE.md` when available.
2. Check `CHEAT_SHEET.md` for known type gotchas.
3. Read existing tests for expected behavior.

## Testing and Validation

- Run targeted crate tests first: `cargo test -p <crate> <test_name> -- --nocapture`
- For trace-driven tests, initialize tracing with:

```rust
let _tracing = init_test_tracing!(&graph);
```

- Use `target/test-logs/` for full debug output when tests fail.
