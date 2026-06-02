# Summary

`ticket board show`, `ticket next`, MCP `board_show`/`next_tickets`, and
HTTP `/api/workflow/next` expose inconsistent, narrowly scoped discovery knobs.
This spec defines the canonical, reusable **selector contract** that all
workflow discovery surfaces must honour.

**Tracking ticket:** `.ticket/tickets/790df512-d8a9-42bd-b3d6-6e2b4d5eda9c`
**Convergence tracker:** `.ticket/tickets/cf4246c3-6539-4f1c-a876-6d34073db7b3`

---

# Selector Axes

Every workflow-discovery surface (`next`, `board show`) MUST support the following
named selector dimensions. All dimensions are optional and independently combinable.

## 1. Workspace / Index-Root

- **Field name (JSON):** `workspace`
- **CLI flag:** `--workspace <name>` (resolves via registered workspace labels)
- **Semantics:** narrows discovery to a single registered ticket root. When
  omitted the active root is used. Multi-root aggregation is a future extension
  and is not part of this contract.
- **Machine-readable output field:** `scope.workspace` (string) — the resolved
  workspace label used for this query.

## 2. Title-Prefix Filter

- **Field name (JSON):** `filter`
- **CLI flag:** `--filter <prefix>`
- **Semantics:** retains only tickets whose title starts with the prefix string
  (case-sensitive). This is the existing `--filter` behaviour, made explicit.
- **Backward compat:** preserved as-is; no migration required.

## 3. Graph-Root Scope

- **Field name (JSON):** `root`
- **CLI positional / flag:** `root <id>` (existing CLI positional for `next`)
- **Semantics:** limits candidates to the set of unresolved prerequisite tickets
  that remain in the reverse-dependency tree rooted at the given ticket. When
  provided, the response MUST include `scope.root` with the resolved UUID.
- **Backward compat:** preserved. CLI `ticket next [root]` continues to work.

## 4. Field Predicates *(Phase 1 extension — not in Phase 0)*

- **Field name (JSON):** `where`
- **CLI flag:** `--where key=value` (repeatable, intersection semantics)
- **Semantics:** filter by indexed fields (`component`, `state`, `type`, `tags`).
  This axis is specified here for naming stability but is NOT implemented in
  Phase 0. Surfaces that add it later MUST use these field names.

---

# Composition Semantics

1. All active selector dimensions are applied as **intersection** (AND).
   A ticket must satisfy every supplied dimension to appear in results.
2. If no selector dimensions are supplied, results are repository-wide for the
   active workspace.
3. An empty result set is valid and MUST be returned as `count: 0, items: []`.
   It is NOT an error.
4. Dimension omission means "no constraint on this axis", not "empty set".

---

# Machine-Readable Scope Metadata

Every workflow-discovery response MUST include a top-level `scope` object with
at minimum:

```jsonc
{
  "scope": {
    "workspace": "default",        // resolved workspace label (always present)
    "active_index_root": "/abs/path/to/.ticket",  // resolved store root path
    "filter": null,                // title prefix applied, or null
    "root": null                   // UUID string if a root was specified, else null
  }
}
```

**Rationale:** frontends (ticket-viewer, ticket-vscode) and downstream tools
must not reconstruct which store or scope produced a result; the producing
surface must declare it explicitly.

---

# Backward Compatibility Rules

| Existing input | Status | Notes |
|---|---|---|
| `ticket next [root]` | Preserved | Root positional stays; also exposed as `scope.root` in JSON |
| `ticket next --filter` | Preserved | Also reflected as `scope.filter` in JSON |
| MCP `next_tickets.workspace` | Preserved | |
| MCP `next_tickets.filter` | Preserved | |
| HTTP `?filter=`, `?root=` | Preserved | |

No existing field is renamed or removed. New fields are additive.

---

# Surfaces in Scope (Phase 0)

| Surface | Change |
|---|---|
| CLI `ticket next` | Add `scope` object to JSON output |
| CLI `ticket board show` | Add `scope` object to JSON output |
| MCP `next_tickets` | Add `root` input param; add `scope` object to response |
| MCP `board_show` | Add `scope` object to response |

HTTP `/api/workflow/next` is Phase 1 (depends on `0e375356`).

---

# Regression Matrix

| Scenario | CLI | MCP | HTTP |
|---|---|---|---|
| Workspace-only query | scope.workspace correct | scope.workspace correct | Phase 1 |
| root-scoped next | scope.root = resolved UUID | scope.root = resolved UUID | Phase 1 |
| filter-scoped next | scope.filter = prefix | scope.filter = prefix | Phase 1 |
| No selector (wide) | scope fields null | scope fields null | Phase 1 |
| board show scope | scope.active_index_root set | scope.active_index_root set | Phase 1 |

---

# Related Tickets

- `790df512` — this spec ticket (selector contract definition)
- `68a08b34` — implementation (CLI + MCP scope-aware board/next)
- `0e375356` — Phase 1 (full selector surface across CLI, MCP, HTTP)
- `c031aeb0` — Phase 2 (ticket-api core + adapter boundaries)
- `6484d4b7` — Phase 3 (parity validation suite)
