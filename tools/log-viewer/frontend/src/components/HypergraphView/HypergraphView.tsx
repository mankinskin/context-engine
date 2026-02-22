import { useRef, useEffect, useState } from 'preact/hooks';
import type { HypergraphSnapshot } from '../../types';
import { hypergraphSnapshot, selectedEntry } from '../../store';
import shaderSource from './hypergraph.wgsl?raw';
import './hypergraph.css';
import { buildLayout, type GraphLayout, type LayoutNode } from './layout';
import {
    Vec3,
    mat4Perspective, mat4LookAt, mat4Multiply, mat4Inverse,
    screenToRay, rayPlaneIntersect, vec3Normalize,
} from '../Scene3D/math3d';

// ── constants ──

const QUAD_VERTS = new Float32Array([
    -1, -1,   1, -1,   1, 1,
    -1, -1,   1,  1,  -1, 1,
]);

const NODE_INSTANCE_FLOATS = 12;   // center(3) + radius(1) + color(4) + flags(4)
const EDGE_INSTANCE_FLOATS = 12;   // posA(3) + posB(3) + color(4) + flags(1) + pad(1)

// ── ray-sphere intersection ──

function raySphere(
    ro: Vec3, rd: Vec3, center: Vec3, radius: number,
): number | null {
    const oc: Vec3 = [ro[0] - center[0], ro[1] - center[1], ro[2] - center[2]];
    const a = rd[0] * rd[0] + rd[1] * rd[1] + rd[2] * rd[2];
    const b = 2 * (oc[0] * rd[0] + oc[1] * rd[1] + oc[2] * rd[2]);
    const c = oc[0] * oc[0] + oc[1] * oc[1] + oc[2] * oc[2] - radius * radius;
    const disc = b * b - 4 * a * c;
    if (disc < 0) return null;
    const t = (-b - Math.sqrt(disc)) / (2 * a);
    return t > 0 ? t : null;
}

// ── component ──

export function HypergraphView() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const containerRef = useRef<HTMLDivElement>(null);
    const tooltipRef = useRef<HTMLDivElement>(null);
    const [tooltip, setTooltip] = useState<{ x: number; y: number; node: LayoutNode } | null>(null);

    const snapshot = hypergraphSnapshot.value;

    useEffect(() => {
        const canvas = canvasRef.current;
        const container = containerRef.current;
        if (!canvas || !container || !snapshot) return;

        let destroyed = false;
        let animId = 0;
        const cleanups: (() => void)[] = [];

        const run = async () => {
            // ── build layout ──
            const layout = buildLayout(snapshot);
            if (layout.nodes.length === 0) return;

            // ── WebGPU init ──
            if (!navigator.gpu) return;
            const adapter = await navigator.gpu.requestAdapter();
            if (!adapter || destroyed) return;
            const device = await adapter.requestDevice();
            if (destroyed) { device.destroy(); return; }
            cleanups.push(() => device.destroy());

            const ctx = canvas.getContext('webgpu');
            if (!ctx) return;
            const format = navigator.gpu.getPreferredCanvasFormat();
            ctx.configure({ device, format, alphaMode: 'opaque' });

            // ── shader ──
            const shader = device.createShaderModule({ code: shaderSource });

            // ── quad vertex buffer ──
            const quadVB = device.createBuffer({
                size: QUAD_VERTS.byteLength,
                usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
            });
            device.queue.writeBuffer(quadVB, 0, QUAD_VERTS);

            // ── camera uniform ──
            const camUB = device.createBuffer({
                size: 128,   // mat4(64) + vec4(16) + vec4(16) = 96, pad to 128
                usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            });

            const camBGL = device.createBindGroupLayout({
                entries: [{
                    binding: 0,
                    visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                    buffer: { type: 'uniform' },
                }],
            });

            const camBG = device.createBindGroup({
                layout: camBGL,
                entries: [{ binding: 0, resource: { buffer: camUB } }],
            });

            const pipelineLayout = device.createPipelineLayout({
                bindGroupLayouts: [camBGL],
            });

            // ── node pipeline ──
            const nodePipeline = device.createRenderPipeline({
                layout: pipelineLayout,
                vertex: {
                    module: shader,
                    entryPoint: 'vs_node',
                    buffers: [
                        {   // quad
                            arrayStride: 8,
                            stepMode: 'vertex',
                            attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' as GPUVertexFormat }],
                        },
                        {   // instance
                            arrayStride: NODE_INSTANCE_FLOATS * 4,
                            stepMode: 'instance',
                            attributes: [
                                { shaderLocation: 2, offset: 0,  format: 'float32x3' as GPUVertexFormat },  // center
                                { shaderLocation: 3, offset: 12, format: 'float32'   as GPUVertexFormat },  // radius
                                { shaderLocation: 4, offset: 16, format: 'float32x4' as GPUVertexFormat },  // color
                                { shaderLocation: 5, offset: 32, format: 'float32x4' as GPUVertexFormat },  // flags
                            ],
                        },
                    ],
                },
                fragment: {
                    module: shader,
                    entryPoint: 'fs_node',
                    targets: [{
                        format,
                        blend: {
                            color: { srcFactor: 'src-alpha', dstFactor: 'one-minus-src-alpha', operation: 'add' },
                            alpha: { srcFactor: 'one', dstFactor: 'one-minus-src-alpha', operation: 'add' },
                        },
                    }],
                },
                primitive: { topology: 'triangle-list' },
                depthStencil: {
                    format: 'depth24plus',
                    depthWriteEnabled: true,
                    depthCompare: 'less',
                },
            });

            // ── edge pipeline ──
            const edgePipeline = device.createRenderPipeline({
                layout: pipelineLayout,
                vertex: {
                    module: shader,
                    entryPoint: 'vs_edge',
                    buffers: [
                        {   // quad (reuse)
                            arrayStride: 8,
                            stepMode: 'vertex',
                            attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' as GPUVertexFormat }],
                        },
                        {   // instance
                            arrayStride: EDGE_INSTANCE_FLOATS * 4,
                            stepMode: 'instance',
                            attributes: [
                                { shaderLocation: 6, offset: 0,  format: 'float32x3' as GPUVertexFormat },  // posA
                                { shaderLocation: 7, offset: 12, format: 'float32x3' as GPUVertexFormat },  // posB
                                { shaderLocation: 8, offset: 24, format: 'float32x4' as GPUVertexFormat },  // color
                                { shaderLocation: 9, offset: 40, format: 'float32'   as GPUVertexFormat },  // flags
                            ],
                        },
                    ],
                },
                fragment: {
                    module: shader,
                    entryPoint: 'fs_edge',
                    targets: [{
                        format,
                        blend: {
                            color: { srcFactor: 'src-alpha', dstFactor: 'one-minus-src-alpha', operation: 'add' },
                            alpha: { srcFactor: 'one', dstFactor: 'one-minus-src-alpha', operation: 'add' },
                        },
                    }],
                },
                primitive: { topology: 'triangle-list' },
                depthStencil: {
                    format: 'depth24plus',
                    depthWriteEnabled: false,
                    depthCompare: 'less',
                },
            });

            // ── instance buffers ──
            const maxNodes = layout.nodes.length;
            const maxEdges = layout.edges.length;

            const nodeIB = device.createBuffer({
                size: Math.max(maxNodes * NODE_INSTANCE_FLOATS * 4, 48),
                usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
            });

            const edgeIB = device.createBuffer({
                size: Math.max(maxEdges * EDGE_INSTANCE_FLOATS * 4, 48),
                usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
            });

            // ── depth buffer ──
            let depthTex: GPUTexture | null = null;
            let depthView: GPUTextureView | null = null;
            let lastW = 0, lastH = 0;

            function ensureDepth(w: number, h: number) {
                if (w === lastW && h === lastH && depthTex) return;
                depthTex?.destroy();
                depthTex = device.createTexture({
                    size: [w, h],
                    format: 'depth24plus',
                    usage: GPUTextureUsage.RENDER_ATTACHMENT,
                });
                depthView = depthTex.createView();
                lastW = w; lastH = h;
            }

            // ── camera state ──
            let camYaw = 0.5;
            let camPitch = 0.4;
            let camDist = Math.max(6, layout.nodes.length * 0.5);
            let camTargetY = (layout.maxWidth - 1) * 0.75;
            const camTarget: Vec3 = [0, camTargetY, 0];

            function getCamPos(): Vec3 {
                return [
                    camTarget[0] + camDist * Math.cos(camPitch) * Math.sin(camYaw),
                    camTarget[1] + camDist * Math.sin(camPitch),
                    camTarget[2] + camDist * Math.cos(camPitch) * Math.cos(camYaw),
                ];
            }

            // ── interaction state ──
            let selectedIdx = -1;
            let hoverIdx = -1;
            let dragIdx = -1;
            let dragPlaneY = 0;
            let dragOffset: Vec3 = [0, 0, 0];
            let orbiting = false;
            let lastMX = 0, lastMY = 0;
            let mouseX = 0, mouseY = 0;

            function getViewProj(): { viewProj: Float32Array; camPos: Vec3 } {
                const camPos = getCamPos();
                const view = mat4LookAt(camPos, camTarget, [0, 1, 0]);
                const proj = mat4Perspective(
                    Math.PI / 4,
                    canvas.clientWidth / Math.max(canvas.clientHeight, 1),
                    0.1, 200,
                );
                return { viewProj: mat4Multiply(proj, view), camPos };
            }

            function pickNode(sx: number, sy: number): number {
                const { viewProj, camPos } = getViewProj();
                const invVP = mat4Inverse(viewProj);
                if (!invVP) return -1;
                const ray = screenToRay(sx, sy, canvas.clientWidth, canvas.clientHeight, invVP);

                let bestT = Infinity;
                let bestIdx = -1;
                for (const n of layout.nodes) {
                    const t = raySphere(ray.origin, ray.direction, [n.x, n.y, n.z], n.radius);
                    if (t !== null && t < bestT) {
                        bestT = t;
                        bestIdx = n.index;
                    }
                }
                return bestIdx;
            }

            // ── mouse handlers ──

            const onMouseDown = (e: MouseEvent) => {
                const rect = canvas.getBoundingClientRect();
                mouseX = e.clientX - rect.left;
                mouseY = e.clientY - rect.top;
                lastMX = e.clientX;
                lastMY = e.clientY;

                if (e.button === 0) {
                    const hit = pickNode(mouseX, mouseY);
                    if (hit >= 0) {
                        const node = layout.nodeMap.get(hit);
                        if (node) {
                            dragIdx = hit;
                            dragPlaneY = node.y;
                            const { viewProj } = getViewProj();
                            const invVP = mat4Inverse(viewProj);
                            if (invVP) {
                                const ray = screenToRay(mouseX, mouseY, canvas.clientWidth, canvas.clientHeight, invVP);
                                const pt = rayPlaneIntersect(ray, dragPlaneY);
                                if (pt) {
                                    dragOffset = [node.x - pt[0], 0, node.z - pt[2]];
                                }
                            }
                        }
                        e.preventDefault();
                    } else {
                        // Click empty → deselect or start orbit
                        orbiting = true;
                    }
                } else if (e.button === 2) {
                    orbiting = true;
                    e.preventDefault();
                }
            };

            const onMouseMove = (e: MouseEvent) => {
                const rect = canvas.getBoundingClientRect();
                mouseX = e.clientX - rect.left;
                mouseY = e.clientY - rect.top;

                if (dragIdx >= 0) {
                    const node = layout.nodeMap.get(dragIdx);
                    if (node) {
                        const { viewProj } = getViewProj();
                        const invVP = mat4Inverse(viewProj);
                        if (invVP) {
                            const ray = screenToRay(mouseX, mouseY, canvas.clientWidth, canvas.clientHeight, invVP);
                            const pt = rayPlaneIntersect(ray, dragPlaneY);
                            if (pt) {
                                node.x = pt[0] + dragOffset[0];
                                node.z = pt[2] + dragOffset[2];
                            }
                        }
                    }
                    return;
                }

                if (orbiting) {
                    const dx = e.clientX - lastMX;
                    const dy = e.clientY - lastMY;
                    camYaw += dx * 0.005;
                    camPitch = Math.max(-1.2, Math.min(1.2, camPitch + dy * 0.005));
                    lastMX = e.clientX;
                    lastMY = e.clientY;
                }
            };

            const onMouseUp = (e: MouseEvent) => {
                if (dragIdx >= 0 && e.button === 0) {
                    // Select the node on release (if barely moved)
                    const rect = canvas.getBoundingClientRect();
                    const sx = e.clientX - rect.left;
                    const sy = e.clientY - rect.top;
                    const moved = Math.abs(sx - mouseX) + Math.abs(sy - mouseY);
                    // We always select on drag release for simplicity
                    selectedIdx = dragIdx;
                }
                if (!orbiting && dragIdx < 0 && e.button === 0) {
                    // Clicked empty space
                    selectedIdx = -1;
                    setTooltip(null);
                }
                dragIdx = -1;
                orbiting = false;
            };

            const onWheel = (e: WheelEvent) => {
                camDist = Math.max(2, Math.min(80, camDist + e.deltaY * 0.02));
                e.preventDefault();
            };

            const onCtx = (e: Event) => e.preventDefault();

            canvas.addEventListener('mousedown', onMouseDown);
            window.addEventListener('mousemove', onMouseMove);
            window.addEventListener('mouseup', onMouseUp);
            canvas.addEventListener('wheel', onWheel, { passive: false });
            canvas.addEventListener('contextmenu', onCtx);

            cleanups.push(() => {
                canvas.removeEventListener('mousedown', onMouseDown);
                window.removeEventListener('mousemove', onMouseMove);
                window.removeEventListener('mouseup', onMouseUp);
                canvas.removeEventListener('wheel', onWheel);
                canvas.removeEventListener('contextmenu', onCtx);
            });

            // ── render loop ──

            const t0 = performance.now() / 1000;
            const nodeData = new Float32Array(maxNodes * NODE_INSTANCE_FLOATS);
            const edgeData = new Float32Array(maxEdges * EDGE_INSTANCE_FLOATS);

            // Precompute edge colors from pattern index
            const PATTERN_COLORS: [number, number, number][] = [
                [0.45, 0.55, 0.7],
                [0.7, 0.45, 0.55],
                [0.5, 0.7, 0.45],
                [0.65, 0.55, 0.7],
                [0.7, 0.65, 0.4],
                [0.4, 0.7, 0.65],
            ];

            function frame() {
                if (destroyed) return;
                animId = requestAnimationFrame(frame);

                // resize
                const dpr = window.devicePixelRatio || 1;
                const cw = container.clientWidth;
                const ch = container.clientHeight;
                const pw = Math.max(1, Math.floor(cw * dpr));
                const ph = Math.max(1, Math.floor(ch * dpr));
                if (canvas.width !== pw || canvas.height !== ph) {
                    canvas.width = pw;
                    canvas.height = ph;
                }
                ensureDepth(pw, ph);

                const time = performance.now() / 1000 - t0;
                const { viewProj, camPos } = getViewProj();

                // Hover detection
                if (dragIdx < 0 && !orbiting) {
                    const newHover = pickNode(mouseX, mouseY);
                    if (newHover !== hoverIdx) {
                        hoverIdx = newHover;
                        if (hoverIdx >= 0) {
                            const n = layout.nodeMap.get(hoverIdx);
                            if (n) {
                                setTooltip({ x: mouseX, y: mouseY, node: n });
                            }
                        } else {
                            setTooltip(null);
                        }
                    }
                    // Update tooltip position
                    if (hoverIdx >= 0) {
                        const n = layout.nodeMap.get(hoverIdx);
                        if (n) setTooltip({ x: mouseX, y: mouseY, node: n });
                    }
                }

                // Cursor
                if (dragIdx >= 0) canvas.style.cursor = 'grabbing';
                else if (hoverIdx >= 0) canvas.style.cursor = 'grab';
                else canvas.style.cursor = 'default';

                // Connected set for selected node
                const connectedSet = new Set<number>();
                const connectedEdges = new Set<string>();
                if (selectedIdx >= 0) {
                    connectedSet.add(selectedIdx);
                    const sel = layout.nodeMap.get(selectedIdx);
                    if (sel) {
                        for (const ci of sel.childIndices) connectedSet.add(ci);
                        for (const pi of sel.parentIndices) connectedSet.add(pi);
                    }
                    for (const e of layout.edges) {
                        if (e.from === selectedIdx || e.to === selectedIdx) {
                            connectedEdges.add(`${e.from}-${e.to}-${e.patternIdx}`);
                        }
                    }
                }

                // ── fill node instances ──
                for (let i = 0; i < layout.nodes.length; i++) {
                    const n = layout.nodes[i]!;
                    const off = i * NODE_INSTANCE_FLOATS;
                    nodeData[off + 0] = n.x;
                    nodeData[off + 1] = n.y;
                    nodeData[off + 2] = n.z;
                    nodeData[off + 3] = n.radius;
                    nodeData[off + 4] = n.color[0];
                    nodeData[off + 5] = n.color[1];
                    nodeData[off + 6] = n.color[2];
                    nodeData[off + 7] = n.color[3];
                    // flags: selected, hovered, isAtom, dimmed
                    const isSel = n.index === selectedIdx ? 1 : 0;
                    const isHov = n.index === hoverIdx ? 1 : 0;
                    const isAtom = n.isAtom ? 1 : 0;
                    const dimmed = selectedIdx >= 0 && !connectedSet.has(n.index) ? 1 : 0;
                    nodeData[off + 8]  = isSel;
                    nodeData[off + 9]  = isHov;
                    nodeData[off + 10] = isAtom;
                    nodeData[off + 11] = dimmed;
                }
                device.queue.writeBuffer(nodeIB, 0, nodeData);

                // ── fill edge instances ──
                for (let i = 0; i < layout.edges.length; i++) {
                    const e = layout.edges[i]!;
                    const a = layout.nodeMap.get(e.from);
                    const b = layout.nodeMap.get(e.to);
                    if (!a || !b) continue;

                    const off = i * EDGE_INSTANCE_FLOATS;
                    edgeData[off + 0] = a.x;
                    edgeData[off + 1] = a.y;
                    edgeData[off + 2] = a.z;
                    edgeData[off + 3] = b.x;
                    edgeData[off + 4] = b.y;
                    edgeData[off + 5] = b.z;

                    const pc = PATTERN_COLORS[e.patternIdx % PATTERN_COLORS.length]!;
                    const highlighted = connectedEdges.has(`${e.from}-${e.to}-${e.patternIdx}`);
                    const alpha = selectedIdx >= 0 ? (highlighted ? 0.8 : 0.12) : 0.4;
                    edgeData[off + 6] = pc[0];
                    edgeData[off + 7] = pc[1];
                    edgeData[off + 8] = pc[2];
                    edgeData[off + 9] = alpha;
                    edgeData[off + 10] = highlighted ? 1 : 0;
                    edgeData[off + 11] = 0;
                }
                device.queue.writeBuffer(edgeIB, 0, edgeData);

                // ── camera uniform ──
                const camBuf = new Float32Array(32);  // 128 bytes = 32 floats
                camBuf.set(viewProj, 0);
                camBuf.set([camPos[0], camPos[1], camPos[2], 0], 16);
                camBuf.set([time, 0, 0, 0], 20);
                device.queue.writeBuffer(camUB, 0, camBuf);

                // ── draw ──
                const encoder = device.createCommandEncoder();
                const pass = encoder.beginRenderPass({
                    colorAttachments: [{
                        view: ctx.getCurrentTexture().createView(),
                        clearValue: { r: 0.04, g: 0.04, b: 0.06, a: 1 },
                        loadOp: 'clear',
                        storeOp: 'store',
                    }],
                    depthStencilAttachment: {
                        view: depthView!,
                        depthClearValue: 1,
                        depthLoadOp: 'clear',
                        depthStoreOp: 'store',
                    },
                });

                // Draw edges first (behind)
                pass.setPipeline(edgePipeline);
                pass.setVertexBuffer(0, quadVB);
                pass.setVertexBuffer(1, edgeIB);
                pass.setBindGroup(0, camBG);
                pass.draw(6, layout.edges.length);

                // Draw nodes
                pass.setPipeline(nodePipeline);
                pass.setVertexBuffer(0, quadVB);
                pass.setVertexBuffer(1, nodeIB);
                pass.setBindGroup(0, camBG);
                pass.draw(6, layout.nodes.length);

                pass.end();
                device.queue.submit([encoder.finish()]);
            }

            frame();
        };

        run();

        return () => {
            destroyed = true;
            cancelAnimationFrame(animId);
            cleanups.forEach(fn => fn());
        };
    }, [snapshot]);

    if (!snapshot) {
        return (
            <div class="hypergraph-container">
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

    return (
        <div ref={containerRef} class="hypergraph-container">
            <canvas ref={canvasRef} class="hypergraph-canvas" />

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

            {tooltip && (
                <div
                    ref={tooltipRef}
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

            <div class="hypergraph-hud">
                <span>Left drag: Move nodes</span>
                <span>Right drag / Left empty: Orbit</span>
                <span>Scroll: Zoom</span>
                <span>Click node: Select</span>
            </div>
        </div>
    );
}
