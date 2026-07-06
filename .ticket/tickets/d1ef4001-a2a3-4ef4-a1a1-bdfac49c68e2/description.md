# Session Objective
Resolve the current static_complexity batch for tools and reduce 29 findings from the baseline.

# Scope Definition
Batch-2 scope = static_complexity findings under the ROOT `tools/` directory (excluding `memory-api/tools/`, which belongs to batch-3 memory-api). Baseline = exactly 29 findings.

# Scope Guardrails
- Stay inside root `tools/` unless a blocker requires a dependency fix outside scope.
- Do not start the next batch until this ticket meets done criteria.

# Implementation Steps
1. Capture exact finding rows for this batch from the baseline audit artifact.
2. Group findings into 2 to 5 micro-chunks and handle one chunk at a time.
3. After each chunk, run the narrowest compile/test check relevant to touched files.
4. Re-run audit summary and record count delta.
5. If blockers remain, create follow-up tickets and link them before handoff.

# Validation Commands
- Full baseline refresh: cargo run -p audit-cli --bin audit -- --json run .
- Narrow crate test: cargo test -p <crate>
- Ticket health sanity: ./target/debug/ticket.exe health --workspace . --all --toon

# Acceptance Criteria
- Findings in this batch are resolved or have explicit blocker tickets linked.
- No increase in other categories caused by this batch.
- Batch notes include before and after counts and next unresolved action.

# Progress Log

## Baseline (2026-07-06, copilot-opus48)
Baseline artifact: target/tmp/batch2_tools_audit_before.json
Root-tools static_complexity findings: 29 (all cyclomatic_complexity). Full row set:
- tools/cli/peek-cli/src/main.rs :: validate_inspection (13)
- tools/dungeon-crawler/src/enemy.rs :: random_enemy (25)
- tools/dungeon-crawler/src/game.rs :: handle_explore_cmd (38), handle_combat_cmd (32), look (13), do_talk (17), do_buy (16)
- tools/dungeon-crawler/src/world.rs :: ensure_generated (14), draw_map (17)
- tools/peek-api/src/lib.rs :: skeletonize_rust (35)
- tools/viewer/doc-viewer/src/http.rs :: scan_crate_source_files (19), query_docs (16)  [DONE chunk 4]
- tools/viewer/doc-viewer/src/main.rs :: main (19)  [DONE chunk 4]
- tools/viewer/doc-viewer/src/markdown_ast.rs :: node_to_json (35), collect_text (23)  [DONE chunk 1]
- tools/viewer/doc-viewer/src/mcp/mod.rs :: search_all (13), validate (42)  [DONE chunk 4]
- tools/viewer/doc-viewer/src/schema.rs :: to_markdown (20)  [DONE chunk 4]
- tools/viewer/doc-viewer/src/tools/agents.rs :: validate (30), add_frontmatter (15), health_dashboard (13), to_markdown (14)  [DONE chunk 3]
- tools/viewer/doc-viewer/src/tools/crates.rs :: discover_crates_with_diagnostics (14), update_crate_index (20), search_crate_docs (13), sync_crate_docs (17), compare_crate_docs (14)  [DONE chunk 2]
- tools/viewer/log-viewer/frontend/dioxus/src/app.rs :: App (14)
- tools/viewer/log-viewer/src/handlers.rs :: search_log (14)

## Chunk 1 — markdown_ast.rs (2026-07-06, copilot-opus48) — DONE
Files changed: tools/viewer/doc-viewer/src/markdown_ast.rs
Change: split node_to_json (was 35) into node_to_json + node_to_json_inline + node_to_json_refs + node_to_json_ext via wildcard delegation; split collect_text (was 23) into collect_text + collect_text_rest + collect_text_more. Behavior-preserving mechanical split.
Validation: `cargo test -p doc-viewer markdown_ast` → 7 passed, 0 failed.
Audit delta: root-tools static_complexity 29 → 27 (both markdown_ast findings resolved). No other category regressed. After artifact: target/tmp/batch2_tools_audit_chunk1_after.json

## Chunk 2 — tools/crates.rs (2026-07-06, copilot-opus48) — DONE
Files changed: tools/viewer/doc-viewer/src/tools/crates.rs
Changes (all behavior-preserving helper extractions):
- discover_crates_with_diagnostics (14): extracted per-entry body into collect_crate_entry.
- update_crate_index (20): extracted shared set/add/remove logic into apply_source_file_updates, used in both module + crate branches.
- search_crate_docs (13): extracted per-crate loop body into search_within_crate.
- sync_crate_docs (17): extracted gather_sync_source_files, analyze_sync_source_files, compare_sync_docs.
- compare_crate_docs (14): extracted push_missing_doc_suggestions + push_stale_doc_suggestions.
Validation: `cargo test -p doc-viewer` → 37 passed, 0 failed.
Audit delta: root-tools static_complexity 27 → 22 (all 5 crates.rs findings resolved, 0 new). Overall static_complexity 70 → 63; no other category regressed (file_length 183→183). After artifact: target/tmp/batch2_tools_audit_chunk2_after.json

## Chunk 3 — tools/agents.rs (2026-07-06, copilot-opus48) — DONE
Files changed: tools/viewer/doc-viewer/src/tools/agents.rs
Changes (all behavior-preserving helper extractions):
- validate (30): extracted validate_document_file, validate_index_file, validate_index_coverage, validate_index_format; main loop now just dispatches per doc_type.
- add_frontmatter (15): extracted per-file body into process_frontmatter_file.
- health_dashboard (13): extracted per-category tally into tally_category_health.
- HealthDashboard::to_markdown (14): extracted status_icon free fn for the 3 threshold blocks.
- Added `Path` to std::path imports.
Validation: `cargo test -p doc-viewer` → 37 passed, 0 failed.
Audit delta: root-tools static_complexity 22 → 18 (all 4 agents.rs findings resolved, 0 new). Overall static_complexity 70 → 59; no other category regressed (file_length 183→183). After artifact: target/tmp/batch2_tools_audit_chunk3_after.json

## Checkpoint commit (2026-07-06, user) — DONE
Chunks 1-3 source refactors committed as 45d3e1b (refactor(doc-viewer): extract helpers for batch-2 complexity chunks 1-3). Ticket evidence committed separately as 995ddc5 (chore(tickets): record d1ef4001 chunk-3 progress and deltas). Root repo + context-stack submodule verified clean; ticket confirmed still in-implementation.

## Chunk 4 — remaining doc-viewer files (2026-07-06, copilot-opus48) — DONE
Files changed: tools/viewer/doc-viewer/src/http.rs, main.rs, mcp/mod.rs, schema.rs
Changes (all behavior-preserving helper extractions):
- http.rs scan_crate_source_files (19): extracted scan_dir_by_extension shared directory walker (used for both src/ and agents/docs/).
- http.rs query_docs (16): extracted collect_query_docs + read_doc_content_ast.
- main.rs main (19): extracted resolve_workspace_root, resolve_crates_dirs, spawn_background_mcp_server, startup_mode_label.
- mcp/mod.rs search_all (13): extracted append_agent_doc_search + append_crate_doc_search.
- mcp/mod.rs validate (42): split target dispatch into validate_agent_docs, validate_crate_docs_target, validate_all_target; further extracted regenerate_index_action + add_frontmatter_action; added markdown_or_error helper.
- schema.rs SyncAnalysisResult::to_markdown (20): extracted push_errors_section, push_summary_section, push_public_items_section, push_suggestions_section + push_public_item_group free fn.
Validation: `cargo test -p doc-viewer` → 37 passed, 0 failed.
Audit delta: root-tools static_complexity 18 → 12 (all 6 remaining doc-viewer findings resolved, 0 new after follow-up trims of main→13→OK and validate_agent_docs→15→OK). Overall static_complexity 70 → 53; no other category regressed (file_length 183→183). After artifact: target/tmp/batch2_tools_audit_chunk4_after.json
doc-viewer crate is now fully clear of static_complexity findings.

# Next Unresolved Action
Continue with chunk 5. Remaining root-tools count: 12. Remaining clusters (none in doc-viewer):
- tools/viewer/log-viewer/frontend/dioxus/src/app.rs (1): App (14)
- tools/viewer/log-viewer/src/handlers.rs (1): search_log (14)
- tools/cli/peek-cli/src/main.rs (1): validate_inspection (13)
- tools/peek-api/src/lib.rs (1): skeletonize_rust (35)
- tools/dungeon-crawler (8): enemy.rs random_enemy (25); game.rs handle_explore_cmd (38), handle_combat_cmd (32), look (13), do_talk (17), do_buy (16); world.rs ensure_generated (14), draw_map (17)
Suggested chunk 5: log-viewer (app.rs + handlers.rs = 2) or peek (peek-cli + peek-api = 2) as a cohesive small chunk; dungeon-crawler (8) as its own larger chunk. Checkpoint commit for chunk 4 recommended before chunk 5.

# Handoff Notes
Board check-in: copilot-opus48 on this ticket (heartbeat refreshed). Authoritative restart source is this description (session-store history not populated for this track). Governing tracker: 5f9542bf (root), depends on this batch; batch-3 (memory-api) e179f11a is blocked behind this. Cumulative: batch-2 tools 29 → 12 (17 resolved across chunks 1-4); doc-viewer crate fully green for tests (37 passed) and fully clear of static_complexity findings. Chunk-4 source changes (http.rs, main.rs, mcp/mod.rs, schema.rs) are uncommitted pending checkpoint.