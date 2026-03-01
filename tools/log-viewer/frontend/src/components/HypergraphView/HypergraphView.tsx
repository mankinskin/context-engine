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
import { hypergraphSnapshot, activeSearchStep, activeSearchState, activeSearchPath, activePathEvent, activePathStep, selectHighlightMode } from '../../store';
import './hypergraph.css';
import { buildLayout, computeFocusedLayout, computeSearchPathLayout, type GraphLayout, type CameraAxes, type FocusedLayoutOffsets } from './layout';

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
        camera.resetForLayout(newLayout.nodes.length, newLayout.center);
        // Reset selection/hover since node indices may differ
        interRef.current.selectedIdx = -1;
        interRef.current.hoverIdx = -1;
        setSelectedIdx(-1);
    }, [snapshot, camera, setSelectedIdx]);

    // ── Focused layout: set animation targets for connected nodes ──
    // When a search path with a root is active, anchor layout on the root
    // so start_path nodes stay stable and children expand below the root.
    // Gated by selectHighlightMode — when off, clicking only focuses the camera.
    //
    // Two-phase approach for performance:
    //   Phase 1 (once, on selection change): compute abstract 2D offsets
    //   Phase 2 (per frame, cheap): project offsets using current camera axes
    const currentSearchPath = activeSearchPath.value;
    const highlightMode = selectHighlightMode.value;
    const focusedOffsetsRef = useRef<FocusedLayoutOffsets | null>(null);
    useEffect(() => {
        const curLayout = layoutRef.current;
        if (!curLayout) return;

        if (selectedIdx >= 0 && highlightMode) {
            // Save original positions on first selection
            if (!originalPositionsRef.current) {
                const saved = new Map<number, { x: number; y: number; z: number }>();
                for (const n of curLayout.nodes) {
                    saved.set(n.index, { x: n.tx, y: n.ty, z: n.tz });
                }
                originalPositionsRef.current = saved;
            }

            // Phase 1: compute abstract 2D offsets (expensive, runs once)
            let layoutResult: FocusedLayoutOffsets | null;
            if (currentSearchPath?.root) {
                layoutResult = computeSearchPathLayout(curLayout, currentSearchPath, selectedIdx);
            } else {
                layoutResult = computeFocusedLayout(curLayout, selectedIdx);
            }
            focusedOffsetsRef.current = layoutResult;

            if (!layoutResult) return;

            const { anchorIdx, offsets } = layoutResult;
            const anchorOrig = originalPositionsRef.current.get(anchorIdx);
            if (!anchorOrig) return;

            // Phase 2: lightweight per-frame projection of offsets onto camera axes
            let rafId: number;
            const projectLayout = () => {
                // Reset all targets to originals
                const origPositions = originalPositionsRef.current;
                if (origPositions) {
                    for (const n of curLayout.nodes) {
                        const orig = origPositions.get(n.index);
                        if (orig) { n.tx = orig.x; n.ty = orig.y; n.tz = orig.z; }
                    }
                }

                // Get current camera orientation
                const axes: CameraAxes = camera.getAxes();
                const [rx, ry, rz] = axes.right;
                const [ux, uy, uz] = axes.up;

                // Project each offset: worldPos = anchor + dRight * right + dUp * up
                for (const [idx, off] of offsets) {
                    const node = curLayout.nodeMap.get(idx);
                    if (node) {
                        node.tx = anchorOrig.x + off.dRight * rx + off.dUp * ux;
                        node.ty = anchorOrig.y + off.dRight * ry + off.dUp * uy;
                        node.tz = anchorOrig.z + off.dRight * rz + off.dUp * uz;
                    }
                }

                rafId = requestAnimationFrame(projectLayout);
            };

            projectLayout();

            // Focus camera on the selected node's layout position (after projection
            // has updated tx/ty/tz to reflect the layout, not the original position)
            const selNode = curLayout.nodeMap.get(selectedIdx);
            if (selNode) {
                camera.focusOn([selNode.tx, selNode.ty, selNode.tz]);
            }

            return () => cancelAnimationFrame(rafId);
        } else if (selectedIdx >= 0) {
            // Focus-only mode: restore original positions and just pan camera
            focusedOffsetsRef.current = null;
            if (originalPositionsRef.current && curLayout) {
                for (const n of curLayout.nodes) {
                    const orig = originalPositionsRef.current.get(n.index);
                    if (orig) { n.tx = orig.x; n.ty = orig.y; n.tz = orig.z; }
                }
                originalPositionsRef.current = null;
            }
            const selNode = curLayout.nodeMap.get(selectedIdx);
            if (selNode) {
                camera.focusOn([selNode.x, selNode.y, selNode.z]);
            }
        } else {
            // Deselected — animate back to original positions
            focusedOffsetsRef.current = null;
            if (originalPositionsRef.current && curLayout) {
                for (const n of curLayout.nodes) {
                    const orig = originalPositionsRef.current.get(n.index);
                    if (orig) { n.tx = orig.x; n.ty = orig.y; n.tz = orig.z; }
                }
                originalPositionsRef.current = null;
            }
        }
    }, [selectedIdx, camera, currentSearchPath, highlightMode]);

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
                // camera.focusOn is handled by the focused layout effect
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
            // camera.focusOn is handled by the focused layout effect
            setSelectedIdx(nodeIndex);
        }
    }, [setSelectedIdx]);

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
