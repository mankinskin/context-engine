/// <reference types="@webgpu/types" />
/**
 * WgpuOverlay — GPU-accelerated canvas overlay using WebGPU.
 *
 * WebGPU is the browser counterpart of wgpu (Rust GPU library).
 * When wgpu is compiled to WebAssembly it targets WebGPU/WebGL2,
 * using the exact same WGSL shader language shown here.
 *
 * The overlay renders two GPU effects simultaneously:
 *   1. An animated aurora background (full-screen noise / fbm)
 *   2. Per-element border glows — the DOM is scanned each frame for
 *      elements matching ELEMENT_SELECTORS; their bounding rects are
 *      uploaded to a GPU storage buffer.  The WGSL fragment shader
 *      loops over those rects and applies a uniquely-coloured animated
 *      glow to the inner border of each element, with the colour driven
 *      by the element's selector index ("element id").
 *
 * pointer-events: none on the canvas ensures normal UI interaction is
 * unaffected.
 */
import { useEffect, useRef } from 'preact/hooks';
import { signal } from '@preact/signals';

export const gpuOverlayEnabled = signal(false);

// ---------------------------------------------------------------------------
// Element scanning
// ---------------------------------------------------------------------------

/**
 * CSS selectors for UI regions to shade.
 * Each selector gets its own stable hue (index / length of this array).
 */
const ELEMENT_SELECTORS = [
    '.header',
    '.sidebar',
    '.tab-bar',
    '.filter-panel',
    '.view-container',
    '.stats-view',
    '.flow-graph-container',
    '.log-list',
    '.code-viewer',
];

/** Maximum number of DOM elements passed to the GPU per frame. */
const MAX_ELEMENTS = 32;

/** f32 values per element in the storage buffer: [x, y, w, h, hue, _p0, _p1, _p2] */
const ELEM_FLOATS = 8;
const ELEM_BYTES  = ELEM_FLOATS * 4;  // 32 bytes, 16-byte aligned

/** Scan matching DOM elements and build a Float32Array for the GPU. */
function collectElements(): { data: Float32Array; count: number } {
    const data = new Float32Array(MAX_ELEMENTS * ELEM_FLOATS);
    let count = 0;
    // Single querySelectorAll pass with a combined selector, then resolve
    // the hue for each element via element.matches() for the first selector
    // that matches it.  This is much cheaper than one querySelectorAll per
    // selector (especially when called every animation frame).
    const combined = ELEMENT_SELECTORS.join(', ');
    document.querySelectorAll(combined).forEach(el => {
        if (count >= MAX_ELEMENTS) return;
        const r = el.getBoundingClientRect();
        if (r.width === 0 || r.height === 0) return;
        const si  = ELEMENT_SELECTORS.findIndex(sel => el.matches(sel));
        const hue = si >= 0 ? si / ELEMENT_SELECTORS.length : 0;
        const base = count * ELEM_FLOATS;
        data[base + 0] = r.left;
        data[base + 1] = r.top;
        data[base + 2] = r.width;
        data[base + 3] = r.height;
        data[base + 4] = hue;
        // data[base + 5..7] = 0 (padding)
        count++;
    });
    return { data, count };
}

// ---------------------------------------------------------------------------
// WGSL shaders — same language used by wgpu on native and WebAssembly targets
// ---------------------------------------------------------------------------

const WGSL_VERTEX = /* wgsl */`
@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4f {
    // Full-screen triangle pair covering the NDC quad
    var pos = array<vec2f, 6>(
        vec2f(-1.0, -1.0), vec2f( 1.0, -1.0), vec2f(-1.0,  1.0),
        vec2f(-1.0,  1.0), vec2f( 1.0, -1.0), vec2f( 1.0,  1.0)
    );
    return vec4f(pos[vi], 0.0, 1.0);
}
`;

const WGSL_FRAGMENT = /* wgsl */`
// ---- uniforms / storage ---------------------------------------------------

struct Uniforms {
    time          : f32,
    width         : f32,
    height        : f32,
    element_count : f32,   // replaces former _pad
}

// Each element: rect(x,y,w,h) + hue + 3 padding f32 = 32 bytes (aligned)
struct ElemRect {
    rect : vec4f,   // x, y, w, h  (screen-space pixels, y=0 at top-left)
    hue  : f32,
    _p0  : f32,
    _p1  : f32,
    _p2  : f32,
}

@group(0) @binding(0) var<uniform>          u     : Uniforms;
@group(0) @binding(1) var<storage, read>    elems : array<ElemRect>;

// ---- colour helper -------------------------------------------------------

// Converts a hue value in [0, 1] to an RGB colour (saturation=1, value=1).
fn hue_to_rgb(h: f32) -> vec3f {
    let h6 = h * 6.0;
    let r  = abs(h6 - 3.0) - 1.0;
    let g  = 2.0 - abs(h6 - 2.0);
    let b  = 2.0 - abs(h6 - 4.0);
    return clamp(vec3f(r, g, b), vec3f(0.0), vec3f(1.0));
}

// ---- noise helpers (aurora background) -----------------------------------

fn hash2(p: vec2f) -> f32 {
    return fract(sin(dot(p, vec2f(127.1, 311.7))) * 43758.5453);
}

fn smooth_noise(p: vec2f) -> f32 {
    let i  = floor(p);
    let f  = fract(p);
    let uv = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash2(i),                   hash2(i + vec2f(1.0, 0.0)), uv.x),
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
    let px = pos.xy;
    let uv = px / vec2f(u.width, u.height);
    let t  = u.time * 0.35;

    // --- Aurora background ------------------------------------------------
    var p  = uv * 3.5 + vec2f(t * 0.25, 0.0);
    let n1 = fbm(p);
    let n2 = fbm(p + vec2f(0.0, t * 0.08) + vec2f(n1 * 1.8));
    let n3 = fbm(p + vec2f(n2 * 1.4, 0.0) - vec2f(0.0, t * 0.06));
    let band      = smoothstep(0.25, 0.80, n3) * (1.0 - uv.y * 0.9);
    let intensity = band * 0.18;
    let c1  = vec3f(0.10, 0.42, 0.50);
    let c2  = vec3f(0.18, 0.35, 0.58);
    let c3  = vec3f(0.38, 0.25, 0.52);
    let aurora_rgb = mix(mix(c1, c2, n2), c3, n1 * 0.6);
    var out = vec4f(aurora_rgb * intensity, intensity * 0.55);

    // --- Per-element border glow ------------------------------------------
    // For each DOM element, pixels inside its bounding rect receive an
    // animated coloured glow that is brightest at the inner edge and fades
    // inward.  The glow colour is derived from the element's selector index
    // (its "element id"), giving each UI region a unique stable colour.
    //
    // Complexity: O(pixels × element_count).  With MAX_ELEMENTS = 32 this
    // is well within GPU budget — modern hardware runs thousands of shader
    // invocations in parallel.  Raise MAX_ELEMENTS cautiously on complex UIs.
    let count = u32(u.element_count);
    for (var i = 0u; i < count; i++) {
        let e  = elems[i];
        let r  = e.rect;       // x, y, w, h
        let ex = r.x;
        let ey = r.y;
        let ew = r.z;
        let eh = r.w;

        // Only shade pixels inside this element's rect
        if px.x >= ex && px.x < ex + ew && px.y >= ey && px.y < ey + eh {
            // Inward distance to the nearest edge
            let dx   = min(px.x - ex, ex + ew - px.x);
            let dy   = min(px.y - ey, ey + eh - px.y);
            let dist = min(dx, dy);

            // Glow profile: rises from 0 at the edge, peaks around 3 px
            // inside, then fades out by 16 px inward.
            let glow = smoothstep(0.0, 2.0, dist) * smoothstep(16.0, 4.0, dist);

            // Slowly-drifting hue + per-element animated pulse
            let hue   = fract(e.hue + u.time * 0.04);
            let pulse = 0.55 + 0.45 * sin(u.time * 1.8 + e.hue * 6.28318);
            let rgb   = hue_to_rgb(hue);
            let alpha = glow * 0.42 * pulse;

            // Additive blend so element glows stack with the aurora
            out = out + vec4f(rgb * alpha, alpha);
        }
    }

    return clamp(out, vec4f(0.0), vec4f(1.0));
}
`;

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

interface GpuState {
    device:        GPUDevice;
    pipeline:      GPURenderPipeline;
    uniformBuffer: GPUBuffer;
    elemBuffer:    GPUBuffer;
    bindGroup:     GPUBindGroup;
    context:       GPUCanvasContext;
    animId:        number;
}

export function WgpuOverlay() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const gpuRef    = useRef<GpuState | null>(null);

    // --- keep canvas sized to the viewport --------------------------------
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

            const device = await adapter.requestDevice();
            if (cancelled) { device.destroy(); return; }

            const ctx    = canvas!.getContext('webgpu') as GPUCanvasContext;
            const format = navigator.gpu.getPreferredCanvasFormat();
            ctx.configure({ device, format, alphaMode: 'premultiplied' });

            const shader = device.createShaderModule({
                label: 'element-shader-wgsl',
                code:  WGSL_VERTEX + '\n' + WGSL_FRAGMENT,
            });

            // Uniform buffer (16 bytes): [time, width, height, element_count]
            const uniformBuffer = device.createBuffer({
                size:  16,
                usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            });

            // Storage buffer: up to MAX_ELEMENTS × 32 bytes
            const elemBuffer = device.createBuffer({
                size:  MAX_ELEMENTS * ELEM_BYTES,
                usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
            });

            const bindGroupLayout = device.createBindGroupLayout({
                entries: [
                    {
                        binding:    0,
                        visibility: GPUShaderStage.FRAGMENT,
                        buffer:     { type: 'uniform' },
                    },
                    {
                        binding:    1,
                        visibility: GPUShaderStage.FRAGMENT,
                        buffer:     { type: 'read-only-storage' },
                    },
                ],
            });

            const pipeline = device.createRenderPipeline({
                layout:   device.createPipelineLayout({ bindGroupLayouts: [bindGroupLayout] }),
                vertex:   { module: shader, entryPoint: 'vs_main' },
                fragment: {
                    module:     shader,
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
                layout:  bindGroupLayout,
                entries: [
                    { binding: 0, resource: { buffer: uniformBuffer } },
                    { binding: 1, resource: { buffer: elemBuffer    } },
                ],
            });

            const state: GpuState = {
                device, pipeline, uniformBuffer, elemBuffer, bindGroup, context: ctx, animId: 0,
            };
            gpuRef.current = state;

            const startTime = performance.now();

            function frame() {
                if (cancelled) return;
                const time = (performance.now() - startTime) / 1000;

                // Upload per-frame data
                const { data, count } = collectElements();
                device.queue.writeBuffer(uniformBuffer, 0,
                    new Float32Array([time, canvas!.width, canvas!.height, count]));
                if (count > 0) {
                    device.queue.writeBuffer(elemBuffer, 0,
                        data.buffer, 0, count * ELEM_BYTES);
                }

                // Render pass
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
                zIndex:        'var(--z-overlay)',
            }}
        />
    );
}

function teardown(state: GpuState | null) {
    if (!state) return;
    cancelAnimationFrame(state.animId);
    state.device.destroy();
}
