# How-To Guides Index

Persistent guidance for common patterns and troubleshooting.

## Confidence Ratings

| Rating | Meaning | Agent Action |
|--------|---------|-------------|
| 🟢 **High** | Verified, current, complete | Trust and apply directly |
| 🟡 **Medium** | Mostly accurate, may have gaps | Apply but verify edge cases |
| 🔴 **Low** | Outdated or incomplete | Use as starting point, explore thoroughly |

## Quick Search by Tag

| Tag | Description |
|-----|-------------|
| `#testing` | Test setup, tracing, debugging |
| `#tracing` | Logging infrastructure |
| `#api` | API usage patterns |
| `#macros` | Macro usage |
| `#refactoring` | Code transformation tools |

---

## All Guides

### TOKEN_TEST_LABELING_GUIDE.md
**Confidence:** 🟢 High - Recently verified, complete solution

**Summary:** Fix token test labeling so logs show readable names like `"a"` instead of `T0w1`.

**Tags:** `#testing` `#tracing` `#logging` `#debugging`

**Solves:**
- Tokens showing as `T0w1` instead of readable names
- Missing graph parameter in `init_test_tracing!()`
- Test graph registration issues

---

### COMPACT_FORMAT_GUIDE.md
**Confidence:** 🟢 High - Current, well-documented system

**Summary:** Compact formatting system for readable tracing logs with multiple output levels.

**Tags:** `#tracing` `#logging` `#formatting`

**Solves:**
- Verbose log output
- Single-line vs multi-line format control
- Custom type formatting in logs

---

### TRACING_GUIDE.md
**Confidence:** 🟢 High - Stable infrastructure, actively used

**Summary:** Tracing infrastructure, structured logging, test log files, and configuration.

**Tags:** `#tracing` `#logging` `#testing`

**Solves:**
- Tracing setup in tests
- Log level and filter configuration
- Test log file usage
- Structured logging with spans

---

### ROOTED_PATH_MACRO_GUIDE.md
**Confidence:** 🟡 Medium - Functional but may have undocumented edge cases

**Summary:** Using `rooted_path!` macro for clean path construction.

**Tags:** `#macros` `#api` `#paths`

**Solves:**
- Verbose path construction
- IndexRangePath/PatternRangePath creation
- Paths with child locations

---

### UNIFIED_API_GUIDE.md
**Confidence:** 🟢 High - Complete implementation, tested

**Summary:** Unified API for parsing, generating, and transforming import/export statements in refactor-tool.

**Tags:** `#refactoring` `#api` `#imports`

**Solves:**
- Processing Rust import/export statements
- Merging and transforming use statements
- Import analysis and replacement
