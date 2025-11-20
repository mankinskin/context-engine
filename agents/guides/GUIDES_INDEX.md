# Guides Index

> **⚠️ UPDATE: On create OR successful use of any guide**

**Create guide:** Add entry + tags + date  
**Use guide:** Verify/add tags, note new use cases  
**Defer:** Note "TODO: update GUIDES_INDEX" after current task

## Index

### TOKEN_TEST_LABELING_GUIDE.md
**Description:** How to fix token test labeling so logs show readable names like `"a"` instead of `T0w1`.

**Tags:** `#testing` `#tracing` `#logging` `#token-display` `#debugging` `#init_test_tracing` `#register_test_graph`

**Created:** 2025-11-20

**Common Issues Solved:**
- Tokens showing as `T0w1` instead of `"a"` in test logs
- `init_test_tracing!()` not showing readable token names
- Test graph registration not working

---

### COMPACT_FORMAT_GUIDE.md
**Description:** How to use compact formatting system for readable tracing logs with multiple output levels.

**Tags:** `#logging` `#tracing` `#formatting` `#debugging` `#display` `#compact-format`

**Common Issues Solved:**
- Log output too verbose or hard to read
- Need single-line vs multi-line format control
- Formatting custom types in logs

---

### TRACING_GUIDE.md
**Description:** Complete guide to tracing infrastructure, structured logging, test log files, and configuration.

**Tags:** `#tracing` `#logging` `#debugging` `#testing` `#log-files` `#log-levels` `#test-workflow`

**Common Issues Solved:**
- Setting up tracing in tests
- Configuring log levels and filters
- Finding and using test log files
- Structured logging with spans and events

---

### ROOTED_PATH_MACRO_GUIDE.md
**Description:** Using the `rooted_path!` macro to construct path variants with clean, concise syntax.

**Tags:** `#macros` `#paths` `#api` `#rooted-paths` `#syntax` `#patterns`

**Common Issues Solved:**
- Verbose path construction code
- Creating IndexRangePath, PatternRangePath variants
- Building paths with child locations

---

### UNIFIED_API_GUIDE.md
**Description:** Unified API for parsing, generating, and transforming import/export statements in refactor-tool.

**Tags:** `#refactor-tool` `#api` `#imports` `#exports` `#code-transformation` `#parsing`

**Common Issues Solved:**
- Processing Rust import/export statements
- Merging and transforming use statements
- Import analysis and replacement

---

## Tag Categories
Testing: `#testing` `#unit-tests` `#test-setup`  
Debug: `#debugging` `#tracing` `#logging` `#error-analysis`  
API: `#search-api` `#trace-api` `#insert-api` `#read-api`  
Arch: `#graph-structure` `#paths` `#patterns` `#tokens`  
Issues: `#token-display` `#cache` `#performance` `#thread-safety`  
Macros: `#init_test_tracing` `#insert_atoms` `#register_test_graph`  
Workflow: `#build` `#test-workflow` `#debug-workflow`
