---
tags: `#spec-viewer` `#design` `#navigation` `#routing` `#dioxus`
summary: Concrete URL and state model for spec-viewer entry × view navigation and reachable page graph
status: 📋
---

# Design: Spec Viewer Entry × View Navigation

## Problem

`spec-viewer` currently exposes multiple useful UI surfaces, but the click-through graph is incomplete and the navigation state is not modeled as a single coherent contract.

The current router exposes:

- `/specs` for the root browse page
- `/specs/graph` for the collection graph
- `/specs/:id` for direct detail pages
- `/specs/tree` for a separate tree page

This leads to two UX problems:

1. Some pages are effectively URL-first instead of clearly reachable from the in-app navigation graph.
2. The active detail view (`Body`, `Sections`, `CodeRefs`, `Health`, later `Graph` or other views) is not preserved as a first-class dimension when the selected spec changes.

The target model follows the same broad pattern already used in `log-viewer-dioxus`: navigation should be representable as a 2-dimensional state of `entry + view`.

## Goals

- Make the important spec-viewer surfaces reachable by clicking through the app.
- Preserve the active view when the user switches from one spec to another.
- Keep deep links and browser history meaningful.
- Remove the standalone `/specs/tree` route as a primary UX destination.
- Make `/specs` the single root browse surface that absorbs the current tree/list role.

## Canonical State Model

Navigation state is split into two axes:

- `entry`
  - `none` for collection-level destinations
  - `spec:<id>` for a selected spec
- `view`
  - collection-level: `browse`, `graph`
  - spec-level: `body`, `sections`, `coderefs`, `health`

`entry` answers "what entity is selected?"

`view` answers "which presentation of that entity or collection is active?"

## Valid Combinations

### Collection-level states

- `(none, browse)` → root browse page
- `(none, graph)` → collection graph page

### Spec-level states

- `(spec:<id>, body)`
- `(spec:<id>, sections)`
- `(spec:<id>, coderefs)`
- `(spec:<id>, health)`

Invalid combinations should never be emitted by the app and must be normalized on URL load.

## URL Model

Use path + query to keep the URL readable while making `view` explicit.

### Canonical URLs

- `/specs`
  - means `(none, browse)`
- `/specs/graph`
  - means `(none, graph)`
- `/specs/:id`
  - means `(spec:<id>, body)` by default
- `/specs/:id?view=sections`
  - means `(spec:<id>, sections)`
- `/specs/:id?view=coderefs`
  - means `(spec:<id>, coderefs)`
- `/specs/:id?view=health`
  - means `(spec:<id>, health)`

### Legacy route handling

- `/specs/tree` should stop being a primary navigation destination.
- Existing deep links to `/specs/tree` should redirect to `/specs`.

## In-App Navigation Semantics

### Root browse page (`/specs`)

`/specs` becomes the main tree/list browse surface. It must expose click paths to:

- selected spec detail state
- collection graph state

The old standalone tree behavior is folded into this page by rendering root-level entries in a list/tree-oriented browse UI.

### Graph page (`/specs/graph`)

The graph remains a collection-level view.

Selecting a node from the graph should navigate to a spec-level state using the current global spec view preference:

- if global view is valid for specs, open `/specs/:id?view=<global-view>`
- otherwise use the selected spec's remembered last view
- otherwise fall back to `/specs/:id`

### Detail page (`/specs/:id`)

Changing tabs updates only `view`, not `entry`.

Selecting a different spec from any in-app chooser updates `entry` while preserving `view` if possible.

## View Preservation Rules

Maintain two related pieces of client state:

- `global_spec_view`: the currently active spec-level view the user most recently chose
- `last_view_by_spec[id]`: the last valid spec-level view used for each specific spec

### On tab/view change inside a spec

- set `global_spec_view = new_view`
- set `last_view_by_spec[id] = new_view`
- write the new URL for `(spec:<id>, new_view)`

### On spec switch

Given current source state `(spec:<old>, current_view)` and destination `spec:<new>`:

1. If `current_view` is valid for the destination, navigate to `(spec:<new>, current_view)`.
2. Else if `last_view_by_spec[new]` exists and is valid, navigate to `(spec:<new>, last_view_by_spec[new])`.
3. Else navigate to `(spec:<new>, body)`.

This matches the interviewed requirement: preserve the current tab globally, but fall back to the last active tab per spec if the global tab cannot be shown.

## Browser History Semantics

History entries must represent the full normalized state, not just the selected spec id.

That means:

- switching from `body` to `sections` pushes or replaces history according to the chosen tab-navigation policy
- switching from spec `A` to spec `B` while on `sections` records the destination as `B + sections`
- browser back/forward restores both `entry` and `view`

The important invariant is:

> Replaying browser history must restore the same visible page state the user saw when that history entry was created.

## Reachable Page Graph

Every primary page must be reachable by clicks only after entering the app at `/specs`.

Required graph:

- `/specs` → `/specs/graph`
- `/specs` → `/specs/:id?view=<resolved-view>`
- `/specs/graph` → `/specs`
- `/specs/graph` → `/specs/:id?view=<resolved-view>`
- `/specs/:id?...` → `/specs`
- `/specs/:id?...` → `/specs/graph`
- `/specs/:id?...` → `/specs/:other?...`

No primary workflow should require the user to know or hand-edit a URL to reach one of those states.

## Proposed Implementation Breakdown

1. Introduce a concrete navigation contract for `entry × view`, including normalization and legacy-route handling.
2. Refactor the root browse page so `/specs` absorbs the tree-view role and exposes coherent click-through links.
3. Rework spec switching to preserve the active view with per-spec fallback.
4. Add browser regression coverage for reachability, history restoration, and deep links.

## Verification Matrix

### Click-through reachability

- Starting from `/specs`, reach graph and a specific spec detail page by clicks only.
- Starting from detail, return to browse and graph by clicks only.

### Entry × view preservation

- Open spec `A` on `Sections`.
- Switch to spec `B`.
- Confirm `Sections` remains active if valid.
- If not valid, confirm fallback uses `last_view_by_spec[B]`, else `Body`.

### History restoration

- Navigate across `(A, body)` → `(A, sections)` → `(B, sections)` → `(none, graph)`.
- Use browser back/forward and confirm both axes restore correctly.

### Deep links

- Load `/specs/:id?view=health` directly and confirm the corresponding state renders.
- Load `/specs/tree` and confirm it normalizes to `/specs`.

## Related Ticket

- Parent design ticket: `521e18b7-bed4-4588-886c-e25d6c8ddc8b`