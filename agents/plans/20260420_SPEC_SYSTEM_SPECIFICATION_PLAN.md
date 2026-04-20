# Implementation Plan — Specifying the Spec System Itself

**Date:** 2026-04-20
**Tickets:** [13a57a83](../../.ticket/tickets/13a57a83-df99-4031-87e2-844772758ebb/) (P8 spec-system bootstrap), partially overlaps [9242a906](../../.ticket/tickets/9242a906-cba9-43a4-b45e-942465379a7b/) (ticket-api specs).
**Status:** in-progress (Phase 1 complete: bootstrap done)

## Goal

Produce a high-quality, hand-curated specification database for the spec system
crates (`memory-api`, `spec-api`, `spec-cli`, `spec-mcp`, `spec-http`) that
serves as both **documentation** and the **canonical fixture** for spec tooling.

## Current State (after this session)

`spec bootstrap` has been run on all five crates. The store contains **38 specs**:

| Crate | Root + module specs | Health |
|---|---|---|
| `memory-api` | 1 + 16 | ok |
| `spec-api` | 1 + 6 | ok |
| `spec-cli` | 1 + 3 | ok |
| `spec-mcp` | 1 + 1 | ok |
| `spec-http` | 1 + 7 | ok |

All specs are in state `draft`. CodeRefs validate (`spec refs <id> validate`
returns `valid: true` for sampled specs). What's missing:

- **Bodies are auto-generated stubs.** Every body just says "Bootstrapped from
  source analysis. See child specs for individual module documentation." Real
  design rationale, invariants, and acceptance criteria are absent.
- **No domain root spec.** The ticket asks for a `spec-system` umbrella spec
  with an architecture diagram covering `memory-api → spec-api → CLI/MCP/HTTP`.
  Bootstrap creates one root per crate but no cross-crate spec.
- **No `sections/`.** The ticket calls for `design.md`, `risks.md`,
  `acceptance.md` per module spec. None exist yet.
- **No state advancement.** All specs are `draft`; none have been
  `reviewed`/`approved`/`implemented`/`verified`.

## Scope (in)

1. Author a `spec-system` umbrella root spec with architecture diagram.
2. Rewrite the auto-generated bodies for the **5 crate roots** and the
   **most important module specs** (selection table below) with hand-curated
   content that covers: purpose, key types, invariants, common pitfalls.
3. Add `design.md` + `acceptance.md` sections to each crate root.
4. Add cross-component `linked` edges where appropriate (e.g.
   `spec-api/store --linked--> memory-api/storage/entity-store`).
5. Advance specs to `reviewed` once content exists, then `approved`.

## Scope (out)

- `ticket-api` and `context-*` specs — separate ticket [9242a906](../../.ticket/tickets/9242a906-cba9-43a4-b45e-942465379a7b/).
- Skill generation (`docs/skills/*.md` from spec data) — ticket [eddf5d2e](../../.ticket/tickets/eddf5d2e-e1b6-4ec9-b88f-d50bd192b194/).
- Spec-to-code sync (auto line-number updates) — ticket [80e25216](../../.ticket/tickets/80e25216-7ba9-4fd9-bc80-3311f1d2a604/).

## Phases

### Phase 1 — Bootstrap (DONE)

- [x] Add scan root, run `spec bootstrap` on memory-api, spec-api, spec-cli, spec-mcp, spec-http
- [x] Fix slug normalization bug (`_` → `-`) in `bootstrap.rs`
- [x] Verify `spec health --all` is clean
- [x] Verify CodeRefs validate

**Output:** 38 specs in `.spec/specs/`, all `state=draft`.

### Phase 2 — Umbrella spec

Create the `spec-system` root spec covering the whole domain.

```bash
spec create \
  --title "Spec System" \
  --slug spec-system \
  --component spec-system \
  --scope domain \
  --body-file spec-system-body.md
```

Body must cover:
- One-paragraph elevator pitch (link spec ↔ code, hierarchy, multi-transport)
- Architecture diagram (mermaid) showing `memory-api → spec-api → {spec-cli, spec-mcp, spec-http}`
- State machine reference
- On-disk layout

Then re-parent the five crate root specs:

```bash
spec update spec-api    --field 'parent=<spec-system-uuid>'
spec update memory-api  --field 'parent=<spec-system-uuid>'
spec update spec-cli    --field 'parent=<spec-system-uuid>'
spec update spec-mcp    --field 'parent=<spec-system-uuid>'
spec update spec-http   --field 'parent=<spec-system-uuid>'
```

### Phase 3 — Hand-curate priority specs

Curate the bodies for these specs (in priority order). For each: rewrite
`body.md`, add a `design.md` section, then advance to `reviewed`.

**Priority A — Foundation (must be excellent):**
| Slug | Why |
|---|---|
| `memory-api` | Foundation for spec-api and ticket-api both |
| `memory-api/storage/entity-store` | Generic store contract |
| `memory-api/model/schema` | Schema registry mechanism |
| `spec-api` | Crate overview |
| `spec-api/manifest` | SpecManifest contract |
| `spec-api/store` | SpecStore behaviour, slug uniqueness |
| `spec-api/code-ref` | CodeRef format and validation |
| `spec-api/slug` | Slug rules (the bug we just hit) |

**Priority B — Tools surface:**
| Slug | Why |
|---|---|
| `spec-cli` | All subcommands, JSON shape, `bootstrap` |
| `spec-cli/cli/commands/bootstrap` | Algorithm details |
| `spec-mcp` | Tool catalog, contract stability |
| `spec-mcp/server` | Tool definitions and registration |
| `spec-http` | REST surface, route ordering quirk |
| `spec-http/routes` | Endpoint table |
| `spec-http/handlers/specs` | Read/write split |

**Priority C — Leaf modules:**
Remaining 23 module specs — minimal hand-curation; bullet list of public items
plus a 1-paragraph "what does this module do" is enough.

### Phase 4 — Cross-component edges

Add `linked` edges to make navigation natural. (Edge support exists in
memory-api but **needs verification** that spec-api exposes it — see open
question below.)

Suggested edges:
- `spec-api/store` ↔ `memory-api/storage/entity-store`
- `spec-api/manifest` ↔ `memory-api/model/entity`
- `spec-api/default-schema` ↔ `memory-api/model/schema`
- `spec-cli` ↔ `spec-api`
- `spec-mcp` ↔ `spec-api`
- `spec-http` ↔ `spec-api`

### Phase 5 — Acceptance & state advancement

For each Priority A+B spec:

1. Add `acceptance.md` section listing observable acceptance criteria.
2. `spec refs <id> validate --workspace-root .` → must pass.
3. `spec update <id> --state reviewed`.
4. `spec update <id> --state approved`.
5. (For specs whose code is already merged) `spec update <id> --state implemented`.

`verified` is held back until end-to-end tests for the spec component exist
and pass — schema requires `reviewed` + `approved` in history first.

## Open Questions / Risks

1. **`spec-api` does not expose `parent_of` or `linked` edge management
   from the CLI.** The schema declares these edge kinds (in
   `crates/spec-api/schemas/specification.toml`) but I see no `spec link`
   subcommand in `args.rs`. `parent` is set as a manifest field, not as a
   real edge in the store. **Need to confirm:** is parent-child actually
   stored as an edge, or only as a manifest string field? If only a string,
   Phase 4 (cross-component `linked` edges) requires either:
   - Adding a `spec link` CLI subcommand mirroring `ticket link`, or
   - Storing edges via direct manipulation of the underlying `EntityStore`,
     or
   - Deferring Phase 4 to a follow-up ticket and recording links as
     `linked_to` text fields on the manifest (loses query power).

2. **No `revert`/`undo` on spec updates.** If a hand-curated body is wrong,
   the only recovery is re-writing the body (history is append-only).
   Acceptable risk; tickets have the same property until `--undo` was added.

3. **Bootstrap re-runs on a curated store would skip existing slugs but
   never refresh CodeRefs after refactors.** Spec-to-code sync (ticket
   [80e25216](../../.ticket/tickets/80e25216-7ba9-4fd9-bc80-3311f1d2a604/)) addresses this — out of scope here.

4. **No diff/preview before applying body edits.** Authoring discipline:
   write each body in `humans/tmp/spec-bodies/<slug>.md`, review, then
   `spec update <id> --body-file <path>`.

## Acceptance Criteria

- [ ] `spec-system` root spec exists with architecture diagram in body
- [ ] Five crate root specs are re-parented under `spec-system`
- [ ] All Priority A specs have hand-curated bodies, `design.md` section, and
      `acceptance.md` section
- [ ] All Priority B specs have hand-curated bodies and `acceptance.md`
- [ ] `spec health --all` returns 0 issues
- [ ] `spec refs <id> validate --workspace-root .` returns `valid: true`
      for every Priority A + B spec
- [ ] All Priority A + B specs are at state `approved` or `implemented`
- [ ] (Conditional on Phase 4 viability) cross-component `linked` edges added

## Estimated Volume

~30 hand-curated bodies + ~15 `design.md` sections + ~15 `acceptance.md`
sections + 1 umbrella spec. Most bodies are short (50–150 lines of
Markdown), so this is a multi-session effort. Suggested batching: one crate
per session, Priority A first.

## Related Tickets

- [13a57a83](../../.ticket/tickets/13a57a83-df99-4031-87e2-844772758ebb/) — Bootstrap: write spec files for the spec system itself (this plan)
- [9242a906](../../.ticket/tickets/9242a906-cba9-43a4-b45e-942465379a7b/) — Bootstrap: write spec files for ticket-api interfaces
- [eddf5d2e](../../.ticket/tickets/eddf5d2e-e1b6-4ec9-b88f-d50bd192b194/) — Skill generation (consumes the spec data this plan produces)
- [80e25216](../../.ticket/tickets/80e25216-7ba9-4fd9-bc80-3311f1d2a604/) — Spec-to-code sync (keeps CodeRefs fresh after Phase 1)
- [00798e96](../../.ticket/tickets/00798e96-3d82-436e-963c-af347e76ede0/) — Spec creation templates with acceptance criteria
