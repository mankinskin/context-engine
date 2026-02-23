import { useRef, useEffect } from 'preact/hooks';
import shaderSource from './scene3d.wgsl?raw';
import './scene3d.css';
import {
    Vec3, Mat4,
    vec3Sub, vec3Add, vec3Scale, vec3Normalize,
    mat4Perspective, mat4LookAt, mat4Multiply,
    mat4Translate, mat4ScaleMat, mat4RotateY, mat4Inverse,
    mat4TransformPoint, mat4TransformDir,
    screenToRay, rayAABBIntersect, rayPlaneIntersect,
    Ray,
} from './math3d';

// ── types ──

interface SceneObject {
    position: Vec3;
    scale: Vec3;
    color: [number, number, number, number];
    rotationY: number;
}

// ── geometry ──

/** Unit cube (−0.5 → 0.5), 36 verts, interleaved pos+normal (6 floats each) */
function createCubeGeometry(): Float32Array {
    const faces: { n: Vec3; v: Vec3[] }[] = [
        { n: [0,0,1],  v: [[-0.5,-0.5,0.5],[0.5,-0.5,0.5],[0.5,0.5,0.5],[-0.5,0.5,0.5]] },       // +Z
        { n: [0,0,-1], v: [[0.5,-0.5,-0.5],[-0.5,-0.5,-0.5],[-0.5,0.5,-0.5],[0.5,0.5,-0.5]] },    // −Z
        { n: [1,0,0],  v: [[0.5,-0.5,0.5],[0.5,-0.5,-0.5],[0.5,0.5,-0.5],[0.5,0.5,0.5]] },         // +X
        { n: [-1,0,0], v: [[-0.5,-0.5,-0.5],[-0.5,-0.5,0.5],[-0.5,0.5,0.5],[-0.5,0.5,-0.5]] },    // −X
        { n: [0,1,0],  v: [[-0.5,0.5,0.5],[0.5,0.5,0.5],[0.5,0.5,-0.5],[-0.5,0.5,-0.5]] },         // +Y
        { n: [0,-1,0], v: [[-0.5,-0.5,-0.5],[0.5,-0.5,-0.5],[0.5,-0.5,0.5],[-0.5,-0.5,0.5]] },     // −Y
    ];
    const d: number[] = [];
    for (const { n, v: [a, b, c, e] } of faces) {
        d.push(a[0],a[1],a[2], n[0],n[1],n[2]);
        d.push(b[0],b[1],b[2], n[0],n[1],n[2]);
        d.push(c[0],c[1],c[2], n[0],n[1],n[2]);
        d.push(a[0],a[1],a[2], n[0],n[1],n[2]);
        d.push(c[0],c[1],c[2], n[0],n[1],n[2]);
        d.push(e[0],e[1],e[2], n[0],n[1],n[2]);
    }
    return new Float32Array(d);
}

// ── constants ──

const UNIFORM_STRIDE = 256;   // WebGPU minUniformBufferOffsetAlignment
const MAX_DRAWS = 16;
const CUBE_VERTS = 36;

const INITIAL_OBJECTS: SceneObject[] = [
    { position: [0, 0.5, 0],       scale: [1, 1, 1],       color: [0.95, 0.25, 0.21, 1], rotationY: 0 },
    { position: [2.5, 1.0, 0.8],   scale: [0.5, 2.0, 0.5], color: [0.18, 0.60, 0.95, 1], rotationY: 0.2 },
    { position: [-2.2, 0.2, -1.2], scale: [1.8, 0.4, 1.5], color: [0.22, 0.88, 0.38, 1], rotationY: 0.5 },
    { position: [1.2, 0.25, -2.3], scale: [0.5, 0.5, 0.5], color: [0.98, 0.85, 0.15, 1], rotationY: 0.8 },
    { position: [-1.5, 0.5, 2.2],  scale: [1, 1, 1],       color: [0.72, 0.32, 0.95, 1], rotationY: 1.1 },
    { position: [3.2, 0.45, -2.0], scale: [0.9, 0.9, 1.4], color: [0.15, 0.85, 0.82, 1], rotationY: 0.4 },
];

// ── component ──

export function Scene3D() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const containerRef = useRef<HTMLDivElement>(null);
    const resetRef = useRef<(() => void) | null>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        const container = containerRef.current;
        if (!canvas || !container) return;

        let destroyed = false;
        let animId = 0;
        const cleanups: (() => void)[] = [];

        const run = async () => {
            // ── WebGPU init ──
            if (!navigator.gpu) {
                const msg = document.createElement('div');
                msg.className = 'scene3d-error';
                msg.textContent = 'WebGPU is not supported in this browser';
                container.appendChild(msg);
                cleanups.push(() => msg.remove());
                return;
            }

            const adapter = await navigator.gpu.requestAdapter();
            if (!adapter || destroyed) return;
            const device = await adapter.requestDevice();
            if (destroyed) { device.destroy(); return; }
            cleanups.push(() => device.destroy());

            const ctx = canvas.getContext('webgpu');
            if (!ctx) return;

            const format = navigator.gpu.getPreferredCanvasFormat();
            ctx.configure({ device, format, alphaMode: 'opaque' });

            // ── shaders & pipeline ──
            const shader = device.createShaderModule({ code: shaderSource });

            const cubeVerts = createCubeGeometry();
            const vb = device.createBuffer({
                size: cubeVerts.byteLength,
                usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
            });
            device.queue.writeBuffer(vb, 0, cubeVerts);

            const ub = device.createBuffer({
                size: UNIFORM_STRIDE * MAX_DRAWS,
                usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            });

            const bgl = device.createBindGroupLayout({
                entries: [{
                    binding: 0,
                    visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
                    buffer: { type: 'uniform', hasDynamicOffset: true, minBindingSize: 192 },
                }],
            });

            const bg = device.createBindGroup({
                layout: bgl,
                entries: [{ binding: 0, resource: { buffer: ub, size: 192 } }],
            });

            const pipeline = device.createRenderPipeline({
                layout: device.createPipelineLayout({ bindGroupLayouts: [bgl] }),
                vertex: {
                    module: shader,
                    entryPoint: 'vs_main',
                    buffers: [{
                        arrayStride: 24,
                        attributes: [
                            { shaderLocation: 0, offset: 0, format: 'float32x3' as GPUVertexFormat },
                            { shaderLocation: 1, offset: 12, format: 'float32x3' as GPUVertexFormat },
                        ],
                    }],
                },
                fragment: {
                    module: shader,
                    entryPoint: 'fs_main',
                    targets: [{ format }],
                },
                depthStencil: {
                    format: 'depth24plus',
                    depthWriteEnabled: true,
                    depthCompare: 'less',
                },
                primitive: { topology: 'triangle-list', cullMode: 'back' },
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

            // ── scene state ──
            const objects: SceneObject[] = INITIAL_OBJECTS.map(o => ({
                ...o,
                position: [...o.position] as Vec3,
                scale: [...o.scale] as Vec3,
                color: [...o.color] as [number, number, number, number],
            }));

            // camera (spherical)
            let camYaw = 0.6;
            let camPitch = 0.35;
            let camDist = 10;
            const camTarget: Vec3 = [0, 0.5, 0];

            // interaction
            let dragIdx = -1;
            let dragPlaneY = 0;
            let dragOffset: Vec3 = [0, 0, 0];
            let orbiting = false;
            let lastMX = 0, lastMY = 0;
            let hoverIdx = -1;
            let mouseX = 0, mouseY = 0;

            function getCamPos(): Vec3 {
                return [
                    camTarget[0] + camDist * Math.cos(camPitch) * Math.sin(camYaw),
                    camTarget[1] + camDist * Math.sin(camPitch),
                    camTarget[2] + camDist * Math.cos(camPitch) * Math.cos(camYaw),
                ];
            }

            function getModelMatrix(obj: SceneObject, time: number, idx: number): Mat4 {
                const isDragged = idx === dragIdx;
                const bob = isDragged ? 0 : Math.sin(time * 1.2 + obj.position[0] * 3 + obj.position[2] * 2) * 0.04;
                const lift = isDragged ? 0.2 : 0;
                const t = mat4Translate([obj.position[0], obj.position[1] + bob + lift, obj.position[2]]);
                const r = mat4RotateY(obj.rotationY);
                const s = mat4ScaleMat(obj.scale);
                return mat4Multiply(mat4Multiply(t, r), s);
            }

            // ── picking ──

            function pickObject(
                sx: number, sy: number, w: number, h: number,
                viewProj: Mat4, time: number,
            ): { index: number; t: number } | null {
                const invVP = mat4Inverse(viewProj);
                if (!invVP) return null;
                const ray = screenToRay(sx, sy, w, h, invVP);

                let bestT = Infinity;
                let bestIdx = -1;

                for (let i = 0; i < objects.length; i++) {
                    const model = getModelMatrix(objects[i], time, i);
                    const invModel = mat4Inverse(model);
                    if (!invModel) continue;

                    const localRay: Ray = {
                        origin: mat4TransformPoint(invModel, ray.origin),
                        direction: mat4TransformDir(invModel, ray.direction),
                    };
                    const t = rayAABBIntersect(localRay, [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]);
                    if (t !== null && t < bestT) { bestT = t; bestIdx = i; }
                }

                return bestIdx >= 0 ? { index: bestIdx, t: bestT } : null;
            }

            function getViewProj(): Mat4 {
                const camPos = getCamPos();
                const view = mat4LookAt(camPos, camTarget, [0, 1, 0]);
                const proj = mat4Perspective(Math.PI / 4, canvas.clientWidth / canvas.clientHeight, 0.1, 100);
                return mat4Multiply(proj, view);
            }

            // ── mouse handlers ──

            const onMouseDown = (e: MouseEvent) => {
                const rect = canvas.getBoundingClientRect();
                const sx = e.clientX - rect.left;
                const sy = e.clientY - rect.top;
                lastMX = e.clientX;
                lastMY = e.clientY;

                if (e.button === 0) {
                    const time = performance.now() / 1000 - t0;
                    const vp = getViewProj();
                    const hit = pickObject(sx, sy, canvas.clientWidth, canvas.clientHeight, vp, time);
                    if (hit) {
                        dragIdx = hit.index;
                        dragPlaneY = objects[hit.index].position[1];
                        const invVP = mat4Inverse(vp)!;
                        const ray = screenToRay(sx, sy, canvas.clientWidth, canvas.clientHeight, invVP);
                        const pt = rayPlaneIntersect(ray, dragPlaneY);
                        if (pt) {
                            dragOffset = [
                                objects[hit.index].position[0] - pt[0],
                                0,
                                objects[hit.index].position[2] - pt[2],
                            ];
                        }
                        e.preventDefault();
                    } else {
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
                    const vp = getViewProj();
                    const invVP = mat4Inverse(vp);
                    if (invVP) {
                        const ray = screenToRay(mouseX, mouseY, canvas.clientWidth, canvas.clientHeight, invVP);
                        const pt = rayPlaneIntersect(ray, dragPlaneY);
                        if (pt) {
                            objects[dragIdx].position[0] = pt[0] + dragOffset[0];
                            objects[dragIdx].position[2] = pt[2] + dragOffset[2];
                        }
                    }
                    return;
                }

                if (orbiting) {
                    const dx = e.clientX - lastMX;
                    const dy = e.clientY - lastMY;
                    camYaw += dx * 0.005;
                    camPitch = Math.max(-1.4, Math.min(1.4, camPitch + dy * 0.005));
                    lastMX = e.clientX;
                    lastMY = e.clientY;
                }
            };

            const onMouseUp = () => { dragIdx = -1; orbiting = false; };

            const onWheel = (e: WheelEvent) => {
                camDist = Math.max(3, Math.min(30, camDist + e.deltaY * 0.01));
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

            // ── reset ──

            resetRef.current = () => {
                INITIAL_OBJECTS.forEach((init, i) => {
                    objects[i].position = [...init.position] as Vec3;
                });
                camYaw = 0.6; camPitch = 0.35; camDist = 10;
            };

            // ── render loop ──

            const uniformBuf = new ArrayBuffer(UNIFORM_STRIDE * MAX_DRAWS);
            const t0 = performance.now() / 1000;
            const lightDir: Vec3 = vec3Normalize([0.5, 0.8, 0.3]);

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
                const camPos = getCamPos();
                const view = mat4LookAt(camPos, camTarget, [0, 1, 0]);
                const proj = mat4Perspective(Math.PI / 4, pw / ph, 0.1, 100);
                const viewProj = mat4Multiply(proj, view);

                // hover detection
                if (dragIdx < 0 && !orbiting) {
                    const hit = pickObject(mouseX, mouseY, cw, ch, viewProj, time);
                    hoverIdx = hit ? hit.index : -1;
                }

                // cursor
                if (dragIdx >= 0) canvas.style.cursor = 'grabbing';
                else if (hoverIdx >= 0) canvas.style.cursor = 'grab';
                else canvas.style.cursor = 'default';

                // ── fill uniform data ──
                let draws = 0;

                // ground plane
                {
                    const model = mat4Multiply(mat4Translate([0, 0, 0]), mat4ScaleMat([24, 0.01, 24]));
                    const fv = new Float32Array(uniformBuf, UNIFORM_STRIDE * draws, 48);
                    fv.set(viewProj, 0);
                    fv.set(model, 16);
                    fv.set([0.1, 0.1, 0.12, 1], 32);
                    fv.set([lightDir[0], lightDir[1], lightDir[2], 0], 36);
                    fv.set([camPos[0], camPos[1], camPos[2], 0], 40);
                    fv.set([1, 0, time, 0], 44);  // isGround=1
                    draws++;
                }

                // objects
                for (let i = 0; i < objects.length; i++) {
                    const obj = objects[i];
                    const model = getModelMatrix(obj, time, i);
                    const fv = new Float32Array(uniformBuf, UNIFORM_STRIDE * draws, 48);
                    fv.set(viewProj, 0);
                    fv.set(model, 16);
                    fv.set(obj.color, 32);
                    fv.set([lightDir[0], lightDir[1], lightDir[2], 0], 36);
                    fv.set([camPos[0], camPos[1], camPos[2], 0], 40);
                    fv.set([0, hoverIdx === i ? 1 : 0, time, dragIdx === i ? 1 : 0], 44);
                    draws++;
                }

                device.queue.writeBuffer(ub, 0, new Uint8Array(uniformBuf), 0, UNIFORM_STRIDE * draws);

                // ── draw ──
                const encoder = device.createCommandEncoder();
                const pass = encoder.beginRenderPass({
                    colorAttachments: [{
                        view: ctx.getCurrentTexture().createView(),
                        clearValue: { r: 0.06, g: 0.06, b: 0.08, a: 1 },
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

                pass.setPipeline(pipeline);
                pass.setVertexBuffer(0, vb);
                for (let i = 0; i < draws; i++) {
                    pass.setBindGroup(0, bg, [UNIFORM_STRIDE * i]);
                    pass.draw(CUBE_VERTS);
                }

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
    }, []);

    return (
        <div ref={containerRef} class="scene3d-container">
            <canvas ref={canvasRef} class="scene3d-canvas" />
            <div class="scene3d-hud">
                <span>Left drag: Move objects</span>
                <span>Right drag: Orbit</span>
                <span>Scroll: Zoom</span>
                <button class="scene3d-reset" onClick={() => resetRef.current?.()}>
                    Reset
                </button>
            </div>
        </div>
    );
}
