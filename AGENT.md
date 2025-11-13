# Agent Development Guide

> **⚠️ READ THIS FILE FIRST** before responding to any request in this workspace.

Workspace-specific reference for the context-engine project.

> **Keep this file concise and scannable.** Remove outdated info. Focus on what's actionable.
>
> **Update this file** when project structure, test locations, or common issues change.

## Problem-Solving Approach

### Context-First Strategy

**For complex tasks: Gather context BEFORE coding.**

**Explore the workspace:**
- Use `ls`, `pwd`, `find`, `tree` to understand file structure
- Navigate directories to discover relevant files
- Check for documentation, tests, logs before modifying code
- Verify file locations rather than assuming paths

**Context levels:**
- ❌ Error only → ⚠️ Error + code → ✓ + tests/expectations → ✓✓ + data flow → ✓✓✓ + architecture

**Collect context via:**
1. File system exploration (ls, find, locate relevant directories)
2. Documentation (README, DOCUMENTATION_ANALYSIS.md, bug-reports/)
3. Test files and expectations
4. Logs (`target/test-logs/`)
5. Git history (`git log -p -- <file>`)
6. Backtracking through code/data flow
7. Ask user when: multiple interpretations, contradictory expectations, unclear intent

**Red flags (need more context):**
- Repeated similar fixes failing
- Unclear actual vs expected values
- Uncertain about component ownership or data semantics
- Many unidentified function/type references in your working context

## Project Structure

Multi-crate workspace for context analysis and graph traversal:
- `context-search` - Pattern matching and graph search algorithms  
- `context-trace` - Graph traversal and path tracing
- `context-insert` - Context insertion operations
- `context-read` - Context reading and expansion

## Documentation Resources

**Priority order when researching:**
1. `<crate>/README.md` - Purpose and API overview
2. `<crate>/DOCUMENTATION_ANALYSIS.md` - Detailed structural analysis (update when making significant changes)
3. `<crate>/src/tests/` - Usage examples and expected behavior
4. `bug-reports/` - Known issues and fix options
5. `cargo doc --package <crate> --no-deps --document-private-items --open` - Generated API docs

## Testing & Debugging

### Test Commands
```bash
cargo test -p context-search -- --nocapture              # Run crate tests with output
RUST_LOG=trace cargo test -p context-search -- --nocapture  # With detailed logging
cargo test --package context-search find_ancestor2 -- --nocapture  # Specific test
```

### Workspace-Specific Test Setup
Add to beginning of test functions:
```rust
let _tracing = init_test_tracing!();  // Enables tracing for this test
```

### Test Organization
- `context-search/src/tests/search/ancestor.rs` - Ancestor finding tests
- `context-search/src/tests/search/mod.rs` - General search tests
- Other crates: `<crate>/src/tests/`

### Debugging
- **Failed test logs**: `target/test-logs/<test_name>.log` (preserved on failure)
- **Log levels**: `RUST_LOG=error|warn|info|debug|trace`
- **Module-specific**: `RUST_LOG=context_search::search=trace`

**Debug workflow:**
1. Track: data flow origin, control flow, component ownership, dependencies
2. Before fix: state understanding, identify gaps, explain why fix should work
3. After failure: re-examine output, check for different underlying issue, consider different layer

## Bug Reports

**Check `bug-reports/` directory** before investigating issues.

### Creating Bug Reports
Filename: `BUG_<component>_<short_description>.md`

Required sections:
- **Summary**: One-line description
- **Root Cause**: What's wrong and why
- **Evidence**: Test commands, error output, logs, code snippets
- **Fix Options**: Proposed solutions with pros/cons
- **Related Files**: Bug location and affected tests

Create reproducing test if one doesn't exist.

## Temporary Work Files

Use `agent-tmp/` for temporary analysis and debugging files. Never commit.

Naming conventions:
- `analysis_<topic>.md`
- `test_output_<test_name>.txt`
- `debug_<component>_<issue>.log`

Move important findings to proper documentation before cleanup.

## Key Documentation Files

### Context-Search
- `TRACING_GUIDE.md`, `TESTING_PLAN.md`, `SEARCH_API_EXAMPLES.md`
- `agent-tmp/PATTERN_MATCHING_EXAMPLE.md`, `agent-tmp/RESULT_ARCHITECTURE_ANALYSIS.md`

### Context-Trace
- `TRACING_GUIDE.md`, `TRACING_IMPLEMENTATION.md`, `TEST_EXPECTATIONS.md`

### Other
- Root: `README.md`, `DOCUMENTATION_SUMMARY.md`
- `refactor-tool/` - Multiple feature guides
