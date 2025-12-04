# Compact Log Formatting Guide

This guide explains how to use the compact formatting system for readable tracing logs.

## Overview

The compact formatting system provides three levels of output detail:

1. **Compact Display** (`%Compact(value)`) - Single-line compact format
2. **Indented Debug** (`?Compact(value)`) - Multi-line indented format  
3. **Full Debug** (`?DebugFull(value)`) - Original Debug output

## Quick Start

```rust
use context_trace::{compact, debug_full, logging::Compact};

let path = IndexRangePath::new_empty(root);

// Compact single-line format
tracing::info!(%Compact(&path), "Processing path");
// Output: Path(Root(Pat(T3w3, dd8822df)),[0..0])

// Or using the macro
tracing::info!(path = %compact!(path), "Processing path");

// Multi-line indented format
tracing::debug!(path = ?Compact(&path), "Detailed path");
// Output:
// Path {
//   root: Root(Pat(T3w3, dd8822df)),
//   range: [0..0]
// }

// Full Debug when you need all details
tracing::trace!(path = ?DebugFull(&path), "Full details");
// or: path = ?debug_full!(path)
```

## Usage Patterns

### Basic Logging

```rust
// Compact format for standard info logs
tracing::info!(
    path = %Compact(&path),
    position = %cursor.atom_position.into():usize,
    "Starting search"
);

// Indented format for debug logs
tracing::debug!(
    state = ?Compact(&compare_state),
    "Current comparison state"
);
```

### Comparing Formats

```rust
// Use Compact for most logs
tracing::info!(path = %Compact(&path), "Processing");

// Use DebugFull when investigating issues
if suspicious_condition {
    tracing::warn!(
        compact = ?Compact(&path),
        full = ?DebugFull(&path),
        "Unexpected state"
    );
}
```

### Custom Types

Implement `CompactFormat` trait for your types:

```rust
use context_trace::logging::compact_format::{CompactFormat, write_indent};
use std::fmt;

impl CompactFormat for MyType {
    fn fmt_compact(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyType(id:{}, status:{})", self.id, self.status)
    }
    
    fn fmt_indented(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        writeln!(f, "MyType {{")?;
        write_indent(f, indent + 1)?;
        writeln!(f, "id: {},", self.id)?;
        write_indent(f, indent + 1)?;
        writeln!(f, "status: {}", self.status)?;
        write_indent(f, indent)?;
        write!(f, "}}")
    }
}
```

## Available Implementations

### context-trace

- `RootedRangePath<R>` where `R: PathRoot + Display`
  - Compact: `Path(root,[start..end] depth:+s/+e)`
  - Indented: Multi-line with root and range details

### context-search

- `CompareState<Q, I>`
  - Compact: `Compare(mode:G, query@3, index@2, checkpoint@2)`
  - Indented: Multi-line with query, index, and checkpoint cursors

- Helper functions for `PathCursor<P, S>`:
  - `fmt_cursor_compact()` - `Cursor(pos:3, path:...)`
  - `fmt_cursor_indented()` - Multi-line with position and path

## Format Examples

### RootedRangePath

**Compact (`%Compact`):**
```
Path(Root(Pat(T10w2, dd8822df)),[0..0])
Path(Root(Pat(T5w1, abc123)),[1..2] depth:+0/+1)
```

**Indented (`?Compact`):**
```
Path {
  root: Root(Pat(T10w2, dd8822df)),
  range: [0..0]
}
```

### CompareState

**Compact (`%Compact`):**
```
Compare(mode:G, query@3, index@2, checkpoint@2)
Compare(mode:Q, query@5, index@5, checkpoint@3)
```

**Indented (`?Compact`):**
```
CompareState<Candidate, Candidate> {
  mode: GraphMajor,
  query: Cursor(pos:3, path:...),
  index: Cursor(pos:2, path:...),
  checkpoint: Cursor(pos:2, path:...)
}
```

## Best Practices

1. **Use Compact for INFO logs** - Keep production logs concise
2. **Use Indented for DEBUG logs** - Easier to read when debugging
3. **Use DebugFull sparingly** - Only when you need complete details
4. **Combine with field names** - `path = %Compact(&path)` is better than bare `%Compact(&path)`
5. **Consider log level** - More detail at TRACE, less at INFO

## Migration from String Formatting

**Before:**
```rust
tracing::info!("Processing path: {}", path);
tracing::debug!("State: {:?}", state);
```

**After:**
```rust
tracing::info!(path = %Compact(&path), "Processing path");
tracing::debug!(state = ?Compact(&state), "Current state");
```

Benefits:
- Structured logging (fields can be filtered/searched)
- Consistent formatting across codebase
- Less verbose than full Debug output
- Easy to switch between compact and detailed views

## Environment Variables

Control output detail via log filtering:

```bash
# INFO level - see compact formats
LOG_STDOUT=1 LOG_FILTER=info cargo test

# DEBUG level - see indented formats
LOG_STDOUT=1 LOG_FILTER=debug cargo test

# TRACE level - all details including DebugFull
LOG_STDOUT=1 LOG_FILTER=trace cargo test

# Module-specific
LOG_STDOUT=1 LOG_FILTER=context_search::search=trace cargo test
```

## Implementation Notes

### Why Three Levels?

1. **Compact** - Production logs, quick scanning
2. **Indented** - Development debugging, understanding structure
3. **Full** - Deep investigation, when you need everything

### Performance

- Compact formatting is lazy (only evaluates when log level matches)
- No allocation overhead for disabled log levels
- Display trait for compact, Debug trait for indented

### Future Enhancements

Consider adding compact formatters for:
- `SearchQueue` / `VecDeque<CompareState>`
- `HypergraphRef` (showing size, not full contents)
- `TraceCache` (showing hit rates)
- `MatchIterator` (showing current state)
