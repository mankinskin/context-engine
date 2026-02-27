/// <reference types="@webgpu/types" />
/**
 * WebGPU overlay renderer hook for hypergraph edges and grid.
 * Registers a render callback with the shared WgpuOverlay system.
 */
import { useEffect, useRef } from 'preact/hooks';
import { mat4Multiply, mat4Inverse } from '../../Scene3D/math3d';
import { worldToScreen, worldScaleAtDepth, edgePairKey, edgeTripleKey } from '../utils/math';
import type { GraphLayout } from '../layout';
import type { CameraController } from './useCamera';
import type { InteractionState } from './useMouseInteraction';
import type { VisualizationState } from './useVisualizationState';
import paletteWgsl from '../../../effects/palette.wgsl?raw';
import shaderSource from '../hypergraph.wgsl?raw';
import { buildPaletteBuffer, PALETTE_BYTE_SIZE } from '../../../effects/palette';
import { themeColors } from '../../../store/theme';
import {
    overlayGpu,
    registerOverlayRenderer,
    unregisterOverlayRenderer,
    markOverlayScanDirty,
    setOverlayParticleVP,
    setOverlayParticleViewport,
    setOverlayRefDepth,
    setOverlayWorldScale,
    setOverlayCameraPos,
    type OverlayRenderCallback,
} from '../../WgpuOverlay/WgpuOverlay';

// ── Constants ──

const QUAD_VERTS = new Float32Array([-1, -1, 1, -1, 1, 1, -1, -1, 1, 1, -1, 1]);

const EDGE_INSTANCE_FLOATS = 12;
const GRID_LINE_FLOATS = 12;

const PATTERN_COLORS: [number, number, number][] = [
    [0.45, 0.55, 0.7],
    [0.7, 0.45, 0.55],
    [0.5, 0.7, 0.45],
    [0.65, 0.55, 0.7],
    [0.7, 0.65, 0.4],
    [0.4, 0.7, 0.65],
];

const PATH_EDGE_COLOR: [number, number, number] = [0.1, 0.75, 0.95];

// Search path edge colors (VizPathGraph-based, more precise than trace_path pairs)
// Start & end paths share a uniform teal – arrows in the shader distinguish direction
const SP_PATH_EDGE_COLOR: [number, number, number] = [0.25, 0.75, 1.0];  // uniform teal for start & end paths
const SP_ROOT_EDGE_COLOR: [number, number, number] = [1.0, 0.85, 0.3];   // gold for root edge
const CANDIDATE_EDGE_COLOR: [number, number, number] = [0.55, 0.4, 0.8]; // muted violet for candidate edges

// Parent/child edge colors for selection-mode highlighting
const PARENT_EDGE_COLOR: [number, number, number] = [0.95, 0.65, 0.2];  // warm amber for parent edges
const CHILD_EDGE_COLOR: [number, number, number] = [0.3, 0.7, 0.9];    // cool teal for child edges

/**
 * Hook to set up and manage the WebGPU overlay renderer for hypergraph visualization.
 */
export function useOverlayRenderer(
    containerRef: { current: HTMLDivElement | null },
    nodeLayerRef: { current: HTMLDivElement | null },
    layoutRef: { current: GraphLayout | null },
    camera: CameraController,
    interRef: { current: InteractionState },
    vizState: VisualizationState
): void {
    const gpu = overlayGpu.value;
    const curLayout = layoutRef.current;

    // Keep vizState in a ref so the render callback always reads the latest
    // without requiring GPU resource teardown/rebuild on every search step change.
    const vizStateRef = useRef(vizState);
    vizStateRef.current = vizState;

    useEffect(() => {
        const container = containerRef.current;
        const nodeLayer = nodeLayerRef.current;
        if (!gpu || !curLayout || !container || !nodeLayer || curLayout.nodes.length === 0) return;

        const { device, format } = gpu;

        // ── Create pipelines & buffers using the shared overlay device ──
        const fullShader = paletteWgsl + '\n' + shaderSource;
        const shader = device.createShaderModule({ code: fullShader });

        const quadVB = device.createBuffer({
            size: QUAD_VERTS.byteLength,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        device.queue.writeBuffer(quadVB, 0, QUAD_VERTS);

        const camUB = device.createBuffer({
            size: 128,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });
        const paletteUB = device.createBuffer({
            size: PALETTE_BYTE_SIZE,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });

        const camBGL = device.createBindGroupLayout({
            entries: [
                {
                    binding: 0,
                    visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                    buffer: { type: 'uniform' },
                },
                { binding: 1, visibility: GPUShaderStage.FRAGMENT, buffer: { type: 'uniform' } },
            ],
        });
        const camBG = device.createBindGroup({
            layout: camBGL,
            entries: [
                { binding: 0, resource: { buffer: camUB } },
                { binding: 1, resource: { buffer: paletteUB } },
            ],
        });
        const pipelineLayout = device.createPipelineLayout({ bindGroupLayouts: [camBGL] });

        const edgeVertexBuffers: GPUVertexBufferLayout[] = [
            {
                arrayStride: 8,
                stepMode: 'vertex',
                attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' as GPUVertexFormat }],
            },
            {
                arrayStride: EDGE_INSTANCE_FLOATS * 4,
                stepMode: 'instance',
                attributes: [
                    { shaderLocation: 6, offset: 0, format: 'float32x3' as GPUVertexFormat },
                    { shaderLocation: 7, offset: 12, format: 'float32x3' as GPUVertexFormat },
                    { shaderLocation: 8, offset: 24, format: 'float32x4' as GPUVertexFormat },
                    { shaderLocation: 9, offset: 40, format: 'float32' as GPUVertexFormat },
                    { shaderLocation: 10, offset: 44, format: 'float32' as GPUVertexFormat },
                ],
            },
        ];
        const edgeBlend: GPUBlendState = {
            color: { srcFactor: 'src-alpha', dstFactor: 'one-minus-src-alpha', operation: 'add' },
            alpha: { srcFactor: 'one', dstFactor: 'one-minus-src-alpha', operation: 'add' },
        };

        const edgePipeline = device.createRenderPipeline({
            layout: pipelineLayout,
            vertex: { module: shader, entryPoint: 'vs_edge', buffers: edgeVertexBuffers },
            fragment: { module: shader, entryPoint: 'fs_edge', targets: [{ format, blend: edgeBlend }] },
            primitive: { topology: 'triangle-list' },
            depthStencil: { format: 'depth24plus', depthWriteEnabled: false, depthCompare: 'always' },
        });

        const gridPipeline = device.createRenderPipeline({
            layout: pipelineLayout,
            vertex: { module: shader, entryPoint: 'vs_edge', buffers: edgeVertexBuffers },
            fragment: { module: shader, entryPoint: 'fs_edge', targets: [{ format, blend: edgeBlend }] },
            primitive: { topology: 'triangle-list' },
            depthStencil: { format: 'depth24plus', depthWriteEnabled: false, depthCompare: 'always' },
        });

        // ── Instance buffers ──
        const maxEdges = curLayout.edges.length;
        const edgeIB = device.createBuffer({
            size: Math.max(maxEdges * EDGE_INSTANCE_FLOATS * 4, 48),
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });

        // ── Grid lines at y=0 plane ──
        const GRID_EXTENT = 20;
        const GRID_STEP = 2;
        const gridLines: number[] = [];
        for (let i = -GRID_EXTENT; i <= GRID_EXTENT; i += GRID_STEP) {
            gridLines.push(i, 0, -GRID_EXTENT, i, 0, GRID_EXTENT, 0.25, 0.22, 0.18, 0.06, 0, 0);
            gridLines.push(-GRID_EXTENT, 0, i, GRID_EXTENT, 0, i, 0.25, 0.22, 0.18, 0.06, 0, 0);
        }
        gridLines.push(-GRID_EXTENT, 0, 0, GRID_EXTENT, 0, 0, 0.55, 0.25, 0.15, 0.12, 0, 0); // X red
        gridLines.push(0, 0, -GRID_EXTENT, 0, 0, GRID_EXTENT, 0.15, 0.25, 0.55, 0.12, 0, 0); // Z blue
        const gridData = new Float32Array(gridLines);
        const gridCount = gridLines.length / GRID_LINE_FLOATS;
        const gridIB = device.createBuffer({
            size: gridData.byteLength,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        device.queue.writeBuffer(gridIB, 0, gridData);

        // ── Render state (captured by callback closure) ──
        const edgeDataBuf = new Float32Array(maxEdges * EDGE_INSTANCE_FLOATS);

        // Pre-allocated per-frame buffers (avoids GC pressure at 60fps)
        const postMatrix = new Float32Array(16);
        const invPostMatrix = new Float32Array(16);
        const camBuf = new Float32Array(32);
        const connectedSet = new Set<number>();
        const connectedEdgeKeys = new Set<number>();
        const pathEdgeKeys = new Set<number>();
        // Persistent parent candidates — carried forward from parent_explore
        // so that visit_parent and root_explore still show candidate edges.
        let lastParentCandidates: number[] = [];
        let cachedPaletteColors: unknown = null;
        let cachedPaletteBuf: Float32Array | null = null;

        // ── Overlay render callback ──
        const renderCallback: OverlayRenderCallback = (pass, dev, time, dt, canvasW, canvasH, _depthView) => {
            // Get container bounds in viewport coords
            const rect = container.getBoundingClientRect();
            const vx = Math.max(0, Math.round(rect.left));
            const vy = Math.max(0, Math.round(rect.top));
            const vw = Math.min(Math.round(rect.width), canvasW - vx);
            const vh = Math.min(Math.round(rect.height), canvasH - vy);

            if (vw <= 0 || vh <= 0) return;

            // Restrict rendering to the container's region of the overlay canvas
            pass.setViewport(vx, vy, vw, vh, 0, 1);
            pass.setScissorRect(vx, vy, vw, vh);

            const { viewProj, camPos } = camera.getViewProj(vw, vh, dt);
            const inter = interRef.current;

            // ── Pass viewProj to particle system for world-space projection ──
            const W = canvasW,
                H = canvasH;
            const sx = vw / W,
                sy = vh / H;
            const tx = (2 * vx + vw) / W - 1;
            const ty = 1 - (2 * vy + vh) / H;
            // Reuse pre-allocated matrix
            postMatrix.fill(0);
            postMatrix[0] = sx;
            postMatrix[5] = sy;
            postMatrix[10] = 1;
            postMatrix[15] = 1;
            postMatrix[12] = tx;
            postMatrix[13] = ty;
            const fullVP = mat4Multiply(postMatrix, viewProj);
            const invSubVP = mat4Inverse(viewProj);
            if (invSubVP) {
                invPostMatrix.fill(0);
                invPostMatrix[0] = 1 / sx;
                invPostMatrix[5] = 1 / sy;
                invPostMatrix[10] = 1;
                invPostMatrix[15] = 1;
                invPostMatrix[12] = -tx / sx;
                invPostMatrix[13] = -ty / sy;
                const fullInvVP = mat4Multiply(invSubVP, invPostMatrix);
                setOverlayParticleVP(fullVP, fullInvVP);
                setOverlayCameraPos(camPos[0], camPos[1], camPos[2]);
            }
            setOverlayParticleViewport(vx, vy, vw, vh);

            // Compute reference NDC depth
            const camState = camera.stateRef.current;
            const ttx = camState.target[0],
                tty = camState.target[1],
                ttz = camState.target[2];
            const vp = viewProj;
            const tw = vp[3]! * ttx + vp[7]! * tty + vp[11]! * ttz + vp[15]!;
            const refZ = tw > 0.001 ? (vp[2]! * ttx + vp[6]! * tty + vp[10]! * ttz + vp[14]!) / tw : 0;
            setOverlayRefDepth(refZ);

            // Compute world scale
            const dist = Math.sqrt(
                (camPos[0] - ttx) ** 2 + (camPos[1] - tty) ** 2 + (camPos[2] - ttz) ** 2
            );
            const fov = Math.PI / 4;
            const worldScale = (2 * dist * Math.tan(fov / 2)) / vh;
            setOverlayWorldScale(worldScale);

            // ── Connected set for selection highlighting (reuse sets) ──
            connectedSet.clear();
            connectedEdgeKeys.clear();
            if (inter.selectedIdx >= 0) {
                connectedSet.add(inter.selectedIdx);
                const sel = curLayout.nodeMap.get(inter.selectedIdx);
                if (sel) {
                    for (const ci of sel.childIndices) connectedSet.add(ci);
                    for (const pi of sel.parentIndices) connectedSet.add(pi);
                }
                for (const e of curLayout.edges) {
                    if (e.from === inter.selectedIdx || e.to === inter.selectedIdx) {
                        connectedEdgeKeys.add(edgeTripleKey(e.from, e.to, e.patternIdx));
                    }
                }
            }

            // ── Animate nodes toward targets ──
            const lerpSpeed = 12; // exponential decay rate (higher = snappier)
            const lerpFactor = 1 - Math.exp(-lerpSpeed * dt);
            for (const n of curLayout.nodes) {
                n.x += (n.tx - n.x) * lerpFactor;
                n.y += (n.ty - n.y) * lerpFactor;
                n.z += (n.tz - n.z) * lerpFactor;
            }

            // ── Position DOM nodes ──
            const nodeDivs = nodeLayer.children;
            const curVizInvolved = vizStateRef.current.involvedNodes;
            for (let i = 0; i < curLayout.nodes.length && i < nodeDivs.length; i++) {
                const n = curLayout.nodes[i]!;
                const el = nodeDivs[i] as HTMLDivElement;
                const screen = worldToScreen([n.x, n.y, n.z], viewProj, vw, vh);
                const scale = worldScaleAtDepth(camPos, [n.x, n.y, n.z], vh);
                const pixelScale = Math.max(0.1, (scale * n.radius * 2.5) / 80);

                if (!screen.visible || pixelScale < 0.02) {
                    el.style.display = 'none';
                    continue;
                }
                el.style.display = '';

                // Dim nodes not connected to mouse-selected node, but never dim
                // nodes that are part of the active visualization (search path etc.)
                const dimmed = inter.selectedIdx >= 0
                    && !connectedSet.has(n.index)
                    && !curVizInvolved.has(n.index);
                el.style.opacity = dimmed ? '0.15' : '1';
                const zIdx = Math.round((1 - screen.z) * 1000);
                el.style.zIndex = String(zIdx);
                el.style.transform = `translate(-50%, -50%) translate(${screen.x.toFixed(1)}px, ${screen.y.toFixed(1)}px) scale(${pixelScale.toFixed(3)})`;
                el.setAttribute('data-depth', screen.z.toFixed(4));
            }

            markOverlayScanDirty();

            // ── Fill edge instances (read vizState from ref for latest value) ──
            const curVizState = vizStateRef.current;
            const vizTracePath = curVizState.location?.trace_path ?? [];
            pathEdgeKeys.clear();
            for (let p = 0; p < vizTracePath.length - 1; p++) {
                const from = vizTracePath[p]!,
                    to = vizTracePath[p + 1]!;
                pathEdgeKeys.add(edgePairKey(from, to));
                pathEdgeKeys.add(edgePairKey(to, from));
            }

            // Search path edge keys (from VizPathGraph — precise triple keys)
            const spStartKeys = curVizState.searchStartEdgeKeys;
            const spRootKey = curVizState.searchRootEdgeKey;
            const spEndKeys = curVizState.searchEndEdgeKeys;
            const hasSearchPath = spStartKeys.size > 0 || spRootKey !== null || spEndKeys.size > 0;
            const hasViz = vizTracePath.length > 0 || curVizState.selectedNode != null || hasSearchPath;

            // Track parent candidates across steps: parent_explore sets them,
            // they persist through visit_parent / root_explore / match_advance,
            // and reset on any other transition (new phase).
            const trans = curVizState.transition;
            if (trans?.kind === 'parent_explore') {
                lastParentCandidates = trans.parent_candidates;
            } else if (
                trans?.kind !== 'visit_parent' &&
                trans?.kind !== 'root_explore' &&
                trans?.kind !== 'match_advance'
            ) {
                lastParentCandidates = [];
            }

            // Candidate node set (pending + current candidates + carried-forward)
            const candidateNodes = new Set<number>();
            if (curVizState.candidateParent != null) candidateNodes.add(curVizState.candidateParent);
            if (curVizState.candidateChild != null) candidateNodes.add(curVizState.candidateChild);
            for (const n of curVizState.pendingParents) candidateNodes.add(n);
            for (const n of curVizState.pendingChildren) candidateNodes.add(n);
            // Include carried-forward parent candidates from last parent_explore.
            for (const n of lastParentCandidates) candidateNodes.add(n);

            for (let i = 0; i < curLayout.edges.length; i++) {
                const e = curLayout.edges[i]!;
                const a = curLayout.nodeMap.get(e.from);
                const b = curLayout.nodeMap.get(e.to);
                if (!a || !b) continue;
                const off = i * EDGE_INSTANCE_FLOATS;
                edgeDataBuf[off] = a.x;
                edgeDataBuf[off + 1] = a.y;
                edgeDataBuf[off + 2] = a.z;
                edgeDataBuf[off + 3] = b.x;
                edgeDataBuf[off + 4] = b.y;
                edgeDataBuf[off + 5] = b.z;

                // Search path edge identification (pair keys — pattern_idx independent)
                const pairKey = edgePairKey(e.from, e.to);
                const isSpStartEdge = spStartKeys.has(pairKey);
                const isSpRootEdge = spRootKey === pairKey;
                const isSpEndEdge = spEndKeys.has(pairKey);
                const isSearchPathEdge = isSpStartEdge || isSpRootEdge || isSpEndEdge;

                // Legacy trace_path-based detection (pair keys, fallback)
                const isPathEdge = !isSearchPathEdge && pathEdgeKeys.has(edgePairKey(e.from, e.to));
                const highlighted = connectedEdgeKeys.has(edgeTripleKey(e.from, e.to, e.patternIdx));

                // Detect candidate edges: one endpoint is a pending/candidate node,
                // and the edge is not already part of the confirmed search path.
                const isCandidateEdge = !isSearchPathEdge && !isPathEdge &&
                    candidateNodes.size > 0 &&
                    (candidateNodes.has(e.from) || candidateNodes.has(e.to));

                let r: number, g: number, b2: number, alpha: number, hlFlag: number;
                if (isSpRootEdge) {
                    // Gold for root edge
                    r = SP_ROOT_EDGE_COLOR[0]; g = SP_ROOT_EDGE_COLOR[1]; b2 = SP_ROOT_EDGE_COLOR[2];
                    alpha = 0.95; hlFlag = 1;
                } else if (isSpStartEdge) {
                    // Uniform teal for upward path (arrows in shader show direction)
                    r = SP_PATH_EDGE_COLOR[0]; g = SP_PATH_EDGE_COLOR[1]; b2 = SP_PATH_EDGE_COLOR[2];
                    alpha = 0.9; hlFlag = 1;
                } else if (isSpEndEdge) {
                    // Uniform teal for downward path (arrows in shader show direction)
                    r = SP_PATH_EDGE_COLOR[0]; g = SP_PATH_EDGE_COLOR[1]; b2 = SP_PATH_EDGE_COLOR[2];
                    alpha = 0.9; hlFlag = 1;
                } else if (isPathEdge) {
                    r = PATH_EDGE_COLOR[0];
                    g = PATH_EDGE_COLOR[1];
                    b2 = PATH_EDGE_COLOR[2];
                    alpha = 0.9;
                    hlFlag = 1;
                } else if (isCandidateEdge) {
                    // Muted violet for candidate/pending edges – more transparent
                    r = CANDIDATE_EDGE_COLOR[0];
                    g = CANDIDATE_EDGE_COLOR[1];
                    b2 = CANDIDATE_EDGE_COLOR[2];
                    alpha = 0.30;
                    hlFlag = 0;
                } else if (inter.selectedIdx >= 0) {
                    if (highlighted) {
                        // Differentiate parent vs child edges of selected node
                        const isParentEdge = e.to === inter.selectedIdx;
                        if (isParentEdge) {
                            r = PARENT_EDGE_COLOR[0]; g = PARENT_EDGE_COLOR[1]; b2 = PARENT_EDGE_COLOR[2];
                        } else {
                            r = CHILD_EDGE_COLOR[0]; g = CHILD_EDGE_COLOR[1]; b2 = CHILD_EDGE_COLOR[2];
                        }
                        alpha = 0.85;
                        hlFlag = 1;
                    } else {
                        const pc = PATTERN_COLORS[e.patternIdx % PATTERN_COLORS.length]!;
                        r = pc[0]; g = pc[1]; b2 = pc[2];
                        alpha = 0.12;
                        hlFlag = 0;
                    }
                } else if (hasViz) {
                    const pc = PATTERN_COLORS[e.patternIdx % PATTERN_COLORS.length]!;
                    r = pc[0];
                    g = pc[1];
                    b2 = pc[2];
                    alpha = 0.12;
                    hlFlag = 0;
                } else {
                    const pc = PATTERN_COLORS[e.patternIdx % PATTERN_COLORS.length]!;
                    r = pc[0];
                    g = pc[1];
                    b2 = pc[2];
                    alpha = 0.4;
                    hlFlag = 0;
                }

                edgeDataBuf[off + 6] = r;
                edgeDataBuf[off + 7] = g;
                edgeDataBuf[off + 8] = b2;
                edgeDataBuf[off + 9] = alpha;
                edgeDataBuf[off + 10] = hlFlag;
                // edgeType: 0=grid, 1=normal, 2=SP-start, 3=SP-root, 4=SP-end, 5=trace-path, 6=candidate
                edgeDataBuf[off + 11] = isSpStartEdge ? 2
                    : isSpRootEdge ? 3
                        : isSpEndEdge ? 4
                            : isPathEdge ? 5
                                : isCandidateEdge ? 6
                                    : 1;  // normal edge (energy beam)
            }
            dev.queue.writeBuffer(edgeIB, 0, edgeDataBuf);

            // ── Camera + palette uniforms (reuse pre-allocated buffer) ──
            camBuf.set(viewProj, 0);
            camBuf.set([camPos[0], camPos[1], camPos[2], 0], 16);
            camBuf.set([time, 0, 0, 0], 20);
            dev.queue.writeBuffer(camUB, 0, camBuf);

            // Only rebuild palette buffer when theme colors object changes
            const currentColors = themeColors.value;
            if (currentColors !== cachedPaletteColors) {
                cachedPaletteColors = currentColors;
                cachedPaletteBuf = buildPaletteBuffer(currentColors);
            }
            dev.queue.writeBuffer(paletteUB, 0, cachedPaletteBuf!);

            // ── Draw grid, edges ──
            pass.setPipeline(gridPipeline);
            pass.setVertexBuffer(0, quadVB);
            pass.setVertexBuffer(1, gridIB);
            pass.setBindGroup(0, camBG);
            pass.draw(6, gridCount);

            pass.setPipeline(edgePipeline);
            pass.setVertexBuffer(0, quadVB);
            pass.setVertexBuffer(1, edgeIB);
            pass.setBindGroup(0, camBG);
            pass.draw(6, curLayout.edges.length);

            // Restore viewport/scissor to full canvas
            pass.setViewport(0, 0, canvasW, canvasH, 0, 1);
            pass.setScissorRect(0, 0, canvasW, canvasH);
        };

        registerOverlayRenderer(renderCallback);

        return () => {
            unregisterOverlayRenderer(renderCallback);
            quadVB.destroy();
            camUB.destroy();
            paletteUB.destroy();
            edgeIB.destroy();
            gridIB.destroy();
        };
    }, [gpu, curLayout, camera]);
}
