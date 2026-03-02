/**
 * Edge highlighting computation for search path visualization.
 *
 * Given a VizPathGraph and snapshot edges, computes Sets of edge pair keys
 * for start-path, root, and end-path highlighting.  These pair keys are
 * pattern_idx-independent and match the layout edges used by the overlay
 * renderer.
 *
 * Extracted from useVisualizationState so the logic is independently
 * testable without React/Preact hooks.
 */

import type { SnapshotEdge, VizPathGraph, Transition } from '../types/generated';

// ---------------------------------------------------------------------------
// Edge pair key (duplicated from HypergraphView/utils/math to avoid
// circular dependency on the component tree — identical implementation).
// ---------------------------------------------------------------------------

/** Encode two node indices into a single numeric key (supports up to 65535 nodes). */
export function edgePairKey(from: number, to: number): number {
    return (from << 16) | to;
}

// ---------------------------------------------------------------------------
// Graph topology helpers
// ---------------------------------------------------------------------------

/** Build a child-adjacency map from snapshot edges. */
export function buildChildMap(edges: SnapshotEdge[]): Map<number, number[]> {
    const childMap = new Map<number, number[]>();
    for (const e of edges) {
        let children = childMap.get(e.from);
        if (!children) {
            children = [];
            childMap.set(e.from, children);
        }
        if (!children.includes(e.to)) {
            children.push(e.to);
        }
    }
    return childMap;
}

/**
 * BFS from ancestor to descendant through parent→child edges.
 * Returns the path as [ancestor, ..., descendant] or null if unreachable.
 */
export function findDescendantPath(
    ancestor: number,
    descendant: number,
    childMap: Map<number, number[]>,
): number[] | null {
    if (ancestor === descendant) return [ancestor];

    const visited = new Set<number>();
    const parentOf = new Map<number, number>();
    const queue = [ancestor];
    visited.add(ancestor);

    while (queue.length > 0) {
        const node = queue.shift()!;
        const children = childMap.get(node);
        if (!children) continue;

        for (const child of children) {
            if (visited.has(child)) continue;
            visited.add(child);
            parentOf.set(child, node);

            if (child === descendant) {
                // Reconstruct path
                const path: number[] = [];
                let curr = child;
                while (curr !== ancestor) {
                    path.unshift(curr);
                    curr = parentOf.get(curr)!;
                }
                path.unshift(ancestor);
                return path;
            }

            queue.push(child);
        }
    }

    return null; // unreachable
}

/**
 * Determine the end-path target node from the current transition.
 * For visit_child, child_match, child_mismatch the target is clear.
 * Returns null for transitions that don't involve child exploration,
 * signalling that end-path edges should use VizPathGraph fallback.
 */
export function endPathTargetFromTransition(trans: Transition | null): number | null {
    if (!trans) return null;
    switch (trans.kind) {
        case 'visit_child':    return trans.to;
        case 'child_match':    return trans.node;
        case 'child_mismatch': return trans.node;
        default:               return null;
    }
}

/**
 * Whether the current transition is part of child-exploration.
 * When false, end-path edges should only appear if the VizPathGraph
 * still has end_path entries (persistent state from prior steps).
 */
function isChildExplorationTransition(trans: Transition | null): boolean {
    if (!trans) return false;
    switch (trans.kind) {
        case 'visit_child':
        case 'child_match':
        case 'child_mismatch':
        case 'candidate_match':
            return true;
        default:
            return false;
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

export interface SearchEdgeKeys {
    /** Edge pair keys for start_path edges (upward exploration). */
    startEdgeKeys: Set<number>;
    /** Edge pair keys for the root edge(s) — may include intermediate hops. */
    rootEdgeKeys: Set<number>;
    /** Edge pair keys for end_path edges (downward comparison). */
    endEdgeKeys: Set<number>;
}

/**
 * Compute the search-path edge pair key sets for highlighting.
 *
 * Uses BFS through the snapshot topology to find intermediate graph edges
 * between VizPathGraph nodes.  Falls back to direct VizPathGraph edge refs
 * when BFS cannot find a path (disconnected topologies, stale snapshots).
 *
 * **Fixes over previous inline implementation:**
 * 1. Root edge now covers ALL intermediate hops (was: only first hop).
 * 2. BFS null results fall back to VizPathGraph edge refs (was: silently dropped).
 * 3. End-path edges are cleared when transitioning away from child exploration.
 */
export function computeSearchEdgeKeys(
    sp: VizPathGraph,
    snapshotEdges: SnapshotEdge[] | null,
    transition: Transition | null,
): SearchEdgeKeys {
    const startEdgeKeys = new Set<number>();
    const rootEdgeKeys = new Set<number>();
    const endEdgeKeys = new Set<number>();

    const childMap = snapshotEdges ? buildChildMap(snapshotEdges) : null;

    // ── Start path edges ──
    // Start edges point UP (from=child, to=parent), but layout edges
    // always go parent→child.  We compute parent→child pair keys.
    if (childMap) {
        // Build the ordered chain: start_node → start_path[0] → … → start_path[n]
        const startChain: number[] = [];
        if (sp.start_node) startChain.push(sp.start_node.index);
        for (const n of sp.start_path) startChain.push(n.index);

        for (let i = 0; i < startChain.length - 1; i++) {
            // parent is upper node (chain[i+1]), child is lower (chain[i])
            const parent = startChain[i + 1]!;
            const child = startChain[i]!;
            const path = findDescendantPath(parent, child, childMap);
            if (path) {
                for (let j = 0; j < path.length - 1; j++) {
                    startEdgeKeys.add(edgePairKey(path[j]!, path[j + 1]!));
                }
            } else {
                // FIX: BFS failed — fall back to direct edge pair key
                // instead of silently dropping the edge.
                startEdgeKeys.add(edgePairKey(parent, child));
            }
        }
    } else {
        // No snapshot topology — use start_edges directly (swap for layout direction)
        for (const e of sp.start_edges) {
            startEdgeKeys.add(edgePairKey(e.to, e.from));
        }
    }

    // ── Root edge ──
    if (sp.root_edge && sp.root) {
        if (childMap) {
            const startTop = sp.start_path.length > 0
                ? sp.start_path[sp.start_path.length - 1]!.index
                : sp.start_node?.index ?? -1;
            if (startTop >= 0) {
                const path = findDescendantPath(sp.root.index, startTop, childMap);
                if (path) {
                    // FIX: Add ALL intermediate edges, not just the first hop.
                    for (let j = 0; j < path.length - 1; j++) {
                        rootEdgeKeys.add(edgePairKey(path[j]!, path[j + 1]!));
                    }
                } else {
                    // BFS failed — fall back to direct edge ref
                    rootEdgeKeys.add(edgePairKey(sp.root_edge.to, sp.root_edge.from));
                }
            } else {
                rootEdgeKeys.add(edgePairKey(sp.root_edge.to, sp.root_edge.from));
            }
        } else {
            rootEdgeKeys.add(edgePairKey(sp.root_edge.to, sp.root_edge.from));
        }
    }

    // ── End path edges ──
    // FIX: Only compute end edges when the transition is child-exploration
    // related, OR when VizPathGraph still has end_path entries (persistent
    // state).  Clears stale end edges when moving to parent exploration.
    if (childMap && sp.root) {
        // Prefer the transition's explicit target (visit_child, child_match,
        // child_mismatch) so arrows update during child comparison.
        let target = endPathTargetFromTransition(transition);

        // Fall back to last end_path node only if we're still in
        // child-exploration or have no transition at all.
        if (target == null && sp.end_path.length > 0) {
            if (isChildExplorationTransition(transition) || transition == null) {
                target = sp.end_path[sp.end_path.length - 1]!.index;
            }
            // Otherwise: transition moved away from child exploration —
            // end edges are cleared (the set stays empty).
        }

        if (target != null && target !== sp.root.index) {
            const path = findDescendantPath(sp.root.index, target, childMap);
            if (path) {
                for (let j = 0; j < path.length - 1; j++) {
                    endEdgeKeys.add(edgePairKey(path[j]!, path[j + 1]!));
                }
            } else {
                // FIX: BFS failed — fall back to direct end_edges
                for (const e of sp.end_edges) {
                    endEdgeKeys.add(edgePairKey(e.from, e.to));
                }
            }
        }
    } else {
        // Fallback: use end_edges directly (no snapshot topology).
        for (const e of sp.end_edges) {
            endEdgeKeys.add(edgePairKey(e.from, e.to));
        }
    }

    return { startEdgeKeys, rootEdgeKeys, endEdgeKeys };
}
