# Plan: Nesting View Mode for Hypergraph Visualization

**Date:** 2026-03-07  
**Status:** Ready for Implementation  
**Interview:** [20260307_INTERVIEW_NESTING_VIEW.md](../interviews/20260307_INTERVIEW_NESTING_VIEW.md)  
**Estimated Complexity:** High (>100 lines, >10 files)

## Objective

Implement a hierarchical nesting view for the hypergraph visualization that:
1. Shows selected node expanded with children inside (row layout)
2. Shows parent nodes as concentric layered shells around the selected node
3. Supports duplicate mode (default) where child nodes appear both in their original position AND inside the expanded parent

## Interview Summary

| Feature | Decision |
|---------|----------|
| Expansion style | Row layout (horizontal row below label) |
| Parent positioning | Layered shells (concentric, deeper = larger) |
| Parent visual | Dimmed + larger (semi-transparent, scaled up) |
| Duplicate appearance | Identical with badge + special edge endpoint indicator |
| Click duplicate | Navigate to original node |
| Original when duplicated | Stays visible but dimmed |
| Edges | Only to originals; internal edges hidden → node highlights |
| Depth | User-configurable for both parents and children |
| Navigation | Click parent label → smooth transition |
| Default | Remember last (fallback: nesting + duplicate mode) |
| Toggle location | ControlsHUD |
| Performance | Smart culling (hide off-screen duplicates) |

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    HypergraphViewCore                        │
│  ┌─────────────────┐  ┌──────────────────┐                  │
│  │ useNestingState │  │ useNestingLayout │                  │
│  │ (settings +     │  │ (shell positions,│                  │
│  │  localStorage)  │  │  child positions)│                  │
│  └────────┬────────┘  └────────┬─────────┘                  │
│           │                    │                             │
│           ▼                    ▼                             │
│  ┌────────────────────────────────────────┐                 │
│  │           NestingManager               │                 │
│  │  - Duplicate node creation/tracking    │                 │
│  │  - Shell layout computation            │                 │
│  │  - Edge-to-highlight conversion        │                 │
│  └────────────────────────────────────────┘                 │
│           │                                                  │
│           ▼                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  NodeLayer   │  │ edgeBuilder  │  │nodePositioner│       │
│  │ (duplicates) │  │ (originals)  │  │ (shells)     │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

## Detailed Design

### 1. Nesting State & Settings

**New types** in `types.ts`:
```typescript
interface NestingSettings {
    enabled: boolean;              // Master toggle
    duplicateMode: boolean;        // true = show duplicates, false = reparent/move
    parentDepth: number;           // How many parent shell levels (1-5, default 2)
    childDepth: number;            // How many child levels to show (1-3, default 1)
}

interface NestingState {
    settings: NestingSettings;
    selectedIdx: number;           // Current center node
    expandedShells: ShellNode[];   // Parent nodes arranged in shells
    duplicates: DuplicateNode[];   // Child duplicates inside expanded parent
}

interface ShellNode {
    nodeIdx: number;
    shellLevel: number;            // 1 = direct parent, 2 = grandparent, etc.
    angle: number;                 // Position on shell arc (radians)
    scale: number;                 // Visual scale (larger for deeper shells)
}

interface DuplicateNode {
    originalIdx: number;
    duplicateId: string;           // Unique ID for DOM key
    parentIdx: number;             // Which expanded parent contains this
    slotIndex: number;             // Position in row layout
}
```

### 2. Shell Layout Algorithm

Parents arranged in concentric shells around selected node:
- **Shell 1** (direct parents): Small arc above, scale 1.2x
- **Shell 2** (grandparents): Larger arc, scale 1.5x  
- **Shell N**: Progressively larger arcs, max scale ~2.5x

```typescript
function computeShellLayout(
    layout: GraphLayout,
    centerIdx: number,
    parentDepth: number
): ShellNode[] {
    const shells: ShellNode[] = [];
    const visited = new Set<number>([centerIdx]);
    
    let currentLevel = [centerIdx];
    for (let level = 1; level <= parentDepth; level++) {
        const nextLevel: number[] = [];
        for (const idx of currentLevel) {
            const node = layout.nodeMap.get(idx);
            if (!node) continue;
            for (const parentIdx of node.parentIndices) {
                if (visited.has(parentIdx)) continue;
                visited.add(parentIdx);
                nextLevel.push(parentIdx);
            }
        }
        
        // Distribute parents on arc
        const arcSpan = Math.PI * 0.6;  // 108 degrees
        const startAngle = Math.PI / 2 - arcSpan / 2;
        nextLevel.forEach((parentIdx, i) => {
            const t = nextLevel.length > 1 ? i / (nextLevel.length - 1) : 0.5;
            shells.push({
                nodeIdx: parentIdx,
                shellLevel: level,
                angle: startAngle + t * arcSpan,
                scale: 1 + level * 0.4,  // 1.4, 1.8, 2.2, ...
            });
        });
        
        currentLevel = nextLevel;
    }
    return shells;
}
```

### 3. Duplicate Node Rendering

In `NodeLayer.tsx`, render duplicates as additional DOM elements:

```typescript
interface NodeLayerProps {
    nodes: LayoutNode[];
    maxWidth: number;
    vizState: VisualizationState;
    duplicates?: DuplicateNode[];           // NEW
    duplicatePositions?: Map<string, Position>; // NEW
}

// Render regular nodes
{nodes.map(node => <NodeElement key={node.index} ... />)}

// Render duplicates with badge
{duplicates?.map(dup => (
    <NodeElement 
        key={dup.duplicateId}
        node={nodes.find(n => n.index === dup.originalIdx)!}
        isDuplicate={true}
        duplicateId={dup.duplicateId}
        ...
    />
))}
```

Duplicate badge CSS:
```css
.hg-node.hg-duplicate::after {
    content: '⤴';  /* or custom icon */
    position: absolute;
    top: -4px;
    right: -4px;
    font-size: 10px;
    background: var(--hg-accent);
    border-radius: 50%;
    padding: 2px;
}
```

### 4. Edge → Node Highlight Conversion

When a node is inside an expanded parent, its parent↔child edges become node highlights instead:

```typescript
interface EdgeHighlight {
    nodeIdx: number;
    side: 'parent' | 'child';  // Which end of the edge
    color: [number, number, number, number];
}

function convertEdgesToHighlights(
    layout: GraphLayout,
    expandedIdx: number,
    childIndices: number[]
): EdgeHighlight[] {
    const highlights: EdgeHighlight[] = [];
    // The parent node gets highlighted for each child relationship
    highlights.push({
        nodeIdx: expandedIdx,
        side: 'parent',
        color: [0.3, 0.8, 0.4, 0.6],  // Subtle green glow
    });
    // Each child inside gets highlighted
    for (const childIdx of childIndices) {
        highlights.push({
            nodeIdx: childIdx,
            side: 'child', 
            color: [0.3, 0.8, 0.4, 0.6],
        });
    }
    return highlights;
}
```

### 5. Smart Culling for Performance

Off-screen duplicates are hidden via CSS `display: none`:

```typescript
function cullDuplicates(
    duplicates: DuplicateNode[],
    positions: Map<string, Position>,
    viewport: DOMRect
): Set<string> {
    const MARGIN = 100;
    const visible = new Set<string>();
    for (const dup of duplicates) {
        const pos = positions.get(dup.duplicateId);
        if (!pos) continue;
        if (pos.x >= -MARGIN && pos.x <= viewport.width + MARGIN &&
            pos.y >= -MARGIN && pos.y <= viewport.height + MARGIN) {
            visible.add(dup.duplicateId);
        }
    }
    return visible;
}
```

## File Changes

### New Files
| File | Purpose |
|------|---------|
| `hooks/useNestingState.ts` | Settings state + localStorage persistence |
| `nesting/shellLayout.ts` | Shell position computation |
| `nesting/duplicateManager.ts` | Duplicate node creation/tracking |
| `nesting/edgeHighlights.ts` | Edge → node highlight conversion |

### Modified Files
| File | Changes |
|------|---------|
| `types.ts` | Add `NestingSettings`, `ShellNode`, `DuplicateNode` types |
| `hooks/index.ts` | Export `useNestingState` |
| `components/NodeLayer.tsx` | Render duplicates, duplicate badge styling |
| `components/ControlsHUD.tsx` | Add nesting toggles + depth sliders |
| `components/NodeElement.tsx` | Handle `isDuplicate` prop, badge rendering |
| `gpu/edgeBuilder.ts` | Skip edges to duplicates, generate highlight data |
| `animation/nodePositioner.ts` | Position shell nodes + duplicates |
| `hooks/useOverlayRenderer.ts` | Integrate nesting manager, pass settings |
| `HypergraphViewCore.tsx` | Wire up nesting state and layout |
| `hypergraph.css` | Shell node styles, duplicate badge, highlight glow |
| `decomposition/manager.ts` | Deprecate/remove (replaced by nesting system) |

## Execution Steps

### Phase 1: Foundation (Settings & Types)
- [ ] 1.1 Add types to `types.ts`
- [ ] 1.2 Create `hooks/useNestingState.ts` with localStorage
- [ ] 1.3 Export from `hooks/index.ts`
- [ ] 1.4 Add toggles to `ControlsHUD.tsx`

### Phase 2: Shell Layout
- [ ] 2.1 Create `nesting/shellLayout.ts`
- [ ] 2.2 Integrate shell computation in `useOverlayRenderer`
- [ ] 2.3 Update `nodePositioner.ts` to position shell nodes
- [ ] 2.4 Add shell node CSS (dimmed, scaled)

### Phase 3: Duplicate System
- [ ] 3.1 Create `nesting/duplicateManager.ts`
- [ ] 3.2 Add duplicate rendering to `NodeLayer.tsx`
- [ ] 3.3 Add duplicate badge styling
- [ ] 3.4 Original node dimming when duplicated

### Phase 4: Edge Handling
- [ ] 4.1 Create `nesting/edgeHighlights.ts`
- [ ] 4.2 Modify `edgeBuilder.ts` to skip internal edges
- [ ] 4.3 Apply node highlight glow styling

### Phase 5: Navigation & Transitions
- [ ] 5.1 Click handler for shell parent navigation
- [ ] 5.2 Click handler for duplicate → original navigation
- [ ] 5.3 Smooth animation transitions between selections

### Phase 6: Polish
- [ ] 6.1 Smart culling implementation
- [ ] 6.2 Depth slider controls
- [ ] 6.3 Clean up old decomposition code
- [ ] 6.4 Testing with various graph structures

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Performance with many duplicates | Smart culling, limit max duplicates |
| Complex state management | Clear ownership: NestingManager owns duplicates |
| Animation jank during transitions | Use requestAnimationFrame, batch DOM updates |
| Confusion about duplicate vs original | Clear badge, click always goes to original |

## Validation Criteria

- [ ] Toggle enables/disables nesting view
- [ ] Parent shells render at correct positions and scales
- [ ] Duplicates appear inside expanded parent with badge
- [ ] Original nodes dim when duplicated
- [ ] Internal edges hidden, node highlights visible
- [ ] Click duplicate navigates to original
- [ ] Click shell parent transitions view smoothly
- [ ] Depth sliders work correctly
- [ ] Settings persist across page reloads
- [ ] Performance acceptable with 100+ nodes
