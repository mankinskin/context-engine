/// <reference types="@webgpu/types" />
/**
 * WgpuOverlay — GPU-accelerated canvas overlay using WebGPU.
 *
 * WebGPU is the browser counterpart of wgpu (Rust GPU library).
 * When wgpu is compiled to WebAssembly it targets WebGPU/WebGL2,
 * using the exact same WGSL shader language shown here.
 *
 * The overlay renders an animated aurora effect with WGSL shaders
 * on a fixed canvas that floats above all DOM elements
 * (pointer-events: none ensures normal UI interaction).
 */
import { useEffect, useRef } from 'preact/hooks';
import { signal } from '@preact/signals';

export const gpuOverlayEnabled = signal(false);

// ---------------------------------------------------------------------------
// WGSL shaders — same language used by wgpu on native and WebAssembly targets
// ---------------------------------------------------------------------------

const WGSL_VERTEX = /* wgsl */`
@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4f {
    // Full-screen triangle pair (two triangles covering the NDC quad)
    var pos = array<vec2f, 6>(
        vec2f(-1.0, -1.0), vec2f( 1.0, -1.0), vec2f(-1.0,  1.0),
        vec2f(-1.0,  1.0), vec2f( 1.0, -1.0), vec2f( 1.0,  1.0)
    );
    return vec4f(pos[vi], 0.0, 1.0);
}
`;

const WGSL_FRAGMENT = /* wgsl */`
struct Uniforms {
    time   : f32,
    width  : f32,
    height : f32,
    _pad   : f32,
}

@group(0) @binding(0) var<uniform> u: Uniforms;

// ---- noise helpers -------------------------------------------------------

fn hash2(p: vec2f) -> f32 {
    return fract(sin(dot(p, vec2f(127.1, 311.7))) * 43758.5453);
}

fn smooth_noise(p: vec2f) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let uv = f * f * (3.0 - 2.0 * f);   // smoothstep
    return mix(
        mix(hash2(i),                hash2(i + vec2f(1.0, 0.0)), uv.x),
        mix(hash2(i + vec2f(0.0, 1.0)), hash2(i + vec2f(1.0, 1.0)), uv.x),
        uv.y
    );
}

fn fbm(p_in: vec2f) -> f32 {
    var val  = 0.0;
    var amp  = 0.5;
    var freq = 1.0;
    var p    = p_in;
    for (var i = 0; i < 5; i++) {
        val  += amp * smooth_noise(p * freq);
        amp  *= 0.5;
        freq *= 2.0;
    }
    return val;
}

// ---- main fragment -------------------------------------------------------

@fragment
fn fs_main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let uv = pos.xy / vec2f(u.width, u.height);
    let t  = u.time * 0.35;

    // Layered fbm producing an aurora-like curtain
    var p = uv * 3.5 + vec2f(t * 0.25, 0.0);
    let n1 = fbm(p);
    let n2 = fbm(p + vec2f(0.0, t * 0.08) + vec2f(n1 * 1.8));
    let n3 = fbm(p + vec2f(n2 * 1.4, 0.0) - vec2f(0.0, t * 0.06));

    // Soft band along the top edge with vertical falloff
    let band = smoothstep(0.25, 0.80, n3) * (1.0 - uv.y * 0.9);
    let intensity = band * 0.18;

    // Colour palette matching the app theme: teal → blue → purple
    let c1 = vec3f(0.10, 0.42, 0.50);  // teal
    let c2 = vec3f(0.18, 0.35, 0.58);  // blue
    let c3 = vec3f(0.38, 0.25, 0.52);  // purple
    let col = mix(mix(c1, c2, n2), c3, n1 * 0.6);

    return vec4f(col * intensity, intensity * 0.55);
}
`;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

interface GpuState {
    device: GPUDevice;
    pipeline: GPURenderPipeline;
    uniformBuffer: GPUBuffer;
    bindGroup: GPUBindGroup;
    context: GPUCanvasContext;
    animId: number;
}

export function WgpuOverlay() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const gpuRef    = useRef<GpuState | null>(null);

    // --- resize canvas to fill viewport ------------------------------------
    useEffect(() => {
        if (!gpuOverlayEnabled.value) return;
        const canvas = canvasRef.current;
        if (!canvas) return;

        const sync = () => {
            canvas.width  = window.innerWidth;
            canvas.height = window.innerHeight;
        };
        sync();
        window.addEventListener('resize', sync);
        return () => window.removeEventListener('resize', sync);
    }, [gpuOverlayEnabled.value]);

    // --- WebGPU init & render loop ----------------------------------------
    useEffect(() => {
        if (!gpuOverlayEnabled.value) {
            teardown(gpuRef.current);
            gpuRef.current = null;
            return;
        }

        const canvas = canvasRef.current;
        if (!canvas) return;

        let cancelled = false;

        async function init() {
            if (!('gpu' in navigator)) {
                console.warn('[WgpuOverlay] WebGPU not supported in this browser.');
                return;
            }

            const adapter = await navigator.gpu.requestAdapter();
            if (!adapter || cancelled) return;

            const device  = await adapter.requestDevice();
            if (cancelled) { device.destroy(); return; }

            const ctx    = canvas!.getContext('webgpu') as GPUCanvasContext;
            const format = navigator.gpu.getPreferredCanvasFormat();
            ctx.configure({ device, format, alphaMode: 'premultiplied' });

            const shader = device.createShaderModule({
                label: 'aurora-wgsl',
                code: WGSL_VERTEX + '\n' + WGSL_FRAGMENT,
            });

            // 16-byte uniform buffer: [time, width, height, _pad]
            const uniformBuffer = device.createBuffer({
                size: 16,
                usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            });

            const bindGroupLayout = device.createBindGroupLayout({
                entries: [{
                    binding: 0,
                    visibility: GPUShaderStage.FRAGMENT,
                    buffer: { type: 'uniform' },
                }],
            });

            const pipeline = device.createRenderPipeline({
                layout: device.createPipelineLayout({ bindGroupLayouts: [bindGroupLayout] }),
                vertex:   { module: shader, entryPoint: 'vs_main' },
                fragment: {
                    module: shader,
                    entryPoint: 'fs_main',
                    targets: [{
                        format,
                        blend: {
                            color: { srcFactor: 'src-alpha', dstFactor: 'one-minus-src-alpha', operation: 'add' },
                            alpha: { srcFactor: 'one',       dstFactor: 'one-minus-src-alpha', operation: 'add' },
                        },
                    }],
                },
                primitive: { topology: 'triangle-list' },
            });

            const bindGroup = device.createBindGroup({
                layout: bindGroupLayout,
                entries: [{ binding: 0, resource: { buffer: uniformBuffer } }],
            });

            const state: GpuState = { device, pipeline, uniformBuffer, bindGroup, context: ctx, animId: 0 };
            gpuRef.current = state;

            const startTime = performance.now();

            function frame() {
                if (cancelled) return;
                const time = (performance.now() - startTime) / 1000;
                device.queue.writeBuffer(uniformBuffer, 0,
                    new Float32Array([time, canvas!.width, canvas!.height, 0]));

                const enc  = device.createCommandEncoder();
                const pass = enc.beginRenderPass({
                    colorAttachments: [{
                        view:       ctx.getCurrentTexture().createView(),
                        loadOp:     'clear',
                        storeOp:    'store',
                        clearValue: { r: 0, g: 0, b: 0, a: 0 },
                    }],
                });
                pass.setPipeline(pipeline);
                pass.setBindGroup(0, bindGroup);
                pass.draw(6);
                pass.end();
                device.queue.submit([enc.finish()]);

                state.animId = requestAnimationFrame(frame);
            }

            if (!cancelled) state.animId = requestAnimationFrame(frame);
        }

        init().catch(console.error);

        return () => {
            cancelled = true;
            teardown(gpuRef.current);
            gpuRef.current = null;
        };
    }, [gpuOverlayEnabled.value]);

    if (!gpuOverlayEnabled.value) return null;

    return (
        <canvas
            ref={canvasRef}
            aria-hidden="true"
            style={{
                position:      'fixed',
                top:           0,
                left:          0,
                width:         '100vw',
                height:        '100vh',
                pointerEvents: 'none',
                zIndex:        9999,
            }}
        />
    );
}

function teardown(state: GpuState | null) {
    if (!state) return;
    cancelAnimationFrame(state.animId);
    state.device.destroy();
}
