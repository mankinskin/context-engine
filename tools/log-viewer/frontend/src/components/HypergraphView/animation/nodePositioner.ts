/**
 * DOM node CSS-transform positioning — projects 3D node positions to
 * screen-space and updates DOM element styles.
 *
 * Also handles back-projection of decomposition-reparented children so
 * that edges track their on-screen positions correctly.
 */
import type { GraphLayout, LayoutNode } from '../layout';
import type { InteractionState } from '../hooks/useMouseInteraction';
import { worldToScreen, worldScaleAtDepth } from '../utils/math';
import type { DecompositionManager } from '../decomposition/manager';

import {
    markOverlayScanDirty,
} from '../../WgpuOverlay/WgpuOverlay';

// ── Types ──

export interface PositionContext {
    layout: GraphLayout;
    nodeElMap: Map<number, HTMLDivElement>;
    viewProj: Float32Array;
    invSubVP: Float32Array | null;
    camPos: [number, number, number];
    vw: number;
    vh: number;
    containerRect: DOMRect;
    inter: InteractionState;
    vizInvolvedNodes: Set<number>;
    connectedSet: Set<number>;
    decomposition: DecompositionManager;
}

/**
 * Position all DOM node elements via CSS transforms.
 *
 * Decomposition-reparented children are skipped for 3D transforms but get
 * their world coords back-projected from screen position so edges still
 * connect to the right place.
 */
export function positionDOMNodes(ctx: PositionContext): void {
    const {
        layout, nodeElMap, viewProj, invSubVP, camPos, vw, vh,
        containerRect, inter, vizInvolvedNodes, connectedSet, decomposition,
    } = ctx;

    const reparentedSet = decomposition.getReparentedSet();
    const childParentMap = decomposition.getChildParentMap();
    const expandedNodes = decomposition.getExpandedNodes();
    const vp = viewProj;

    for (let i = 0; i < layout.nodes.length; i++) {
        const n = layout.nodes[i]!;
        const el = nodeElMap.get(n.index);
        if (!el) continue;

        if (reparentedSet.has(n.index)) {
            // Child is inside decomposition row — no 3D transforms.
            el.style.display = '';
            el.style.opacity = '1';
            el.style.transform = '';
            el.style.zIndex = '';

            // Back-project DOM screen position to world coords
            backProjectReparentedChild(n, el, containerRect, vw, vh, invSubVP, vp, layout, childParentMap);
            continue;
        }

        const screen = worldToScreen([n.x, n.y, n.z], viewProj, vw, vh);
        const scale = worldScaleAtDepth(camPos, [n.x, n.y, n.z], vh);
        const pixelScale = Math.max(0.1, (scale * n.radius * 2.5) / 80);

        if (!screen.visible || pixelScale < 0.02) {
            el.style.display = 'none';
            continue;
        }
        el.style.display = '';

        // Dim nodes not connected to selected node (but never dim viz-involved nodes)
        const dimmed = inter.selectedIdx >= 0
            && !connectedSet.has(n.index)
            && !vizInvolvedNodes.has(n.index);
        el.style.opacity = dimmed ? '0.15' : '1';

        // Imperative class toggling for selected/hover
        el.classList.toggle('selected', n.index === inter.selectedIdx);
        el.classList.toggle('span-highlighted', n.index === inter.hoverIdx);

        const zIdx = Math.round((1 - screen.z) * 1000);
        const isExpanded = expandedNodes.has(n.index);
        el.style.zIndex = (n.index === inter.selectedIdx) ? '10000'
            : isExpanded ? '9999'
                : String(zIdx);

        // Expanded parent: anchor at top-center
        if (isExpanded) {
            el.style.transform = `translate(-50%, 0%) translate(${screen.x.toFixed(1)}px, ${screen.y.toFixed(1)}px) scale(${pixelScale.toFixed(3)})`;
        } else {
            el.style.transform = `translate(-50%, -50%) translate(${screen.x.toFixed(1)}px, ${screen.y.toFixed(1)}px) scale(${pixelScale.toFixed(3)})`;
        }

        el.setAttribute('data-depth', screen.z.toFixed(4));
    }

    markOverlayScanDirty();
}

// ── Internal helper ──

function backProjectReparentedChild(
    n: LayoutNode,
    el: HTMLDivElement,
    containerRect: DOMRect,
    vw: number,
    vh: number,
    invSubVP: Float32Array | null,
    vp: Float32Array,
    layout: GraphLayout,
    childParentMap: Map<number, number>,
): void {
    if (!invSubVP) return;

    const childRect = el.getBoundingClientRect();
    const csx = (childRect.left + childRect.width / 2) - containerRect.left;
    const csy = (childRect.top + childRect.height / 2) - containerRect.top;

    // Use expanded parent's depth as reference
    const parentIdx = childParentMap.get(n.index)!;
    const pn = layout.nodeMap.get(parentIdx);
    const pz = pn ? pn.z : n.z;
    const pcz = vp[2]! * (pn?.x ?? 0) + vp[6]! * (pn?.y ?? 0) + vp[10]! * pz + vp[14]!;
    const pcw = vp[3]! * (pn?.x ?? 0) + vp[7]! * (pn?.y ?? 0) + vp[11]! * pz + vp[15]!;
    const pndcZ = pcw > 0.001 ? pcz / pcw : 0;

    const ndcX = (csx / vw) * 2 - 1;
    const ndcY = 1 - (csy / vh) * 2;
    const inv = invSubVP;
    const ux = inv[0]! * ndcX + inv[4]! * ndcY + inv[8]! * pndcZ + inv[12]!;
    const uy = inv[1]! * ndcX + inv[5]! * ndcY + inv[9]! * pndcZ + inv[13]!;
    const uz = inv[2]! * ndcX + inv[6]! * ndcY + inv[10]! * pndcZ + inv[14]!;
    const uw = inv[3]! * ndcX + inv[7]! * ndcY + inv[11]! * pndcZ + inv[15]!;
    if (Math.abs(uw) > 0.001) {
        n.x = ux / uw;
        n.y = uy / uw;
        n.z = uz / uw;
        // Snap targets so lerp doesn't pull them back
        n.tx = n.x;
        n.ty = n.y;
        n.tz = n.z;
    }
}
