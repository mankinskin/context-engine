/**
 * Tests for search path edge highlighting computation.
 *
 * Tests the pure functions extracted from useVisualizationState that
 * compute edge pair key sets for start-path, root, and end-path highlighting.
 */

import { describe, it, expect } from 'vitest';
import type { SnapshotEdge, VizPathGraph, Transition, PathNode, EdgeRef } from '../types/generated';
import {
    buildChildMap,
    findDescendantPath,
    endPathTargetFromTransition,
    edgePairKey,
    computeSearchEdgeKeys,
} from './edge-highlighting';
import { emptyPathGraph } from './reconstruction';

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

function pn(index: number, width = 1): PathNode {
    return { index, width };
}

function er(from: number, to: number, pattern_idx = 0, sub_index = 0): EdgeRef {
    return { from, to, pattern_idx, sub_index };
}

function se(from: number, to: number, pattern_idx = 0, sub_index = 0): SnapshotEdge {
    return { from, to, pattern_idx, sub_index };
}

/** Build a minimal VizPathGraph for testing. */
function makePath(overrides: Partial<VizPathGraph> = {}): VizPathGraph {
    return { ...emptyPathGraph(), ...overrides };
}

// ---------------------------------------------------------------------------
// buildChildMap
// ---------------------------------------------------------------------------

describe('buildChildMap', () => {
    it('builds adjacency list from snapshot edges', () => {
        const edges: SnapshotEdge[] = [se(10, 5), se(10, 6), se(20, 10)];
        const map = buildChildMap(edges);

        expect(map.get(10)).toEqual([5, 6]);
        expect(map.get(20)).toEqual([10]);
        expect(map.get(5)).toBeUndefined();
    });

    it('handles multiple patterns for same parent', () => {
        const edges: SnapshotEdge[] = [se(10, 5, 0), se(10, 6, 1), se(10, 7, 0, 1)];
        const map = buildChildMap(edges);

        expect(map.get(10)).toEqual([5, 6, 7]);
    });

    it('returns empty map for empty edges', () => {
        const map = buildChildMap([]);
        expect(map.size).toBe(0);
    });
});

// ---------------------------------------------------------------------------
// findDescendantPath
// ---------------------------------------------------------------------------

describe('findDescendantPath', () => {
    const edges: SnapshotEdge[] = [se(100, 50), se(50, 20), se(50, 30), se(20, 5)];
    const childMap = buildChildMap(edges);

    it('finds direct parent→child path', () => {
        const path = findDescendantPath(100, 50, childMap);
        expect(path).toEqual([100, 50]);
    });

    it('finds multi-hop path', () => {
        const path = findDescendantPath(100, 5, childMap);
        expect(path).toEqual([100, 50, 20, 5]);
    });

    it('returns single-element for ancestor === descendant', () => {
        const path = findDescendantPath(50, 50, childMap);
        expect(path).toEqual([50]);
    });

    it('returns null when descendant is unreachable', () => {
        const path = findDescendantPath(20, 100, childMap);
        expect(path).toBeNull();
    });

    it('returns null for disconnected nodes', () => {
        const extra = buildChildMap([se(1, 2), se(10, 20)]);
        expect(findDescendantPath(1, 20, extra)).toBeNull();
    });
});

// ---------------------------------------------------------------------------
// endPathTargetFromTransition
// ---------------------------------------------------------------------------

describe('endPathTargetFromTransition', () => {
    it('returns to for visit_child', () => {
        const trans: Transition = {
            kind: 'visit_child', from: 1, to: 5, child_index: 0, width: 1,
            edge: er(1, 5), replace: false,
        };
        expect(endPathTargetFromTransition(trans)).toBe(5);
    });

    it('returns node for child_match', () => {
        const trans: Transition = { kind: 'child_match', node: 7, cursor_pos: 3 };
        expect(endPathTargetFromTransition(trans)).toBe(7);
    });

    it('returns node for child_mismatch', () => {
        const trans: Transition = {
            kind: 'child_mismatch', node: 7, cursor_pos: 3, expected: 10, actual: 11,
        };
        expect(endPathTargetFromTransition(trans)).toBe(7);
    });

    it('returns null for non-child transitions', () => {
        expect(endPathTargetFromTransition({ kind: 'start_node', node: 1, width: 1 })).toBeNull();
        expect(endPathTargetFromTransition({
            kind: 'visit_parent', from: 1, to: 10, entry_pos: 0, width: 2, edge: er(1, 10),
        })).toBeNull();
        expect(endPathTargetFromTransition(null)).toBeNull();
    });
});

// ---------------------------------------------------------------------------
// computeSearchEdgeKeys — start path
// ---------------------------------------------------------------------------

describe('computeSearchEdgeKeys: start path', () => {
    // Topology: 100 → 50 → 20 → 5
    const snapshotEdges: SnapshotEdge[] = [se(100, 50), se(50, 20), se(20, 5)];

    it('computes start edges via BFS through intermediate nodes', () => {
        const sp = makePath({
            start_node: pn(5),
            start_path: [pn(20), pn(50)],
            start_edges: [er(5, 20), er(20, 50)],
        });

        const { startEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, null);

        // Expected edges (parent→child for layout): 50→20, 20→5
        expect(startEdgeKeys.has(edgePairKey(50, 20))).toBe(true);
        expect(startEdgeKeys.has(edgePairKey(20, 5))).toBe(true);
        expect(startEdgeKeys.size).toBe(2);
    });

    it('fills intermediate edges when BFS finds multi-hop path', () => {
        // start_path skips node 50: claims edge from 5 directly to 100
        // but in the graph 100→50→20→5
        const sp = makePath({
            start_node: pn(5),
            start_path: [pn(100)],
            start_edges: [er(5, 100)],
        });

        const { startEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, null);

        // BFS from 100 to 5 finds 100→50→20→5, so 3 edges
        expect(startEdgeKeys.has(edgePairKey(100, 50))).toBe(true);
        expect(startEdgeKeys.has(edgePairKey(50, 20))).toBe(true);
        expect(startEdgeKeys.has(edgePairKey(20, 5))).toBe(true);
        expect(startEdgeKeys.size).toBe(3);
    });

    it('falls back to direct edge when BFS fails', () => {
        // Node 99 is not in the graph topology
        const sp = makePath({
            start_node: pn(5),
            start_path: [pn(99)],
            start_edges: [er(5, 99)],
        });

        const { startEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, null);

        // BFS from 99 to 5 fails → falls back to direct key (99, 5)
        expect(startEdgeKeys.has(edgePairKey(99, 5))).toBe(true);
        expect(startEdgeKeys.size).toBe(1);
    });

    it('falls back to edge refs when no snapshot edges', () => {
        const sp = makePath({
            start_node: pn(5),
            start_path: [pn(20)],
            start_edges: [er(5, 20)],
        });

        const { startEdgeKeys } = computeSearchEdgeKeys(sp, null, null);

        // No childMap → swaps from/to: edgePairKey(to=20, from=5)
        expect(startEdgeKeys.has(edgePairKey(20, 5))).toBe(true);
        expect(startEdgeKeys.size).toBe(1);
    });
});

// ---------------------------------------------------------------------------
// computeSearchEdgeKeys — root edge
// ---------------------------------------------------------------------------

describe('computeSearchEdgeKeys: root edge', () => {
    // Topology: 200 → 100 → 50 → 20 → 5
    const snapshotEdges: SnapshotEdge[] = [
        se(200, 100), se(100, 50), se(50, 20), se(20, 5),
    ];

    it('highlights all intermediate root edge hops', () => {
        // Root at 200, start_path top is 50 (skipping 100)
        const sp = makePath({
            start_node: pn(5),
            start_path: [pn(20), pn(50)],
            start_edges: [er(5, 20), er(20, 50)],
            root: pn(200),
            root_edge: er(50, 200),
        });

        const { rootEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, null);

        // BFS from 200 to 50: 200→100→50, gives 2 edges
        expect(rootEdgeKeys.has(edgePairKey(200, 100))).toBe(true);
        expect(rootEdgeKeys.has(edgePairKey(100, 50))).toBe(true);
        expect(rootEdgeKeys.size).toBe(2);
    });

    it('highlights single root edge when direct', () => {
        const sp = makePath({
            start_node: pn(5),
            start_path: [pn(20), pn(50)],
            start_edges: [er(5, 20), er(20, 50)],
            root: pn(100),
            root_edge: er(50, 100),
        });

        const { rootEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, null);

        expect(rootEdgeKeys.has(edgePairKey(100, 50))).toBe(true);
        expect(rootEdgeKeys.size).toBe(1);
    });

    it('falls back to direct edge when BFS fails', () => {
        // Root 999 not in graph
        const sp = makePath({
            start_node: pn(5),
            root: pn(999),
            root_edge: er(5, 999),
        });

        const { rootEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, null);

        // Falls back to edgePairKey(root_edge.to=999, root_edge.from=5)
        expect(rootEdgeKeys.has(edgePairKey(999, 5))).toBe(true);
        expect(rootEdgeKeys.size).toBe(1);
    });

    it('empty when no root_edge', () => {
        const sp = makePath({ start_node: pn(5) });
        const { rootEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, null);
        expect(rootEdgeKeys.size).toBe(0);
    });
});

// ---------------------------------------------------------------------------
// computeSearchEdgeKeys — end path
// ---------------------------------------------------------------------------

describe('computeSearchEdgeKeys: end path', () => {
    // Topology: 100 → 50 → 20 → 5, also 100 → 50 → 30
    const snapshotEdges: SnapshotEdge[] = [
        se(100, 50), se(50, 20), se(50, 30), se(20, 5),
    ];

    it('computes end edges from root to transition target', () => {
        const sp = makePath({
            start_node: pn(5),
            root: pn(100),
            root_edge: er(5, 100),
            end_path: [pn(50), pn(20)],
            end_edges: [er(100, 50), er(50, 20)],
        });

        const trans: Transition = {
            kind: 'visit_child', from: 50, to: 20, child_index: 0, width: 1,
            edge: er(50, 20), replace: false,
        };

        const { endEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, trans);

        // BFS from 100 to 20: 100→50→20
        expect(endEdgeKeys.has(edgePairKey(100, 50))).toBe(true);
        expect(endEdgeKeys.has(edgePairKey(50, 20))).toBe(true);
        expect(endEdgeKeys.size).toBe(2);
    });

    it('falls back to last end_path node when no child transition', () => {
        const sp = makePath({
            start_node: pn(5),
            root: pn(100),
            root_edge: er(5, 100),
            end_path: [pn(50), pn(30)],
            end_edges: [er(100, 50), er(50, 30)],
        });

        // root_explore is considered child-exploration, so end_path fallback is used
        const trans: Transition = {
            kind: 'root_explore', root: 100, width: 2, edge: er(5, 100),
        };

        const { endEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, trans);

        // Falls back to end_path[last] = 30; BFS from 100 to 30: 100→50→30
        expect(endEdgeKeys.has(edgePairKey(100, 50))).toBe(true);
        expect(endEdgeKeys.has(edgePairKey(50, 30))).toBe(true);
        expect(endEdgeKeys.size).toBe(2);
    });

    it('clears end edges when transition is not child-related', () => {
        const sp = makePath({
            start_node: pn(5),
            root: pn(100),
            root_edge: er(5, 100),
            end_path: [pn(50), pn(30)],
            end_edges: [er(100, 50), er(50, 30)],
        });

        // parent_explore is NOT child-exploration → end edges should be cleared
        const trans: Transition = {
            kind: 'parent_explore', current_root: 100, parent_candidates: [200],
        };

        const { endEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, trans);

        expect(endEdgeKeys.size).toBe(0);
    });

    it('clears end edges for visit_parent transition', () => {
        const sp = makePath({
            start_node: pn(5),
            root: pn(100),
            root_edge: er(5, 100),
            end_path: [pn(50)],
            end_edges: [er(100, 50)],
        });

        const trans: Transition = {
            kind: 'visit_parent', from: 100, to: 200, entry_pos: 0, width: 3,
            edge: er(100, 200),
        };

        const { endEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, trans);

        expect(endEdgeKeys.size).toBe(0);
    });

    it('uses end_path fallback when transition is null', () => {
        const sp = makePath({
            start_node: pn(5),
            root: pn(100),
            root_edge: er(5, 100),
            end_path: [pn(50)],
            end_edges: [er(100, 50)],
        });

        const { endEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, null);

        expect(endEdgeKeys.has(edgePairKey(100, 50))).toBe(true);
        expect(endEdgeKeys.size).toBe(1);
    });

    it('falls back to direct end_edges when BFS fails', () => {
        // Root 100 exists, but target 999 doesn't
        const sp = makePath({
            start_node: pn(5),
            root: pn(100),
            root_edge: er(5, 100),
            end_path: [pn(999)],
            end_edges: [er(100, 999)],
        });

        const trans: Transition = {
            kind: 'visit_child', from: 100, to: 999, child_index: 0, width: 1,
            edge: er(100, 999), replace: false,
        };

        const { endEdgeKeys } = computeSearchEdgeKeys(sp, snapshotEdges, trans);

        // BFS fails → fallback to direct end_edges
        expect(endEdgeKeys.has(edgePairKey(100, 999))).toBe(true);
        expect(endEdgeKeys.size).toBe(1);
    });
});

// ---------------------------------------------------------------------------
// computeSearchEdgeKeys — empty / null inputs
// ---------------------------------------------------------------------------

describe('computeSearchEdgeKeys: edge cases', () => {
    it('returns empty sets for empty path graph', () => {
        const sp = emptyPathGraph();
        const { startEdgeKeys, rootEdgeKeys, endEdgeKeys } = computeSearchEdgeKeys(sp, null, null);
        expect(startEdgeKeys.size).toBe(0);
        expect(rootEdgeKeys.size).toBe(0);
        expect(endEdgeKeys.size).toBe(0);
    });

    it('handles start_node only (no start_path, no root)', () => {
        const sp = makePath({ start_node: pn(5) });
        const { startEdgeKeys, rootEdgeKeys, endEdgeKeys } = computeSearchEdgeKeys(sp, [], null);
        expect(startEdgeKeys.size).toBe(0);
        expect(rootEdgeKeys.size).toBe(0);
        expect(endEdgeKeys.size).toBe(0);
    });
});
