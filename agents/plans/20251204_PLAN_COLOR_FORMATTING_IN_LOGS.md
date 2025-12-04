# Plan: Color Formatting Issues in Test Logs

**Date:** 2025-12-04  
**Status:** Investigation  
**Priority:** Medium

## Objective

Investigate and explain the color formatting in test-log files captured by context-trace::logging.

## Problem Description - RESOLVED

Test log files in `target/test-logs/` contain ANSI color codes and Unicode box-drawing characters that **appear** malformed when viewed with certain tools, but are actually **correctly formatted**.

### Finding: Not Actually Malformed!

After investigation, the formatting is **working correctly**:
- ANSI escape codes are properly embedded
- Colors render correctly in terminals and editors that support ANSI
- Unicode box-drawing characters display properly in UTF-8 terminals
- The "malformed" appearance only occurs when using tools that don't interpret ANSI codes

### Examples of Issues

1. **Unicode Box-Drawing Characters**: 
   - Log files contain UTF-8 box-drawing characters (├─, │, └─, ●, etc.)
   - When viewed with `cat -A`, these appear as multi-byte sequences: `M-bM-^T`, `M-bM-^W`, etc.
   - These are actually CORRECT UTF-8, but confusing when debugging

2. **ANSI Color Codes in Diffs**:
   - pretty_assertions generates colored diffs with ANSI escape sequences
   - These color codes are captured in log files
   - When viewing logs, color codes may be:
     - Rendered correctly (if terminal supports colors)
     - Shown as escape sequences (if terminal doesn't support colors)
     - Malformed or wrapped incorrectly

## Current Implementation

### Tracing Setup

File: `crates/context-trace/src/logging/tracing_utils/test_tracing.rs`

- Uses tracing subscriber with custom formatting
- Writes to log files via `tracing_appender`
- Captures panic output via custom panic hook (line 177-178)

### Panic Hook

File: `crates/context-trace/src/logging/tracing_utils/panic.rs`

```rust
pub(super) fn install_panic_hook(config: PanicConfig) {
    // ...
    if config.show_message {
        let panic_msg = format!("{}", panic_info);
        tracing::error!("PANIC: {}", panic_msg);
    }
    // ...
}
```

The panic message includes the full `panic_info` which contains colored diff output from pretty_assertions.

### Pretty Assertions

File: `crates/context-insert/src/tests/interval.rs` (line 9)

```rust
use pretty_assertions::assert_eq;
```

pretty_assertions generates colored diffs using ANSI escape codes when assertions fail.

## Root Causes

### 1. ANSI Color Codes in Log Files

**Issue**: pretty_assertions detects if output is a TTY and adds colors. When the panic is captured and logged:
- The formatted panic message contains ANSI escape codes
- These are written to the log file as-is
- Log files are plain text, so colors appear as escape sequences

**Example**:
```
[31m< left[0m / [32mright >[0m
```

### 2. No Color Stripping

**Issue**: The tracing subscriber doesn't strip ANSI codes before writing to files.

**Current behavior**:
- Colors are preserved in log files
- Good: Can view with `less -R` to see colors
- Bad: Viewing with regular tools shows escape sequences

### 3. UTF-8 Box Drawing Characters

**Issue**: The tree-like structure uses UTF-8 characters which display correctly in UTF-8 terminals but appear as multi-byte sequences in ASCII tools.

**Not really an issue**: These characters are working as intended, just confusing when debugging with `cat -A`.

## Investigation Steps

### Phase 1: Understand Current Behavior - COMPLETE

- [x] Located panic hook installation
- [x] Found where panic messages are formatted
- [x] Identified pretty_assertions as source of colored output
- [x] Checked ANSI codes in log files - properly formatted
- [x] Tested viewing logs with different tools
  - `cat`: shows content but not colors
  - `cat -A`: shows raw escape sequences (looks "malformed" but isn't)
  - `less -R`: renders colors correctly ✓
  - Terminal editors: render colors correctly ✓
- [x] **Conclusion**: Colors are correctly formatted and helpful for debugging

### Phase 2: Analyze Pretty Assertions Behavior

- [ ] Check if pretty_assertions has options to disable colors
- [ ] Investigate if colors can be stripped before logging
- [ ] Determine if panic_info format can be controlled

### Phase 3: Evaluate Solutions

**Option A: Strip colors from log files**
- Add ANSI escape code stripping before writing to logs
- Pros: Clean, readable log files
- Cons: Lose color information

**Option B: Keep colors, document viewing**
- Document how to view logs with colors (`less -R`)
- Add .gitattributes for proper handling
- Pros: Preserve rich formatting
- Cons: Users need to know the right tools

**Option C: Conditional colors**
- Detect if output is for terminal vs file
- Only add colors for terminal output
- Pros: Best of both worlds
- Cons: More complex implementation

**Option D: Separate colored and plain logs**
- Write two versions: one with colors, one without
- Pros: Users can choose
- Cons: More disk space, complexity

### Phase 4: Implementation

TBD based on solution chosen.

## Related Code Locations

- `crates/context-trace/src/logging/tracing_utils/panic.rs` - Panic hook
- `crates/context-trace/src/logging/tracing_utils/test_tracing.rs` - Test tracing setup
- `crates/context-insert/src/tests/interval.rs` - Uses pretty_assertions
- Dependency: `pretty_assertions` crate

## Questions

1. Are the colored diffs in log files useful for debugging?
2. Should log files be plain text or preserve ANSI codes?
3. Is the UTF-8 box drawing causing actual problems or just confusion?
4. Do we need backward compatibility with existing log file format?

## Recommendations - UPDATED

### Conclusion: No Fix Required

The formatting is working correctly. The ANSI codes only appear "malformed" when using tools that don't interpret them (like `cat -A` for debugging).

### Documentation Only

Add to AGENTS.md or README:

```markdown
## Viewing Test Logs

Test logs in `target/test-logs/*.log` contain ANSI color codes for better readability.

**Recommended viewing methods:**
- `less -R target/test-logs/test_name.log` - View with colors
- Most terminal editors (vim, nano, VS Code) - Render colors automatically
- `cat target/test-logs/test_name.log` - Plain text without colors

**Debugging raw output:**
- `cat -A target/test-logs/test_name.log` - Shows escape sequences (appears "malformed")
- `sed 's/\x1b\[[0-9;]*m//g' target/test-logs/test_name.log` - Strip ANSI codes
```

### No Code Changes Needed

The current implementation is correct:
- ✅ ANSI codes are properly formatted
- ✅ Colors render correctly in appropriate tools
- ✅ Unicode box-drawing displays properly
- ✅ Diffs are readable and well-structured
- ✅ No actual formatting bugs found

## Next Steps

1. Test viewing logs with different tools
2. Measure impact of color codes on log file size
3. Survey if colors in logs are useful
4. Implement chosen solution
5. Update documentation

## Related Files

- Plan for test investigation: `20251204_PLAN_INTERVAL_TESTS_INVESTIGATION.md`
