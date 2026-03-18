---
tags: `#guide` `#context-cli` `#repl` `#workflow` `#read` `#show` `#workspace`
summary: How to use the context-cli REPL — workspace lifecycle, read/show commands, output format reference, and worked examples including the aaa decomposition
---

# Context-CLI Usage Guide

**Reference for the `context-cli` interactive REPL and subcommand interface.**

---

## Quick Start

```bash
# Build the CLI
cargo build -p context-cli

# Launch the REPL
./target/debug/context-cli

# Or pipe commands non-interactively
printf '%s\n' 'create myws' 'read aaa' 'show' | ./target/debug/context-cli
```

The REPL shows a prompt with the active workspace name. Type `help` for the full
command list. Send EOF (`Ctrl-D` / end of pipe) to exit.

---

## Workspace Lifecycle

Every graph command requires an **active workspace**. Workspaces are stored under
`.context-engine/<name>/` in the working directory.

| Command | Description |
|---------|-------------|
| `create <name>` | Create a new empty workspace and activate it |
| `open <name>` | Open an existing workspace and activate it |
| `close` | Close the active workspace (unsaved changes are lost) |
| `save` | Persist the active workspace to disk |
| `delete <name>` | Delete a workspace from disk |
| `list` | List all available workspaces |
| `ws` | Show the name of the currently active workspace |

### Example: fresh workspace

```
create demo
(workspace 'demo' is now active)
```

---

## Graph-Building Commands

### `read <text>`

Read a text string through the hypergraph.  The read pipeline:

1. Segments the input into **unknown** atoms (new characters) and **known**
   subsequences (characters already in the graph).
2. Inserts new atoms for unknown characters.
3. Runs the expansion/overlap pipeline on known segments to discover or create
   compound tokens.
4. Returns the **root token** that represents the full input string.

```
read aaa
```

Output format:

```
Root: "aaa" (index: 3, width: 3)
Text: "aaa"
Tree:
  "aaa" [3] (width: 3)
    'a' [0]
    "aa" [2] (width: 2)
      'a' [0]
      'a' [0]
```

- **Root line** — label, vertex index, and width (atom count).
- **Text line** — the original input string reconstructed from the root.
- **Tree** — one decomposition tree.  Atoms are printed with single quotes;
  compound tokens with double quotes.

> **Note:** when a token has multiple child patterns (e.g., `aaa → [[aa, a], [a, aa]]`)
> the tree shows one decomposition (the most recently committed one).  Use `show`
> or `show <index>` to see all child patterns.

### `read <index>`

Read an existing vertex by its numeric index.

```
read 3
```

### `read --file <path>`

Read the contents of a file through the graph.

```
read --file /path/to/input.txt
```

---

## Inspection Commands

### `show`

Display a compact view of **every vertex** in the active workspace.

```
show
```

Output format:

```
Graph: 3 vertices

[   0] "a" (w:1, atom)
       parents:  "aa"(1), "aaa"(2)
[   1] "aa" (w:2, pattern)
       children: "a"(0) -> "a"(0)
       parents:  "aaa"(2)
[   2] "aaa" (w:3, pattern)
       children[0]: "aa"(1) -> "a"(0)
       children[1]: "a"(0) -> "aa"(1)
```

Column meaning:

| Field | Meaning |
|-------|---------|
| `[index]` | Numeric vertex index |
| `"label"` | String label reconstructed from atoms |
| `w:N` | Width — number of leaf atoms spanned |
| `atom` / `pattern` | Whether this vertex is a leaf atom or a compound |
| `children[N]: …` | The Nth child pattern (one line per pattern) |
| `parents: …` | All parent vertices that contain this vertex as a child |

### `show <index>`

Show details for a single vertex.

```
show 3
```

Output format:

```
Vertex 2 "aaa" (width: 3, pattern)
  Pattern 0: "aa"(1), "a"(0)
  Pattern 1: "a"(0), "aa"(1)
  Parents: 0
```

### `vertices`

List all vertices in tabular form (index, label, width, kind).

### `atoms`

List only atom vertices.

### `stats`

Print aggregate graph metrics.

```
Graph Statistics:
  Vertices:  3
  Atoms:     1
  Patterns:  2
  Edges:     6
  Max width: 3
```

### `vertex <index>`

Show detailed internal info for one vertex (children, parents, width).

### `search <text>`

Search for an existing token that represents the given text.

```
search aa
```

### `validate`

Run integrity checks on the graph (parent-child consistency, width invariants).

### `snapshot`

Print the full graph as a JSON object (useful for programmatic inspection).

---

## Worked Example: `aaa`

The string `"aaa"` contains one unique atom (`a`) and the repeated bigram `aa`.
After `read aaa` in a fresh workspace the graph should contain:

| Index | Label | Kind | Patterns |
|-------|-------|------|----------|
| 0 | `a` | atom | — |
| 1 | `aa` | pattern | `[a, a]` |
| 2 | `aaa` | pattern | `[aa, a]`, `[a, aa]` |

> **Expected:** exactly **3 vertices** — no duplicate `aa`.  The `aaa` token
> must carry **two** child patterns, one for each valid adjacent binary
> decomposition.

### Healthy output

```
$ printf '%s\n' 'create test' 'read aaa' 'show' 'stats' | ./target/debug/context-cli

(workspace 'test' is now active)
Root: "aaa" (index: 2, width: 3)
Text: "aaa"
Tree:
  "aaa" [2] (width: 3)
    "aa" [1] (width: 2)
      'a' [0]
      'a' [0]
    'a' [0]

Graph: 3 vertices

[   0] "a" (w:1, atom)
       parents:  "aa"(1), "aaa"(2)
[   1] "aa" (w:2, pattern)
       children: "a"(0) -> "a"(0)
       parents:  "aaa"(2)
[   2] "aaa" (w:3, pattern)
       children[0]: "aa"(1) -> "a"(0)
       children[1]: "a"(0) -> "aa"(1)
Graph Statistics:
  Vertices:  3
  Atoms:     1
  Patterns:  2
  Edges:     6
  Max width: 3
```

### Duplicate-vertex symptom (bug)

If the graph shows **4 vertices** with two separate `"aa"` entries (indices 1 and 2),
the `try_extend_tail_with` path in `RootManager` created a second `aa` vertex via
`insert_pattern([a, a])` instead of reusing the existing one.

Root cause: `insert_pattern` always allocates a fresh vertex.  The fix is to
detect when `old_root`'s child pattern already equals `[last_child, token]` and
reuse `old_root` as the `combined` token directly — see
`crates/context-read/src/pipeline/root.rs` `try_extend_tail_with`.

```
Graph: 4 vertices          ← should be 3

[   0] "a" (w:1, atom)
       parents:  "aa"(1), "aa"(2), "aaa"(3)
[   1] "aa" (w:2, pattern) ← duplicate (created by wrap_root)
       children: "a"(0) -> "a"(0)
       parents:  "aaa"(3)
[   2] "aa" (w:2, pattern) ← duplicate (created by insert_pattern in try_extend_tail_with)
       children: "a"(0) -> "a"(0)
       parents:  "aaa"(3)
[   3] "aaa" (w:3, pattern)
       children[0]: "a"(0) -> "aa"(2)
       children[1]: "aa"(1) -> "a"(0)
```

---

## Other Useful Commands

| Command | Description |
|---------|-------------|
| `insert <text>` | Insert a text sequence (split-join pipeline, not read pipeline) |
| `text <index>` | Reconstruct the leaf-atom string for a vertex |
| `render` | Render an ASCII DAG of the graph |
| `help` | Print the full command reference |
| `quit` / `exit` | Exit the REPL |

---

## Non-Interactive / Scripted Use

Pipe commands via `printf` for scripted or CI use (see also
`20260314_CLI_PRINTF_SCRIPTING_GUIDE.md`):

```bash
# Inspect aaa decomposition
printf '%s\n' \
  'create aaa-test' \
  'read aaa' \
  'show' \
  'stats' \
| ./target/debug/context-cli

# Verify vertex count equals 3 (no duplicates)
printf '%s\n' 'create check' 'read aaa' 'stats' \
  | ./target/debug/context-cli \
  | grep Vertices
# Expected:   Vertices:  3
```

---

## Workspace Storage Layout

```
.context-engine/
└── <workspace-name>/
    ├── graph.bin       # bincode-serialised Hypergraph
    ├── metadata.json   # timestamps, description
    └── .lock           # advisory write lock
```

Changes are **only persisted when you `save`**.  Closing without saving discards
all in-memory changes.

---

## Related Documentation

- `agents/guides/20260314_CLI_PRINTF_SCRIPTING_GUIDE.md` — advanced scripting patterns
- `agents/guides/20260314_CONTEXT_API_INSERT_SEMANTICS_GUIDE.md` — insertion semantics
- `crates/context-api/README.md` — API reference
- `crates/context-read/src/pipeline/root.rs` — `RootManager` (read pipeline internals)