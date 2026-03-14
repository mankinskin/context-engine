# Plans Index

Plans for major refactorings and features before execution.

## Active Plans

| Date | File | Status | Summary |
|------|------|--------|---------|
| 2026-03-14 | [PLAN_CONTEXT_API_PHASE3_1](20260314_PLAN_CONTEXT_API_PHASE3_1.md) | design | Phase 3.1 — Per-command tracing log capture in context-cli and context-mcp, plus log query/analysis tools (JQ, analyze, search) |
| 2026-03-10 | [PLAN_CONTEXT_API_OVERVIEW](20260310_PLAN_CONTEXT_API_OVERVIEW.md) | 📋 | context-api master plan — unified API for context-* crates with CLI, MCP, HTTP, TypeScript adapters |
| 2026-03-10 | [PLAN_CONTEXT_API_PHASE1](20260310_PLAN_CONTEXT_API_PHASE1.md) | 📋 | Phase 1 — Foundation + CLI: crate skeleton, workspace management, persistence, basic commands, REPL |
| 2026-03-10 | [PLAN_CONTEXT_API_PHASE2](20260310_PLAN_CONTEXT_API_PHASE2.md) | 📋 | Phase 2 — Algorithm commands: search, insert, read, debug commands, token resolution |
| 2026-03-10 | [PLAN_CONTEXT_API_PHASE3](20260310_PLAN_CONTEXT_API_PHASE3.md) | 📋 | Phase 3 — MCP adapter: single `execute` tool over stdio using rmcp |
| 2026-03-10 | [PLAN_CONTEXT_API_PHASE4](20260310_PLAN_CONTEXT_API_PHASE4.md) | 📋 | Phase 4 — HTTP + GraphQL adapter: RPC endpoint, REST convenience routes, optional GraphQL |
| 2026-03-10 | [PLAN_CONTEXT_API_PHASE5](20260310_PLAN_CONTEXT_API_PHASE5.md) | 📋 | Phase 5 — TypeScript types package, centralized ts-rs generation, export/import, instruction language design |
| 2026-03-04 | [PLAN_VIEWER_REFACTORING_AND_MOBILE](20260304_PLAN_VIEWER_REFACTORING_AND_MOBILE.md) | ready | HypergraphView refactoring + extraction, file tree sync, mobile support |
| 2026-03-03 | [PLAN_SEARCH_EVENT_REFACTORING](20260303_PLAN_SEARCH_EVENT_REFACTORING.md) | ready | Search event emission refactoring — PathNode, IntoTransition, tentative root, query stream |
| 2026-02-21 | [VIEWER_API_REFACTORING](20260221_VIEWER_API_REFACTORING.md) | design | Extract shared server infrastructure from viewer-api |
| 2026-02-18 | [PLAN_CONTEXT_READ_COMPLETION](20260218_PLAN_CONTEXT_READ_COMPLETION.md) | design | Complete context-read crate for text indexing |
| 2026-02-15 | [PLAN_MCP_DOCS_SERVER_IMPROVEMENTS](20260215_PLAN_MCP_DOCS_SERVER_IMPROVEMENTS.md) | completed | Refactor tools.rs into modular structure |
| 2026-02-15 | [PLAN_MCP_CRATE_DOCS](20260215_PLAN_MCP_CRATE_DOCS.md) | design | Extend MCP server to support crate API documentation |
| 2026-01-27 | [SELECTIVE_PARTITION_MERGE](20260127_SELECTIVE_PARTITION_MERGE.md) | design | Transform complete merge to selective partition creation |
| 2026-01-27 | [SELECTIVE_PARTITION_MERGE_IMPL](20260127_SELECTIVE_PARTITION_MERGE_IMPL.md) | design | Implementation details for selective partition merge |
| 2026-01-25 | [PLAN_partition_merge_refactor](20260125_PLAN_partition_merge_refactor.md) | design | Partition merge refactoring plan |
| 2026-01-25 | [PROMPT_partition_merge_refactor](20260125_PROMPT_partition_merge_refactor.md) | design | Partition merge refactoring prompt |
| 2026-01-15 | [PLAN_fine_grained_locking](20260115_PLAN_fine_grained_locking.md) | design | Fine-grained locking design |
| 2025-12-06 | [PLAN_END_TO_END_TEST_REGISTRY](20251206_PLAN_END_TO_END_TEST_REGISTRY.md) | design | End-to-end test registry plan |
| 2025-12-04 | [PLAN_INTERVAL_TESTS_INVESTIGATION](20251204_PLAN_INTERVAL_TESTS_INVESTIGATION.md) | in-progress | Investigation of interval_graph test failures |
| 2025-12-04 | [PLAN_FIX_INTERVAL_TESTS](20251204_PLAN_FIX_INTERVAL_TESTS.md) | ready | Fix interval_graph test failures |
| 2025-12-04 | [PLAN_FIX_INDEX_PREFIX_TEST](20251204_PLAN_FIX_INDEX_PREFIX_TEST.md) | ready | Fix index_prefix1 test failure |
| 2025-12-04 | [PLAN_FIX_INDEX_POSTFIX_TEST](20251204_PLAN_FIX_INDEX_POSTFIX_TEST.md) | ready | Fix index_postfix1 test failure |
| 2025-12-04 | [PLAN_COLOR_FORMATTING_IN_LOGS](20251204_PLAN_COLOR_FORMATTING_IN_LOGS.md) | design | Add color formatting to log output |
| 2025-12-04 | [PLAN_THREAD_LOCAL_PATTERN](20251204_PLAN_THREAD_LOCAL_PATTERN.md) | design | Thread-local pattern management design |
| 2025-12-03 | [BEST_MATCH_IMPLEMENTATION_STRATEGY](20251203_BEST_MATCH_IMPLEMENTATION_STRATEGY.md) | design | Best match tracking implementation strategy |
| 2025-12-03 | [PLAN_CANDIDATE_STATE_CONTROL](20251203_PLAN_CANDIDATE_STATE_CONTROL.md) | design | CandidateState type parameter design |
| 2025-12-03 | [PLAN_PHASE2_CHECKPOINTED_STATE_ADVANCE](20251203_PLAN_PHASE2_CHECKPOINTED_STATE_ADVANCE.md) | design | Phase 2 checkpointed state advancement |
| 2025-12-03 | [PLAN_STATUS_CHECKPOINT](20251203_PLAN_STATUS_CHECKPOINT.md) | design | Status checkpoint tracking |
| 2025-11-21 | [PLAN_position_annotated_paths](20251121_PLAN_position_annotated_paths.md) | design | Position-annotated path structures |
| 2025-01-16 | [INTERIOR_MUTABILITY_REFACTOR_STATUS](20250116_INTERIOR_MUTABILITY_REFACTOR_STATUS.md) | design | Interior mutability refactor status |
| 2025-01-04 | [IMPL_root_join_refactoring](20250104_IMPL_root_join_refactoring.md) | design | Root join refactoring implementation |
| 2025-01-03 | [SPEC_root_join_refactoring](20250103_SPEC_root_join_refactoring.md) | design | Root join refactoring specification |

## Templates

| File | Summary |
|------|---------|
| [PLAN_TEMPLATE](20251203_PLAN_TEMPLATE.md) | Template for new plans |
