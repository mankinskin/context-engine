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
 * Shaders live in vertex.wgsl / fragment.wgsl (same directory) and are
 * bundled at build time via Vite's `?raw` import.
 *
 * pointer-events: none on the canvas ensures normal UI interaction is
 * unaffected.
 */
import { useEffect, useRef } from 'preact/hooks';
import { signal } from '@preact/signals';
import vsCode from './vertex.wgsl?raw';
import fsCode from './fragment.wgsl?raw';

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

// Shaders are loaded from vertex.wgsl / fragment.wgsl at build time via
// Vite's `?raw` import.  See src/types/wgsl.d.ts for the TypeScript module
// declaration.

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
                code:  vsCode + '\n' + fsCode,
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
