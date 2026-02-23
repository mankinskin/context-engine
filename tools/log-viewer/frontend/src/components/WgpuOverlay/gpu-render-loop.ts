/// <reference types="@webgpu/types" />
/**
 * GPU render loop — rAF orchestration, uniform packing, compute/render passes.
 *
 * Reads element data from the ElementScanner, uploads via GpuBufferManager,
 * and rebuilds bind groups when the buffer generation changes.
 */

import type { GpuPipelines } from './gpu-init';
import type { GpuBufferManager } from './gpu-buffers';
import type { ElementScanner } from './element-scanner';
import {
    ELEM_FLOATS, NUM_PARTICLES, COMPUTE_WORKGROUP,
    KIND_SELECTED, KIND_PANIC,
    SPARK_START, SPARK_END,
    EMBER_START, EMBER_END,
    RAY_START, RAY_END,
    GLITTER_START, GLITTER_END,
} from './element-types';
import { getOverlayCallbacks, consumeCaptureRequest, hasCaptureRequest } from './overlay-api';
import { captureFrame } from './thumbnail-capture';
import { effectSettings, themeColors, CURSOR_STYLE_VALUE } from '../../store/theme';

export class RenderLoop {
    private readonly pipelines: GpuPipelines;
    private readonly buffers: GpuBufferManager;
    private readonly scanner: ElementScanner;
    private readonly canvas: HTMLCanvasElement;

    private computeBindGroup: GPUBindGroup;
    private renderBindGroup: GPUBindGroup;
    private lastGeneration: number;

    // Depth buffer for 3D callbacks
    private depthTexture: GPUTexture | null = null;
    private depthView: GPUTextureView | null = null;
    private depthW = 0;
    private depthH = 0;

    private animId = 0;
    private cancelled = false;
    private prevTime = performance.now() / 1000;
    private readonly startTime = performance.now();

    // Mouse position
    private mx = -9999;
    private my = -9999;

    // Hover tracking — detect impact (new hover start) for metal spark burst
    private prevHoverIdx = -1;
    private hoverStartTime = 0;

    // Dirty tracking — skip GPU submission when scene is static
    private prevRenderMx = -9999;
    private prevRenderMy = -9999;
    private prevCanvasW = 0;
    private prevCanvasH = 0;
    private prevSelectedIdx = -1;
    private prevEffects: object | null = null;
    private prevParticlesEnabled = true;
    private prevSparksEnabled = true;
    private prevEmbersEnabled = true;
    private prevBeamsEnabled = true;
    private prevGlitterEnabled = true;
    private frameSkipCounter = 0;  // For 30fps limiter in particles-only mode

    constructor(
        pipelines: GpuPipelines,
        buffers: GpuBufferManager,
        scanner: ElementScanner,
        canvas: HTMLCanvasElement,
    ) {
        this.pipelines = pipelines;
        this.buffers = buffers;
        this.scanner = scanner;
        this.canvas = canvas;

        this.lastGeneration = buffers.generation;
        this.computeBindGroup = this.buildComputeBindGroup();
        this.renderBindGroup = this.buildRenderBindGroup();
    }

    // --- Bind group builders -----------------------------------------------

    private buildComputeBindGroup(): GPUBindGroup {
        const { device, computeBGL } = this.pipelines;
        const b = this.buffers;
        return device.createBindGroup({
            layout: computeBGL,
            entries: [
                { binding: 0, resource: { buffer: b.uniformBuffer  } },
                { binding: 1, resource: { buffer: b.elemBuffer     } },
                { binding: 2, resource: { buffer: b.particleBuffer } },
                { binding: 3, resource: { buffer: b.paletteBuffer  } },
            ],
        });
    }

    private buildRenderBindGroup(): GPUBindGroup {
        const { device, renderBGL } = this.pipelines;
        const b = this.buffers;
        return device.createBindGroup({
            layout: renderBGL,
            entries: [
                { binding: 0, resource: { buffer: b.uniformBuffer  } },
                { binding: 1, resource: { buffer: b.elemBuffer     } },
                { binding: 2, resource: { buffer: b.particleBuffer } },
                { binding: 3, resource: { buffer: b.paletteBuffer  } },
            ],
        });
    }

    private rebuildBindGroupsIfNeeded(): void {
        const gen = this.buffers.generation;
        if (gen !== this.lastGeneration) {
            this.computeBindGroup = this.buildComputeBindGroup();
            this.renderBindGroup = this.buildRenderBindGroup();
            this.lastGeneration = gen;
        }
    }

    /** Ensure depth texture exists and matches canvas size. */
    private ensureDepth(w: number, h: number): void {
        if (w === this.depthW && h === this.depthH && this.depthTexture) return;
        this.depthTexture?.destroy();
        this.depthTexture = this.pipelines.device.createTexture({
            size: [w, h],
            format: 'depth24plus',
            usage: GPUTextureUsage.RENDER_ATTACHMENT,
        });
        this.depthView = this.depthTexture.createView();
        this.depthW = w;
        this.depthH = h;
    }

    // --- Public API --------------------------------------------------------

    setMouse(x: number, y: number): void {
        this.mx = x;
        this.my = y;
    }

    start(): void {
        this.cancelled = false;
        this.animId = requestAnimationFrame(this.frame);
    }

    stop(): void {
        this.cancelled = true;
        cancelAnimationFrame(this.animId);
    }

    // --- Frame -------------------------------------------------------------

    private frame = (): void => {
        if (this.cancelled) return;

        const scanner = this.scanner;

        // Let the scanner update rect measurements for stale/visible elements
        const dataChanged = scanner.updateFrame();

        // If a full re-scan just occurred (view change), kill all particles
        if (scanner.didFullRescan) {
            this.buffers.resetParticles();
        }

        // Consume accumulated scroll delta for this frame
        const scrollDelta = scanner.consumeScrollDelta();

        const nowSec = performance.now() / 1000;
        const time = (performance.now() - this.startTime) / 1000;
        const dt   = Math.min(nowSec - this.prevTime, 0.05);
        this.prevTime = nowSec;

        const count = scanner.count;
        const data  = scanner.data;
        const mx = this.mx;
        const my = this.my;

        // --- Hover detection -----------------------------------------------
        // When multiple elements overlap at the mouse position, prefer the
        // one with the highest kind value.  This ensures effect-preview
        // containers (kind 8–11) win over structural parents (kind 0) that
        // also contain the cursor.
        let hoverIdx = -1;
        let hoverKind = -1;
        for (let i = 0; i < count; i++) {
            const base = i * ELEM_FLOATS;
            const ex = data[base]!;
            const ey = data[base + 1]!;
            const ew = data[base + 2]!;
            const eh = data[base + 3]!;
            if (mx >= ex && mx < ex + ew && my >= ey && my < ey + eh) {
                const k = data[base + 5]!;
                if (k >= hoverKind) {
                    hoverIdx = i;
                    hoverKind = k;
                }
            }
        }

        let hoverChanged = false;
        if (hoverIdx !== this.prevHoverIdx) {
            this.hoverStartTime = time;
            this.prevHoverIdx = hoverIdx;
            hoverChanged = true;
        }

        // --- Selected element detection ------------------------------------
        // When multiple elements have KIND_SELECTED (e.g. an "always on"
        // demo element plus a user-toggled one), prefer the one that is
        // also being hovered — that's the element the user is interacting
        // with.  Otherwise fall back to the last match so newly-toggled
        // elements (later in DOM order) win over permanent ones.
        let selectedIdx = -1;
        for (let i = 0; i < count; i++) {
            const base = i * ELEM_FLOATS;
            const k = data[base + 5]!;
            if (k === KIND_SELECTED) {
                selectedIdx = i;
                if (i === hoverIdx) break; // hovered selected element wins
            } else if (k === KIND_PANIC && selectedIdx < 0) {
                selectedIdx = i;
            }
        }

        // --- Frame skip: avoid GPU work when scene is static ---------------
        const eff = effectSettings.value;
        const particlesEnabled = eff.sparksEnabled || eff.embersEnabled
            || eff.beamsEnabled || eff.glitterEnabled;

        // "Minimal background" mode — smoke and grain both disabled
        // In this mode, the shader uses a cheap gradient instead of noise.
        const minimalBackground = (!eff.smokeEnabled || eff.smokeIntensity === 0)
            && eff.grainIntensity === 0;

        // Time-dependent effects that require continuous rendering
        const anyAnimated =
            (eff.smokeEnabled && eff.smokeIntensity > 0)
            || (eff.crtEnabled && eff.crtFlicker > 0)
            || eff.cursorStyle !== 'default'
            || (hoverIdx >= 0 && (eff.cinderEnabled || particlesEnabled))
            || (selectedIdx >= 0 && eff.beamsEnabled);

        // Input changes since last rendered frame
        const inputDirty =
            mx !== this.prevRenderMx || my !== this.prevRenderMy
            || this.canvas.width !== this.prevCanvasW
            || this.canvas.height !== this.prevCanvasH
            || scrollDelta.dx !== 0 || scrollDelta.dy !== 0
            || scanner.didFullRescan || dataChanged
            || hoverChanged
            || selectedIdx !== this.prevSelectedIdx
            || eff !== this.prevEffects
            || hasCaptureRequest();

        if (!anyAnimated && !inputDirty) {
            this.animId = requestAnimationFrame(this.frame);
            return;
        }

        // Frame rate limiter: skip every other frame when background is minimal
        // and only particles are animating. This halves GPU load.
        const particlesOnlyAnimated = anyAnimated && minimalBackground
            && !(eff.smokeEnabled && eff.smokeIntensity > 0)
            && !(eff.crtEnabled && eff.crtFlicker > 0)
            && eff.cursorStyle === 'default';
        if (particlesOnlyAnimated && !inputDirty) {
            this.frameSkipCounter = (this.frameSkipCounter + 1) % 2;
            if (this.frameSkipCounter !== 0) {
                this.animId = requestAnimationFrame(this.frame);
                return;
            }
        } else {
            this.frameSkipCounter = 0;
        }

        // Track state for next frame's dirty check
        this.prevRenderMx = mx;
        this.prevRenderMy = my;
        this.prevCanvasW = this.canvas.width;
        this.prevCanvasH = this.canvas.height;
        this.prevSelectedIdx = selectedIdx;
        this.prevEffects = eff;

        // Zero-fill particle ranges for types that just got disabled
        if (!eff.sparksEnabled && this.prevSparksEnabled) {
            this.buffers.resetParticleRange(SPARK_START, SPARK_END - SPARK_START);
        }
        if (!eff.embersEnabled && this.prevEmbersEnabled) {
            this.buffers.resetParticleRange(EMBER_START, EMBER_END - EMBER_START);
        }
        if (!eff.beamsEnabled && this.prevBeamsEnabled) {
            this.buffers.resetParticleRange(RAY_START, RAY_END - RAY_START);
        }
        if (!eff.glitterEnabled && this.prevGlitterEnabled) {
            this.buffers.resetParticleRange(GLITTER_START, GLITTER_END - GLITTER_START);
        }
        this.prevSparksEnabled = eff.sparksEnabled;
        this.prevEmbersEnabled = eff.embersEnabled;
        this.prevBeamsEnabled = eff.beamsEnabled;
        this.prevGlitterEnabled = eff.glitterEnabled;
        this.prevParticlesEnabled = particlesEnabled;

        // --- Pack uniforms -------------------------------------------------
        const u = this.buffers.uniformF32;
        u[0] = time;
        u[1] = this.canvas.width;
        u[2] = this.canvas.height;
        u[3] = count;
        u[4] = mx;
        u[5] = my;
        u[6] = dt;
        u[7] = hoverIdx;
        u[8] = this.hoverStartTime;
        u[9] = selectedIdx;
        const crtOn = eff.crtEnabled;
        u[10] = crtOn ? eff.crtScanlinesH / 100 : 0.0;
        u[11] = crtOn ? eff.crtScanlinesV / 100 : 0.0;
        u[12] = crtOn ? eff.crtEdgeShadow / 100 : 0.0;
        u[13] = crtOn ? eff.crtFlicker / 100 : 0.0;
        u[14] = CURSOR_STYLE_VALUE[eff.cursorStyle] ?? 0;
        u[15] = eff.smokeEnabled ? eff.smokeIntensity / 100 : 0.0;
        u[16] = eff.smokeEnabled ? eff.smokeSpeed / 100 : 0.0;
        u[17] = eff.smokeEnabled ? eff.smokeWarmScale / 100 : 0.0;
        u[18] = eff.smokeEnabled ? eff.smokeCoolScale / 100 : 0.0;
        u[19] = eff.smokeEnabled ? eff.smokeMossScale / 100 : 0.0;
        u[20] = eff.grainIntensity / 100;
        u[21] = eff.grainCoarseness / 100;
        u[22] = eff.grainSize / 100;
        u[23] = eff.vignetteStrength / 100;
        u[24] = eff.underglowStrength / 100;
        u[25] = eff.sparksEnabled ? eff.sparkSpeed / 100 : 0.0;
        u[26] = eff.embersEnabled ? eff.emberSpeed / 100 : 0.0;
        u[27] = eff.beamsEnabled ? eff.beamSpeed / 100 : 0.0;
        u[28] = eff.glitterEnabled ? eff.glitterSpeed / 100 : 0.0;
        u[29] = eff.beamHeight;
        u[30] = eff.beamCount;
        u[31] = eff.beamDrift / 100;
        u[32] = scrollDelta.dx;
        u[33] = scrollDelta.dy;
        u[34] = eff.sparksEnabled ? eff.sparkCount / 100 : 0.0;
        u[35] = eff.sparksEnabled ? eff.sparkSize / 100 : 0.0;
        u[36] = eff.embersEnabled ? eff.emberCount / 100 : 0.0;
        u[37] = eff.embersEnabled ? eff.emberSize / 100 : 0.0;
        u[38] = eff.glitterEnabled ? eff.glitterCount / 100 : 0.0;
        u[39] = eff.glitterEnabled ? eff.glitterSize / 100 : 0.0;
        u[40] = eff.cinderEnabled ? eff.cinderSize / 100 : 0.0;
        this.buffers.uploadUniforms();

        // Upload palette
        this.buffers.uploadPalette(themeColors.value);

        // Upload element data (may grow the buffer)
        this.buffers.uploadElements(data, count);

        // Rebuild bind groups if buffer was reallocated
        this.rebuildBindGroupsIfNeeded();

        // --- GPU encoding --------------------------------------------------
        const { device, context, computePipeline, renderPipeline, particlePipeline } = this.pipelines;
        const enc = device.createCommandEncoder();

        // Ensure depth buffer for 3D callbacks
        this.ensureDepth(this.canvas.width, this.canvas.height);

        // Compute pass: simulate particles (skip when all particle types disabled)
        if (particlesEnabled) {
            const computePass = enc.beginComputePass();
            computePass.setPipeline(computePipeline);
            computePass.setBindGroup(0, this.computeBindGroup);
            computePass.dispatchWorkgroups(Math.ceil(NUM_PARTICLES / COMPUTE_WORKGROUP));
            computePass.end();
        }

        // Render pass (with depth attachment for 3D callbacks)
        const renderPass = enc.beginRenderPass({
            colorAttachments: [{
                view:       context.getCurrentTexture().createView(),
                loadOp:     'clear',
                storeOp:    'store',
                clearValue: { r: 0, g: 0, b: 0, a: 1 },
            }],
            depthStencilAttachment: {
                view: this.depthView!,
                depthClearValue: 1,
                depthLoadOp: 'clear',
                depthStoreOp: 'store',
            },
        });

        // Draw 1: full-screen quad (aurora + elements + CRT)
        renderPass.setPipeline(renderPipeline);
        renderPass.setBindGroup(0, this.renderBindGroup);
        renderPass.draw(6);

        // Draw 2: instanced particle quads (skip when all disabled)
        if (particlesEnabled) {
            renderPass.setPipeline(particlePipeline);
            renderPass.setBindGroup(0, this.renderBindGroup);
            renderPass.draw(6, NUM_PARTICLES);
        }

        // Draw 3+: external overlay renderers
        for (const cb of getOverlayCallbacks()) {
            cb(renderPass, device, time, dt, this.canvas.width, this.canvas.height, this.depthView!);
        }

        renderPass.end();
        device.queue.submit([enc.finish()]);

        // --- One-shot capture ---
        const captureResolve = consumeCaptureRequest();
        if (captureResolve) {
            captureResolve(captureFrame(this.canvas));
        }

        this.animId = requestAnimationFrame(this.frame);
    };
}
