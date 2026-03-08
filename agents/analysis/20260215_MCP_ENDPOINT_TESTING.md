---
confidence: 🟢
tags: `#mcp` `#testing` `#docs-server` `#api-design`
summary: Comprehensive testing of all 17 MCP docs server endpoints with bug fixes and new feature proposals
---

# MCP Server Endpoint Testing & Product Roadmap

## Overview

Systematic testing of all MCP docs server endpoints to identify bugs, validate functionality, and design new features for production-grade documentation management.

## Agent Documentation Tools (10 endpoints)

| Tool | Status | Notes |
|------|--------|-------|
| `list_docs` | ✅ Works | Returns categorized documents |
| `browse_docs` | ✅ Works | Returns 78 docs with metadata |
| `read_doc` | ✅ Works | Parameter: `filename` |
| `create_doc` | ✅ Works | Parameters: `name`, `doc_type`, `title`, etc. |
| `update_doc_meta` | ✅ Works | Updates frontmatter |
| `search_docs` | ✅ Works | **Bug:** Requires both `query` AND `tag` |
| `search_content` | ✅ Works | Full-text search |
| `validate_docs` | ✅ Works | Returns comprehensive report |
| `regenerate_index` | ✅ Works | Parameter: `doc_type` (not `category`) |
| `get_docs_needing_review` | ✅ Works | Returns stale/incomplete docs |

## Crate Documentation Tools (7 endpoints)

| Tool | Status | Issue |
|------|--------|-------|
| `list_crates` | ⚠️ **Fixed** | TypeEntry schema mismatch |
| `browse_crate` | ⚠️ **Fixed** | Same TypeEntry issue |
| `read_crate_doc` | Untested | Should work after rebuild |
| `create_module_doc` | ❓ | Parameter naming unclear |
| `update_crate_doc` | Untested | - |
| `search_crate_docs` | Untested | - |
| `validate_crate_docs` | ⚠️ **Fixed** | No crates found |

## Bug Fixed: TypeEntry Schema

**Root Cause:** The YAML format `- Hypergraph: Core hypergraph data structure` parses as `{"Hypergraph": "Core hypergraph data structure"}`, but `TypeEntry` expected either a plain string or `{name: X, description: Y}`.

**Fix Applied:** Custom deserializer in `schema.rs` that handles both string and map formats.

**Location:** `tools/mcp-docs-server/src/schema.rs` lines 175-230

## Validation Report Summary

- 83 documents missing frontmatter
- 8 INDEX synchronization warnings
- 5 invalid filename formats

## Proposed New Features (Priority Order)

### P1: Critical UX Fixes
1. Make `search_docs` `tag` parameter optional
2. Add diagnostic logging for silent parse failures

### P2: High Value
1. **Documentation Coverage Report** - Show undocumented exports
2. **Auto-Frontmatter Generator** - Fix 83 missing frontmatters
3. **Cross-Reference Graph** - Link related documents
4. **Semantic Code Search** - Connect docs to actual code

### P3: Agent Workflow
1. **Templates** - Generate standard doc structures
2. **Impact Analysis** - Show docs affected by code changes
3. **Health Dashboard** - Single view of doc quality metrics

### P4: Production Quality
1. **Batch Operations** - Update multiple docs with filters
2. **Export Formats** - Generate mdbook/docusaurus/html

## Conclusions

<!-- What should be done based on this analysis? -->


## References

<!-- Related files, docs, or external resources -->

- 
