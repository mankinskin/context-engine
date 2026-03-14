---
tags: `#context-api` `#design` `#instruction-language` `#dsl` `#future`
summary: Design sketch for a human-readable instruction language (DSL) for graph operations
status: 📋
---

# Design: Instruction Language for context-engine

## Motivation

A domain-specific language (DSL) for the context-engine hypergraph would provide:

1. **Batch Operations** — Execute multiple graph operations in a single script file, useful for setting up test fixtures, reproducible experiments, and automated workflows.
2. **Reproducibility** — Version-controlled graph definitions that can be replayed deterministically.
3. **Scripting** — Pipe operations together, integrate with shell scripts, or use as input to the CLI.
4. **Human Readability** — More natural than raw JSON `Command` payloads for interactive use.
5. **Education** — Lower the barrier for understanding hypergraph operations by providing a concrete syntax.

## Relationship to the Command Enum

Each instruction in the DSL maps **1:1** to a `Command` variant (defined in `crates/context-api/src/commands/mod.rs`). The instruction language is **syntactic sugar** over `Command` JSON — the parser produces `Vec<Command>` which is then executed sequentially via the existing `execute()` function.

```text
Instruction Text  →  Parser  →  Vec<Command>  →  execute()  →  Vec<CommandResult>
```

This means:
- No new execution semantics are needed
- All existing adapters (CLI, HTTP, MCP) can accept instruction programs
- Error handling follows the same `ApiError` / `CommandResult` pattern

## Grammar Sketch (PEG-style)

```peg
program         ← statement*
statement       ← (workspace_stmt / atom_stmt / pattern_stmt / 
                    insert_stmt / search_stmt / read_stmt /
                    debug_stmt / log_stmt / comment) NEWLINE

# Comments
comment         ← '#' [^\n]*

# Workspace lifecycle
workspace_stmt  ← 'create' IDENT
                 / 'open' IDENT
                 / 'close' IDENT
                 / 'save' IDENT
                 / 'delete' IDENT
                 / 'list' 'workspaces'

# Atom operations
atom_stmt       ← 'atom' CHAR (',' CHAR)*     # AddAtoms
                 / 'get' 'atom' CHAR            # GetAtom
                 / 'list' 'atoms'               # ListAtoms

# Pattern operations
pattern_stmt    ← 'pattern' CHAR+              # AddSimplePattern
                 / 'get' 'vertex' INTEGER       # GetVertex
                 / 'list' 'vertices'            # ListVertices

# Insert operations
insert_stmt     ← 'insert' STRING              # InsertSequence
                 / 'insert' 'match' token_ref+  # InsertFirstMatch

# Search operations
search_stmt     ← 'search' STRING              # SearchSequence
                 / 'search' token_ref+          # SearchPattern

# Read operations
read_stmt       ← 'read' INTEGER               # ReadPattern
                 / 'text' INTEGER               # ReadAsText

# Debug / introspection
debug_stmt      ← 'snapshot'                   # GetSnapshot
                 / 'stats'                      # GetStatistics
                 / 'validate'                   # ValidateGraph
                 / 'show'                       # ShowGraph
                 / 'show' INTEGER               # ShowVertex

# Log operations
log_stmt        ← 'logs' log_opts?             # ListLogs
                 / 'log' STRING log_opts?       # GetLog
                 / 'query' STRING STRING        # QueryLog
                 / 'analyze' STRING             # AnalyzeLog
                 / 'search' 'logs' STRING       # SearchLogs
                 / 'delete' 'log' STRING        # DeleteLog
                 / 'delete' 'logs' log_opts?    # DeleteLogs

# Tokens and primitives
token_ref       ← INTEGER / STRING
log_opts        ← '--' IDENT '=' VALUE ('--' IDENT '=' VALUE)*
IDENT           ← [a-zA-Z_][a-zA-Z0-9_-]*
CHAR            ← "'" [^'] "'"
STRING          ← '"' [^"]* '"'
INTEGER         ← [0-9]+
NEWLINE         ← '\n' / EOF
```

## Example Programs

### Basic Graph Construction

```
# Create and populate a simple graph
create my_graph

# Add individual atoms
atom 'a', 'b', 'c', 'd', 'e'

# Create patterns from atoms
pattern a b c
pattern c d e

# Insert sequences (auto-creates atoms and patterns)
insert "hello"
insert "world"
insert "hello world"

# Save the workspace
save my_graph
```

### Search and Read

```
open my_graph

# Search for a sequence
search "hello"

# Search by token references
search 0 1 2

# Read a pattern tree
read 5

# Read as text
text 5

close my_graph
```

### Debugging and Introspection

```
open my_graph

# Get graph statistics
stats

# Validate graph integrity
validate

# Show full graph
show

# Show a specific vertex
show 3

# Get a JSON snapshot
snapshot

close my_graph
```

### Log Analysis

```
open my_graph

# List log files
logs

# List with pattern filter
logs --pattern="insert"

# Get log entries
log "2024-01-15_insert.jsonl" --limit=50

# Run a JQ query
query "2024-01-15_insert.jsonl" '.level == "ERROR"'

# Analyze a log file
analyze "2024-01-15_insert.jsonl"

# Search across all logs
search logs "error"

close my_graph
```

### Batch Fixture Setup

```
# Setup script for integration tests
create test_fixture

atom 'h', 'e', 'l', 'o', 'w', 'r', 'd'

insert "hello"
insert "world"
insert "held"
insert "lowered"

save test_fixture
close test_fixture
```

## Command Mapping Reference

| Instruction | Command Variant | Notes |
|---|---|---|
| `create <name>` | `CreateWorkspace { name }` | |
| `open <name>` | `OpenWorkspace { name }` | |
| `close <name>` | `CloseWorkspace { name }` | |
| `save <name>` | `SaveWorkspace { name }` | |
| `delete <name>` | `DeleteWorkspace { name }` | |
| `list workspaces` | `ListWorkspaces` | |
| `atom 'a', 'b'` | `AddAtoms { chars: ['a', 'b'] }` | Single atom → `AddAtom` |
| `get atom 'a'` | `GetAtom { ch: 'a' }` | |
| `list atoms` | `ListAtoms` | |
| `pattern a b c` | `AddSimplePattern { atoms: ['a','b','c'] }` | |
| `get vertex 5` | `GetVertex { index: 5 }` | |
| `list vertices` | `ListVertices` | |
| `search "hello"` | `SearchSequence { text: "hello" }` | |
| `search 0 1 2` | `SearchPattern { query: [Index(0), ...] }` | |
| `insert "hello"` | `InsertSequence { text: "hello" }` | |
| `insert match 0 1` | `InsertFirstMatch { query: [Index(0), ...] }` | |
| `read 5` | `ReadPattern { index: 5 }` | |
| `text 5` | `ReadAsText { index: 5 }` | |
| `snapshot` | `GetSnapshot` | |
| `stats` | `GetStatistics` | |
| `validate` | `ValidateGraph` | |
| `show` | `ShowGraph` | |
| `show 3` | `ShowVertex { index: 3 }` | |

## Parser Implementation Approach

### Recommended: `winnow` crate

The `winnow` crate (successor to `nom`) is recommended for the parser implementation:

- **Streaming-friendly** — Can parse from stdin or file
- **Excellent error messages** — Built-in error recovery and diagnostics
- **Rust-idiomatic** — Combinator-based, composes well with existing code
- **Well-maintained** — Active development, good documentation

Alternative: `pest` (PEG-based, generates parser from grammar file — good for formal grammar but heavier dependency).

### Implementation Sketch

```rust
// Future: crates/context-api/src/instruction.rs
use winnow::prelude::*;

pub fn parse_program(input: &str) -> Result<Vec<Command>, ParseError> {
    let statements: Vec<Command> = input
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .map(|line| parse_statement(line))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(statements)
}

fn parse_statement(input: &str) -> Result<Command, ParseError> {
    // Dispatch on first keyword
    let tokens: Vec<&str> = input.split_whitespace().collect();
    match tokens.first() {
        Some(&"create") => parse_create(tokens),
        Some(&"open") => parse_open(tokens),
        Some(&"atom") => parse_atom(tokens),
        Some(&"insert") => parse_insert(tokens),
        Some(&"search") => parse_search(tokens),
        // ... etc
        _ => Err(ParseError::UnknownCommand(tokens[0].to_string()))
    }
}
```

## Open Questions

### 1. Implicit Workspace Context

Should the language support an implicit "current workspace" so that every command doesn't need a workspace parameter?

```
# Option A: Explicit (matches Command enum exactly)
create my_graph
atom my_graph 'a'
insert my_graph "hello"

# Option B: Implicit context (syntactic sugar)
create my_graph   # sets current workspace
atom 'a'          # uses current workspace
insert "hello"    # uses current workspace
```

**Recommendation:** Option B with implicit context. The parser maintains a `current_workspace: Option<String>` state variable set by `create`/`open` and cleared by `close`. Commands that require a workspace use the implicit context. This is purely a parser-level transformation — the generated `Command` values always include the explicit workspace name.

### 2. Variables and Expressions

Should the language support variables for storing intermediate results?

```
# With variables
let token = insert "hello"
read $token.index
```

**Recommendation:** Not in the initial version. Keep the language purely declarative (sequence of commands). Variables can be added in a future iteration if needed.

### 3. Conditionals and Loops

```
# Conditional
if search "hello" then
  insert "world"
end

# Loop
for word in "hello" "world" "foo"
  insert $word
end
```

**Recommendation:** Not in the initial version. For complex logic, users should use the JSON Command API directly or write Rust/Python scripts. The instruction language targets simple batch operations.

### 4. Error Handling

What happens when a command fails mid-script?

**Recommendation:** Default to **fail-fast** (stop on first error). Optionally support a `--continue-on-error` flag that logs errors but continues execution. Each command result is collected and returned as `Vec<Result<CommandResult, ApiError>>`.

### 5. Comments

**Decision:** Use `#` for line comments (shell-style). No block comments in the initial version.

### 6. String Escaping

**Decision:** Use standard JSON-style string escaping within double quotes. Single-quoted characters (`'a'`) are used only for atom chars.

### 7. Output Format

When the instruction language is used via CLI, how should results be displayed?

**Recommendation:** Default to a human-readable summary. Support `--json` flag for machine-readable JSON output of all `CommandResult` values.

## Future Extensions (Not in Initial Implementation)

1. **Import/include** — `include "setup.ctx"` to compose scripts
2. **Variables** — `let result = search "hello"`
3. **Assertions** — `assert search "hello"` (for testing)
4. **Pipes** — `search "hello" | read` (chain operations)
5. **Export/Import** — `export json "backup.json"` / `import "backup.json"`
6. **REPL mode** — Interactive prompt with history and tab completion
7. **Workspace aliases** — `use my_graph as g`
8. **Batch from file** — `context-cli run script.ctx`

## File Extension

**Recommendation:** `.ctx` (short for "context") for instruction files.

```bash
# Execute a script file
context-cli run my_setup.ctx

# Or pipe from stdin
echo 'create test' | context-cli run -
```

## Non-Goals

- **Turing completeness** — This is not a general-purpose programming language
- **Type system** — No type annotations or type checking
- **Modules/packages** — No import system (beyond simple `include`)
- **Concurrency** — Commands execute sequentially (parallelism is handled at the adapter layer)