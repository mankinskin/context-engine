/**
 * DOM node CSS-transform positioning — projects 3D node positions to
 * screen-space and updates DOM element styles.
 *
 * Also handles back-projection of decomposition-reparented children so
 * that edges track their on-screen positions correctly.
 *
 * Nesting view adds shell containers and duplicate node positioning.
 */
import type { GraphLayout, LayoutNode } from '../layout';
import type { InteractionState } from '../hooks/useMouseInteraction';
import { worldToScreen, worldScaleAtDepth } from '../utils/math';
import type { DecompositionManager } from '../decomposition/manager';
import type { ShellNode, DuplicateNode, EdgeHighlight } from '../types';

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
    /** Nesting shell nodes to position. */
    shells?: ShellNode[];
    /** Duplicate nodes to position. */
    duplicates?: DuplicateNode[];
    /** Nesting edge highlights (for glow styling). */
    nestingHighlights?: EdgeHighlight[];
    /** Container element for SVG connector overlay. */
    containerEl?: HTMLElement;
    /** Whether nesting is enabled (always uses clones). */
    nestingEnabled?: boolean;
    /** Whether duplicate mode is on (show originals at 3D positions). */
    duplicateMode?: boolean;
}

/**
 * Position all DOM node elements via CSS transforms.
 *
 * Decomposition-reparented children are skipped for 3D transforms but get
 * their world coords back-projected from screen position so edges still
 * connect to the right place.
 *
 * Nodes whose screen position falls outside a margin around the viewport
 * are culled (display: none) to save layout/paint cost.
 */
export function positionDOMNodes(ctx: PositionContext): void {
    const {
        layout, nodeElMap, viewProj, invSubVP, camPos, vw, vh,
        containerRect, inter, vizInvolvedNodes, connectedSet, decomposition,
        shells, duplicates, nestingHighlights, containerEl, nestingEnabled, duplicateMode,
    } = ctx;

    const clonedChildSet = decomposition.getClonedChildIndices();
    const childParentMap = decomposition.getChildParentMap();
    const expandedNodes = decomposition.getExpandedNodes();
    const vp = viewProj;

    // Frustum culling margin (pixels outside viewport before culling)
    const CULL_MARGIN = 200;

    // Collect screen positions for SVG connector drawing
    const nodeScreenPos = new Map<number, { x: number; y: number }>();

    for (let i = 0; i < layout.nodes.length; i++) {
        const n = layout.nodes[i]!;
        const el = nodeElMap.get(n.index);
        if (!el) continue;

        if (clonedChildSet.has(n.index) && !duplicateMode) {
            // Dup=off: child is shown inside parent clone — hide the original.
            el.style.display = 'none';
            continue;
        }

        const screen = worldToScreen([n.x, n.y, n.z], viewProj, vw, vh);
        nodeScreenPos.set(n.index, { x: screen.x, y: screen.y });
        const scale = worldScaleAtDepth(camPos, [n.x, n.y, n.z], vh);
        const pixelScale = Math.max(0.1, (scale * n.radius * 2.5) / 80);

        // Frustum culling: hide nodes well outside the viewport
        if (!screen.visible || pixelScale < 0.02
            || screen.x < -CULL_MARGIN || screen.x > vw + CULL_MARGIN
            || screen.y < -CULL_MARGIN || screen.y > vh + CULL_MARGIN) {
            el.style.display = 'none';
            continue;
        }
        el.style.display = '';

        const isExpanded = expandedNodes.has(n.index);

        // Dim nodes not connected to selected node (but never dim viz-involved
        // or expanded nodes — expanded parents must always look active).
        const dimmed = inter.selectedIdx >= 0
            && !isExpanded
            && !connectedSet.has(n.index)
            && !vizInvolvedNodes.has(n.index);
        el.style.opacity = dimmed ? '0.15' : '1';

        // Imperative class toggling for selected/hover
        // Expanded parents always appear selected so they stay visually active
        // (e.g. the search-path root during visit_child transitions).
        el.classList.toggle('selected', n.index === inter.selectedIdx || isExpanded);
        el.classList.toggle('span-highlighted', n.index === inter.hoverIdx);

        const zIdx = Math.round((1 - screen.z) * 1000);
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

    // ── Nesting: position shell containers ──
    if (shells && shells.length > 0 && inter.selectedIdx >= 0) {
        const selNode = layout.nodeMap.get(inter.selectedIdx);
        if (selNode) {
            const selScreen = worldToScreen([selNode.x, selNode.y, selNode.z], viewProj, vw, vh);
            const selScale = worldScaleAtDepth(camPos, [selNode.x, selNode.y, selNode.z], vh);

            for (const shell of shells) {
                const shellEl = containerRef(nodeElMap, `shell-${shell.nodeIdx}`);
                if (!shellEl) continue;

                const pixelWidth = shell.width * selScale * 0.015;
                const pixelHeight = shell.height * selScale * 0.015;
                const cx = selScreen.x + shell.centerX * selScale * 0.015;
                const cy = selScreen.y + shell.centerY * selScale * 0.015;

                shellEl.style.display = '';
                shellEl.style.width = `${pixelWidth.toFixed(1)}px`;
                shellEl.style.height = `${pixelHeight.toFixed(1)}px`;
                shellEl.style.transform = `translate(-50%, -50%) translate(${cx.toFixed(1)}px, ${cy.toFixed(1)}px)`;
                shellEl.style.zIndex = String(Math.max(0, 9000 - shell.shellLevel * 10));
            }
        }
    }

    // ── Nesting: position duplicate nodes ──
    if (duplicates && duplicates.length > 0 && inter.selectedIdx >= 0) {
        const selNode = layout.nodeMap.get(inter.selectedIdx);
        if (selNode) {
            const selScreen = worldToScreen([selNode.x, selNode.y, selNode.z], viewProj, vw, vh);
            const selScale = worldScaleAtDepth(camPos, [selNode.x, selNode.y, selNode.z], vh);
            const pixelScale = Math.max(0.1, (selScale * selNode.radius * 2.5) / 80);

            for (const dup of duplicates) {
                const dupEl = containerRef(nodeElMap, dup.duplicateId);
                if (!dupEl) continue;

                // Position duplicates in a row below the selected node's center
                const spacing = 90 * pixelScale;
                const totalWidth = duplicates.length * spacing;
                const startX = selScreen.x - totalWidth / 2 + spacing / 2;
                const dx = startX + dup.slotIndex * spacing;
                const dy = selScreen.y + 30 * pixelScale;

                dupEl.style.display = '';
                dupEl.style.opacity = '1';
                dupEl.style.transform = `translate(-50%, -50%) translate(${dx.toFixed(1)}px, ${dy.toFixed(1)}px) scale(${(pixelScale * 0.85).toFixed(3)})`;
                dupEl.style.zIndex = '9500';
            }
        }
    }

    // ── Nesting: clean up stale highlight classes from all nodes ──
    // This runs every frame to ensure highlight classes from a previous
    // nesting configuration don't linger when nesting is toggled off.
    for (const [, el] of nodeElMap) {
        el.classList.remove('hg-nesting-highlight', 'hg-nesting-highlight-parent', 'hg-nesting-highlight-child');
    }

    // ── Nesting: apply highlight glow to nodes involved in hidden edges ──
    if (nestingHighlights && nestingHighlights.length > 0) {
        for (const hl of nestingHighlights) {
            const el = nodeElMap.get(hl.nodeIdx);
            if (el) {
                el.classList.add('hg-nesting-highlight');
                el.classList.toggle('hg-nesting-highlight-parent', hl.role === 'parent');
                el.classList.toggle('hg-nesting-highlight-child', hl.role === 'child');
            }
        }
    }

    // ── Nesting: update clone element classes (selection / hover / viz) ──
    if (nestingEnabled && containerEl) {
        const cloneEls = containerEl.querySelectorAll<HTMLDivElement>('.hg-decomp-child[data-clone]');
        for (const cloneEl of cloneEls) {
            const origIdx = Number(cloneEl.getAttribute('data-node-idx'));
            if (isNaN(origIdx)) continue;
            cloneEl.classList.toggle('selected', origIdx === inter.selectedIdx);
            cloneEl.classList.toggle('span-highlighted', origIdx === inter.hoverIdx);

            // Sync viz-* classes from the original node element so that
            // clone highlights stay in sync with the visualization state.
            const origEl = nodeElMap.get(origIdx);
            if (origEl) {
                // Remove stale viz classes from clone
                for (const cls of Array.from(cloneEl.classList)) {
                    if (cls.startsWith('viz-') && !origEl.classList.contains(cls)) {
                        cloneEl.classList.remove(cls);
                    }
                }
                // Add current viz classes from original
                for (const cls of Array.from(origEl.classList)) {
                    if (cls.startsWith('viz-') && !cloneEl.classList.contains(cls)) {
                        cloneEl.classList.add(cls);
                    }
                }
            }
        }
    }

    // ── Nesting: SVG connector edges from originals to clones ──
    if (nestingEnabled && duplicateMode && containerEl) {
        updateNestingConnectors(containerEl, containerRect, nodeScreenPos, nodeElMap, inter.selectedIdx);
    } else if (containerEl) {
        const svg = containerEl.querySelector<SVGSVGElement>(':scope > .hg-nesting-connectors');
        if (svg) svg.style.display = 'none';
    }

    markOverlayScanDirty();
}

/**
 * Look up a DOM element by data-shell-idx or data-duplicate-id from the
 * node layer. Falls back to querySelector if not in the nodeElMap.
 */
function containerRef(_nodeElMap: Map<number, HTMLDivElement>, key: string): HTMLDivElement | null {
    // Shell elements use data-shell-idx, duplicates use data-duplicate-id
    const el = document.querySelector<HTMLDivElement>(`[data-shell-idx="${key.replace('shell-', '')}"], [data-duplicate-id="${key}"]`);
    return el;
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

// ── Nesting connector helpers ──

/**
 * Intersection of a ray from (cx,cy) toward (tx,ty) with an axis-aligned rect.
 */
function rectBorderPoint(
    cx: number, cy: number, hw: number, hh: number,
    tx: number, ty: number,
): [number, number] {
    const dx = tx - cx;
    const dy = ty - cy;
    if (dx === 0 && dy === 0) return [cx, cy];
    const sx = Math.abs(dx) > 0 ? hw / Math.abs(dx) : Infinity;
    const sy = Math.abs(dy) > 0 ? hh / Math.abs(dy) : Infinity;
    const s = Math.min(sx, sy);
    return [cx + dx * s, cy + dy * s];
}

function getOrCreateConnectorSvg(container: HTMLElement): SVGSVGElement {
    let svg = container.querySelector<SVGSVGElement>(':scope > .hg-nesting-connectors');
    if (!svg) {
        svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
        svg.setAttribute('class', 'hg-nesting-connectors');
        container.appendChild(svg);
    }
    return svg;
}

/**
 * Draw SVG connector lines from original nodes to their clones inside
 * the expanded parent's decomposition patterns.
 */
function updateNestingConnectors(
    containerEl: HTMLElement,
    containerRect: DOMRect,
    nodeScreenPos: Map<number, { x: number; y: number }>,
    nodeElMap: Map<number, HTMLDivElement>,
    selectedIdx: number,
): void {
    const cloneEls = containerEl.querySelectorAll<HTMLDivElement>('.hg-decomp-child[data-clone]');
    if (cloneEls.length === 0) {
        const svg = containerEl.querySelector<SVGSVGElement>(':scope > .hg-nesting-connectors');
        if (svg) svg.style.display = 'none';
        return;
    }

    const svg = getOrCreateConnectorSvg(containerEl);
    svg.style.display = '';
    // Clear previous frame
    while (svg.firstChild) svg.removeChild(svg.firstChild);

    for (const cloneEl of cloneEls) {
        const origIdx = Number(cloneEl.getAttribute('data-node-idx'));
        if (isNaN(origIdx)) continue;

        const origPos = nodeScreenPos.get(origIdx);
        if (!origPos) continue;

        const origEl = nodeElMap.get(origIdx);
        if (!origEl || origEl.style.display === 'none') continue;

        const cloneRect = cloneEl.getBoundingClientRect();
        if (cloneRect.width === 0) continue;

        const cloneCx = cloneRect.left + cloneRect.width / 2 - containerRect.left;
        const cloneCy = cloneRect.top + cloneRect.height / 2 - containerRect.top;

        // Compute border intersection points
        const origRect = origEl.getBoundingClientRect();
        const [origBx, origBy] = rectBorderPoint(
            origPos.x, origPos.y,
            origRect.width / 2, origRect.height / 2,
            cloneCx, cloneCy,
        );
        const [cloneBx, cloneBy] = rectBorderPoint(
            cloneCx, cloneCy,
            cloneRect.width / 2, cloneRect.height / 2,
            origPos.x, origPos.y,
        );

        const isHighlighted = origIdx === selectedIdx;
        const hlClass = isHighlighted ? ' hg-connector-highlighted' : '';

        // Line
        const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
        line.setAttribute('x1', origBx.toFixed(1));
        line.setAttribute('y1', origBy.toFixed(1));
        line.setAttribute('x2', cloneBx.toFixed(1));
        line.setAttribute('y2', cloneBy.toFixed(1));
        line.setAttribute('class', 'hg-connector-line' + hlClass);
        svg.appendChild(line);

        // Circle at original border
        const c1 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
        c1.setAttribute('cx', origBx.toFixed(1));
        c1.setAttribute('cy', origBy.toFixed(1));
        c1.setAttribute('r', isHighlighted ? '4.5' : '3.5');
        c1.setAttribute('class', 'hg-connector-dot' + hlClass);
        svg.appendChild(c1);

        // Circle at clone border
        const c2 = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
        c2.setAttribute('cx', cloneBx.toFixed(1));
        c2.setAttribute('cy', cloneBy.toFixed(1));
        c2.setAttribute('r', isHighlighted ? '4.5' : '3.5');
        c2.setAttribute('class', 'hg-connector-dot' + hlClass);
        svg.appendChild(c2);
    }
}
