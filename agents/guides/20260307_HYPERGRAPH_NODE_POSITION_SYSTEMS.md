# Hypergraph Node Position Systems Reference

> All systems that modify node positions in the HypergraphView, their active settings, and the cascading enable chain.

## Settings Cascade

```
autoLayout=OFF ──→ nesting hidden, duplication hidden
    │
autoLayout=ON  ──→ nesting toggle shown
    │                    │
    │              nesting=OFF ──→ duplication hidden
    │              nesting=ON  ──→ duplication toggle + depth sliders shown
    │                    │
    │              duplication=OFF
    │              duplication=ON
```

**Defaults:** `autoLayout=OFF`, `nesting=OFF`, `duplication=OFF`

## Behavior Matrix

| autoLayout | nesting | duplication | Behavior |
|------------|---------|-------------|----------|
| OFF | - | - | Nodes draggable. Selecting pans camera only. No layout changes. |
| ON | OFF | - | Nodes fixed. Selecting moves parents into focused layout. Children highlighted (not expanded). |
| ON | ON | OFF | Selecting expands node (decomposition rows). Parents shown as shell containers. Children reparented inside. |
| ON | ON | ON | Same as above + children shown both inside parent AND at original graph position. |

## Systems That Modify Node Positions

### 1. Force-Directed Layout (`layout.ts` → `buildLayout()`)
- **When:** On snapshot load (once)
- **What:** Initial XZ circular + spring simulation, Y by width
- **Output:** `LayoutNode.{x,y,z}` and `{tx,ty,tz}` (animation targets)
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/layout.ts`

### 2. Animation Lerp (`animation/nodeAnimator.ts` → `animateNodes()`)
- **When:** Every frame
- **What:** Exponential decay lerp from `{x,y,z}` → `{tx,ty,tz}`
- **Gated by:** Always active
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/animation/nodeAnimator.ts`

### 3. Focused Layout (`layout.ts` → `computeFocusedLayout()`)
- **When:** Node selected AND `autoLayout=ON`
- **What:** Computes 2D offsets for parent nodes (fan layout above selected). Sets `{tx,ty,tz}` using camera axes projection.
- **Gated by:** `autoLayout` prop in `HypergraphViewCore`
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/layout.ts`

### 4. Search Path Layout (`layout.ts` → `computeSearchPathLayout()`)
- **When:** Node selected AND `autoLayout=ON` AND active search path has root
- **What:** Anchors layout on search path root instead of selected node
- **Gated by:** `autoLayout` + `currentSearchPath?.root`
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/layout.ts`

### 5. Active Transform Application (`hooks/useOverlayRenderer.ts`)
- **When:** Every frame when focused layout is active
- **What:** Resets all `{tx,ty,tz}` to base positions, then layers focused offsets on top using camera axes
- **Gated by:** `selectedIdx >= 0 && focusedOffsets != null && basePositions != null`
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/hooks/useOverlayRenderer.ts`

### 6. Mouse Drag (`hooks/useMouseInteraction.ts`)
- **When:** `autoLayout=OFF` AND user drags a node
- **What:** Ray-plane intersection updates `{x,y,z}` and `{tx,ty,tz}` directly
- **Gated by:** `!autoLayoutRef.current`
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/hooks/useMouseInteraction.ts`

### 7. DOM Positioning (`animation/nodePositioner.ts` → `positionDOMNodes()`)
- **When:** Every frame
- **What:** Projects 3D `{x,y,z}` → screen CSS transforms. Also positions shell containers and duplicate nodes.
- **Gated by:** Always active
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/animation/nodePositioner.ts`

### 8. Decomposition Reparenting (`decomposition/manager.ts`)
- **When:** `nesting=ON` AND node selected
- **What:** Reparents child DOM elements into decomposition rows inside expanded parent. Back-projects screen position to world coords for edge connectivity.
- **Gated by:** `nestingSettings.enabled`
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/decomposition/manager.ts`

### 9. Shell Layout (`nesting/shellLayout.ts` → `computeShellLayout()`)
- **When:** `nesting=ON` AND node selected
- **What:** Computes parent shell containers (Russian-doll nesting)
- **Gated by:** `nestingSettings.enabled`
- **Parameters:** `parentDepth` (1-5)
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/nesting/shellLayout.ts`

### 10. Duplicate Manager (`nesting/duplicateManager.ts` → `buildDuplicates()`)
- **When:** `nesting=ON` AND `duplication=ON` AND node selected
- **What:** Creates DuplicateNode descriptors for children shown both inside parent and at original position
- **Gated by:** `nestingSettings.enabled && nestingSettings.duplicateMode`
- **Parameters:** `childDepth` (1-3)
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/nesting/duplicateManager.ts`

### 11. Edge Highlights (`nesting/edgeHighlights.ts`)
- **When:** `nesting=ON` AND duplicates exist
- **What:** Hides parent↔child edges, applies glow classes to involved nodes
- **Gated by:** Same as duplicates
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/nesting/edgeHighlights.ts`

### 12. Base Position Reset (overlay renderer)
- **When:** Focused layout deactivates OR children released from decomposition
- **What:** Resets `{tx,ty,tz}` to original force-directed positions so nodes animate home
- **File:** `tools/log-viewer/frontend/src/components/HypergraphView/hooks/useOverlayRenderer.ts`

## Key Files

| File | Role |
|------|------|
| `HypergraphView.tsx` | Signal wrapper, reads store, passes props |
| `HypergraphViewCore.tsx` | Signal-free core (extractable to viewer-api) |
| `layout.ts` | Force-directed + focused layout algorithms |
| `hooks/useOverlayRenderer.ts` | Frame loop orchestrator (GPU + DOM + nesting) |
| `hooks/useMouseInteraction.ts` | Drag/select/hover (gates drag on `!autoLayout`) |
| `hooks/useNestingState.ts` | NestingSettings persistence (localStorage) |
| `animation/nodeAnimator.ts` | Lerp toward targets |
| `animation/nodePositioner.ts` | 3D→screen projection + shell/dup positioning |
| `decomposition/manager.ts` | DOM reparenting for expanded nodes |
| `nesting/shellLayout.ts` | Parent shell geometry |
| `nesting/duplicateManager.ts` | Duplicate node descriptors |
| `nesting/edgeHighlights.ts` | Edge hiding + glow highlights |
| `components/ControlsHUD.tsx` | UI controls with cascading enables |
| `components/NodeLayer.tsx` | DOM rendering (shells, nodes, duplicates) |
| `store/index.ts` | `autoLayoutEnabled` signal |

## Viewer-API Extraction Notes

`HypergraphViewCore` is already signal-free and designed for extraction. To move to viewer-api:
1. Move all files under `HypergraphView/` except `HypergraphView.tsx` (the signal wrapper)
2. The `autoLayoutEnabled` signal currently lives in log-viewer's store and is read directly by `ControlsHUD` — this needs to be converted to a prop/callback pattern
3. `ControlsHUD` still imports `autoLayoutEnabled` from the store — refactor to accept `autoLayout` + `onAutoLayoutChange` props
