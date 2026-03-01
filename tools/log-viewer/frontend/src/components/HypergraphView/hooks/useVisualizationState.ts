/**
 * Hook for deriving visualization state from graph operation events.
 * Extracts node roles and styling information from search/insert/read states.
 * Integrates VizPathGraph for precise search path edge highlighting.
 */
import { useMemo } from 'preact/hooks';
import type { GraphOpEvent, LocationInfo, SnapshotEdge, Transition, VizPathGraph } from '../../../types/generated';
import { edgePairKey } from '../utils/math';
import { allNodeIndices } from '../../../search-path/reconstruction';

// ---------------------------------------------------------------------------
// Graph topology helpers for intermediate edge computation
// ---------------------------------------------------------------------------

/** Build a child-adjacency map from snapshot edges. */
function buildChildMap(edges: SnapshotEdge[]): Map<number, number[]> {
    const childMap = new Map<number, number[]>();
    for (const e of edges) {
        let children = childMap.get(e.from);
        if (!children) {
            children = [];
            childMap.set(e.from, children);
        }
        children.push(e.to);
    }
    return childMap;
}

/**
 * BFS from ancestor to descendant through parent→child edges.
 * Returns the path as [ancestor, ..., descendant] or null if unreachable.
 */
function findDescendantPath(
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
 */
function endPathTargetFromTransition(trans: Transition | null): number | null {
    if (!trans) return null;
    switch (trans.kind) {
        case 'visit_child':    return trans.to;
        case 'child_match':    return trans.node;
        case 'child_mismatch': return trans.node;
        default:               return null;
    }
}

export interface VisualizationState {
    /** Primary node being operated on */
    selectedNode: number | null;
    /** Root of current exploration */
    rootNode: number | null;
    /** Nodes in the trace path from root to current */
    tracePath: Set<number>;
    /** Completed/matched nodes */
    completedNodes: Set<number>;
    /** Pending parent candidates */
    pendingParents: Set<number>;
    /** Pending child candidates */
    pendingChildren: Set<number>;
    /** Start node (for start_node transition) */
    startNode: number | null;
    /** Current candidate parent being explored */
    candidateParent: number | null;
    /** Current candidate child being explored */
    candidateChild: number | null;
    /** Node that matched */
    matchedNode: number | null;
    /** Node that mismatched */
    mismatchedNode: number | null;
    /** All nodes involved in current visualization (for dimming others) */
    involvedNodes: Set<number>;
    /** Whether any visualization state is active */
    hasVizState: boolean;
    /** The raw transition for additional context */
    transition: Transition | null;
    /** The raw location info */
    location: LocationInfo | null;
    /** Active search path graph (null when no search path data) */
    searchPath: VizPathGraph | null;
    /** Edge pair keys for start_path edges (upward exploration) */
    searchStartEdgeKeys: Set<number>;
    /** Edge pair key for the root edge (null if no root edge) */
    searchRootEdgeKey: number | null;
    /** Edge pair keys for end_path edges (downward comparison) */
    searchEndEdgeKeys: Set<number>;
}

/**
 * Derive the primary node to focus on from a transition.
 */
export function getPrimaryNode(trans: Transition | null, loc: LocationInfo | null): number | null {
    if (trans) {
        switch (trans.kind) {
            case 'start_node':
                return trans.node;
            case 'visit_parent':
                return trans.to;
            case 'visit_child':
                return trans.to;
            case 'child_match':
                return trans.node;
            case 'child_mismatch':
                return trans.node;
            case 'done':
                return trans.final_node;
            case 'dequeue':
                return trans.node;
            case 'root_explore':
                return trans.root;
            case 'match_advance':
                return trans.root;
            case 'parent_explore':
                return trans.current_root;
            case 'split_start':
                return trans.node;
            case 'split_complete':
                return trans.original_node;
            case 'join_start':
                return trans.nodes[0] ?? null;
            case 'join_step':
                return trans.result;
            case 'join_complete':
                return trans.result_node;
            case 'create_pattern':
                return trans.parent;
            case 'create_root':
                return trans.node;
            case 'update_pattern':
                return trans.parent;
        }
    }

    // Fall back to location info
    if (loc?.root_node != null) return loc.root_node;
    if (loc?.selected_node != null) return loc.selected_node;
    return null;
}

/**
 * Hook to derive visualization state from an active graph operation event
 * and optional search path graph.
 *
 * @param snapshotEdges - Snapshot edges for computing intermediate
 *   graph edges when the VizPathGraph end_edges skip intermediate nodes.
 */
export function useVisualizationState(
    event: GraphOpEvent | null,
    searchPath?: VizPathGraph | null,
    snapshotEdges?: SnapshotEdge[] | null,
): VisualizationState {
    return useMemo(() => {
        const loc = event?.location ?? null;
        const trans = event?.transition ?? null;
        const sp = searchPath ?? null;

        const selectedNode = loc?.selected_node ?? null;
        const rootNode = loc?.root_node ?? null;
        const tracePath = new Set(loc?.trace_path ?? []);
        const completedNodes = new Set(loc?.completed_nodes ?? []);
        const pendingParents = new Set(loc?.pending_parents ?? []);
        const pendingChildren = new Set(loc?.pending_children ?? []);

        // Derive transition-specific node roles
        const startNode: number | null = trans?.kind === 'start_node' ? trans.node : null;
        const candidateParent: number | null = trans?.kind === 'visit_parent' ? trans.to : null;
        const candidateChild: number | null = trans?.kind === 'visit_child' ? trans.to : null;
        const matchedNode: number | null = trans?.kind === 'child_match' ? trans.node : null;
        const mismatchedNode: number | null = trans?.kind === 'child_mismatch' ? trans.node : null;

        // Include parent_candidates from parent_explore transitions in pendingParents.
        // LocationInfo.pending_parents comes from the queue, but the queue may be empty
        // by the time the event is emitted; the transition itself carries the canonical list.
        if (trans?.kind === 'parent_explore') {
            for (const n of trans.parent_candidates) pendingParents.add(n);
        }

        // ── Search path edge key sets (pair keys — pattern_idx independent) ──
        const searchStartEdgeKeys = new Set<number>();
        const searchEndEdgeKeys = new Set<number>();
        let searchRootEdgeKey: number | null = null;

        // Build child-adjacency map once when we have both a search path
        // and snapshot topology.  Used to find intermediate graph edges
        // that the VizPathGraph may skip (e.g. root → leaf shortcuts).
        const childMap = (sp && snapshotEdges) ? buildChildMap(snapshotEdges) : null;

        if (sp) {
            // ── Start path edges ──
            // Start edges point UP (from=child, to=parent), but layout edges
            // always go parent→child. Swap from/to to match layout direction.
            if (childMap) {
                // Use graph topology to find intermediate edges for each
                // link in the start path chain:
                //   start_node → start_path[0] → … → start_path[n]
                // Each link is child→parent; layout edges are parent→child.
                const startChain: number[] = [];
                if (sp.start_node) startChain.push(sp.start_node.index);
                for (const n of sp.start_path) startChain.push(n.index);

                for (let i = 0; i < startChain.length - 1; i++) {
                    // parent is upper node (chain[i+1]), child is lower (chain[i])
                    const path = findDescendantPath(startChain[i + 1]!, startChain[i]!, childMap);
                    if (path) {
                        for (let j = 0; j < path.length - 1; j++) {
                            searchStartEdgeKeys.add(edgePairKey(path[j]!, path[j + 1]!));
                        }
                    }
                }
            } else {
                for (const e of sp.start_edges) {
                    searchStartEdgeKeys.add(edgePairKey(e.to, e.from));
                }
            }

            // ── Root edge ──
            if (sp.root_edge) {
                if (childMap && sp.root) {
                    // Root edge connects top-of-start-path → root.
                    // root is the parent; start_path top is the child.
                    const startTop = sp.start_path.length > 0
                        ? sp.start_path[sp.start_path.length - 1]!.index
                        : sp.start_node?.index ?? -1;
                    if (startTop >= 0) {
                        const path = findDescendantPath(sp.root.index, startTop, childMap);
                        if (path && path.length >= 2) {
                            searchRootEdgeKey = edgePairKey(path[0]!, path[1]!);
                        } else {
                            searchRootEdgeKey = edgePairKey(sp.root_edge.to, sp.root_edge.from);
                        }
                    } else {
                        searchRootEdgeKey = edgePairKey(sp.root_edge.to, sp.root_edge.from);
                    }
                } else {
                    searchRootEdgeKey = edgePairKey(sp.root_edge.to, sp.root_edge.from);
                }
            }

            // ── End path edges ──
            // Compute the actual graph path from root to the target child,
            // filling in intermediate edges that end_edges may skip.
            if (childMap && sp.root) {
                // Determine the deepest target node: prefer the transition's
                // target (visit_child/child_match/child_mismatch) so the
                // arrows update during child comparison, then fall back to
                // the last end_path node.
                let target = endPathTargetFromTransition(trans);
                if (target == null && sp.end_path.length > 0) {
                    target = sp.end_path[sp.end_path.length - 1]!.index;
                }

                if (target != null && target !== sp.root.index) {
                    const path = findDescendantPath(sp.root.index, target, childMap);
                    if (path) {
                        for (let j = 0; j < path.length - 1; j++) {
                            searchEndEdgeKeys.add(edgePairKey(path[j]!, path[j + 1]!));
                        }
                        // Also add all intermediate nodes to involvedNodes
                        // (computed later, but captured in `endPathIntermediates`
                        // so they aren't dimmed).
                    }
                }
            } else {
                // Fallback: use end_edges directly (no snapshot topology).
                for (const e of sp.end_edges) {
                    searchEndEdgeKeys.add(edgePairKey(e.from, e.to));
                }
            }
        }

        // Build the set of all "involved" nodes for dimming non-involved ones
        const involvedNodes = new Set<number>();
        if (loc) {
            if (selectedNode != null) involvedNodes.add(selectedNode);
            if (rootNode != null) involvedNodes.add(rootNode);
            for (const n of loc.trace_path) involvedNodes.add(n);
            for (const n of loc.completed_nodes) involvedNodes.add(n);
            for (const n of loc.pending_parents) involvedNodes.add(n);
            for (const n of loc.pending_children) involvedNodes.add(n);
        }
        if (startNode != null) involvedNodes.add(startNode);
        if (candidateParent != null) involvedNodes.add(candidateParent);
        if (candidateChild != null) involvedNodes.add(candidateChild);
        if (matchedNode != null) involvedNodes.add(matchedNode);
        if (mismatchedNode != null) involvedNodes.add(mismatchedNode);
        // Also include transition 'from' nodes
        if (trans?.kind === 'visit_parent' || trans?.kind === 'visit_child') {
            involvedNodes.add(trans.from);
        }

        // Include search path nodes in the involved set
        if (sp) {
            for (const idx of allNodeIndices(sp)) {
                involvedNodes.add(idx);
            }
        }

        // Include intermediate nodes discovered via edge key computation.
        // The edgePairKey encodes (from << 16 | to), so decode both indices.
        for (const key of searchStartEdgeKeys) {
            involvedNodes.add(key >>> 16);
            involvedNodes.add(key & 0xFFFF);
        }
        for (const key of searchEndEdgeKeys) {
            involvedNodes.add(key >>> 16);
            involvedNodes.add(key & 0xFFFF);
        }
        if (searchRootEdgeKey != null) {
            involvedNodes.add(searchRootEdgeKey >>> 16);
            involvedNodes.add(searchRootEdgeKey & 0xFFFF);
        }

        const hasVizState = involvedNodes.size > 0;

        return {
            selectedNode,
            rootNode,
            tracePath,
            completedNodes,
            pendingParents,
            pendingChildren,
            startNode,
            candidateParent,
            candidateChild,
            matchedNode,
            mismatchedNode,
            involvedNodes,
            hasVizState,
            transition: trans,
            location: loc,
            searchPath: sp,
            searchStartEdgeKeys,
            searchRootEdgeKey,
            searchEndEdgeKeys,
        };
    }, [event, searchPath, snapshotEdges]);
}

/**
 * Compute the CSS visualization classes for a node based on viz state.
 */
export function getNodeVizClasses(nodeIndex: number, viz: VisualizationState): string {
    const {
        startNode,
        selectedNode,
        rootNode,
        candidateParent,
        candidateChild,
        matchedNode,
        mismatchedNode,
        tracePath,
        completedNodes,
        pendingParents,
        pendingChildren,
        hasVizState,
        involvedNodes,
        searchPath,
    } = viz;

    const isStart = nodeIndex === startNode;
    const isSelected = nodeIndex === selectedNode && !isStart;
    const isRoot = nodeIndex === rootNode;

    // Search path node roles — all nodes in the search path get the same
    // start-path or end-path highlight; sp-start/sp-root are additive badges.
    const spStartNode = searchPath?.start_node?.index ?? -1;
    const spRootNode = searchPath?.root?.index ?? -1;
    const isSpStart = nodeIndex === spStartNode;
    const isSpRoot = nodeIndex === spRootNode;
    const isInStartPath = isSpStart || isSpRoot ||
        (searchPath?.start_path.some(n => n.index === nodeIndex) ?? false);
    const isSpEndPath =
        (searchPath?.end_path.some(n => n.index === nodeIndex) ?? false);
    const isCandidateParent = nodeIndex === candidateParent;
    const isCandidateChild = nodeIndex === candidateChild;
    const isMatched = nodeIndex === matchedNode;
    const isMismatched = nodeIndex === mismatchedNode;
    const isPath =
        tracePath.has(nodeIndex) &&
        !isStart &&
        !isRoot &&
        !isCandidateParent &&
        !isCandidateChild &&
        !isMatched &&
        !isMismatched;
    const isCompleted = completedNodes.has(nodeIndex) && !isStart && !isMatched && !isMismatched;
    const isPendingParent = pendingParents.has(nodeIndex) && !isCandidateParent;
    const isPendingChild = pendingChildren.has(nodeIndex) && !isCandidateChild;
    // Search path nodes are never dimmed — they are always "involved"
    const isInSearchPath = isInStartPath || isSpEndPath;
    const isDimmed = hasVizState && !involvedNodes.has(nodeIndex) && !isInSearchPath;

    return [
        isStart && 'viz-start',
        isSelected && 'viz-selected',
        isRoot && 'viz-root',
        isCandidateParent && 'viz-candidate-parent',
        isCandidateChild && 'viz-candidate-child',
        isMatched && 'viz-matched',
        isMismatched && 'viz-mismatched',
        isPath && 'viz-path',
        isCompleted && 'viz-completed',
        isPendingParent && 'viz-pending-parent',
        isPendingChild && 'viz-pending-child',
        isSpStart && 'viz-sp-start',
        isSpRoot && 'viz-sp-root',
        isInStartPath && 'viz-sp-start-path',
        isSpEndPath && 'viz-sp-end-path',
        isDimmed && 'viz-dimmed',
    ]
        .filter(Boolean)
        .join(' ');
}

/**
 * Get the active visualization states for a specific node (for info panel display).
 */
export function getNodeVizStates(nodeIndex: number, viz: VisualizationState): string[] {
    const states: string[] = [];
    if (nodeIndex === viz.startNode) states.push('start');
    if (nodeIndex === viz.selectedNode) states.push('selected');
    if (nodeIndex === viz.rootNode) states.push('root');
    if (nodeIndex === viz.candidateParent) states.push('candidate-parent');
    if (nodeIndex === viz.candidateChild) states.push('candidate-child');
    if (nodeIndex === viz.matchedNode) states.push('matched');
    if (nodeIndex === viz.mismatchedNode) states.push('mismatched');
    if (viz.tracePath.has(nodeIndex)) states.push('path');
    if (viz.completedNodes.has(nodeIndex)) states.push('completed');
    if (viz.pendingParents.has(nodeIndex)) states.push('pending-parent');
    if (viz.pendingChildren.has(nodeIndex)) states.push('pending-child');
    if (viz.searchPath) {
        if (viz.searchPath.start_node?.index === nodeIndex) states.push('sp-start');
        if (viz.searchPath.root?.index === nodeIndex) states.push('sp-root');
        if (viz.searchPath.start_path.some(n => n.index === nodeIndex)) states.push('sp-start-path');
        if (viz.searchPath.end_path.some(n => n.index === nodeIndex)) states.push('sp-end-path');
    }
    return states;
}
