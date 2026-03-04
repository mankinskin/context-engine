/**
 * HypergraphViewCore — signal-free rendering engine.
 *
 * Accepts data via props instead of reading from the log-viewer store.
 * This makes the component extractable to viewer-api in the future.
 *
 * UI chrome panels (SearchStatePanel, InsertStatePanel, etc.) are rendered
 * by the parent wrapper, not by this component.
 */
import { useRef, useEffect, useState, useCallback } from 'preact/hooks';
import type { ComponentChildren } from 'preact';
import './hypergraph.css';
import { buildLayout, computeFocusedLayout, computeSearchPathLayout, type GraphLayout, type FocusedLayoutOffsets } from './layout';
import type { HypergraphViewProps } from './types';

// Hooks
import {
    useCamera,
    useVisualizationState,
    useMouseInteraction,
    useTouchInteraction,
    useOverlayRenderer,
    getPrimaryNode,
} from './hooks';

// Components
import {
    NodeInfoPanel,
    GraphInfoOverlay,
    NodeTooltip,
    NodeLayer,
} from './components';

export interface HypergraphViewCoreProps extends HypergraphViewProps {
    /**
     * Render-prop for additional children.
     * Receives `handleFocusNode` callback to allow panels to trigger node focus.
     */
    renderChildren?: (handleFocusNode: (nodeIndex: number) => void) => ComponentChildren;
}

/**
 * Core hypergraph visualization — no signal dependencies.
 */
export function HypergraphViewCore(props: HypergraphViewCoreProps) {
    const {
        snapshot,
        currentEvent,
        searchPath: currentSearchPath,
        autoLayout,
        snapshotEdges,
        stepKey,
        renderChildren,
    } = props;

    const containerRef = useRef<HTMLDivElement>(null);
    const nodeLayerRef = useRef<HTMLDivElement>(null);

    // Layout state
    const [layout, setLayout] = useState<GraphLayout | null>(null);
    const layoutRef = useRef<GraphLayout | null>(null);
    const originalPositionsRef = useRef<Map<number, { x: number; y: number; z: number }> | null>(null);

    // Camera controller
    const camera = useCamera();

    // Visualization state from search events + search path
    const vizState = useVisualizationState(currentEvent, currentSearchPath, snapshotEdges);

    // Mouse interaction (autoLayout passed via ref so it's always current)
    const autoLayoutRef = useRef(autoLayout);
    autoLayoutRef.current = autoLayout;
    const { selectedIdx, setSelectedIdx, tooltip, interRef } = useMouseInteraction(
        containerRef,
        layoutRef,
        camera,
        autoLayoutRef,
    );

    // Touch interaction (shares selection state via interRef)
    useTouchInteraction(containerRef, layoutRef, camera, interRef, setSelectedIdx);

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
        originalPositionsRef.current = null;
        camera.resetForLayout(newLayout.nodes.length, newLayout.center);
        interRef.current.selectedIdx = -1;
        interRef.current.hoverIdx = -1;
        setSelectedIdx(-1);
    }, [snapshot, camera, setSelectedIdx]);

    // ── Focused layout ──
    const focusedOffsetsRef = useRef<FocusedLayoutOffsets | null>(null);
    useEffect(() => {
        const curLayout = layoutRef.current;
        if (!curLayout) return;

        if (selectedIdx >= 0 && autoLayout) {
            if (!originalPositionsRef.current) {
                const saved = new Map<number, { x: number; y: number; z: number }>();
                for (const n of curLayout.nodes) {
                    saved.set(n.index, { x: n.tx, y: n.ty, z: n.tz });
                }
                originalPositionsRef.current = saved;
            }

            let layoutResult: FocusedLayoutOffsets | null;
            if (currentSearchPath?.root) {
                layoutResult = computeSearchPathLayout(curLayout, currentSearchPath, selectedIdx);
            } else {
                layoutResult = computeFocusedLayout(curLayout, selectedIdx);
            }
            focusedOffsetsRef.current = layoutResult;

            const selNode = curLayout.nodeMap.get(selectedIdx);
            if (selNode) {
                camera.focusOn([selNode.tx, selNode.ty, selNode.tz]);
            }
        } else if (selectedIdx >= 0) {
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
                camera.focusOn([selNode.tx, selNode.ty, selNode.tz]);
            }
        } else {
            focusedOffsetsRef.current = null;
            if (originalPositionsRef.current && curLayout) {
                for (const n of curLayout.nodes) {
                    const orig = originalPositionsRef.current.get(n.index);
                    if (orig) { n.tx = orig.x; n.ty = orig.y; n.tz = orig.z; }
                }
                originalPositionsRef.current = null;
            }
        }
    }, [selectedIdx, camera, currentSearchPath, autoLayout]);

    // Focus camera on primary node when search step changes
    useEffect(() => {
        const curLayout = layoutRef.current;
        if (!curLayout || !currentEvent) return;

        const primaryNode = getPrimaryNode(currentEvent.transition, currentEvent.location);
        if (primaryNode != null) {
            const node = curLayout.nodeMap.get(primaryNode);
            if (node) {
                interRef.current.selectedIdx = primaryNode;
                setSelectedIdx(primaryNode);
            }
        }
    }, [stepKey, camera, setSelectedIdx]);

    // Register WebGPU overlay renderer
    useOverlayRenderer(containerRef, nodeLayerRef, layoutRef, camera, interRef, vizState, setSelectedIdx, focusedOffsetsRef, originalPositionsRef);

    // Handle focus from NodeInfoPanel links
    const handleFocusNode = useCallback((nodeIndex: number) => {
        const curLayout = layoutRef.current;
        if (!curLayout) return;
        const target = curLayout.nodeMap.get(nodeIndex);
        if (target) {
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
            {/* DOM node layer */}
            <div ref={nodeLayerRef} class="hg-node-layer">
                {layout && (
                    <NodeLayer
                        nodes={layout.nodes}
                        maxWidth={maxWidth}
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

            {/* Tooltip */}
            <NodeTooltip tooltip={tooltip} />

            {/* Log-viewer-specific panels injected via render-prop */}
            {renderChildren?.(handleFocusNode)}
        </div>
    );
}
