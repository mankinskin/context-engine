## Bug: Panic in `integration::edge_case_tests::edge_repeated_single_char` (RC-3)

### Reproduction

```sh
cargo test -p context-cli --test cli_integration -- edge_repeated_single_char --nocapture
```

### Panic Output

```
thread 'integration::edge_case_tests::edge_repeated_single_char' panicked at
crates/context-stack/context-trace/src/graph/vertex/data/core.rs:111:17:
assertion `left == right` failed: Pattern width mismatch in index T2w4 token pattern:
(PatternId(0bb14901-9b71-4322-abdc-697566a076cb), "[T0w1, T1w2]")
  left: TokenWidth(3)
 right: TokenWidth(4)
```

### Test Code (tools/cli/context-cli/tests/integration/edge_case_tests.rs)

```rust
fn edge_repeated_single_char() {
    let mut ws = TestWorkspace::new("edge-repeat");
    let result = ws.read_sequence("aaaa");
    let read = unwrap_read_result(&result);
}
```

### Root Cause (RC-3: Repeated-char cursor advancement)

When `read_sequence("aaaa")` processes a sequence of identical atoms, the cursor advancement and `append_to_pattern` logic creates a compound token `T2w4` (intended width 4) but stores `TokenWidth(3)` in the pattern width field. The token's pattern `[T0w1, T1w2]` records sub-tokens with cumulative widths 1 and 3, giving a sum of 4 — but the stored width field on the T2w4 vertex itself is 3, not 4.

The mismatch is detected by the consistency check in:
`crates/context-stack/context-trace/src/graph/vertex/data/core.rs:111`

Root cause: `append_to_pattern` modifies a width field in-place when extending a pattern with an overlapping token, but uses the sub-token's width rather than the cumulative pattern width. For repeated single-char inputs the overlap structure causes the width calculation to under-count by 1 on each compounding step.

This class of bug was previously expected to produce a wrong (width=1) result rather than a panic. The panic indicates a stricter invariant check was added since the last test run.

### Fix Direction

Investigate `append_to_pattern` in `context-trace` (or the caller in `context-read`/`context-insert`). The width assigned to the compound token must equal the sum of the non-overlapping portions of all sub-tokens in the pattern, not the width of the last appended sub-token. Add a unit test in `context-trace` that inserts `"aa"` and `"aaaa"` and asserts the resulting token widths are 2 and 4 respectively.

### Related

- RC-3 class bug — also blocks:
  - `oracle_aa` (ignored)
  - `skill3_exp_m_repeated_char_known_failing` (ignored)
- Fix should be evaluated independently of RC-1 / RC-2 remediation.

### Acceptance Criteria

- `cargo test -p context-cli --test cli_integration -- edge_repeated_single_char` passes.
- `read_sequence("aaaa")` returns `text="aaaa"`, `root.width=4`.
- `oracle_aa` and `skill3_exp_m_repeated_char_known_failing` can be un-ignored and also pass.
- No panic in `context-trace/src/graph/vertex/data/core.rs` for repeated-char inputs.