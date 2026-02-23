/// <reference types="@webgpu/types" />
/**
 * WgpuOverlay — GPU-accelerated canvas rendered **behind** HTML.
 *
 * Architecture:
 *   - Canvas sits at z-index -1 (opaque), HTML backgrounds are transparent
 *     so content appears on top of the shader scene.
 *   - Shaders are split into modules (types/noise/background/particles/compute)
 *     and concatenated at pipeline creation time.
 *   - Compute shader simulates four particle types:
 *       [0..96)    metal sparks (spawn at mouse cursor on hover)
 *       [96..288)  flying embers/ash (continuous rise)
 *       [288..416) angelic beams (pixel-thin tall vertical rays)
 *       [416..512) angelic glitter (twinkle around selected element)
 *   - Background shader renders: smoky texture, static shadows, ember
 *     hover borders, graph nodes, and CRT post-processing.
 *   - Particle shader renders all four particle types via instanced quads
 *     with additive blending.
 *   - `pointer-events: none` keeps HTML fully interactive.
 *
 * Shaders are loaded at build time via Vite's `?raw` import.
 */
import { useEffect, useRef } from 'preact/hooks';
import { signal } from '@preact/signals';

import paletteWgsl from '../../effects/palette.wgsl?raw';
import particleShadingWgsl from '../../effects/particle-shading.wgsl?raw';
import typesCode from './types.wgsl?raw';
import noiseCode from './noise.wgsl?raw';
import bgCode from './background.wgsl?raw';
import particleCode from './particles.wgsl?raw';
import csCode from './compute.wgsl?raw';
import { buildPaletteBuffer, PALETTE_BYTE_SIZE } from '../../effects/palette';
import { themeColors, effectSettings, CURSOR_STYLE_VALUE } from '../../store/theme';

export const gpuOverlayEnabled = signal(true);

// ---------------------------------------------------------------------------
// Overlay render callback system — allows external components (e.g.
// HypergraphView) to draw into the shared WgpuOverlay canvas.
// ---------------------------------------------------------------------------

/**
 * Callback invoked during the overlay's render pass each frame.
 * Receivers can set their own pipeline, bind groups, viewport/scissor,
 * and issue draw calls.  Buffer writes via `device.queue.writeBuffer()`
 * are safe here — they're staged before `queue.submit()`.
 */
export type OverlayRenderCallback = (
    pass: GPURenderPassEncoder,
    device: GPUDevice,
    time: number,
    dt: number,
    canvasWidth: number,
    canvasHeight: number,
) => void;

const _overlayCallbacks = new Set<OverlayRenderCallback>();

export function registerOverlayRenderer(cb: OverlayRenderCallback): void {
    _overlayCallbacks.add(cb);
}

export function unregisterOverlayRenderer(cb: OverlayRenderCallback): void {
    _overlayCallbacks.delete(cb);
}

/**
 * Exposes the shared GPU device + canvas format so external components
 * can create pipelines compatible with the overlay's render pass.
 * `null` when WebGPU is not available or the overlay is disabled.
 */
export const overlayGpu = signal<{ device: GPUDevice; format: GPUTextureFormat } | null>(null);

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

/** Number of particles simulated by the compute shader. */
const NUM_PARTICLES    = 512;
/**
 * Floats per particle: [px, py, vx, vy, life, max_life, hue, size,
 *                        kind, spawn_t, _p1, _p2]
 */
const PARTICLE_FLOATS  = 12;
const PARTICLE_BYTES   = PARTICLE_FLOATS * 4;  // 48 bytes
const PARTICLE_BUF_SIZE = NUM_PARTICLES * PARTICLE_BYTES;
const COMPUTE_WORKGROUP = 64;

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
 */
const KIND_STRUCTURAL = 0;
const KIND_ERROR      = 1;
const KIND_WARN       = 2;
const KIND_INFO       = 3;
const KIND_DEBUG      = 4;
const KIND_SPAN_HL    = 5;
const KIND_SELECTED   = 6;
const KIND_PANIC      = 7;

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
// Event-driven element collector — scans the DOM only when something changes
// (DOM mutations, scroll, resize, or explicit external trigger).  The render
// loop reads the cached snapshot, so GPU frames never trigger layout recalc
// unless a dirty flag is set.
// ---------------------------------------------------------------------------

/** Reusable buffer — avoids a 4 KB allocation every scan. */
const _elemData  = new Float32Array(MAX_ELEMENTS * ELEM_FLOATS);
/** Reusable 112-byte uniform upload buffer (28 × f32). */
const _uniformF32 = new Float32Array(28);

/** Hover tracking — detect impact (new hover start) for metal spark burst. */
let _prevHoverIdx   = -1;
let _hoverStartTime = 0;

/** Cached element snapshot read by the GPU render loop. */
let _cachedData  = _elemData;
let _cachedCount = 0;

/** Dirty flag — set by scroll/resize/mutation events to trigger a re-scan on the next frame. */
let _scanDirty = false;

/**
 * Mark the element scan as dirty so positions are re-queried on the next
 * animation frame.  Call this from any component that moves, adds, or
 * removes overlay-tracked DOM elements (e.g. HypergraphView after
 * repositioning nodes).
 */
export function markOverlayScanDirty(): void {
    _scanDirty = true;
}

function scanElements(): void {
    _elemData.fill(0);
    let count = 0;
    const vh = window.innerHeight;

    // Query each selector group separately — O(selectors) queries but no
    // per-element re-matching, which is much cheaper overall.
    for (let si = 0; si < SELECTOR_META.length && count < MAX_ELEMENTS; si++) {
        const meta = SELECTOR_META[si];
        if (!meta) continue;
        const { sel, hue, kind } = meta;
        const elems = document.querySelectorAll(sel);
        for (let j = 0; j < elems.length && count < MAX_ELEMENTS; j++) {
            const r = elems[j]!.getBoundingClientRect();
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

    _cachedData  = _elemData;
    _cachedCount = count;
}

// Shaders are loaded from types/noise/background/particles/compute .wgsl at
// build time via Vite's `?raw` import.  See src/types/wgsl.d.ts.

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

interface GpuState {
    device:           GPUDevice;
    renderPipeline:   GPURenderPipeline;
    particlePipeline: GPURenderPipeline;
    computePipeline:  GPUComputePipeline;
    uniformBuffer:    GPUBuffer;
    elemBuffer:       GPUBuffer;
    particleBuffer:   GPUBuffer;
    paletteBuffer:    GPUBuffer;
    computeBindGroup: GPUBindGroup;
    renderBindGroup:  GPUBindGroup;
    context:          GPUCanvasContext;
    animId:           number;
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

    // --- toggle custom-cursor class when GPU cursor is active ---------------
    useEffect(() => {
        const style = effectSettings.value.cursorStyle;
        if (gpuOverlayEnabled.value && style !== 'default') {
            document.documentElement.classList.add('gpu-custom-cursor');
        } else {
            document.documentElement.classList.remove('gpu-custom-cursor');
        }
        return () => document.documentElement.classList.remove('gpu-custom-cursor');
    }, [gpuOverlayEnabled.value, effectSettings.value.cursorStyle]);

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
        let mutationObserver: MutationObserver | null = null;
        const onScroll = () => { _scanDirty = true; };
        const onResize = () => { _scanDirty = true; };

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
            ctx.configure({ device, format, alphaMode: 'opaque' });

            // Expose device for external renderers (e.g. HypergraphView)
            overlayGpu.value = { device, format };

            // --- Shader modules (concatenated from split files) ----------------
            const sharedCode = paletteWgsl + '\n' + typesCode + '\n' + noiseCode + '\n';
            const renderShared = sharedCode + particleShadingWgsl + '\n';

            const renderShader = device.createShaderModule({
                label: 'background-shader',
                code:  renderShared + bgCode,
            });

            const particleShader = device.createShaderModule({
                label: 'particle-shader',
                code:  renderShared + particleCode,
            });

            const computeShader = device.createShaderModule({
                label: 'compute-shader',
                code:  sharedCode + csCode,
            });

            // --- Buffers -------------------------------------------------------
            // Uniform buffer (112 bytes): [time, width, height, element_count,
            //   mouse_x, mouse_y, delta_time, hover_elem, hover_start_time,
            //   selected_elem, crt_scanlines_h, crt_scanlines_v,
            //   crt_edge_shadow, crt_flicker, cursor_style,
            //   smoke_intensity, smoke_speed, warm_scale, cool_scale, fine_scale,
            //   grain_intensity, grain_coarseness, grain_size,
            //   vignette_str, underglow_str, _pad2, _pad3, _pad4]
            const uniformBuffer = device.createBuffer({
                size:  112,
                usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            });

            // Storage buffer: up to MAX_ELEMENTS × 32 bytes
            const elemBuffer = device.createBuffer({
                size:  MAX_ELEMENTS * ELEM_BYTES,
                usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
            });

            // Particle buffer: read_write in compute, read-only in fragment
            const particleBuffer = device.createBuffer({
                size:  PARTICLE_BUF_SIZE,
                usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
            });

            // Zero-init particle buffer (all dead particles)
            device.queue.writeBuffer(particleBuffer, 0,
                new Float32Array(NUM_PARTICLES * PARTICLE_FLOATS));

            // Palette uniform buffer (theme colors for shaders)
            const paletteBuffer = device.createBuffer({
                size:  PALETTE_BYTE_SIZE,
                usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
            });

            // --- Bind group layouts --------------------------------------------
            // Compute: uniform + elems (read) + particles (read_write)
            const computeBGL = device.createBindGroupLayout({
                entries: [
                    { binding: 0, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
                    { binding: 1, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 2, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } },
                    { binding: 3, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
                ],
            });

            // Render: uniform + elems (read) + particles (read)
            // VERTEX | FRAGMENT visibility so vs_particle can read particles
            const renderBGL = device.createBindGroupLayout({
                entries: [
                    { binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: { type: 'uniform' } },
                    { binding: 1, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: { type: 'read-only-storage' } },
                    { binding: 2, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: { type: 'read-only-storage' } },
                    { binding: 3, visibility: GPUShaderStage.FRAGMENT, buffer: { type: 'uniform' } },
                ],
            });

            // --- Pipelines -----------------------------------------------------
            const computePipeline = device.createComputePipeline({
                layout:  device.createPipelineLayout({ bindGroupLayouts: [computeBGL] }),
                compute: { module: computeShader, entryPoint: 'cs_main' },
            });

            const renderPipelineLayout = device.createPipelineLayout({ bindGroupLayouts: [renderBGL] });

            const renderPipeline = device.createRenderPipeline({
                layout:   renderPipelineLayout,
                vertex:   { module: renderShader, entryPoint: 'vs_main' },
                fragment: {
                    module:     renderShader,
                    entryPoint: 'fs_main',
                    targets: [{ format }],
                },
                primitive: { topology: 'triangle-list' },
            });

            // Particle pipeline: instanced quads with additive blend
            const particlePipeline = device.createRenderPipeline({
                layout:   renderPipelineLayout,
                vertex:   { module: particleShader, entryPoint: 'vs_particle' },
                fragment: {
                    module:     particleShader,
                    entryPoint: 'fs_particle',
                    targets: [{
                        format,
                        blend: {
                            color: { srcFactor: 'one', dstFactor: 'one', operation: 'add' },
                            alpha: { srcFactor: 'one', dstFactor: 'one', operation: 'add' },
                        },
                    }],
                },
                primitive: { topology: 'triangle-list' },
            });

            // --- Bind groups ---------------------------------------------------
            const computeBindGroup = device.createBindGroup({
                layout:  computeBGL,
                entries: [
                    { binding: 0, resource: { buffer: uniformBuffer  } },
                    { binding: 1, resource: { buffer: elemBuffer     } },
                    { binding: 2, resource: { buffer: particleBuffer } },
                    { binding: 3, resource: { buffer: paletteBuffer  } },
                ],
            });

            const renderBindGroup = device.createBindGroup({
                layout:  renderBGL,
                entries: [
                    { binding: 0, resource: { buffer: uniformBuffer  } },
                    { binding: 1, resource: { buffer: elemBuffer     } },
                    { binding: 2, resource: { buffer: particleBuffer } },
                    { binding: 3, resource: { buffer: paletteBuffer  } },
                ],
            });

            const state: GpuState = {
                device, renderPipeline, particlePipeline, computePipeline,
                uniformBuffer, elemBuffer, particleBuffer, paletteBuffer,
                computeBindGroup, renderBindGroup, context: ctx, animId: 0,
            };
            gpuRef.current = state;

            let prevTime = performance.now() / 1000;
            const startTime = performance.now();

            // Initial scan
            scanElements();

            // Mark scan dirty on scroll / resize so positions update on next frame
            window.addEventListener('scroll', onScroll, true); // capture phase catches inner scrolls
            window.addEventListener('resize', onResize);

            // Watch for DOM mutations that affect tracked elements:
            // class changes (hover, selected, level), child additions/removals,
            // and inline style changes (HypergraphView node transforms).
            mutationObserver = new MutationObserver(() => { _scanDirty = true; });
            mutationObserver.observe(document.body, {
                childList: true,
                subtree: true,
                attributes: true,
                attributeFilter: ['class', 'style'],
            });

            function frame() {
                if (cancelled) return;

                // Re-scan if anything changed since last frame
                if (_scanDirty) {
                    _scanDirty = false;
                    scanElements();
                }

                const nowSec = performance.now() / 1000;
                const time = (performance.now() - startTime) / 1000;
                const dt   = Math.min(nowSec - prevTime, 0.05); // cap at 50ms
                prevTime   = nowSec;

                // Upload cached element snapshot (no DOM access here)
                const count = _cachedCount;
                const mx = mouseRef.current.x;
                const my = mouseRef.current.y;

                // Determine hovered element index
                let hoverIdx = -1;
                for (let i = 0; i < count; i++) {
                    const base = i * ELEM_FLOATS;
                    const ex = _cachedData[base]!;
                    const ey = _cachedData[base + 1]!;
                    const ew = _cachedData[base + 2]!;
                    const eh = _cachedData[base + 3]!;

                    if (mx >= ex && mx < ex + ew && my >= ey && my < ey + eh) {
                        hoverIdx = i;
                        // Don't break — last match wins (topmost in DOM order)
                    }
                }

                // Track hover transitions for metal-spark burst
                if (hoverIdx !== _prevHoverIdx) {
                    _hoverStartTime = time;
                    _prevHoverIdx   = hoverIdx;
                }

                // Find selected element index (kind === KIND_SELECTED)
                let selectedIdx = -1;
                for (let i = 0; i < count; i++) {
                    const base = i * ELEM_FLOATS;
                    if (_cachedData[base + 5] === KIND_SELECTED) {
                        selectedIdx = i;
                        break; // first selected element wins
                    }
                }

                _uniformF32[0] = time;
                _uniformF32[1] = canvas!.width;
                _uniformF32[2] = canvas!.height;
                _uniformF32[3] = count;
                _uniformF32[4] = mx;
                _uniformF32[5] = my;
                _uniformF32[6] = dt;
                _uniformF32[7] = hoverIdx;
                _uniformF32[8] = _hoverStartTime;
                _uniformF32[9] = selectedIdx;
                const eff = effectSettings.value;
                const crtOn = eff.crtEnabled;
                _uniformF32[10] = crtOn ? eff.crtScanlinesH / 100 : 0.0;
                _uniformF32[11] = crtOn ? eff.crtScanlinesV / 100 : 0.0;
                _uniformF32[12] = crtOn ? eff.crtEdgeShadow / 100 : 0.0;
                _uniformF32[13] = crtOn ? eff.crtFlicker / 100 : 0.0;
                _uniformF32[14] = CURSOR_STYLE_VALUE[eff.cursorStyle] ?? 0;
                _uniformF32[15] = eff.smokeIntensity / 100;
                _uniformF32[16] = eff.smokeSpeed / 100;       // 0–500 → 0.0–5.0
                _uniformF32[17] = eff.smokeWarmScale / 100;   // 0–200 → 0.0–2.0
                _uniformF32[18] = eff.smokeCoolScale / 100;
                _uniformF32[19] = eff.smokeFineScale / 100;
                _uniformF32[20] = eff.grainIntensity / 100;
                _uniformF32[21] = eff.grainCoarseness / 100;
                _uniformF32[22] = eff.grainSize / 100;
                _uniformF32[23] = eff.vignetteStrength / 100;
                _uniformF32[24] = eff.underglowStrength / 100;
                device.queue.writeBuffer(uniformBuffer, 0, _uniformF32.buffer);

                // Upload current theme palette to GPU
                const palBuf = buildPaletteBuffer(themeColors.value);
                device.queue.writeBuffer(paletteBuffer, 0, palBuf.buffer);

                if (count > 0) {
                    device.queue.writeBuffer(elemBuffer, 0,
                        _cachedData.buffer, 0, count * ELEM_BYTES);
                }

                const enc = device.createCommandEncoder();

                // --- Compute pass: simulate particles --------------------------
                const computePass = enc.beginComputePass();
                computePass.setPipeline(computePipeline);
                computePass.setBindGroup(0, computeBindGroup);
                computePass.dispatchWorkgroups(
                    Math.ceil(NUM_PARTICLES / COMPUTE_WORKGROUP));
                computePass.end();

                // --- Render pass -----------------------------------------------
                const renderPass = enc.beginRenderPass({
                    colorAttachments: [{
                        view:       ctx.getCurrentTexture().createView(),
                        loadOp:     'clear',
                        storeOp:    'store',
                        clearValue: { r: 0, g: 0, b: 0, a: 1 },
                    }],
                });

                // Draw 1: full-screen quad (aurora + elements + CRT)
                renderPass.setPipeline(renderPipeline);
                renderPass.setBindGroup(0, renderBindGroup);
                renderPass.draw(6);

                // Draw 2: instanced particle quads (additive blend)
                renderPass.setPipeline(particlePipeline);
                renderPass.setBindGroup(0, renderBindGroup);
                renderPass.draw(6, NUM_PARTICLES);

                // Draw 3+: external overlay renderers
                for (const cb of _overlayCallbacks) {
                    cb(renderPass, device, time, dt, canvas!.width, canvas!.height);
                }

                renderPass.end();

                device.queue.submit([enc.finish()]);
                state.animId = requestAnimationFrame(frame);
            }

            if (!cancelled) state.animId = requestAnimationFrame(frame);
        }

        init().catch(console.error);

        return () => {
            cancelled = true;
            if (mutationObserver) mutationObserver.disconnect();
            window.removeEventListener('scroll', onScroll, true);
            window.removeEventListener('resize', onResize);
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
                zIndex:        -1,
            }}
        />
    );
}

function teardown(state: GpuState | null) {
    if (!state) return;
    cancelAnimationFrame(state.animId);
    overlayGpu.value = null;
    state.device.destroy();
}
