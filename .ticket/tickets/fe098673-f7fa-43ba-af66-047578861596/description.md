## Problem

Each memory-api tool domain (ticket, spec, rule, audit, test, log, doc, session) owns a store folder (`.ticket/`, `.spec/`, `.rule/`, etc.). Agents and humans currently navigate those stores by scanning raw TOML and markdown files, which is verbose and causes repetitive, hard-to-update guidance infrastructure. The same store layout descriptions are duplicated across `AGENTS.md`, `.agents/instructions/`, and inline prompts.

## Goal

Generate lightweight, committed index artifacts for every memory-api store domain:
- A human-readable `README.md` co-located inside each store folder (e.g. `.ticket/README.md`)
- A compact machine-readable TOON sidecar co-located with the README (e.g. `.ticket/index.toon`)
- An agent-consumable hook under `.agents/` that points agents at the indexed store

This eliminates guidance duplication: instead of repeating store layout in every instruction file, agents reference the single committed index. The store index also serves as the primary surface for similarity search and RAG retrieval.

## Scope

This track lives entirely in the memory-api ecosystem (`memory-viewers/memory-api/`). **The `context-stack/` codebase is not touched at all.** The shared schema types are `IndexEntry` (an entity captured in a memory-api domain index) and `IndexRef` (a cross-reference link between index entries) — named after what they represent inside the generated index, not the store infrastructure.

## Resolved design decisions

- **D1 — Placement:** index nodes live across the file tree — folder-level READMEs, workspace-folder indexes (co-located with the store root), and `.agents/` agent-hook nodes.
- **D2 — Hook type:** git pre-commit hooks, profiled for low commit latency; post-commit fallback if budget exceeded.
- **D3 — Spec depth:** full depth, one file per node, one canonical-named folder per node for its children, relative markdown links.
- **D4 — Rule grouping:** slug-prefix segments for now; no new `category` field.
- **D5 — Committed:** all generated index files committed to git.
- **D6 — Test catalog gate:** wait for test-api and log-api bootstrap to reach `done`.
- **D7 — Missing tests:** emit as `not-run`; catalog is a complete registry.
- **D8 — Encoding:** TOON primary, slim-but-dense references; JSON opt-in only.
- **D9 — Workspace DAG:** each store workspace is a DAG node with multiple parents/children, holding a config folder that indexes parent/child names + locations. Each workspace is the root anchor for its tool execution.

## Child tickets

- `0dba399a` Define IndexEntry schema and serde contract ← blocks all generators
- `e7a0ee3c` IndexEntry TOON sidecar format and validator ← blocks all generators
- `c5e9bb39` Ticket store index generator with git hook integration
- `b9757ba7` Spec store hierarchy generator
- `9336a096` Rule store catalog generator
- `855a1e5d` Audit store status summary generator
- `a72e3aca` Test store catalog generator (gated on test-api + log-api bootstrap)
- `c2409055` Memory workspace DAG indexing