# spec-api Crate: SpecManifest Model

## Objective

Create the `crates/spec-api/` crate with the core specification data model, multi-file folder structure, and human-readable slug system.

## Spec Folder Structure

Each spec is stored as a UUID-named directory with multiple files:

```
<scan_root>/<uuid>/
  spec.toml             ← manifest (metadata, fields, slug, parent, code_refs)
  body.md               ← main specification body
  sections/             ← optional sub-sections
    overview.md
    api-surface.md
    error-handling.md
  assets/               ← diagrams, images
    architecture.png
  .spec-lock            ← advisory lock (fs4 exclusive)
  history.ndjson        ← append-only revision log
```

## SpecManifest Fields

```toml
# spec.toml
id = "uuid"
created_at = "2026-04-18T..."
slug = "ticket-api/storage/store"        # unique human-readable identifier
title = "TicketStore — Central Storage Coordinator"
type = "specification"                   # entity type for schema lookup
state = "draft"                          # lifecycle state

# Hierarchy
parent = "uuid-of-parent-spec"           # optional parent spec ID

# Classification
component = "ticket-api"                 # which crate/tool
scope = "module"                         # crate | module | function | trait | type

# Code references (symbol-level)
[[code_refs]]
file = "crates/ticket-api/src/storage/store.rs"
symbol = "TicketStore"
kind = "struct"
line_start = 45
line_end = 52

[[code_refs]]
file = "crates/ticket-api/src/storage/store.rs"
symbol = "TicketStore::create"
kind = "fn"
line_start = 120
line_end = 180

# Feature tracking
[features]
implemented = ["create", "get", "update", "delete", "list", "search"]
planned = ["bulk_update", "import_export"]
blocked = []
bugs = ["stale index after concurrent writes"]
```

## Slug System

- Slugs are hierarchical, separated by `/`: `ticket-api/storage/store`
- Must be unique within the spec store
- Validated: lowercase alphanumeric + hyphens + slashes
- Used in CLI: `spec get ticket-api/storage/store`
- Resolved via index lookup (slug → UUID)

## Implementation Plan

1. Create `crates/spec-api/Cargo.toml` depending on `memory-api`
2. Define `SpecManifest` extending `memory_api::EntityManifest`
3. Define `CodeRef` struct for symbol-level references
4. Define `FeatureStatus` struct for feature tracking
5. Implement slug validation and uniqueness enforcement
6. Implement multi-file folder read/write extending `EntityFs`
7. Add section management (create/read/update/delete sections)

## Acceptance Criteria

- [ ] `crates/spec-api/` crate compiles
- [ ] SpecManifest supports all fields above
- [ ] Multi-file folder structure: spec.toml + body.md + sections/ + assets/
- [ ] Slug validation rejects invalid slugs
- [ ] CodeRef struct with file, symbol, kind, line_start, line_end
- [ ] Section management: CRUD for sections/*.md files
- [ ] Unit tests for manifest serialization roundtrip