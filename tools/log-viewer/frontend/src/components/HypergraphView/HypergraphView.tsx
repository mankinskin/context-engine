/// <reference types="@webgpu/types" />
/**
 * HypergraphView — DOM-based 3D node display, unified with WgpuOverlay.
 *
 * Nodes are rendered as regular DOM `div` elements with CSS `transform`
 * positioning, styled as `.log-entry` elements so the global WgpuOverlay
 * automatically applies ember glow, angelic beams, and glitter effects
 * via its element scanner.
 *
 * Edges, coordinate grid, and 3D particles are rendered on the **shared
 * WgpuOverlay canvas** through the overlay render callback system — no
 * separate WebGPU canvas is created.  The callback sets viewport/scissor
 * to the container's screen region so draw calls are clipped correctly.
 */
import { useRef, useEffect, useState, useCallback } from 'preact/hooks';
import { hypergraphSnapshot } from '../../store';
import paletteWgsl from '../../effects/palette.wgsl?raw';
import particleShadingWgsl from '../../effects/particle-shading.wgsl?raw';
import shaderSource from './hypergraph.wgsl?raw';
import './hypergraph.css';
import { buildLayout, type GraphLayout, type LayoutNode } from './layout';
import {
    Vec3,
    mat4Perspective, mat4LookAt, mat4Multiply, mat4Inverse,
    screenToRay, rayPlaneIntersect,
} from '../Scene3D/math3d';
import { buildPaletteBuffer, PALETTE_BYTE_SIZE } from '../../effects/palette';
import { themeColors } from '../../store/theme';
import {
    type Particle3D,
    PARTICLE_INSTANCE_FLOATS,
    spawnBeam, spawnGlitter, updateParticles3D, fillParticleBuffer,
} from '../../effects/particle-sim';
import {
    overlayGpu,
    registerOverlayRenderer,
    unregisterOverlayRenderer,
    type OverlayRenderCallback,
} from '../WgpuOverlay/WgpuOverlay';

// ── constants ──

const QUAD_VERTS = new Float32Array([
    -1, -1,   1, -1,   1, 1,
    -1, -1,   1,  1,  -1, 1,
]);

const EDGE_INSTANCE_FLOATS = 12;
const GRID_LINE_FLOATS = 12;

const MAX_BEAMS    = 64;
const MAX_GLITTER  = 96;
const MAX_PARTICLES = MAX_BEAMS + MAX_GLITTER;

// ── helpers ──

function nodeWidthClass(width: number, maxWidth: number): string {
    if (width === 1) return 'level-info';       // atoms → calm blue glow
    const t = (width - 1) / Math.max(maxWidth - 1, 1);
    if (t > 0.7) return 'level-error';          // wide nodes → hot red
    if (t > 0.4) return 'level-warn';           // medium → amber
    return 'level-debug';                       // small compounds → dim green
}

function worldToScreen(
    worldPos: Vec3,
    viewProj: Float32Array,
    cw: number, ch: number,
): { x: number; y: number; z: number; visible: boolean } {
    const vp = viewProj;
    const cx = vp[0]*worldPos[0] + vp[4]*worldPos[1] + vp[8]*worldPos[2]  + vp[12];
    const cy = vp[1]*worldPos[0] + vp[5]*worldPos[1] + vp[9]*worldPos[2]  + vp[13];
    const cz = vp[2]*worldPos[0] + vp[6]*worldPos[1] + vp[10]*worldPos[2] + vp[14];
    const cw2 = vp[3]*worldPos[0] + vp[7]*worldPos[1] + vp[11]*worldPos[2] + vp[15];

    if (cw2 <= 0.001) return { x: -9999, y: -9999, z: 1, visible: false };

    const ndcX = cx / cw2;
    const ndcY = cy / cw2;
    const ndcZ = cz / cw2;

    const sx = (ndcX * 0.5 + 0.5) * cw;
    const sy = (1 - (ndcY * 0.5 + 0.5)) * ch;

    return { x: sx, y: sy, z: ndcZ, visible: ndcZ >= 0 && ndcZ <= 1 };
}

/**
 * Pixels-per-world-unit at a given world position.
 *
 * Uses the Euclidean distance from the camera to the point and the known
 * vertical FOV.  This is completely independent of camera orientation —
 * a node at a given distance from the camera always has the same on-screen
 * scale regardless of which direction the camera faces.
 */
const HALF_FOV_TAN = Math.tan(Math.PI / 8); // tan(fov/2) where fov = PI/4

function worldScaleAtDepth(
    camPos: Vec3,
    worldPos: Vec3,
    ch: number,
): number {
    const dx = worldPos[0] - camPos[0];
    const dy = worldPos[1] - camPos[1];
    const dz = worldPos[2] - camPos[2];
    const dist = Math.sqrt(dx * dx + dy * dy + dz * dz);
    if (dist < 0.001) return ch; // prevent division by zero
    return ch / (2 * dist * HALF_FOV_TAN);
}

function raySphere(
    ro: Vec3, rd: Vec3, center: Vec3, radius: number,
): number | null {
    const oc: Vec3 = [ro[0] - center[0], ro[1] - center[1], ro[2] - center[2]];
    const a = rd[0]*rd[0] + rd[1]*rd[1] + rd[2]*rd[2];
    const b = 2 * (oc[0]*rd[0] + oc[1]*rd[1] + oc[2]*rd[2]);
    const c = oc[0]*oc[0] + oc[1]*oc[1] + oc[2]*oc[2] - radius*radius;
    const disc = b*b - 4*a*c;
    if (disc < 0) return null;
    const t = (-b - Math.sqrt(disc)) / (2 * a);
    return t > 0 ? t : null;
}

// ══════════════════════════════════════════════════════
//  Component
// ══════════════════════════════════════════════════════

export function HypergraphView() {
    const containerRef = useRef<HTMLDivElement>(null);
    const nodeLayerRef = useRef<HTMLDivElement>(null);
    const [tooltip, setTooltip] = useState<{ x: number; y: number; node: LayoutNode } | null>(null);
    const [selectedIdx, setSelectedIdx] = useState(-1);
    const [hoverIdx, setHoverIdx] = useState(-1);

    const snapshot = hypergraphSnapshot.value;
    const gpu = overlayGpu.value;

    const [layout, setLayout] = useState<GraphLayout | null>(null);
    const layoutRef = useRef<GraphLayout | null>(null); // non-reactive ref for callbacks
    const camRef = useRef({
        yaw: 0.5, pitch: 0.4, dist: 6,
        targetY: 0, target: [0, 0, 0] as Vec3,
        // Smooth focus animation
        focusTarget: null as Vec3 | null,   // where the camera should animate to
        focusSpeed: 4.0,                    // lerp speed (units/sec)
    });
    const interRef = useRef({
        dragIdx: -1, dragPlaneY: 0, dragOffset: [0, 0, 0] as Vec3,
        orbiting: false, panning: false,
        lastMX: 0, lastMY: 0, mouseX: 0, mouseY: 0,
        selectedIdx: -1, hoverIdx: -1,
    });

    // Build layout when snapshot changes
    useEffect(() => {
        if (!snapshot) { layoutRef.current = null; setLayout(null); return; }
        const newLayout = buildLayout(snapshot);
        layoutRef.current = newLayout;
        setLayout(newLayout);
        camRef.current.dist = Math.max(6, newLayout.nodes.length * 0.5);
        camRef.current.targetY = (newLayout.maxWidth - 1) * 0.75;
        camRef.current.target = [0, camRef.current.targetY, 0];
        // Reset selection/hover since node indices may differ
        interRef.current.selectedIdx = -1;
        interRef.current.hoverIdx = -1;
        setSelectedIdx(-1);
        setHoverIdx(-1);
        setTooltip(null);
    }, [snapshot]);

    const getCamPos = useCallback((): Vec3 => {
        const c = camRef.current;
        return [
            c.target[0] + c.dist * Math.cos(c.pitch) * Math.sin(c.yaw),
            c.target[1] + c.dist * Math.sin(c.pitch),
            c.target[2] + c.dist * Math.cos(c.pitch) * Math.cos(c.yaw),
        ];
    }, []);

    const getViewProj = useCallback((cw: number, ch: number, dt?: number) => {
        const c = camRef.current;

        // Smooth-lerp camera target toward focusTarget if set
        if (c.focusTarget && dt && dt > 0) {
            const alpha = 1 - Math.exp(-c.focusSpeed * dt);
            c.target = [
                c.target[0] + (c.focusTarget[0] - c.target[0]) * alpha,
                c.target[1] + (c.focusTarget[1] - c.target[1]) * alpha,
                c.target[2] + (c.focusTarget[2] - c.target[2]) * alpha,
            ];
            // Stop animating once close enough
            const dx = c.focusTarget[0] - c.target[0];
            const dy = c.focusTarget[1] - c.target[1];
            const dz = c.focusTarget[2] - c.target[2];
            if (dx * dx + dy * dy + dz * dz < 0.0001) {
                c.target = [...c.focusTarget] as Vec3;
                c.focusTarget = null;
            }
        }

        const camPos = getCamPos();
        const view = mat4LookAt(camPos, c.target, [0, 1, 0]);
        const proj = mat4Perspective(Math.PI / 4, cw / Math.max(ch, 1), 0.1, 200);
        return { viewProj: mat4Multiply(proj, view), camPos };
    }, [getCamPos]);

    // ── Register overlay renderer for edges, grid, particles ──
    useEffect(() => {
        const curLayout = layoutRef.current;
        const container = containerRef.current;
        const nodeLayer = nodeLayerRef.current;
        if (!gpu || !curLayout || !container || !nodeLayer || curLayout.nodes.length === 0) return;

        const { device, format } = gpu;

        // ── Create pipelines & buffers using the shared overlay device ──
        const fullShader = paletteWgsl + '\n' + particleShadingWgsl + '\n' + shaderSource;
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
                { binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: { type: 'uniform' } },
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
                arrayStride: 8, stepMode: 'vertex',
                attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' as GPUVertexFormat }],
            },
            {
                arrayStride: EDGE_INSTANCE_FLOATS * 4, stepMode: 'instance',
                attributes: [
                    { shaderLocation: 6, offset: 0,  format: 'float32x3' as GPUVertexFormat },
                    { shaderLocation: 7, offset: 12, format: 'float32x3' as GPUVertexFormat },
                    { shaderLocation: 8, offset: 24, format: 'float32x4' as GPUVertexFormat },
                    { shaderLocation: 9, offset: 40, format: 'float32'   as GPUVertexFormat },
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
        });

        const gridPipeline = device.createRenderPipeline({
            layout: pipelineLayout,
            vertex: { module: shader, entryPoint: 'vs_edge', buffers: edgeVertexBuffers },
            fragment: { module: shader, entryPoint: 'fs_edge', targets: [{ format, blend: edgeBlend }] },
            primitive: { topology: 'triangle-list' },
        });

        const particlePipeline = device.createRenderPipeline({
            layout: pipelineLayout,
            vertex: {
                module: shader, entryPoint: 'vs_particle',
                buffers: [
                    {
                        arrayStride: 8, stepMode: 'vertex',
                        attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' as GPUVertexFormat }],
                    },
                    {
                        arrayStride: PARTICLE_INSTANCE_FLOATS * 4, stepMode: 'instance',
                        attributes: [
                            { shaderLocation: 2, offset: 0,  format: 'float32x3' as GPUVertexFormat },
                            { shaderLocation: 3, offset: 12, format: 'float32'   as GPUVertexFormat },
                            { shaderLocation: 4, offset: 16, format: 'float32x4' as GPUVertexFormat },
                            { shaderLocation: 5, offset: 32, format: 'float32x4' as GPUVertexFormat },
                        ],
                    },
                ],
            },
            fragment: {
                module: shader, entryPoint: 'fs_particle',
                targets: [{
                    format,
                    blend: {
                        color: { srcFactor: 'src-alpha', dstFactor: 'one', operation: 'add' },
                        alpha: { srcFactor: 'one', dstFactor: 'one', operation: 'add' },
                    },
                }],
            },
            primitive: { topology: 'triangle-list' },
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
            gridLines.push(i, 0, -GRID_EXTENT,  i, 0, GRID_EXTENT,  0.25, 0.22, 0.18, 0.06,  0, 0);
            gridLines.push(-GRID_EXTENT, 0, i,  GRID_EXTENT, 0, i,  0.25, 0.22, 0.18, 0.06,  0, 0);
        }
        gridLines.push(-GRID_EXTENT, 0, 0,  GRID_EXTENT, 0, 0,  0.55, 0.25, 0.15, 0.12,  0, 0); // X red
        gridLines.push(0, 0, -GRID_EXTENT,  0, 0, GRID_EXTENT,  0.15, 0.25, 0.55, 0.12,  0, 0); // Z blue
        const gridData = new Float32Array(gridLines);
        const gridCount = gridLines.length / GRID_LINE_FLOATS;
        const gridIB = device.createBuffer({
            size: gridData.byteLength,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });
        device.queue.writeBuffer(gridIB, 0, gridData);

        const particleIB = device.createBuffer({
            size: Math.max(MAX_PARTICLES * PARTICLE_INSTANCE_FLOATS * 4, 48),
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });

        // ── Render state (captured by callback closure) ──
        const particles: Particle3D[] = [];
        const edgeDataBuf = new Float32Array(maxEdges * EDGE_INSTANCE_FLOATS);
        const particleDataBuf = new Float32Array(MAX_PARTICLES * PARTICLE_INSTANCE_FLOATS);

        const PATTERN_COLORS: [number, number, number][] = [
            [0.45, 0.55, 0.7],  [0.7, 0.45, 0.55],  [0.5, 0.7, 0.45],
            [0.65, 0.55, 0.7],  [0.7, 0.65, 0.4],   [0.4, 0.7, 0.65],
        ];

        // ── Overlay render callback ──
        const renderCallback: OverlayRenderCallback = (pass, dev, time, dt, canvasW, canvasH) => {
            // Get container bounds in viewport coords (= canvas pixel coords,
            // since the overlay canvas uses 1:1 CSS-to-pixel mapping).
            const rect = container.getBoundingClientRect();
            const vx = Math.max(0, Math.round(rect.left));
            const vy = Math.max(0, Math.round(rect.top));
            const vw = Math.min(Math.round(rect.width),  canvasW - vx);
            const vh = Math.min(Math.round(rect.height), canvasH - vy);

            if (vw <= 0 || vh <= 0) return;

            // Restrict rendering to the container's region of the overlay canvas
            pass.setViewport(vx, vy, vw, vh, 0, 1);
            pass.setScissorRect(vx, vy, vw, vh);

            const { viewProj, camPos } = getViewProj(vw, vh, dt);
            const inter = interRef.current;

            // ── Connected set for selection highlighting ──
            const connectedSet = new Set<number>();
            const connectedEdges = new Set<string>();
            if (inter.selectedIdx >= 0) {
                connectedSet.add(inter.selectedIdx);
                const sel = curLayout.nodeMap.get(inter.selectedIdx);
                if (sel) {
                    for (const ci of sel.childIndices) connectedSet.add(ci);
                    for (const pi of sel.parentIndices) connectedSet.add(pi);
                }
                for (const e of curLayout.edges) {
                    if (e.from === inter.selectedIdx || e.to === inter.selectedIdx) {
                        connectedEdges.add(`${e.from}-${e.to}-${e.patternIdx}`);
                    }
                }
            }

            // ── Position DOM nodes (runs every frame via overlay rAF) ──
            const nodeDivs = nodeLayer.children;
            for (let i = 0; i < curLayout.nodes.length && i < nodeDivs.length; i++) {
                const n = curLayout.nodes[i]!;
                const el = nodeDivs[i] as HTMLDivElement;
                const screen = worldToScreen([n.x, n.y, n.z], viewProj, vw, vh);
                const scale = worldScaleAtDepth(camPos, [n.x, n.y, n.z], vh);
                const pixelScale = Math.max(0.1, scale * n.radius * 2.5 / 80);

                if (!screen.visible || pixelScale < 0.02) {
                    el.style.display = 'none';
                    continue;
                }
                el.style.display = '';

                const dimmed = inter.selectedIdx >= 0 && !connectedSet.has(n.index);
                el.style.opacity = dimmed ? '0.15' : '1';
                el.style.transform =
                    `translate(-50%, -50%) translate(${screen.x.toFixed(1)}px, ${screen.y.toFixed(1)}px) scale(${pixelScale.toFixed(3)})`;
                el.style.zIndex = String(Math.round((1 - screen.z) * 10000));
            }

            // ── Fill edge instances ──
            for (let i = 0; i < curLayout.edges.length; i++) {
                const e = curLayout.edges[i]!;
                const a = curLayout.nodeMap.get(e.from);
                const b = curLayout.nodeMap.get(e.to);
                if (!a || !b) continue;
                const off = i * EDGE_INSTANCE_FLOATS;
                edgeDataBuf[off    ] = a.x; edgeDataBuf[off + 1] = a.y; edgeDataBuf[off + 2] = a.z;
                edgeDataBuf[off + 3] = b.x; edgeDataBuf[off + 4] = b.y; edgeDataBuf[off + 5] = b.z;
                const pc = PATTERN_COLORS[e.patternIdx % PATTERN_COLORS.length]!;
                const highlighted = connectedEdges.has(`${e.from}-${e.to}-${e.patternIdx}`);
                const alpha = inter.selectedIdx >= 0 ? (highlighted ? 0.8 : 0.12) : 0.4;
                edgeDataBuf[off + 6] = pc[0]; edgeDataBuf[off + 7] = pc[1]; edgeDataBuf[off + 8] = pc[2];
                edgeDataBuf[off + 9] = alpha;
                edgeDataBuf[off + 10] = highlighted ? 1 : 0;
                edgeDataBuf[off + 11] = 0;
            }
            dev.queue.writeBuffer(edgeIB, 0, edgeDataBuf);

            // ── Update 3D particles ──
            if (inter.selectedIdx >= 0) {
                const sn = curLayout.nodeMap.get(inter.selectedIdx);
                if (sn) {
                    for (let i = 0; i < 4; i++) spawnBeam(particles, sn.x, sn.y, sn.z, sn.radius, time, MAX_BEAMS);
                }
            }
            if (inter.hoverIdx >= 0) {
                const hn = curLayout.nodeMap.get(inter.hoverIdx);
                if (hn) {
                    for (let i = 0; i < 6; i++) spawnGlitter(particles, hn.x, hn.y, hn.z, hn.radius, time, MAX_GLITTER);
                }
            }
            updateParticles3D(particles, dt, time);
            const liveCount = fillParticleBuffer(particles, particleDataBuf, MAX_PARTICLES);
            if (liveCount > 0) {
                dev.queue.writeBuffer(particleIB, 0, particleDataBuf, 0, liveCount * PARTICLE_INSTANCE_FLOATS);
            }

            // ── Camera + palette uniforms ──
            const camBuf = new Float32Array(32);
            camBuf.set(viewProj, 0);
            camBuf.set([camPos[0], camPos[1], camPos[2], 0], 16);
            camBuf.set([time, 0, 0, 0], 20);
            dev.queue.writeBuffer(camUB, 0, camBuf);
            dev.queue.writeBuffer(paletteUB, 0, buildPaletteBuffer(themeColors.value));

            // ── Draw grid, edges, particles ──
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

            if (liveCount > 0) {
                pass.setPipeline(particlePipeline);
                pass.setVertexBuffer(0, quadVB);
                pass.setVertexBuffer(1, particleIB);
                pass.setBindGroup(0, camBG);
                pass.draw(6, liveCount);
            }

            // Restore viewport/scissor to full canvas for subsequent callbacks
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
            particleIB.destroy();
        };
    }, [gpu, snapshot, getViewProj]);

    // ── Mouse interaction ──
    useEffect(() => {
        const container = containerRef.current;
        const layout = layoutRef.current;
        if (!container || !layout) return;

        const inter = interRef.current;

        const onMouseDown = (e: MouseEvent) => {
            const rect = container.getBoundingClientRect();
            inter.mouseX = e.clientX - rect.left;
            inter.mouseY = e.clientY - rect.top;
            inter.lastMX = e.clientX;
            inter.lastMY = e.clientY;

            const cw = container.clientWidth;
            const ch = container.clientHeight;

            if (e.button === 1 || (e.button === 0 && e.shiftKey)) {
                // Middle mouse or Shift+Left → pan
                inter.panning = true;
                camRef.current.focusTarget = null; // cancel any focus animation
                e.preventDefault();
            } else if (e.button === 0) {
                const { viewProj } = getViewProj(cw, ch);
                const invVP = mat4Inverse(viewProj);
                if (invVP) {
                    const ray = screenToRay(inter.mouseX, inter.mouseY, cw, ch, invVP);
                    let bestT = Infinity;
                    let bestIdx = -1;
                    for (const n of layout.nodes) {
                        const t = raySphere(ray.origin, ray.direction, [n.x, n.y, n.z], n.radius * 1.5);
                        if (t !== null && t < bestT) { bestT = t; bestIdx = n.index; }
                    }
                    if (bestIdx >= 0) {
                        const node = layout.nodeMap.get(bestIdx);
                        if (node) {
                            inter.dragIdx = bestIdx;
                            inter.dragPlaneY = node.y;
                            const pt = rayPlaneIntersect(ray, node.y);
                            if (pt) inter.dragOffset = [node.x - pt[0], 0, node.z - pt[2]];
                        }
                        e.preventDefault();
                    } else {
                        inter.orbiting = true;
                    }
                }
            } else if (e.button === 2) {
                inter.orbiting = true;
                e.preventDefault();
            }
        };

        const onMouseMove = (e: MouseEvent) => {
            const rect = container.getBoundingClientRect();
            inter.mouseX = e.clientX - rect.left;
            inter.mouseY = e.clientY - rect.top;
            const cw = container.clientWidth;
            const ch = container.clientHeight;

            if (inter.dragIdx >= 0) {
                const node = layout.nodeMap.get(inter.dragIdx);
                if (node) {
                    const { viewProj } = getViewProj(cw, ch);
                    const invVP = mat4Inverse(viewProj);
                    if (invVP) {
                        const ray = screenToRay(inter.mouseX, inter.mouseY, cw, ch, invVP);
                        const pt = rayPlaneIntersect(ray, inter.dragPlaneY);
                        if (pt) { node.x = pt[0] + inter.dragOffset[0]; node.z = pt[2] + inter.dragOffset[2]; }
                    }
                }
                return;
            }

            if (inter.panning) {
                const dx = e.clientX - inter.lastMX;
                const dy = e.clientY - inter.lastMY;
                inter.lastMX = e.clientX;
                inter.lastMY = e.clientY;

                // Move camera target along right/up vectors in world space
                const cam = camRef.current;
                const speed = cam.dist * 0.002;
                const cosY = Math.cos(cam.yaw);
                const sinY = Math.sin(cam.yaw);
                // Right vector (in XZ plane)
                const rx = cosY, rz = -sinY;
                // Up vector (projected): approximate as world Y
                cam.target = [
                    cam.target[0] - dx * speed * rx,
                    cam.target[1] + dy * speed,
                    cam.target[2] - dx * speed * rz,
                ];
                return;
            }

            if (inter.orbiting) {
                const dx = e.clientX - inter.lastMX;
                const dy = e.clientY - inter.lastMY;
                camRef.current.yaw += dx * 0.005;
                camRef.current.pitch = Math.max(-1.2, Math.min(1.2, camRef.current.pitch + dy * 0.005));
                inter.lastMX = e.clientX;
                inter.lastMY = e.clientY;
                return;
            }

            // Hover detection
            const { viewProj } = getViewProj(cw, ch);
            const invVP = mat4Inverse(viewProj);
            if (invVP) {
                const ray = screenToRay(inter.mouseX, inter.mouseY, cw, ch, invVP);
                let bestT = Infinity;
                let bestIdx = -1;
                for (const n of layout.nodes) {
                    const t = raySphere(ray.origin, ray.direction, [n.x, n.y, n.z], n.radius * 1.5);
                    if (t !== null && t < bestT) { bestT = t; bestIdx = n.index; }
                }
                if (bestIdx !== inter.hoverIdx) {
                    inter.hoverIdx = bestIdx;
                    setHoverIdx(bestIdx);
                    if (bestIdx >= 0) {
                        const n = layout.nodeMap.get(bestIdx);
                        if (n) setTooltip({ x: inter.mouseX, y: inter.mouseY, node: n });
                    } else {
                        setTooltip(null);
                    }
                } else if (bestIdx >= 0) {
                    const n = layout.nodeMap.get(bestIdx);
                    if (n) setTooltip({ x: inter.mouseX, y: inter.mouseY, node: n });
                }
            }
        };

        const onMouseUp = (e: MouseEvent) => {
            if (inter.dragIdx >= 0 && e.button === 0) {
                inter.selectedIdx = inter.dragIdx;
                setSelectedIdx(inter.dragIdx);
                // Focus camera on the selected node
                const node = layout.nodeMap.get(inter.dragIdx);
                if (node) {
                    camRef.current.focusTarget = [node.x, node.y, node.z];
                }
            }
            if (!inter.orbiting && !inter.panning && inter.dragIdx < 0 && e.button === 0) {
                inter.selectedIdx = -1;
                setSelectedIdx(-1);
                setTooltip(null);
            }
            inter.dragIdx = -1;
            inter.orbiting = false;
            inter.panning = false;
        };

        const onWheel = (e: WheelEvent) => {
            camRef.current.dist = Math.max(2, Math.min(80, camRef.current.dist + e.deltaY * 0.02));
            e.preventDefault();
        };

        const onCtx = (e: Event) => e.preventDefault();

        container.addEventListener('mousedown', onMouseDown);
        window.addEventListener('mousemove', onMouseMove);
        window.addEventListener('mouseup', onMouseUp);
        container.addEventListener('wheel', onWheel, { passive: false });
        container.addEventListener('contextmenu', onCtx);

        return () => {
            container.removeEventListener('mousedown', onMouseDown);
            window.removeEventListener('mousemove', onMouseMove);
            window.removeEventListener('mouseup', onMouseUp);
            container.removeEventListener('wheel', onWheel);
            container.removeEventListener('contextmenu', onCtx);
        };
    }, [snapshot, getViewProj]);

    // ── Render ──

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

    return (
        <div ref={containerRef} class="hypergraph-container hg-dom-mode">
            {/* DOM node layer — styled as log-entry elements for WgpuOverlay integration */}
            <div ref={nodeLayerRef} class="hg-node-layer">
                {layout?.nodes.map(n => {
                    const isSel = n.index === selectedIdx;
                    const isHov = n.index === hoverIdx;
                    const levelClass = nodeWidthClass(n.width, maxWidth);
                    return (
                        <div
                            key={n.index}
                            class={`log-entry hg-node ${levelClass} ${isSel ? 'selected' : ''} ${isHov ? 'span-highlighted' : ''} ${n.isAtom ? 'hg-atom' : 'hg-compound'}`}
                            data-node-idx={n.index}
                        >
                            <div class="hg-node-content">
                                <span class={`level-badge ${levelClass}`}>
                                    {n.isAtom ? 'ATOM' : `W${n.width}`}
                                </span>
                                <span class="hg-node-label">{n.label}</span>
                                <span class="hg-node-idx">#{n.index}</span>
                            </div>
                        </div>
                    );
                })}
            </div>

            {/* Info overlay */}
            <div class="hypergraph-info">
                <div class="hg-title">Hypergraph</div>
                <div class="hg-row">
                    <span class="hg-label">Nodes</span>
                    <span class="hg-value">{snapshot.nodes.length}</span>
                </div>
                <div class="hg-row">
                    <span class="hg-label">Edges</span>
                    <span class="hg-value">{snapshot.edges.length}</span>
                </div>
                <div class="hg-row">
                    <span class="hg-label">Atoms</span>
                    <span class="hg-value">{snapshot.nodes.filter(n => n.is_atom).length}</span>
                </div>
            </div>

            {/* Tooltip */}
            {tooltip && (
                <div
                    class="hypergraph-tooltip"
                    style={{ left: `${tooltip.x}px`, top: `${tooltip.y}px` }}
                >
                    <div class="tt-label">{tooltip.node.label}</div>
                    <div class="tt-detail">
                        idx={tooltip.node.index} width={tooltip.node.width}{' '}
                        {tooltip.node.isAtom ? '(atom)' : `(${tooltip.node.childIndices.length} children)`}
                    </div>
                </div>
            )}

            {/* HUD */}
            <div class="hypergraph-hud">
                <span>Left drag: Move nodes</span>
                <span>Right / Left empty: Orbit</span>
                <span>Middle / Shift+Left: Pan</span>
                <span>Scroll: Zoom</span>
                <span>Click node: Select &amp; Focus</span>
            </div>
        </div>
    );
}
