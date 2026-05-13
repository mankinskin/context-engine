---
tags: `#interview` `#context-read` `#api` `#restructuring` `#architecture`
summary: Design interview for context-read module restructuring, public API shape, dead code disposition, and overlap chain design.
status: ✅ complete
date: 2026-03-15
canonical: crates/context-stack/context-read/agents/interviews/20260315_INTERVIEW_CONTEXT_READ_RESTRUCTURE.md
plan: crates/context-stack/context-read/agents/plans/20260315_PLAN_CONTEXT_READ_RESTRUCTURE.md
---

# Interview: context-read Restructuring & Public API Design

> **This is a cross-reference entry.**
> The full interview record lives in the crate-local agent workspace:
> [`crates/context-stack/context-read/agents/interviews/20260315_INTERVIEW_CONTEXT_READ_RESTRUCTURE.md`](../../crates/context-stack/context-read/agents/interviews/20260315_INTERVIEW_CONTEXT_READ_RESTRUCTURE.md)

**Date:** 2026-03-15
**Scope:** `context-read` crate — module layout, public API, dead code, chain design, infrastructure migration

---

## Summary

Eleven design questions were asked and answered during a conversation about
restructuring the `context-read` crate. The answers drive a three-pass
execution plan.

| # | Topic | Decision |
|---|-------|----------|
| Q1 | What does a caller need? | `Token` from any text; `ReadSequenceIter` for REPL composability |
| Q2 | `ReadRequest` — public or remove? | Collapse into `IntoReadInput` trait; keep `HasReadCtx` as `pub(crate)` test convenience |
| Q3 | `context-read` binary? | Remove; `context-cli` covers it; `grammar.rs` → `benches/` |
| Q4 | `segment.rs` split? | Unify input in `input.rs`; segmentation stays in `segment.rs` |
| Q5/6 | `context/` layout + `RootManager`? | Rename `context/` → `pipeline/`; merge `has_read_context.rs` into `input.rs` |
| Q7 | `stack.rs` / `cursor.rs`? | Delete both; design `OverlapChain` as the correct chain structure |
| Q8 | Dead types in `chain/link.rs`? | Merge `BandExpansion`/`BandCap`/`ChainOp` into `OverlapLink`; delete `StartBound`/`EndBound` |
| Q9 | `grammar.rs`? | Move to `benches/grammar.rs`; seed benchmark collection |
| Q10 | `bands/` → `context-trace`? | Yes; delete `bands/` from `context-read`; add `traversal.rs` to `context-trace` |
| Q11 | `complement.rs` stub? | Rename stub; design session required; blocked on downward-path trace-cache construction |

---

## Related Documents

- Full interview: [`crates/context-stack/context-read/agents/interviews/20260315_INTERVIEW_CONTEXT_READ_RESTRUCTURE.md`](../../crates/context-stack/context-read/agents/interviews/20260315_INTERVIEW_CONTEXT_READ_RESTRUCTURE.md)
- Execution plan: [`crates/context-stack/context-read/agents/plans/20260315_PLAN_CONTEXT_READ_RESTRUCTURE.md`](../../crates/context-stack/context-read/agents/plans/20260315_PLAN_CONTEXT_READ_RESTRUCTURE.md)
- Complement design: [`crates/context-stack/context-read/agents/designs/20260315_DESIGN_COMPLEMENT_PATH_BUILDING.md`](../../crates/context-stack/context-read/agents/designs/20260315_DESIGN_COMPLEMENT_PATH_BUILDING.md)