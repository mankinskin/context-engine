/**
 * Hook for deriving visualization state from graph operation events.
 * Extracts node roles and styling information from search/insert/read states.
 */
import { useMemo } from 'preact/hooks';
import type { GraphOpEvent, LocationInfo, Transition } from '../../../types/generated';

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
 * Hook to derive visualization state from an active graph operation event.
 */
export function useVisualizationState(event: GraphOpEvent | null): VisualizationState {
    return useMemo(() => {
        const loc = event?.location ?? null;
        const trans = event?.transition ?? null;

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
        };
    }, [event]);
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
    } = viz;

    const isStart = nodeIndex === startNode;
    const isSelected = nodeIndex === selectedNode && !isStart;
    const isRoot = nodeIndex === rootNode;
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
    const isDimmed = hasVizState && !involvedNodes.has(nodeIndex);

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
    return states;
}
