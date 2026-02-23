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
} from './element-types';
import { getOverlayCallbacks, consumeCaptureRequest } from './overlay-api';
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
        scanner.updateFrame();

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

        if (hoverIdx !== this.prevHoverIdx) {
            this.hoverStartTime = time;
            this.prevHoverIdx = hoverIdx;
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
        const eff = effectSettings.value;
        const crtOn = eff.crtEnabled;
        u[10] = crtOn ? eff.crtScanlinesH / 100 : 0.0;
        u[11] = crtOn ? eff.crtScanlinesV / 100 : 0.0;
        u[12] = crtOn ? eff.crtEdgeShadow / 100 : 0.0;
        u[13] = crtOn ? eff.crtFlicker / 100 : 0.0;
        u[14] = CURSOR_STYLE_VALUE[eff.cursorStyle] ?? 0;
        u[15] = eff.smokeIntensity / 100;
        u[16] = eff.smokeSpeed / 100;
        u[17] = eff.smokeWarmScale / 100;
        u[18] = eff.smokeCoolScale / 100;
        u[19] = eff.smokeFineScale / 100;
        u[20] = eff.grainIntensity / 100;
        u[21] = eff.grainCoarseness / 100;
        u[22] = eff.grainSize / 100;
        u[23] = eff.vignetteStrength / 100;
        u[24] = eff.underglowStrength / 100;
        u[25] = eff.sparkSpeed / 100;
        u[26] = eff.emberSpeed / 100;
        u[27] = eff.beamSpeed / 100;
        u[28] = eff.glitterSpeed / 100;
        u[29] = eff.beamHeight;
        u[30] = eff.beamCount;
        u[31] = eff.beamDrift / 100;
        u[32] = scrollDelta.dx;
        u[33] = scrollDelta.dy;
        u[34] = eff.sparkCount / 100;
        u[35] = eff.sparkSize / 100;
        u[36] = eff.emberCount / 100;
        u[37] = eff.emberSize / 100;
        u[38] = eff.glitterCount / 100;
        u[39] = eff.glitterSize / 100;
        u[40] = eff.cinderSize / 100;
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

        // Compute pass: simulate particles
        const computePass = enc.beginComputePass();
        computePass.setPipeline(computePipeline);
        computePass.setBindGroup(0, this.computeBindGroup);
        computePass.dispatchWorkgroups(Math.ceil(NUM_PARTICLES / COMPUTE_WORKGROUP));
        computePass.end();

        // Render pass
        const renderPass = enc.beginRenderPass({
            colorAttachments: [{
                view:       context.getCurrentTexture().createView(),
                loadOp:     'clear',
                storeOp:    'store',
                clearValue: { r: 0, g: 0, b: 0, a: 1 },
            }],
        });

        // Draw 1: full-screen quad (aurora + elements + CRT)
        renderPass.setPipeline(renderPipeline);
        renderPass.setBindGroup(0, this.renderBindGroup);
        renderPass.draw(6);

        // Draw 2: instanced particle quads (additive blend)
        renderPass.setPipeline(particlePipeline);
        renderPass.setBindGroup(0, this.renderBindGroup);
        renderPass.draw(6, NUM_PARTICLES);

        // Draw 3+: external overlay renderers
        for (const cb of getOverlayCallbacks()) {
            cb(renderPass, device, time, dt, this.canvas.width, this.canvas.height);
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
