/**
 * Force-directed 3D layout engine for hypergraph nodes.
 *
 * Atoms (leaf nodes) are placed at the bottom level (y=0).
 * Compound nodes are placed at y proportional to their width.
 * XZ positions computed via spring-electrical force simulation.
 */

import type { HypergraphSnapshot, HypergraphNode, HypergraphEdge } from '../../types';

export interface LayoutNode {
    index: number;
    label: string;
    width: number;
    isAtom: boolean;
    x: number;
    y: number;
    z: number;
    /** Animation target X — lerp toward this each frame */
    tx: number;
    /** Animation target Y */
    ty: number;
    /** Animation target Z */
    tz: number;
    vx: number;
    vy: number;
    vz: number;
    radius: number;
    color: [number, number, number, number];
    parentIndices: number[];
    childIndices: number[];
}

export interface LayoutEdge {
    from: number;
    to: number;
    patternIdx: number;
    subIndex: number;
}

export interface GraphLayout {
    nodes: LayoutNode[];
    nodeMap: Map<number, LayoutNode>;
    edges: LayoutEdge[];
    maxWidth: number;
}

// ── Color palette (hue-based on node width) ──

function widthColor(width: number, maxWidth: number): [number, number, number, number] {
    if (width === 1) return [0.55, 0.75, 0.95, 1]; // atoms: soft blue
    const t = Math.min((width - 1) / Math.max(maxWidth - 1, 1), 1);
    // gradient from green → orange → red-ish as width grows
    const r = 0.3 + t * 0.6;
    const g = 0.8 - t * 0.4;
    const b = 0.3 + (1 - t) * 0.3;
    return [r, g, b, 1];
}

export function buildLayout(snapshot: HypergraphSnapshot): GraphLayout {
    const maxWidth = Math.max(...snapshot.nodes.map(n => n.width), 1);

    // Build adjacency
    const childMap = new Map<number, Set<number>>();
    const parentMap = new Map<number, Set<number>>();
    for (const e of snapshot.edges) {
        if (!childMap.has(e.from)) childMap.set(e.from, new Set());
        childMap.get(e.from)!.add(e.to);
        if (!parentMap.has(e.to)) parentMap.set(e.to, new Set());
        parentMap.get(e.to)!.add(e.from);
    }

    // Initial positions: circular in XZ, Y by width
    const nodes: LayoutNode[] = snapshot.nodes.map((n, i) => {
        const angle = (i / snapshot.nodes.length) * Math.PI * 2;
        const r = 1 + snapshot.nodes.length * 0.15;
        return {
            index: n.index,
            label: n.label,
            width: n.width,
            isAtom: n.width === 1,
            x: Math.cos(angle) * r * (0.5 + Math.random() * 0.5),
            y: (n.width - 1) * 0.8,
            z: Math.sin(angle) * r * (0.5 + Math.random() * 0.5),
            tx: 0, ty: 0, tz: 0, // set after simulate
            vx: 0, vy: 0, vz: 0,
            radius: 0.15 + Math.min(n.width * 0.06, 0.3),
            color: widthColor(n.width, maxWidth),
            parentIndices: [...(parentMap.get(n.index) || [])],
            childIndices: [...(childMap.get(n.index) || [])],
        };
    });

    const nodeMap = new Map<number, LayoutNode>();
    for (const n of nodes) nodeMap.set(n.index, n);

    // Deduplicate edges
    const edgeSet = new Set<string>();
    const edges: LayoutEdge[] = [];
    for (const e of snapshot.edges) {
        const key = `${e.from}-${e.to}-${e.pattern_idx}`;
        if (!edgeSet.has(key)) {
            edgeSet.add(key);
            edges.push({ from: e.from, to: e.to, patternIdx: e.pattern_idx, subIndex: e.sub_index });
        }
    }

    // Run force simulation
    simulate(nodes, edges, nodeMap, 150);

    // Center the layout
    if (nodes.length > 0) {
        let cx = 0, cz = 0;
        for (const n of nodes) { cx += n.x; cz += n.z; }
        cx /= nodes.length; cz /= nodes.length;
        for (const n of nodes) { n.x -= cx; n.z -= cz; }
    }

    // Sync animation targets to initial positions
    for (const n of nodes) { n.tx = n.x; n.ty = n.y; n.tz = n.z; }

    return { nodes, nodeMap, edges, maxWidth };
}

/**
 * Compute focused-layout positions for a selected node.
 * - Parents arranged above the selected node, grouped by width (smallest closest)
 * - Children arranged below, each pattern_id on its own horizontal line
 * Returns a map of node index → new {x, y, z} position.
 */
export function computeFocusedLayout(
    layout: GraphLayout,
    selectedIdx: number,
): Map<number, { x: number; y: number; z: number }> | null {
    const selected = layout.nodeMap.get(selectedIdx);
    if (!selected) return null;

    const SPACING_X = 1.8;
    const SPACING_Y = 1.5;
    const positions = new Map<number, { x: number; y: number; z: number }>();

    // Selected node stays at its current target position
    positions.set(selectedIdx, { x: selected.tx, y: selected.ty, z: selected.tz });

    // ── Parents above ──
    const parents = selected.parentIndices
        .map(idx => layout.nodeMap.get(idx))
        .filter((n): n is LayoutNode => n != null);

    // Group parents by width, sort ascending (smaller width = closer to selected)
    const parentsByWidth = new Map<number, LayoutNode[]>();
    for (const p of parents) {
        if (!parentsByWidth.has(p.width)) parentsByWidth.set(p.width, []);
        parentsByWidth.get(p.width)!.push(p);
    }

    const sortedWidths = [...parentsByWidth.keys()].sort((a, b) => a - b);
    for (let rowIdx = 0; rowIdx < sortedWidths.length; rowIdx++) {
        const group = parentsByWidth.get(sortedWidths[rowIdx]!)!;
        const y = selected.ty + (rowIdx + 1) * SPACING_Y;
        const totalW = (group.length - 1) * SPACING_X;
        const startX = selected.tx - totalW / 2;
        for (let i = 0; i < group.length; i++) {
            positions.set(group[i]!.index, {
                x: startX + i * SPACING_X,
                y,
                z: selected.tz,
            });
        }
    }

    // ── Children below, grouped by patternIdx ──
    // Each pattern is one valid decomposition of the selected node's string
    // into smaller substrings. Show each decomposition as a tight horizontal
    // row, ordered by sub_index (left-to-right reading order).
    const childEdgesByPattern = new Map<number, { to: number; subIndex: number }[]>();
    for (const e of layout.edges) {
        if (e.from === selectedIdx) {
            if (!childEdgesByPattern.has(e.patternIdx)) {
                childEdgesByPattern.set(e.patternIdx, []);
            }
            childEdgesByPattern.get(e.patternIdx)!.push({ to: e.to, subIndex: e.subIndex });
        }
    }

    const sortedPatterns = [...childEdgesByPattern.entries()].sort((a, b) => a[0] - b[0]);

    // First pass: measure the widest pattern row so we can center-align all rows
    let maxRowNodes = 0;
    for (const [, children] of sortedPatterns) {
        if (children.length > maxRowNodes) maxRowNodes = children.length;
    }

    // Place each pattern as a complete horizontal row.
    // A child node that appears in multiple patterns gets positioned at its
    // lowest (most recently placed) row — this is intentional: the last
    // pattern wins, and earlier patterns may have "gaps" where that node
    // animates down.  In practice shared children are uncommon.
    for (let pi = 0; pi < sortedPatterns.length; pi++) {
        const [, children] = sortedPatterns[pi]!;
        children.sort((a, b) => a.subIndex - b.subIndex);
        const y = selected.ty - (pi + 1) * SPACING_Y;
        const totalW = (children.length - 1) * SPACING_X;
        const startX = selected.tx - totalW / 2;
        for (let ci = 0; ci < children.length; ci++) {
            const childIdx = children[ci]!.to;
            positions.set(childIdx, {
                x: startX + ci * SPACING_X,
                y,
                z: selected.tz,
            });
        }
    }

    return positions;
}

function simulate(
    nodes: LayoutNode[],
    edges: LayoutEdge[],
    nodeMap: Map<number, LayoutNode>,
    iterations: number,
) {
    const repulsion = 1.5;
    const springK = 0.15;
    const springLen = 0.9;
    const damping = 0.85;
    const ySpringK = 0.1;
    const dt = 0.4;

    for (let iter = 0; iter < iterations; iter++) {
        const temp = 1.0 - iter / iterations;

        // Repulsion (all pairs)
        for (let i = 0; i < nodes.length; i++) {
            for (let j = i + 1; j < nodes.length; j++) {
                const a = nodes[i]!;
                const b = nodes[j]!;
                let dx = a.x - b.x;
                let dz = a.z - b.z;
                let dist = Math.sqrt(dx * dx + dz * dz);
                if (dist < 0.01) { dx = Math.random() - 0.5; dz = Math.random() - 0.5; dist = 0.5; }
                const force = repulsion / (dist * dist) * temp;
                const fx = (dx / dist) * force;
                const fz = (dz / dist) * force;
                a.vx += fx; a.vz += fz;
                b.vx -= fx; b.vz -= fz;
            }
        }

        // Spring attraction (edges)
        for (const e of edges) {
            const a = nodeMap.get(e.from);
            const b = nodeMap.get(e.to);
            if (!a || !b) continue;
            let dx = b.x - a.x;
            let dz = b.z - a.z;
            let dist = Math.sqrt(dx * dx + dz * dz);
            if (dist < 0.01) dist = 0.01;
            const force = springK * (dist - springLen) * temp;
            const fx = (dx / dist) * force;
            const fz = (dz / dist) * force;
            a.vx += fx; a.vz += fz;
            b.vx -= fx; b.vz -= fz;
        }

        // Y-axis spring to target level
        for (const n of nodes) {
            const targetY = (n.width - 1) * 0.8;
            n.vy += (targetY - n.y) * ySpringK;
        }

        // Integrate
        for (const n of nodes) {
            n.vx *= damping; n.vy *= damping; n.vz *= damping;
            n.x += n.vx * dt;
            n.y += n.vy * dt;
            n.z += n.vz * dt;
        }
    }

    // Zero out velocities
    for (const n of nodes) { n.vx = 0; n.vy = 0; n.vz = 0; }
}
