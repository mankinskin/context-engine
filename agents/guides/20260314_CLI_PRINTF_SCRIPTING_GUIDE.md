---
tags: `#guide` `#context-cli` `#printf` `#scripting` `#repl` `#automation` `#bash`
summary: How to script the context-cli using printf and piped stdin — covers REPL piping, subcommand chaining, batch workflows, and common pitfalls
---

# CLI Printf & Scripting Guide

**How to automate `context-cli` operations using `printf`, pipes, and shell scripts.**

---

## Overview

The `context-cli` binary supports two execution modes, both of which can be
driven non-interactively from scripts:

| Mode | Invocation | Input method | State |
|------|-----------|--------------|-------|
| **Subcommand** | `context-cli <command> [args]` | CLI arguments | Stateless (one command per invocation) |
| **REPL** | `context-cli` (no subcommand) | stdin line-by-line | Stateful (workspace stays open across commands) |

The REPL mode is the primary target for `printf` piping because it maintains
workspace state across commands — you can open a workspace once and issue many
commands without reopening it each time.

---

## REPL Piping with printf

### Basic Pattern

Pipe a sequence of REPL commands via `printf`. Each command is separated by
`\n`. The REPL reads lines from stdin until EOF, then exits.

```bash
printf 'create demo\natom abcdefghi\ninsert abcdefghi\nstats\nsave\n' | context-cli
```

This runs five REPL commands in sequence:
1. `create demo` — creates and activates workspace "demo"
2. `atom abcdefghi` — adds atoms a, b, c, d, e, f, g, h, i
3. `insert abcdefghi` — inserts the full sequence
4. `stats` — prints graph statistics
5. `save` — persists to disk

### Why printf Over echo

`printf` is preferred over `echo` because:

- **Portable newlines:** `printf 'a\nb\n'` works identically across shells.
  `echo -e` behaviour varies (bash vs dash vs zsh vs sh).
- **No trailing newline surprises:** `printf` gives you exact control.
- **Binary safety:** No interpretation of backslash sequences unless you use
  `%b` or write them in the format string yourself.

```bash
# ✅ Portable, predictable
printf 'create ws\natom abc\nsave\n' | context-cli

# ⚠️ Works in bash with -e, but not portable
echo -e 'create ws\natom abc\nsave' | context-cli

# ❌ Literal \n on some shells
echo 'create ws\natom abc\nsave' | context-cli
```

### Multi-line printf (Readable Scripts)

For longer command sequences, use shell line continuation or a format string
with multiple `%s` arguments:

```bash
printf '%s\n' \
  'create demo' \
  'atom abcdefghi' \
  'insert abcdefghi' \
  'insert abc' \
  'insert def' \
  'insert ghi' \
  'stats' \
  'save' \
| context-cli
```

Each argument to `printf '%s\n'` becomes one line of input, terminated by a
newline.

---

## Subcommand Chaining

For stateless (one-shot) operations, chain subcommands with `&&`:

```bash
context-cli create demo && \
context-cli add-atoms demo abcdefghi && \
context-cli insert-sequence demo abcdefghi && \
context-cli insert-sequence demo abc && \
context-cli stats demo && \
context-cli save demo
```

**Trade-offs vs. REPL piping:**

| Aspect | REPL piping | Subcommand chaining |
|--------|-------------|---------------------|
| Workspace open/close | Once (automatic) | Once per invocation |
| Disk I/O overhead | Minimal (in-memory) | Graph loaded/unloaded each time |
| Error handling | Continues on error | `&&` stops on first failure |
| State across commands | Yes (active workspace) | No (must name workspace each time) |
| Lock contention | Single lock held | Acquired/released per command |

**Rule of thumb:** Use REPL piping for multi-step workflows. Use subcommand
chaining for isolated operations or when you need per-command error handling.

---

## Common Workflows

### Build a Graph From Scratch

```bash
printf '%s\n' \
  'create myproject' \
  'atom abcdefghijklmnopqrstuvwxyz' \
  'atom " "' \
  'insert hello' \
  'insert world' \
  'insert hello world' \
  'stats' \
  'save' \
| context-cli
```

### Insert Sequences From a File

Given a file `sequences.txt` with one sequence per line:

```text
hello
world
hello world
foo bar
```

Pipe it through a subshell that prepends workspace setup:

```bash
{
  printf 'open myproject\n'
  while IFS= read -r line; do
    printf 'insert %s\n' "$line"
  done < sequences.txt
  printf 'save\n'
} | context-cli
```

Or more concisely with `sed`:

```bash
{ printf 'open myproject\n'; sed 's/^/insert /' sequences.txt; printf '\nsave\n'; } | context-cli
```

### Inspect After Insertion

```bash
printf '%s\n' \
  'open myproject' \
  'search hello' \
  'search world' \
  'vertices' \
  'stats' \
| context-cli
```

### Reproduce the Duplicate Vertex Bug Scenario

(See `20260314_CONTEXT_API_INSERT_SEMANTICS_GUIDE.md` for the full bug description.)

```bash
printf '%s\n' \
  'create bugtest' \
  'atom abcdefghi' \
  'insert abcdefghi' \
  'stats' \
  'insert abc' \
  'stats' \
  'vertices' \
  'save' \
| context-cli
```

Compare the two `stats` outputs — `vertex_count` should increase by exactly 1
(the `abc` token). If it increases by more, the duplicate vertex bug is present.

### Validate Graph Integrity

```bash
printf 'open myproject\nvalidate\n' | context-cli
```

### Export a Snapshot as JSON

```bash
printf 'open myproject\nsnapshot\n' | context-cli > snapshot.json
```

Note: human-friendly output (prompts, status messages) goes to stdout alongside
the JSON. To get clean JSON, use subcommand mode instead:

```bash
context-cli open myproject && context-cli snapshot myproject > snapshot.json
```

---

## Working With Special Characters

### Spaces in Insert Text

The REPL `insert` command joins all tokens after the keyword, so spaces are
preserved:

```bash
printf 'insert hello world\n' | context-cli
# Inserts the text "hello world" (with the space)
```

### Quoting in printf

Use single quotes for the `printf` format string to avoid shell expansion:

```bash
# ✅ Single quotes — no expansion
printf '%s\n' 'insert hello world' | ...

# ⚠️ Double quotes — $ and ` are expanded
printf "%s\n" "insert hello world" | ...

# ❌ Unquoted — globbing and word splitting
printf %s\n insert hello world | ...
```

### Special Atom Characters

To add atoms that are shell metacharacters, quote them:

```bash
printf '%s\n' \
  'create special' \
  "atom abc!@#" \
  'save' \
| context-cli
```

The `atom` REPL command treats each character in its argument as a separate
atom, so `atom abc` adds three atoms: `a`, `b`, `c`.

---

## Error Handling

### REPL Behaviour on Errors

The REPL **continues** after errors — it prints an error message to stderr and
processes the next line. This means a failing `insert` won't abort subsequent
commands:

```bash
printf '%s\n' \
  'create demo' \
  'insert x' \
  'insert ab' \
  'stats' \
  'save' \
| context-cli
```

`insert x` fails with `QueryTooShort` (minimum 2 characters), but `insert ab`
and the rest still execute.

### Detecting Errors in Scripts

Errors go to stderr. Capture them separately:

```bash
printf '%s\n' 'open demo' 'insert ab' 'save' \
  | context-cli 2>errors.log

if [ -s errors.log ]; then
  echo "Errors occurred:"
  cat errors.log
fi
```

### Subcommand Error Codes

In subcommand mode, `context-cli` exits with code 1 on error:

```bash
context-cli insert-sequence demo x
echo $?  # 1 — QueryTooShort

context-cli insert-sequence demo ab
echo $?  # 0 — success
```

Use `&&` to stop on first failure, or `||` to handle errors:

```bash
context-cli insert-sequence demo ab || echo "Insert failed"
```

---

## REPL vs. Subcommand Reference

### REPL Commands (piped via stdin)

| Command | Arguments | Description |
|---------|-----------|-------------|
| `create <name>` | workspace name | Create and activate workspace |
| `open <name>` | workspace name | Open and activate workspace |
| `close [<name>]` | optional name | Close workspace (defaults to active) |
| `save [<name>]` | optional name | Save workspace (defaults to active) |
| `list` | — | List all workspaces |
| `delete <name>` | workspace name | Delete workspace from disk |
| `use <name>` | workspace name | Switch active workspace (must be open) |
| `ws` | — | Show active workspace name |
| `atom <chars>` | character string | Add atoms (each char = one atom) |
| `pattern <chars>` | character string | Add pattern from existing atoms |
| `vertex <index>` | numeric index | Show vertex details |
| `vertices` | — | List all vertices |
| `atoms` | — | List all atoms |
| `search <text>` | text string | Search for text sequence |
| `search <r1> <r2>…` | token refs | Search by indices/labels |
| `insert <text>` | text string | Insert text sequence |
| `insert-match <refs>` | 2+ token refs | Insert by token references |
| `insert-bulk <t1> <t2>…` | multiple texts | Bulk insert sequences |
| `read <index>` | numeric index | Read vertex decomposition tree |
| `text <index>` | numeric index | Read vertex as leaf text |
| `validate` | — | Check graph integrity |
| `snapshot` | — | Print graph as JSON |
| `stats` | — | Print graph statistics |
| `show` | — | Show full graph visualisation |
| `show <index>` | numeric index | Show single vertex |

### Equivalent Subcommands (one-shot)

| Subcommand | Example |
|------------|---------|
| `create <name>` | `context-cli create demo` |
| `open <name>` | `context-cli open demo` |
| `add-atom <ws> <ch>` | `context-cli add-atom demo a` |
| `add-atoms <ws> <chars>` | `context-cli add-atoms demo abcde` |
| `add-pattern <ws> <chars>` | `context-cli add-pattern demo abc` |
| `insert-sequence <ws> <text>` | `context-cli insert-sequence demo hello` |
| `insert-first-match <ws> <refs>` | `context-cli insert-first-match demo 0 1 2` |
| `insert-sequences <ws> <texts>` | `context-cli insert-sequences demo hello world` |
| `search-sequence <ws> <text>` | `context-cli search-sequence demo hello` |
| `stats <ws>` | `context-cli stats demo` |
| `validate <ws>` | `context-cli validate demo` |
| `snapshot <ws>` | `context-cli snapshot demo` |
| `save <ws>` | `context-cli save demo` |

---

## Performance Tips

1. **Batch atoms early.** `atom abcdefghijklmnopqrstuvwxyz` is one command
   that adds 26 atoms. Don't issue 26 separate `atom a`, `atom b`, … commands.

2. **Use REPL piping for bulk inserts.** Each subcommand invocation reopens
   and re-serialises the graph. REPL piping keeps it in memory.

3. **Save once at the end.** Don't `save` between every insert — save after
   the full batch.

4. **Order inserts by length.** Insert longer sequences first when building
   hierarchical graphs. The split-join pipeline handles subsequence insertion
   more efficiently when the supersequence already exists.

---

## Common Mistakes

| Mistake | Symptom | Fix |
|---------|---------|-----|
| Missing trailing `\n` in printf | Last command silently dropped | Always end with `\n`: `printf '…\nsave\n'` |
| Forgetting to `save` | Changes lost after pipe finishes | Add `save` as the last REPL command |
| Using subcommand mode without workspace name | CLI error "missing argument" | Subcommands require explicit workspace name (unlike REPL) |
| Piping to subcommand mode | Stdin ignored — subcommand runs once | Use REPL mode (no subcommand) for piped input |
| Inserting single characters | `InsertError::QueryTooShort` | Minimum insert length is 2; use `atom` for single chars |
| Not opening workspace before commands | "No workspace is currently active" | First REPL command should be `create` or `open` |
| Expecting JSON from REPL pipe | Human-readable output mixed with data | Use subcommand mode for machine-parsable output |

---

## Related Documentation

- **Context-API insert semantics:** `agents/guides/20260314_CONTEXT_API_INSERT_SEMANTICS_GUIDE.md`
- **Context-API README:** `crates/context-api/README.md`
- **Insert algorithm guide:** `agents/guides/20251203_CONTEXT_INSERT_GUIDE.md`
- **CLI source:** `tools/context-cli/src/main.rs` (subcommands), `tools/context-cli/src/repl.rs` (REPL)
- **Output formatting:** `tools/context-cli/src/output.rs`
