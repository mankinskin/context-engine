# spec-viewer/routes

Defines the Dioxus `Route` enum and the four page-level components for the spec-viewer SPA.

## Route Table

| Pattern | Route variant | Page component |
|---|---|---|
| `/` | — | Redirect to `/workspace/default` |
| `/workspace/:ws` | `SpecListPage` | Flat list of all specs with search bar |
| `/workspace/:ws/tree` | `SpecTreePage` | Collapsible parent-child hierarchy tree |
| `/workspace/:ws/spec/:slug` | `SpecDetailPage` | Body + sections + CodeRefs + health |

## SpecListPage

- Renders a `SearchBar` at top.
- Below: scrollable list of `SpecCard` rows sorted by last-updated descending.
- Filter chips for `state` (draft / reviewed / approved / implemented / verified) and
  `component`.
- Clicking a row navigates to `SpecDetailPage`.
- SSE subscription via `use_sse`: on `spec.created` / `spec.updated` / `spec.deleted`
  events the list refreshes without full page reload.

## SpecTreePage

- Fetches `GET /api/specs/tree` (or `GET /api/specs/:slug/tree` if a root is selected).
- Renders a recursive `SpecTree` component with collapse/expand toggle per node.
- Nodes display `StateBadge` and click to navigate to `SpecDetailPage`.
- URL-synced expand state stored in `localStorage`.

## SpecDetailPage

- Fetches `GET /api/specs/:slug?full=true` on mount.
- Renders body Markdown via `innerHTML` (sanitised with a Rust DOMPurify equivalent or
  a plain-text fallback for security).
- Tabs: **Body** | **Sections** | **CodeRefs** | **Health**.
- Each section listed in the **Sections** tab; clicking loads `GET /api/specs/:slug/sections/:name`.
- **CodeRefs** tab renders `CodeRefList` with file, symbol, kind, line range.
- **Health** tab calls `GET /api/specs/:slug/health` and shows issue list.
- "Edit in spec-editor" link opens `http://localhost:4003/workspace/:ws/spec/:slug`.
