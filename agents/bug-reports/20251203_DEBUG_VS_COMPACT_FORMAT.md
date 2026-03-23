---
tags: `#bug-report` `#context-trace` `#debugging`
summary: This document explains the separation between standard `Debug` formatting and custom `CompactFormat` formatting in the context-engine codebase.
---

# Debug vs CompactFormat: Separation of Concerns

## Overview

This document explains the separation between standard `Debug` formatting and custom `CompactFormat` formatting in the context-engine codebase.

## Key Principle

**Never override the `Debug` trait on domain types.** Instead:
- Use `derive(Debug)` for standard debug output
- Implement `CompactFormat` for custom formatting
- Use wrapper types to control which formatting is used

## Architecture

### Three Formatting Traits

1. **`Debug`** (Standard Library)
   - Purpose: Standard Rust debugging output
   - Usage: Always derive on domain types
   - Format: Verbose, shows all field names and values
   - Example: `Token { index: 0, width: TokenWidth(1) }`

2. **`Display`** (Standard Library)
   - Purpose: User-facing, single-line representation
   - Usage: Implement for types that need compact display
   - Format: Concise, human-readable
   - Example: `T0w1` for Token

3. **`CompactFormat`** (Custom Trait)
   - Purpose: Log-friendly compact or indented output
   - Usage: Implement via `fmt_compact()` and `fmt_indented()`
   - Format: Controlled via wrapper types
   - Example: `Path(root,[0..0])` or multi-line indented

### Control Mechanisms

```rust
// 1. Standard Debug (verbose)
tracing::debug!(?value);  // Uses Debug trait

// 2. Display (compact single-line)
tracing::debug!(%value);  // Uses Display trait

// 3. CompactFormat via wrapper
use context_trace::logging::Compact;
tracing::debug!(?Compact(&value));  // Uses CompactFormat::fmt_compact

// 4. Force full Debug even if CompactFormat exists
use context_trace::logging::DebugFull;
tracing::debug!(?DebugFull(&value));  // Uses Debug trait explicitly
```

### Global Control

```rust
use context_trace::logging::format_mode;

// Enable compact formatting globally
format_mode::enable_compact_mode();

// Disable (use standard Debug)
format_mode::disable_compact_mode();
```

## Implementation Example: RootedRangePath

### Correct Implementation

```rust
// Domain type - ALWAYS derive Debug
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootedRangePath<R: PathRoot> {
    pub root: R,
    pub start: RootedIndexPath<R>,
    pub end: RootedIndexPath<R>,
}

// Single-line display
impl<R: PathRoot + fmt::Display> fmt::Display for RootedRangePath<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path({},[{}..{}])", self.root, 
               self.start.sub_path.root_entry, 
               self.end.sub_path.root_entry)
    }
}

// Custom compact formatting (in separate file)
impl<R: PathRoot + fmt::Display> CompactFormat for RootedRangePath<R> {
    fn fmt_compact(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Compact version
        write!(f, "Path({},[{}..{}])", self.root, 
               self.start.sub_path.root_entry, 
               self.end.sub_path.root_entry)
    }

    fn fmt_indented(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        // Multi-line indented version
        writeln!(f)?;
        write_indent(f, indent)?;
        writeln!(f, "RootedRangePath {{")?;
        write_indent(f, indent + 1)?;
        writeln!(f, "root: {},", self.root)?;
        write_indent(f, indent + 1)?;
        writeln!(f, "range: [{}..{}]", 
                 self.start.sub_path.root_entry,
                 self.end.sub_path.root_entry)?;
        write_indent(f, indent)?;
        write!(f, "}}")
    }
}
```

### Incorrect Implementation (What NOT to Do)

```rust
// ❌ WRONG: No derive(Debug)
#[derive(Clone, PartialEq, Eq)]
pub struct RootedRangePath<R: PathRoot> {
    // ...
}

// ❌ WRONG: Custom Debug implementation overrides default
impl<R: PathRoot + fmt::Display> fmt::Debug for RootedRangePath<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This overrides the standard Debug behavior
        // Don't do this!
    }
}
```

## Why This Matters

1. **Predictability**: Developers expect `Debug` to work consistently
2. **Tooling**: IDE debuggers and other tools rely on standard `Debug`
3. **Flexibility**: Wrapper types provide explicit control when needed
4. **Separation**: Different concerns (debugging vs logging) need different formats

## Usage Guidelines

### When to use each format

- **`?value` (Debug)**: Default for tracing, shows full structure
- **`%value` (Display)**: Compact single-line in logs
- **`?Compact(&value)` (CompactFormat)**: Explicit compact formatting
- **`?DebugFull(&value)`**: Force verbose Debug even if compact mode is on

### File Organization

- Domain types: `src/path/structs/rooted/mod.rs` (with derive(Debug))
- CompactFormat impls: `src/logging/path_format.rs` (separate file)
- Wrapper types: `src/logging/compact_format.rs` (Compact, DebugFull)

## Changes Made

### Fixed: RootedRangePath

**Before:**
- No `derive(Debug)` on struct
- Custom `Debug` implementation (45+ lines)
- Overrode standard Debug behavior

**After:**
- Added `derive(Debug)` to struct
- Removed custom `Debug` implementation
- Kept `Display` and `CompactFormat` implementations
- Standard Debug works as expected

### Implemented: CompactFormat for Path Types

Added compact formatting implementations for:
- `RootedRangePath<Pattern>` (PatternRangePath)
- `RootedRangePath<IndexRoot>` (IndexRangePath)
- `IndexRoot`
- `PatternLocation`
- `PathCursor<P, S>` where P: CompactFormat

**Example output:**
```
PatternRangePath {
  root: [T10w2, T3w1, T4w1],
  range: [0..0]
}

PathCursor {
  path: PatternPath([T10w2, T3w1, T4w1],[0..0]),
  atom_position: 2,
}
```

### Result

Now `RootedRangePath` has three independent formatting options:
1. `Debug` (derived): Full field names and values
2. `Display`: Compact single-line `Path(root,[0..0])`
3. `CompactFormat`: Controlled via `Compact<&T>` wrapper

## See Also

- `CHEAT_SHEET.md` - Quick reference for using CompactFormat
- `context-trace/src/logging/compact_format.rs` - Core trait and wrappers
- `context-trace/src/logging/path_format.rs` - RootedRangePath formatting
- `LOG_FORMATTING_CONTROL.md` - How to control formatting in logs
