# MCP Docs Server Improvements Plan

**Created:** 2026-02-15  
**Status:** Completed  
**Scope:** Fix critical issues, improve API quality, add missing features

## Objective

Address issues discovered during comprehensive MCP tool testing to make the docs server genuinely useful for agents working on context-* crates.

## Context

Testing revealed that while the MCP server has good tool coverage, several issues prevent it from being truly useful:
- Stale detection doesn't work (no source_files configured)
- Validation logic has path confusion
- Search results lack context
- No way to manage source_files via MCP

## Issues by Priority

### P0: Critical (Blocking Core Functionality)

#### Issue 1: Module Path Validation Mismatch
- **Problem:** `validate_crate_docs` checks for `agents/docs/src/X/` when index.yaml specifies `src/X/` as module paths
- **Root Cause:** Validation expects module paths to be subdirectories under agents/docs/, but they're actually source paths
- **Fix:** Module paths in index.yaml should reference source files, validation should check those exist OR check for corresponding docs
- **Files:** `src/tools.rs` - `validate_crate_docs()` method

#### Issue 2: No `source_files` Configured
- **Problem:** All 5 crates lack `source_files` in index.yaml
- **Impact:** 
  - `check_stale_docs` returns "Unknown" for all crates
  - `sync_crate_docs` fails entirely
- **Fix:** Add source_files to each crate's index.yaml
- **Files:** 
  - `crates/context-trace/agents/docs/index.yaml`
  - `crates/context-search/agents/docs/index.yaml`
  - `crates/context-insert/agents/docs/index.yaml`
  - `crates/context-read/agents/docs/index.yaml`
  - `crates/context-trace-macros/agents/docs/index.yaml`

### P1: API Quality Improvements

#### Issue 3: search_crate_docs Lacks Match Context
- **Problem:** Returns filename + item name but not WHY it matched or surrounding context
- **Fix:** Include snippet showing matched content with context
- **Files:** `src/tools.rs` - `search_crate_docs()` method

#### Issue 4: browse_crate Missing Module Attribution
- **Problem:** Shows types/traits but not which module each belongs to
- **Fix:** Include module path in output for each item
- **Files:** `src/tools.rs` - `browse_crate()` method

#### Issue 5: sync_crate_docs Output Too Verbose
- **Problem:** Returns full regex-parsed source even when just checking
- **Fix:** Add `dry_run` parameter, return summary in dry run mode
- **Files:** `src/tools.rs`, `src/main.rs`

### P2: Missing Features

#### Issue 6: No Way to Update source_files via MCP
- **Problem:** Must manually edit YAML to add source_files
- **Fix:** Add `update_crate_index` tool
- **Files:** `src/tools.rs`, `src/main.rs`

#### Issue 7: No Cross-Crate Dependency View
- **Problem:** Can't see how crates depend on each other's types
- **Fix:** Add `crate_dependencies` tool or enhance browse_crate
- **Defer:** Nice-to-have, can be added later

### P3: Data Quality (Manual/Low Priority)

#### Issue 8: Low Frontmatter Coverage (6.3%)
- **Status:** Informational - requires manual content review

#### Issue 9: 65 Old Documents
- **Status:** Informational - requires human decision on archiving

#### Issue 10: 6 INDEX Sync Issues
- **Status:** Can be fixed by regenerate_index or manual updates

#### Issue 11: context-trace Missing README.md
- **Status:** Should be created manually with crate overview

## Execution Plan

### Phase 1: Fix Critical Issues (P0)
- [x] 1.1 Fix validate_crate_docs path logic
- [x] 1.2 Add source_files to context-trace/agents/docs/index.yaml
- [x] 1.3 Add source_files to context-search/agents/docs/index.yaml
- [x] 1.4 Add source_files to context-insert/agents/docs/index.yaml
- [x] 1.5 Add source_files to context-read/agents/docs/index.yaml
- [x] 1.6 Add source_files to context-trace-macros/agents/docs/index.yaml
- [x] 1.7 Test check_stale_docs and sync_crate_docs work

### Phase 2: API Quality (P1)
- [x] 2.1 Add match context to search_crate_docs output
- [x] 2.2 Add module attribution to browse_crate output
- [x] 2.3 Add summary_only parameter to sync_crate_docs

### Phase 3: Missing Features (P2)
- [x] 3.1 Implement update_crate_index tool

## Validation

After implementation:
1. ✅ `check_stale_docs` shows actual staleness levels (shows "Very Stale" for crates without last_synced)
2. ⚠️ `sync_crate_docs` works but directories need to be listed as individual files
3. ✅ `validate_crate_docs` passes (only shows 1 warning - missing README.md)
4. ✅ `search_crate_docs` shows context around matches
5. ✅ `browse_crate` shows module paths via `all_types` field

## Known Remaining Issues

1. **source_files should list individual files, not directories** - `sync_crate_docs` can't read directories like `src/graph/`, needs `src/graph/mod.rs` etc.
2. **Modules don't have source_files configured** - Stale detection shows "Unknown" for module-level docs
3. **context-trace needs README.md** - validation warning
## Risks

- **Schema changes:** Adding fields to index.yaml may break existing parsing
- **Path resolution:** Different OSes handle paths differently
- **Performance:** Adding context to search may slow large searches

## Notes

- P3 issues require human decisions, not automation
- Cross-crate dependency view is valuable but complex - defer to later
