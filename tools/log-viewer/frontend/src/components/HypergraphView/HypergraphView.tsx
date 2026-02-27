/**
 * HypergraphView — DOM-based 3D node display, unified with WgpuOverlay.
 *
 * Nodes are rendered as regular DOM `div` elements with CSS `transform`
 * positioning. The global WgpuOverlay element scanner picks up `.hg-node`
 * elements and spawns GPU-computed particle effects (sparks, embers, beams,
 * glitter) on them — the same system used for all other views.
 *
 * Edges and coordinate grid are rendered on the **shared WgpuOverlay canvas**
 * through the overlay render callback system.
 */
import { useRef, useEffect, useState, useCallback } from 'preact/hooks';
import { hypergraphSnapshot, activeSearchStep, activeSearchState, activeSearchPath, activePathEvent, activePathStep } from '../../store';
import './hypergraph.css';
import { buildLayout, computeFocusedLayout, type GraphLayout } from './layout';

// Hooks
import {
    useCamera,
    useVisualizationState,
    useMouseInteraction,
    useOverlayRenderer,
    getPrimaryNode,
} from './hooks';

// Components
import {
    SearchStatePanel,
    NodeInfoPanel,
    GraphInfoOverlay,
    NodeTooltip,
    ControlsHUD,
    NodeLayer,
    PathChainPanel,
} from './components';

/**
 * Main hypergraph visualization component.
 */
export function HypergraphView() {
    const containerRef = useRef<HTMLDivElement>(null);
    const nodeLayerRef = useRef<HTMLDivElement>(null);

    const snapshot = hypergraphSnapshot.value;

    // Layout state
    const [layout, setLayout] = useState<GraphLayout | null>(null);
    const layoutRef = useRef<GraphLayout | null>(null);
    const originalPositionsRef = useRef<Map<number, { x: number; y: number; z: number }> | null>(null);

    // Camera controller
    const camera = useCamera();

    // Visualization state from search events + search path
    // Prefer path-group event when a path is selected, fall back to global step
    const currentEvent = activePathEvent.value ?? activeSearchState.value;
    const vizState = useVisualizationState(currentEvent, activeSearchPath.value);

    // Mouse interaction
    const { selectedIdx, setSelectedIdx, hoverIdx, tooltip, interRef } = useMouseInteraction(
        containerRef,
        layoutRef,
        camera
    );

    // Build layout when snapshot changes
    useEffect(() => {
        if (!snapshot) {
            layoutRef.current = null;
            setLayout(null);
            return;
        }
        const newLayout = buildLayout(snapshot);
        layoutRef.current = newLayout;
        setLayout(newLayout);
        originalPositionsRef.current = null; // Clear saved positions for fresh layout
        camera.resetForLayout(newLayout.nodes.length, newLayout.maxWidth);
        // Reset selection/hover since node indices may differ
        interRef.current.selectedIdx = -1;
        interRef.current.hoverIdx = -1;
        setSelectedIdx(-1);
    }, [snapshot, camera, setSelectedIdx]);

    // ── Focused layout: set animation targets for connected nodes ──
    useEffect(() => {
        const curLayout = layoutRef.current;
        if (!curLayout) return;

        if (selectedIdx >= 0) {
            // Save original positions on first selection
            if (!originalPositionsRef.current) {
                const saved = new Map<number, { x: number; y: number; z: number }>();
                for (const n of curLayout.nodes) {
                    saved.set(n.index, { x: n.tx, y: n.ty, z: n.tz });
                }
                originalPositionsRef.current = saved;
            }

            // Reset all targets to originals before recomputing
            for (const n of curLayout.nodes) {
                const orig = originalPositionsRef.current.get(n.index);
                if (orig) { n.tx = orig.x; n.ty = orig.y; n.tz = orig.z; }
            }

            // Compute focused layout and set as animation targets
            const focusedPositions = computeFocusedLayout(curLayout, selectedIdx);
            if (focusedPositions) {
                for (const [idx, pos] of focusedPositions) {
                    const node = curLayout.nodeMap.get(idx);
                    if (node) {
                        node.tx = pos.x;
                        node.ty = pos.y;
                        node.tz = pos.z;
                    }
                }
            }
        } else {
            // Deselected — animate back to original positions
            if (originalPositionsRef.current && curLayout) {
                for (const n of curLayout.nodes) {
                    const orig = originalPositionsRef.current.get(n.index);
                    if (orig) { n.tx = orig.x; n.ty = orig.y; n.tz = orig.z; }
                }
                originalPositionsRef.current = null;
            }
        }
    }, [selectedIdx]);

    // Focus camera on primary node when search step changes
    useEffect(() => {
        const curLayout = layoutRef.current;
        if (!curLayout) return;

        const event = activePathEvent.value ?? activeSearchState.value;
        if (!event) return;

        const primaryNode = getPrimaryNode(event.transition, event.location);

        if (primaryNode != null) {
            const node = curLayout.nodeMap.get(primaryNode);
            if (node) {
                camera.focusOn([node.tx, node.ty, node.tz]);
                interRef.current.selectedIdx = primaryNode;
                setSelectedIdx(primaryNode);
            }
        }
    }, [activeSearchStep.value, activePathStep.value, camera, setSelectedIdx]);

    // Register WebGPU overlay renderer
    useOverlayRenderer(containerRef, nodeLayerRef, layoutRef, camera, interRef, vizState);

    // Handle focus from NodeInfoPanel links
    const handleFocusNode = useCallback((nodeIndex: number) => {
        const curLayout = layoutRef.current;
        if (!curLayout) return;
        const target = curLayout.nodeMap.get(nodeIndex);
        if (target) {
            camera.focusOn([target.tx, target.ty, target.tz]);
            setSelectedIdx(nodeIndex);
        }
    }, [camera, setSelectedIdx]);

    // Empty state
    if (!snapshot) {
        return (
            <div class="hypergraph-container hg-dom-mode">
                <div class="hypergraph-empty">
                    <span>No hypergraph data found in current log</span>
                    <div class="hg-hint">
                        To visualize the graph, call <code>graph.emit_graph_snapshot()</code> in
                        your Rust test after building the graph. This emits a structured tracing
                        event that the log viewer can render.
                    </div>
                </div>
            </div>
        );
    }

    const maxWidth = layout?.maxWidth ?? 1;
    const selectedNode = selectedIdx >= 0 ? layout?.nodeMap.get(selectedIdx) : null;

    return (
        <div ref={containerRef} class="hypergraph-container hg-dom-mode">
            {/* DOM node layer — styled as log-entry elements for WgpuOverlay integration */}
            <div ref={nodeLayerRef} class="hg-node-layer">
                {layout && (
                    <NodeLayer
                        nodes={layout.nodes}
                        maxWidth={maxWidth}
                        selectedIdx={selectedIdx}
                        hoverIdx={hoverIdx}
                        vizState={vizState}
                    />
                )}
            </div>

            {/* Info overlay */}
            <GraphInfoOverlay snapshot={snapshot} />

            {/* Selected Node Info Panel */}
            {selectedNode && layout && (
                <NodeInfoPanel
                    node={selectedNode}
                    layout={layout}
                    vizState={vizState}
                    onFocusNode={handleFocusNode}
                />
            )}

            {/* Search State Panel - floating list of algorithm steps */}
            <SearchStatePanel />

            {/* Path Chain Panel - breadcrumb of current search path */}
            <PathChainPanel onFocusNode={handleFocusNode} />

            {/* Tooltip */}
            <NodeTooltip tooltip={tooltip} />

            {/* HUD */}
            <ControlsHUD />
        </div>
    );
}
