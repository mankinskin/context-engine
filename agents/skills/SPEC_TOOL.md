---
skill: spec-tool
title: "Spec Tool — Authoring and Querying the Specification Database"
level: intermediate
prerequisites:
  - Familiarity with the ticket CLI (similar UX patterns)
status: stable
verified_against:
  - tools/cli/spec-cli/tests/bootstrap_tests.rs
  - tools/http/spec-http/tests/integration_test.rs
last_updated: 2026-04-20
---

# Skill: Spec Tool

The spec system is a self-documenting **specification database** that links design intent
to implementation code. Each spec is a folder containing a `spec.toml` manifest, a
`body.md` description, optional `sections/`, and a list of `code_refs` pointing to
specific symbols (structs, traits, fns) in the workspace. Specs form a hierarchy via
`parent_of` edges and have a state machine
(`draft → reviewed → approved → implemented → verified`).

There are three transports — pick the one that fits the caller:

| Transport | Use when |
|---|---|
| **CLI** (`./target/debug/spec.exe`) | Human-driven, scripting, pipelines |
| **MCP** (`spec-mcp` server, tools prefixed `spec_`) | Inside an LLM agent session |
| **HTTP** (`spec-http`, port 4001) | Frontend / cross-process |

All three share `spec-api`'s `SpecStore`. The on-disk store lives under `.spec/` in
the workspace by default (override with `--index-root` or `SPEC_INDEX_ROOT`).

---

## Anatomy of a Spec

```
.spec/specs/<slug>/
├── spec.toml          # SpecManifest (id, slug, title, component, state, code_refs[])
├── body.md            # Free-form Markdown description
├── sections/          # Optional named subdocuments
│   ├── design.md
│   └── risks.md
└── history.ndjson     # Append-only revision log
```

**Identifiers (in order of preference):**
1. **Slug** — hierarchical, lowercase, `[a-z0-9-]` per segment, `/`-separated
   (e.g. `spec-api/store`, `ticket-api/storage/board`). Underscores are **invalid**;
   `bootstrap` rewrites `_` to `-`.
2. **UUID prefix** (≥4 chars, must be unambiguous).
3. **Full UUID**.

Most commands accept any of these in the `<id>` slot.

**State machine** (`crates/spec-api/schemas/specification.toml`):
- `draft → reviewed → approved → implemented → verified` (success path)
- `reviewed`/`approved` are **required** in history before reaching `verified`
- `cancelled` and `deprecated` are exit terminals (no required history)

---

## Setup

Before any work, ensure the binary is built and a scan root is registered:

```bash
cargo build -p spec-cli
mkdir -p .spec/specs
./target/debug/spec.exe add-root .spec/specs --label default --json
```

The store auto-scans on every CLI invocation, so manually-edited
`.spec/specs/<slug>/spec.toml` files are picked up without an explicit `scan` call.
Use `spec scan --force` to rebuild the redb + Tantivy indexes from disk.

---

## Bootstrapping from Source

The fastest way to populate the store for an existing crate:

```bash
# Dry run — see what would be created
./target/debug/spec.exe bootstrap crates/spec-api --dry-run --json

# Real run — creates root + one spec per .rs file with public items
./target/debug/spec.exe bootstrap crates/spec-api --json
```

What it does (`tools/cli/spec-cli/src/cli/commands/bootstrap.rs`):
- Walks `<crate>/src/`, parses each `.rs` with `syn`
- Extracts public structs / enums / traits / free fns / impl blocks
- Skips `lib.rs` / `main.rs` (covered by the root crate spec)
- Normalizes `mod.rs` → parent module slug
- Creates a CodeRef per extracted item (`file`, `symbol`, `kind`, `line_start`, `line_end`, doc comment)
- Skips slugs that already exist (idempotent)

After bootstrap, **edit** generated bodies to add design rationale, acceptance
criteria, and feature status — bootstrap only seeds the API surface.

---

## Common Workflows

### Create a spec by hand

```bash
./target/debug/spec.exe create \
  --title "Slug Index" \
  --slug spec-api/slug \
  --component spec-api \
  --parent spec-api \
  --body-file /tmp/body.md \
  --json
```

### Read

```bash
./target/debug/spec.exe get spec-api/store --full --json   # manifest + body
./target/debug/spec.exe tree                               # full hierarchy
./target/debug/spec.exe tree spec-api                      # subtree
./target/debug/spec.exe list --where component=spec-api --json
./target/debug/spec.exe search "slug index" --json
```

### State transitions (one-way, schema-validated)

```bash
./target/debug/spec.exe update spec-api/store --state reviewed --json
./target/debug/spec.exe update spec-api/store --state approved --json
./target/debug/spec.exe update spec-api/store --state implemented --json
./target/debug/spec.exe update spec-api/store --state verified --json
```

Skipping `reviewed` or `approved` will be **rejected** at the `verified` gate.

### Field updates

```bash
./target/debug/spec.exe update spec-api/store \
  --field 'priority=high' \
  --field 'scope=public' \
  --json
```

### Sections (named subdocuments)

```bash
./target/debug/spec.exe section add spec-api/store --name design --file /tmp/d.md --json
./target/debug/spec.exe section list spec-api/store --json
./target/debug/spec.exe section get  spec-api/store design --json
./target/debug/spec.exe section delete spec-api/store design --json
```

> **Quirk:** `section list` returns filenames **with** the `.md` suffix
> (e.g. `["design.md", "risks.md"]`). Test assertions and consumers must match.

### Code references

```bash
# List code refs for a spec
./target/debug/spec.exe refs spec-api/store --json

# Validate that file/line ranges still resolve
./target/debug/spec.exe refs spec-api/store validate --workspace-root . --json
```

### Health check

```bash
./target/debug/spec.exe health --all --json | jq '{specs_checked, issues_count}'
./target/debug/spec.exe health spec-api --json
```

---

## MCP Tool Surface

Inside an agent session, prefer `mcp_spec-mcp_*` tools (defined in
`tools/mcp/spec-mcp/src/server.rs`):

| Tool | Purpose |
|---|---|
| `spec_create` / `spec_get` / `spec_update` / `spec_delete` | CRUD |
| `spec_list` / `spec_search` | Query |
| `spec_tree` | Hierarchy |
| `spec_health` | Validation |
| `spec_refs_validate` | CodeRef sanity |
| `spec_section_add` / `_list` / `_get` / `_delete` | Sections |
| `spec_scan` / `spec_add_root` | Index management |

There is **no** `spec_bootstrap` MCP tool yet — bootstrap is CLI-only.

---

## HTTP Surface

`spec-http` exposes 16 REST endpoints (default `127.0.0.1:4001`); see
`tools/http/spec-http/src/routes.rs`. Pattern mirrors `ticket-http`:

```
GET    /api/specs                      list
GET    /api/specs/search?q=...         search
POST   /api/specs                      create
GET    /api/specs/{id}                 manifest
GET    /api/specs/{id}/full            manifest + body
PATCH  /api/specs/{id}                 update fields/state
DELETE /api/specs/{id}                 soft-delete
GET    /api/specs/{id}/tree            subtree
GET    /api/specs/{id}/refs            code refs
POST   /api/specs/{id}/refs/validate   validate
GET    /api/specs/{id}/sections        list sections
POST   /api/specs/{id}/sections        add section
GET    /api/specs/{id}/sections/{n}    get section
DELETE /api/specs/{id}/sections/{n}    delete section
POST   /api/specs/scan                 reindex
POST   /api/specs/add-root             register root
GET    /healthz                        liveness
```

Routes use Axum 0.8's longest-match — static segments (`/health`, `/search`,
`/scan`) win over `/{id}`.

---

## Pitfalls & Gotchas

- **Slug character set is strict.** Only `[a-z0-9-]` per segment, `/` between
  segments. No `_`, no uppercase, no leading/trailing `-`. Bootstrap normalizes
  `_` → `-`; manual `create` calls must do the same.
- **State transitions are one-way.** No `--undo` (unlike tickets). To "revert",
  you must move forward through valid transitions or `delete` and recreate.
- **Soft-delete is not visible to `get`.** A deleted spec returns 404 even though
  the folder remains on disk. Use `list --where deleted=true` if you need to see
  them (TBD: check actual flag name in schema).
- **`scan` is implicit on every CLI call.** This makes the CLI safe but slow on
  large stores. The MCP server scans once at startup; long-running daemons should
  call `spec_scan` after manual file edits.
- **Windows path canonicalization.** Bootstrap canonicalizes both the crate path
  and the workspace root to keep `\\?\` prefixes consistent. If you write custom
  tooling that calls `SpecStore::create` with file paths, canonicalize both sides
  before `strip_prefix`.
- **`section list` returns filenames with `.md`.** Assertions of `"design"`
  against actual `"design.md"` will fail.
- **`section add --name foo`** is rewritten to `foo.md` if the suffix is missing,
  but listing reflects what's on disk.

---

## When to Use Each Spec State

| State | Meaning | Set when |
|---|---|---|
| `draft` | Initial sketch | Created by `bootstrap` or manual `create` |
| `reviewed` | A second pair of eyes signed off the design | After PR review of the spec body itself |
| `approved` | Cleared for implementation | When implementation can begin |
| `implemented` | Code matches the spec | When all CodeRefs validate and tests pass |
| `verified` | Validated end-to-end (schema-required path) | After acceptance criteria pass |
| `deprecated` | No longer current; kept for history | Replaced by a newer spec |
| `cancelled` | Will not be implemented | Scope dropped |

---

## Validation Workflow

Before closing a spec to `verified`:

1. `spec refs <id> validate --workspace-root .` → all refs must resolve
2. `spec health <id>` → no missing required fields
3. Tests covering the spec's component pass
4. Spec history shows `reviewed` and `approved` (schema gate)

---

## Related Files

- Schema: [crates/spec-api/schemas/specification.toml](../../crates/spec-api/schemas/specification.toml)
- Manifest model: [crates/spec-api/src/manifest.rs](../../crates/spec-api/src/manifest.rs)
- Store: [crates/spec-api/src/store.rs](../../crates/spec-api/src/store.rs)
- CodeRef: [crates/spec-api/src/code_ref.rs](../../crates/spec-api/src/code_ref.rs)
- Slug rules: [crates/spec-api/src/slug.rs](../../crates/spec-api/src/slug.rs)
- CLI args: [tools/cli/spec-cli/src/cli/args.rs](../../tools/cli/spec-cli/src/cli/args.rs)
- Bootstrap: [tools/cli/spec-cli/src/cli/commands/bootstrap.rs](../../tools/cli/spec-cli/src/cli/commands/bootstrap.rs)
- MCP server: [tools/mcp/spec-mcp/src/server.rs](../../tools/mcp/spec-mcp/src/server.rs)
- HTTP routes: [tools/http/spec-http/src/routes.rs](../../tools/http/spec-http/src/routes.rs)
