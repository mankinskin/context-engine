# viewer-api: Graph3D component

Canonical specification for the shared 3D dependency-graph Dioxus component
(`viewer-api/frontend/dioxus/src/graph3d/`) used by ticket-viewer and
spec-viewer.

## Public surface

- `Graph3D { layout, container_id, nodes, edges, on_node_click,
  on_node_hover }`.
- `graph3d::can_use_webgpu() -> bool` — runtime probe.
- `interaction::install(container_id, state)` — mouse handlers (orbit /
  pan / zoom / right-button-drag with contextmenu suppression).
- Node DOM: `data-node-idx="<i>"` cards positioned per-frame inside
  `#graph3d-nodes`.

## Demo behavior

The `pages/graph3d.rs` page renders a hard-coded 12-node / 18-edge sample
graph fetched from `/api/demo/graph`:

1. Cards are HTML (Dioxus) overlays positioned by the per-frame callback.
2. Pointer interactions:
   - Left drag: orbit camera.
   - Shift+left drag or right drag: pan.
   - Wheel: zoom.
   - Right-button drag must NOT open the browser context menu.
3. Selection: clicking a node highlights it and shows its metadata.
4. A "fit camera" button resets the view.
5. WebGPU-unavailable fallback renders an SVG version of the same graph.

## Acceptance behavior (validated by e2e)

- The graph mounts with WebGPU available; ≥1 card is positioned with
  `display: block`.
- Right-button drag fires `contextmenu` with `defaultPrevented === true`
  (regression coverage for the existing fix in
  `tools/viewer/viewer-api/frontend/dioxus/src/graph3d/interaction.rs`).
- Plain right-click leaves `defaultPrevented === false`.
- Clicking a node emits `on_node_click` with the matching node id.
- Without WebGPU (no flags), the SVG fallback renders ≥1 `<line>`
  representing an edge.

## Code references

- `tools/viewer/viewer-api/frontend/dioxus/src/graph3d/`
- `tools/viewer/e2e/tests/demo-viewer/graph3d.spec.ts`
- `tools/viewer/e2e/tests/dioxus/graph3d-right-drag.spec.ts` (existing
  regression test, kept alongside the demo-viewer test).
