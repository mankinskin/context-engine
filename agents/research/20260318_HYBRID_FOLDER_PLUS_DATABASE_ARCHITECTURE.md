# Status: TODO

# Hybrid Architecture: Ticket Folder Structures + Database Index

## Vision

Each ticket is both:
- A structured folder with rich assets, and
- A database-backed entity for search, dependencies, status, and transactional workflows.

## Proposed Ticket Folder Layout (Example)

```
<workspace>/tickets/TCK-2026-0001/
  ticket.toml                  # required: canonical manifest
  description.md               # required: rich text body
  checklist.json               # optional/required by ticket type
  dependencies.json            # optional explicit refs
  validation.json              # generated findings or status
  progress.json                # progress snapshots
  assets/
    screenshot-1.png
    trace.log
  history/
    events-000001.jsonl
    events-000002.jsonl
```

## Required/Optional File Policy

- Ticket type defines required files and field schemas.
- Validate on create/update/commit.
- Keep schema versions in manifest (`schema_version`).

## Consistency Model

### Option A: DB-first transaction
1. Begin DB tx
2. Write/validate files to temp
3. Move files atomically
4. Write DB metadata + event
5. Commit DB tx

### Option B: Event-first journal
1. Append intent event
2. Apply FS changes
3. Update DB projection
4. Mark event committed

### Option C: File-first with reconciliation
- Best for permissive local editing but needs robust reconciliation scanner.

## Recommended Starting Point

- Start with Option A for strict correctness.
- Add reconciliation command for repair (`rebuild-index`, `verify-ticket`).

## Data Ownership

- Files are source-of-truth for rich documents/assets.
- DB is source-of-truth for queryable state and relationships.
- Event log is source-of-truth for change history semantics.

## Validation Pipeline

- Manifest schema validation (serde + schema)
- Domain rules (state transition legality, dependency cycles)
- Required-file checks by ticket type
- Reference integrity checks (ticket links, attachment paths)

## TODO

- TODO: Define ticket type registry and schema evolution rules.
- TODO: Define atomic write protocol per OS/filesystem.
- TODO: Design repair commands and failure recovery UX.
- TODO: Define lock granularity (global, per-workspace, per-ticket).
