/**
 * Public API surface for the WgpuOverlay system.
 * Signals, render callback registration, and capture trigger.
 */
import { signal } from '@preact/signals';

// ---------------------------------------------------------------------------
// GPU enabled toggle
// ---------------------------------------------------------------------------

export const gpuOverlayEnabled = signal(true);

// ---------------------------------------------------------------------------
// Shared GPU device + format for external renderers
// ---------------------------------------------------------------------------

/**
 * Exposes the shared GPU device + canvas format so external components
 * can create pipelines compatible with the overlay's render pass.
 * `null` when WebGPU is not available or the overlay is disabled.
 */
export const overlayGpu = signal<{ device: GPUDevice; format: GPUTextureFormat } | null>(null);

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

/** Read-only access to the set of registered callbacks (used by render loop). */
export function getOverlayCallbacks(): ReadonlySet<OverlayRenderCallback> {
    return _overlayCallbacks;
}

// ---------------------------------------------------------------------------
// One-shot frame capture (for theme thumbnails)
// ---------------------------------------------------------------------------

let _captureResolve: ((url: string) => void) | null = null;

/**
 * Request a low-res JPEG thumbnail of the next rendered frame.
 * The promise resolves after the frame is submitted to the GPU.
 */
export function captureOverlayThumbnail(): Promise<string> {
    return new Promise(resolve => {
        _captureResolve = resolve;
    });
}

/** Check if a capture is pending (used by render loop). */
export function consumeCaptureRequest(): ((url: string) => void) | null {
    const resolve = _captureResolve;
    _captureResolve = null;
    return resolve;
}

// ---------------------------------------------------------------------------
// Scan dirty trigger — delegates to the live ElementScanner instance
// ---------------------------------------------------------------------------

/** Callback set by WgpuOverlay component when scanner is created/destroyed. */
let _scanInvalidator: (() => void) | null = null;

/** Register/unregister the scanner's invalidateAll method. */
export function setScanInvalidator(fn: (() => void) | null): void {
    _scanInvalidator = fn;
}

/**
 * Mark the element scan as dirty so positions are re-queried on the next
 * animation frame.  Call this from any component that moves, adds, or
 * removes overlay-tracked DOM elements (e.g. HypergraphView after
 * repositioning nodes).
 */
export function markOverlayScanDirty(): void {
    _scanInvalidator?.();
}
