/// <reference types="@webgpu/types" />
/**
 * WebGPU overlay renderer hook — thin orchestrator.
 *
 * Delegates to:
 *   - gpu/pipeline      — one-time GPU resource creation
 *   - gpu/edgeBuilder    — per-frame edge instance buffer filling
 *   - animation/         — node lerp + DOM positioning
 *   - decomposition/     — expand/collapse DOM reparenting
 */
import { useEffect, useRef, useMemo } from 'preact/hooks';
import { mat4Multiply, mat4Inverse } from '../../Scene3D/math3d';
import type { GraphLayout, FocusedLayoutOffsets } from '../layout';
import type { CameraController } from './useCamera';
import type { InteractionState } from './useMouseInteraction';
import type { VisualizationState } from './useVisualizationState';
import { buildPaletteBuffer } from '../../../effects/palette';
import { themeColors } from '../../../store/theme';
import { edgeTripleKey } from '../utils/math';
import {
    overlayGpu,
    registerOverlayRenderer,
    unregisterOverlayRenderer,
    setOverlayParticleVP,
    setOverlayParticleViewport,
    setOverlayRefDepth,
    setOverlayWorldScale,
    setOverlayCameraPos,
    type OverlayRenderCallback,
} from '../../WgpuOverlay/WgpuOverlay';

import { createGpuResources, destroyGpuResources } from '../gpu/pipeline';
import { buildEdgeInstances, type EdgeBuildContext } from '../gpu/edgeBuilder';
import { animateNodes } from '../animation/nodeAnimator';
import { positionDOMNodes } from '../animation/nodePositioner';
import { DecompositionManager } from '../decomposition/manager';

/**
 * Hook to set up and manage the WebGPU overlay renderer for hypergraph visualization.
 */
export function useOverlayRenderer(
    containerRef: { current: HTMLDivElement | null },
    nodeLayerRef: { current: HTMLDivElement | null },
    layoutRef: { current: GraphLayout | null },
    camera: CameraController,
    interRef: { current: InteractionState },
    vizState: VisualizationState,
    onSelectNode?: (idx: number) => void,
    focusedOffsetsRef?: { current: FocusedLayoutOffsets | null },
    originalPositionsRef?: { current: Map<number, { x: number; y: number; z: number }> | null },
): void {
    const gpu = overlayGpu.value;
    const curLayout = layoutRef.current;

    // Keep vizState in a ref so the render callback always reads the latest
    const vizStateRef = useRef(vizState);
    vizStateRef.current = vizState;

    // ── P2: Connected-set caching ──
    const selectedIdx = interRef.current.selectedIdx;
    const connectedCache = useMemo(() => {
        const connectedSet = new Set<number>();
        const connectedEdgeKeys = new Set<number>();
        if (selectedIdx >= 0 && curLayout) {
            connectedSet.add(selectedIdx);
            const sel = curLayout.nodeMap.get(selectedIdx);
            if (sel) {
                for (const ci of sel.childIndices) connectedSet.add(ci);
                for (const pi of sel.parentIndices) connectedSet.add(pi);
            }
            for (const e of curLayout.edges) {
                if (e.from === selectedIdx || e.to === selectedIdx) {
                    connectedEdgeKeys.add(edgeTripleKey(e.from, e.to, e.patternIdx));
                }
            }
        }
        return { connectedSet, connectedEdgeKeys };
    }, [selectedIdx, curLayout]);

    // Ref to make the cached values available inside the render callback
    const connectedCacheRef = useRef(connectedCache);
    connectedCacheRef.current = connectedCache;

    useEffect(() => {
        const container = containerRef.current;
        const nodeLayer = nodeLayerRef.current;
        if (!gpu || !curLayout || !container || !nodeLayer || curLayout.nodes.length === 0) return;

        const { device, format } = gpu;

        // ── One-time GPU resource creation ──
        const res = createGpuResources(device, format, curLayout.edges.length);

        // ── Decomposition manager ──
        const decomposition = new DecompositionManager(curLayout, nodeLayer, onSelectNode);

        // ── Pre-allocated per-frame scratch buffers ──
        const postMatrix = new Float32Array(16);
        const invPostMatrix = new Float32Array(16);
        const camBuf = new Float32Array(32);
        let cachedPaletteColors: unknown = null;
        let cachedPaletteBuf: Float32Array | null = null;

        // ── Edge build context (reused each frame) ──
        const edgeBuildCtx: EdgeBuildContext = {
            layout: curLayout,
            vizState: vizState,
            inter: interRef.current,
            connectedEdgeKeys: connectedCache.connectedEdgeKeys,
            hiddenDecompEdgeKeys: new Set(),
            lastParentCandidates: [],
        };

        // ── Dirty-flag state for edge buffer (P1 optimization) ──
        let prevVizState: VisualizationState | null = null;
        let prevSelectedIdx = -2; // sentinel
        let prevHoverIdx = -2;
        let prevExpandedSize = -1;
        let prevConnectedRef: Set<number> | null = null;
        let nodesMoving = true; // assume nodes are animating initially

        // ── Overlay render callback ──
        const renderCallback: OverlayRenderCallback = (pass, dev, time, dt, canvasW, canvasH, _depthView) => {
            const rect = container.getBoundingClientRect();
            const vx = Math.max(0, Math.round(rect.left));
            const vy = Math.max(0, Math.round(rect.top));
            const vw = Math.min(Math.round(rect.width), canvasW - vx);
            const vh = Math.min(Math.round(rect.height), canvasH - vy);
            if (vw <= 0 || vh <= 0) return;

            pass.setViewport(vx, vy, vw, vh, 0, 1);
            pass.setScissorRect(vx, vy, vw, vh);

            const { viewProj, camPos } = camera.getViewProj(vw, vh, dt);
            const inter = interRef.current;

            // ── Particle system VP setup ──
            const W = canvasW, H = canvasH;
            const sx = vw / W, sy = vh / H;
            const tx = (2 * vx + vw) / W - 1;
            const ty = 1 - (2 * vy + vh) / H;
            postMatrix.fill(0);
            postMatrix[0] = sx; postMatrix[5] = sy; postMatrix[10] = 1; postMatrix[15] = 1;
            postMatrix[12] = tx; postMatrix[13] = ty;
            const fullVP = mat4Multiply(postMatrix, viewProj);
            const invSubVP = mat4Inverse(viewProj);
            if (invSubVP) {
                invPostMatrix.fill(0);
                invPostMatrix[0] = 1 / sx; invPostMatrix[5] = 1 / sy;
                invPostMatrix[10] = 1; invPostMatrix[15] = 1;
                invPostMatrix[12] = -tx / sx; invPostMatrix[13] = -ty / sy;
                const fullInvVP = mat4Multiply(invSubVP, invPostMatrix);
                setOverlayParticleVP(fullVP, fullInvVP);
                setOverlayCameraPos(camPos[0], camPos[1], camPos[2]);
            }
            setOverlayParticleViewport(vx, vy, vw, vh);

            // Reference depth + world scale
            const camState = camera.stateRef.current;
            const ttx = camState.target[0], tty = camState.target[1], ttz = camState.target[2];
            const vp = viewProj;
            const tw = vp[3]! * ttx + vp[7]! * tty + vp[11]! * ttz + vp[15]!;
            const refZ = tw > 0.001 ? (vp[2]! * ttx + vp[6]! * tty + vp[10]! * ttz + vp[14]!) / tw : 0;
            setOverlayRefDepth(refZ);
            const dist = Math.sqrt((camPos[0] - ttx) ** 2 + (camPos[1] - tty) ** 2 + (camPos[2] - ttz) ** 2);
            const fov = Math.PI / 4;
            setOverlayWorldScale((2 * dist * Math.tan(fov / 2)) / vh);

            // ── Focused layout: project abstract offsets onto camera axes ──
            const focusedOffsets = focusedOffsetsRef?.current;
            if (focusedOffsets && originalPositionsRef?.current) {
                const origPositions = originalPositionsRef.current;
                // Reset all targets to originals first
                for (const n of curLayout.nodes) {
                    const orig = origPositions.get(n.index);
                    if (orig) { n.tx = orig.x; n.ty = orig.y; n.tz = orig.z; }
                }
                // Project each offset using current camera orientation
                const { anchorIdx, offsets } = focusedOffsets;
                const anchorOrig = origPositions.get(anchorIdx);
                if (anchorOrig) {
                    const axes = camera.getAxes();
                    const [rx, ry, rz] = axes.right;
                    const [ux, uy, uz] = axes.up;
                    for (const [idx, off] of offsets) {
                        const node = curLayout.nodeMap.get(idx);
                        if (node) {
                            node.tx = anchorOrig.x + off.dRight * rx + off.dUp * ux;
                            node.ty = anchorOrig.y + off.dRight * ry + off.dUp * uy;
                            node.tz = anchorOrig.z + off.dRight * rz + off.dUp * uz;
                        }
                    }
                }
            }

            // ── Animate nodes ──
            nodesMoving = animateNodes(curLayout.nodes, dt);

            // ── Decomposition management ──
            const curVizState = vizStateRef.current;
            const desiredExpanded = new Set<number>();
            if (inter.selectedIdx >= 0) desiredExpanded.add(inter.selectedIdx);
            const spRootIdx = curVizState.rootNode;
            if (spRootIdx != null && spRootIdx >= 0 && curVizState.searchPath != null) {
                desiredExpanded.add(spRootIdx);
            }
            // Prune: don't expand child of another expanded node
            for (const idx of [...desiredExpanded]) {
                const node = curLayout.nodeMap.get(idx);
                if (!node) continue;
                for (const otherIdx of desiredExpanded) {
                    if (otherIdx === idx) continue;
                    const other = curLayout.nodeMap.get(otherIdx);
                    if (other && other.childIndices.includes(idx)) {
                        desiredExpanded.delete(idx);
                        break;
                    }
                }
            }
            decomposition.update(desiredExpanded);

            // ── Position DOM nodes ──
            const cached = connectedCacheRef.current;
            positionDOMNodes({
                layout: curLayout,
                nodeElMap: decomposition.getNodeElMap(),
                viewProj,
                invSubVP,
                camPos,
                vw, vh,
                containerRect: rect,
                inter,
                vizInvolvedNodes: curVizState.involvedNodes,
                connectedSet: cached.connectedSet,
                decomposition,
            });

            // ── Build and upload edge instances (with dirty-flag optimization) ──
            const curHiddenDecomp = decomposition.getHiddenDecompEdgeKeys();
            const expandedSize = decomposition.getExpandedNodes().size;
            const edgeDirty = nodesMoving
                || curVizState !== prevVizState
                || inter.selectedIdx !== prevSelectedIdx
                || inter.hoverIdx !== prevHoverIdx
                || expandedSize !== prevExpandedSize
                || cached.connectedEdgeKeys !== prevConnectedRef;

            if (edgeDirty) {
                edgeBuildCtx.vizState = curVizState;
                edgeBuildCtx.inter = inter;
                edgeBuildCtx.connectedEdgeKeys = cached.connectedEdgeKeys;
                edgeBuildCtx.hiddenDecompEdgeKeys = curHiddenDecomp;
                buildEdgeInstances(res.edgeDataBuf, edgeBuildCtx);
                dev.queue.writeBuffer(res.edgeIB, 0, res.edgeDataBuf);
                prevVizState = curVizState;
                prevSelectedIdx = inter.selectedIdx;
                prevHoverIdx = inter.hoverIdx;
                prevExpandedSize = expandedSize;
                prevConnectedRef = cached.connectedEdgeKeys;
            }

            // ── Camera + palette uniforms ──
            camBuf.set(viewProj, 0);
            camBuf.set([camPos[0], camPos[1], camPos[2], 0], 16);
            camBuf.set([time, 0, 0, 0], 20);
            dev.queue.writeBuffer(res.camUB, 0, camBuf);

            const currentColors = themeColors.value;
            if (currentColors !== cachedPaletteColors) {
                cachedPaletteColors = currentColors;
                cachedPaletteBuf = buildPaletteBuffer(currentColors);
            }
            dev.queue.writeBuffer(res.paletteUB, 0, cachedPaletteBuf!);

            // ── Draw grid, edges ──
            pass.setPipeline(res.gridPipeline);
            pass.setVertexBuffer(0, res.quadVB);
            pass.setVertexBuffer(1, res.gridIB);
            pass.setBindGroup(0, res.camBG);
            pass.draw(6, res.gridCount);

            pass.setPipeline(res.edgePipeline);
            pass.setVertexBuffer(0, res.quadVB);
            pass.setVertexBuffer(1, res.edgeIB);
            pass.setBindGroup(0, res.camBG);
            pass.draw(6, curLayout.edges.length);

            pass.setViewport(0, 0, canvasW, canvasH, 0, 1);
            pass.setScissorRect(0, 0, canvasW, canvasH);
        };

        registerOverlayRenderer(renderCallback);

        return () => {
            decomposition.collapseAll();
            unregisterOverlayRenderer(renderCallback);
            destroyGpuResources(res);
        };
    }, [gpu, curLayout, camera]);
}
