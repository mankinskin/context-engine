/// <reference types="@webgpu/types" />
/**
 * WgpuOverlay — GPU-accelerated canvas composited **on top** of HTML.
 *
 * Uses premultiplied-alpha compositing to achieve true post-processing:
 *
 *   - CRT effects (scanlines, vignette, phosphor) are output as alpha,
 *     which darkens the HTML beneath through the browser's compositor.
 *   - Scene content (aurora, element glows) adds light on top of HTML.
 *   - `pointer-events: none` keeps HTML fully interactive.
 *
 * This hooks into the browser's native GPU compositing pipeline — the
 * browser blends the WebGPU output with the HTML render target in
 * GPU space (zero JS cost, no texture capture needed).
 *
 * Shaders live in vertex.wgsl / fragment.wgsl (same directory) and are
 * bundled at build time via Vite's `?raw` import.
 */
import { useEffect, useRef } from 'preact/hooks';
import { signal } from '@preact/signals';
import { cytoscapeInstance } from '../FlowGraph/FlowGraph';
import vsCode from './vertex.wgsl?raw';
import fsCode from './fragment.wgsl?raw';

export const gpuOverlayEnabled = signal(true);

// ---------------------------------------------------------------------------
// Element scanning
// ---------------------------------------------------------------------------

/**
 * CSS selectors for UI regions to shade.
 * Each selector gets its own stable hue (index / length of this array).
 *
 * Entries are grouped:
 *   0-8  : structural UI regions  (low-intensity border glow)
 *   9-13 : log entry levels       (colour-coded per severity)
 *   14   : highlighted span group (bright shimmer)
 *   15   : selected log entry     (intense focus glow)
 *   16   : panic entries          (alarm pulse)
 */
const ELEMENT_SELECTORS = [
    // --- structural regions (hue 0.00 – 0.53) ---
    '.header',
    '.sidebar',
    '.tab-bar',
    '.filter-panel',
    '.view-container',
    '.stats-view',
    '.flow-graph-container',
    '.log-list',
    '.code-viewer',
    // --- per-level log entries (hue 0.53 – 0.82) ---
    '.log-entry.level-error',
    '.log-entry.level-warn',
    '.log-entry.level-info',
    '.log-entry.level-debug',
    '.log-entry.level-trace',
    // --- interactive states ---
    '.log-entry.span-highlighted',
    '.log-entry.selected',
    '.log-entry.panic-entry',
];

/** Maximum number of DOM elements passed to the GPU per frame. */
const MAX_ELEMENTS = 128;

/** f32 values per element in the storage buffer: [x, y, w, h, hue, kind, _p1, _p2] */
const ELEM_FLOATS = 8;
const ELEM_BYTES  = ELEM_FLOATS * 4;  // 32 bytes, 16-byte aligned

/**
 * Element kind constants — passed to the shader so it can vary the glow
 * style per element category.
 *   0 = structural UI region   (subtle border glow)
 *   1 = error log entry        (hot red pulse)
 *   2 = warn log entry         (amber shimmer)
 *   3 = info log entry         (calm blue)
 *   4 = debug / trace entry    (dim ambient)
 *   5 = span-highlighted       (bright shimmer wave)
 *   6 = selected entry         (intense focus ring)
 *   7 = panic entry            (alarm strobe)
 *   8 = graph node              (3D-shaded rounded rectangle)
 */
const KIND_STRUCTURAL = 0;
const KIND_ERROR      = 1;
const KIND_WARN       = 2;
const KIND_INFO       = 3;
const KIND_DEBUG      = 4;
const KIND_SPAN_HL    = 5;
const KIND_SELECTED   = 6;
const KIND_PANIC      = 7;
const KIND_GRAPH_NODE = 8;

/** Map selector index → element kind for the shader. */
function selectorKind(selectorIndex: number): number {
    if (selectorIndex < 9)  return KIND_STRUCTURAL;
    if (selectorIndex === 9)  return KIND_ERROR;
    if (selectorIndex === 10) return KIND_WARN;
    if (selectorIndex === 11) return KIND_INFO;
    if (selectorIndex === 12) return KIND_DEBUG;
    if (selectorIndex === 13) return KIND_DEBUG; // trace → same as debug
    if (selectorIndex === 14) return KIND_SPAN_HL;
    if (selectorIndex === 15) return KIND_SELECTED;
    if (selectorIndex === 16) return KIND_PANIC;
    return KIND_STRUCTURAL;
}

// ---------------------------------------------------------------------------
// Pre-computed selector metadata (avoids per-element `matches()` calls)
// ---------------------------------------------------------------------------
const SELECTOR_META: Array<{ sel: string; hue: number; kind: number }> =
    ELEMENT_SELECTORS.map((sel, i) => ({
        sel,
        hue:  i / ELEMENT_SELECTORS.length,
        kind: selectorKind(i),
    }));

// ---------------------------------------------------------------------------
// Throttled element collector — scans the DOM at a fixed cadence (not every
// animation frame) and caches the result.  The render loop reads the cached
// snapshot, so GPU frames never trigger layout recalc.
// ---------------------------------------------------------------------------
const SCAN_INTERVAL_MS = 120;   // ~8 Hz DOM scanning

/** Reusable buffer — avoids a 4 KB allocation every scan. */
const _elemData  = new Float32Array(MAX_ELEMENTS * ELEM_FLOATS);
/** Reusable 32-byte uniform upload buffer. */
const _uniformF32 = new Float32Array(8);

/** Cached element snapshot read by the GPU render loop. */
let _cachedData  = _elemData;
let _cachedCount = 0;

function scanElements(): void {
    _elemData.fill(0);
    let count = 0;
    const vw = window.innerWidth;
    const vh = window.innerHeight;

    // Query each selector group separately — O(selectors) queries but no
    // per-element re-matching, which is much cheaper overall.
    for (let si = 0; si < SELECTOR_META.length && count < MAX_ELEMENTS; si++) {
        const { sel, hue, kind } = SELECTOR_META[si];
        const elems = document.querySelectorAll(sel);
        for (let j = 0; j < elems.length && count < MAX_ELEMENTS; j++) {
            const r = elems[j].getBoundingClientRect();
            if (r.width === 0 || r.height === 0) continue;
            if (r.bottom < 0 || r.top > vh) continue;
            const base = count * ELEM_FLOATS;
            _elemData[base    ] = r.left;
            _elemData[base + 1] = r.top;
            _elemData[base + 2] = r.width;
            _elemData[base + 3] = r.height;
            _elemData[base + 4] = hue;
            _elemData[base + 5] = kind;
            count++;
        }
    }

    // --- Collect graph nodes from Cytoscape --------------------------------
    const cy = cytoscapeInstance.value;
    if (cy && !cy.destroyed()) {
        const container = cy.container();
        if (container) {
            const containerRect = container.getBoundingClientRect();
            const nodes = cy.nodes();
            for (let i = 0; i < nodes.length && count < MAX_ELEMENTS; i++) {
                const node = nodes[i];
                const bb = node.renderedBoundingBox({ includeLabels: false });
                const x = containerRect.left + bb.x1;
                const y = containerRect.top + bb.y1;
                const w = bb.x2 - bb.x1;
                const h = bb.y2 - bb.y1;
                if (y + h < 0 || y > vh) continue;
                if (x + w < 0 || x > vw) continue;
                const level = (node.data('level') || 'info').toUpperCase();
                const levelHue = level === 'ERROR' ? 0.0
                               : level === 'WARN'  ? 0.08
                               : level === 'INFO'  ? 0.58
                               : level === 'DEBUG' ? 0.75
                               : level === 'TRACE' ? 0.48
                               : 0.58;
                const nodeType = node.data('type');
                const p1 = nodeType === 'span_enter' ? 1.0
                         : nodeType === 'span_exit'  ? 2.0
                         : 0.0;
                const base = count * ELEM_FLOATS;
                _elemData[base    ] = x;
                _elemData[base + 1] = y;
                _elemData[base + 2] = w;
                _elemData[base + 3] = h;
                _elemData[base + 4] = levelHue;
                _elemData[base + 5] = KIND_GRAPH_NODE;
                _elemData[base + 6] = p1;
                _elemData[base + 7] = 0.0;
                count++;
            }
        }
    }

    _cachedData  = _elemData;
    _cachedCount = count;
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
    const mouseRef  = useRef<{ x: number; y: number }>({ x: -9999, y: -9999 });

    // --- track mouse position for 3D hover effect -------------------------
    useEffect(() => {
        const onMove = (e: MouseEvent) => {
            mouseRef.current.x = e.clientX;
            mouseRef.current.y = e.clientY;
        };
        const onLeave = () => {
            mouseRef.current.x = -9999;
            mouseRef.current.y = -9999;
        };
        window.addEventListener('mousemove', onMove);
        window.addEventListener('mouseleave', onLeave);
        return () => {
            window.removeEventListener('mousemove', onMove);
            window.removeEventListener('mouseleave', onLeave);
        };
    }, []);

    // --- toggle gpu-active class on document root -------------------------
    useEffect(() => {
        if (gpuOverlayEnabled.value) {
            document.documentElement.classList.add('gpu-active');
        } else {
            document.documentElement.classList.remove('gpu-active');
        }
        return () => document.documentElement.classList.remove('gpu-active');
    }, [gpuOverlayEnabled.value]);

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
        let scanTimer: ReturnType<typeof setInterval> | null = null;

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

            // Uniform buffer (32 bytes): [time, width, height, element_count, mouse_x, mouse_y, pad, pad]
            const uniformBuffer = device.createBuffer({
                size:  32,
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
                            color: { srcFactor: 'one', dstFactor: 'one-minus-src-alpha', operation: 'add' },
                            alpha: { srcFactor: 'one', dstFactor: 'one-minus-src-alpha', operation: 'add' },
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

            // Start throttled DOM scanner (runs independently of GPU frames)
            scanTimer = setInterval(scanElements, SCAN_INTERVAL_MS);
            scanElements(); // initial scan

            function frame() {
                if (cancelled) return;
                const time = (performance.now() - startTime) / 1000;

                // Upload cached element snapshot (no DOM access here)
                const count = _cachedCount;
                const mx = mouseRef.current.x;
                const my = mouseRef.current.y;
                _uniformF32[0] = time;
                _uniformF32[1] = canvas!.width;
                _uniformF32[2] = canvas!.height;
                _uniformF32[3] = count;
                _uniformF32[4] = mx;
                _uniformF32[5] = my;
                _uniformF32[6] = 0;
                _uniformF32[7] = 0;
                device.queue.writeBuffer(uniformBuffer, 0, _uniformF32);
                if (count > 0) {
                    device.queue.writeBuffer(elemBuffer, 0,
                        _cachedData.buffer, 0, count * ELEM_BYTES);
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
            if (scanTimer) clearInterval(scanTimer);
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
