/// WebGPU overlay — port of the TypeScript `WgpuOverlay` system.
///
/// Manages a GPU canvas that renders:
///   - **background** : atmospheric smoke + per-node GPU card effects (background.wgsl)
///   - **compute**    : particle physics simulation (compute.wgsl)
///   - **particles**  : instanced particle rendering (particles.wgsl)
///   - **callbacks**  : 3-D overlay renderers (e.g. Scene3D)
///
/// All GPU descriptors are built as `js_sys::Object` + `Reflect::set` and cast to
/// typed web-sys parameter types via `JsCast::unchecked_ref()`.  This avoids IDL
/// overload complexity and is compatible across all web-sys 0.3.x releases.
use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{Array, Float32Array, Function, Object, Reflect};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    GpuAdapter, GpuBindGroup, GpuBuffer, GpuCanvasContext, GpuCommandBuffer, GpuCommandEncoder,
    GpuComputePassEncoder, GpuDevice, GpuQueue, GpuRenderPassEncoder, GpuTexture, GpuTextureView,
    HtmlCanvasElement,
};

use crate::theme;

// ── Embedded WGSL shaders ─────────────────────────────────────────────────────

const PALETTE_WGSL: &str =
    include_str!("../../../frontend/src/effects/palette.wgsl");
const PARTICLE_SHADING_WGSL: &str =
    include_str!("../../../frontend/src/effects/particle-shading.wgsl");
const TYPES_WGSL: &str =
    include_str!("../../../frontend/src/components/WgpuOverlay/types.wgsl");
const NOISE_WGSL: &str =
    include_str!("../../../frontend/src/components/WgpuOverlay/noise.wgsl");
const BACKGROUND_WGSL: &str =
    include_str!("../../../frontend/src/components/WgpuOverlay/background.wgsl");
const PARTICLES_WGSL: &str =
    include_str!("../../../frontend/src/components/WgpuOverlay/particles.wgsl");
const COMPUTE_WGSL: &str =
    include_str!("../../../frontend/src/components/WgpuOverlay/compute.wgsl");

// ── Constants ─────────────────────────────────────────────────────────────────

const ELEM_FLOATS: usize = 8;
const MAX_ELEMS: usize = 128;
const NUM_PARTICLES: usize = 640;
const PARTICLE_FLOATS: usize = 12;
const UNIFORMS_BYTES: usize = 352;
const PALETTE_BYTES: usize = 384;

// ── Render callback type ──────────────────────────────────────────────────────

/// Per-frame render callback. The callback may issue draw calls into the active
/// render pass but must NOT end the pass or submit to the queue.
pub type RenderCallback = Box<
    dyn Fn(
        &GpuRenderPassEncoder,
        &GpuDevice,
        &GpuQueue,
        f64,  // elapsed seconds
        f64,  // delta seconds
        u32,  // canvas_w
        u32,  // canvas_h
        Option<&GpuTextureView>, // shared depth view
    ),
>;

type CallbackVec = Vec<RenderCallback>;

// ── OverlayContext ────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct OverlayContext {
    pub gpu: StoredValue<Option<SendWrapper<Rc<RefCell<GpuInner>>>>>,
    /// Set to `true` (once) by `start_overlay` after WebGPU init completes.
    /// Child components may watch this reactive signal to begin GPU work.
    pub gpu_ready: RwSignal<bool>,
    pub callbacks: StoredValue<SendWrapper<Rc<RefCell<CallbackVec>>>>,
}

impl OverlayContext {
    /// Register a render callback. Returns an opaque ID for later removal.
    pub fn register(&self, cb: RenderCallback) -> usize {
        let cbs_sw = self.callbacks.get_value();
        let mut v = cbs_sw.borrow_mut();
        let id = v.len();
        v.push(cb);
        id
    }

    /// Replace the callback at `id` with a no-op.
    pub fn unregister(&self, id: usize) {
        let cbs_sw = self.callbacks.get_value();
        let mut v = cbs_sw.borrow_mut();
        if let Some(slot) = v.get_mut(id) {
            *slot = Box::new(|_, _, _, _, _, _, _, _| {});
        }
    }
}

// ── Camera VP (for 3D smoke) ─────────────────────────────────────────────────

struct ParticleCam {
    eye:    [f32; 3],
    vp:     [f32; 16],
    inv_vp: [f32; 16],
}

impl Default for ParticleCam {
    fn default() -> Self {
        // Default: perspective-like projection so smoke shader's
        // screen_to_ray_dir produces sane world-space rays before
        // HypergraphView starts writing real camera data.
        //
        // Place the eye 5 units back on Z, looking at origin.
        // VP = simple perspective(fov=60°, aspect=1, near=0.1, far=100)
        let fov_y: f32 = 60.0_f32.to_radians();
        let f = 1.0 / (fov_y / 2.0).tan(); // ~1.732
        let near = 0.1_f32;
        let far  = 100.0_f32;
        let nf = near - far;
        #[rustfmt::skip]
        let vp = [
            f, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (far + near) / nf, -1.0,
            0.0, 0.0, 2.0 * far * near / nf, 0.0,
        ];
        // Compute a simple inverse for this symmetric perspective matrix.
        let inv_f = 1.0 / f;
        let d = 2.0 * far * near / nf; // same as vp[14]
        let c = (far + near) / nf;     // same as vp[10]
        #[rustfmt::skip]
        let inv_vp = [
            inv_f, 0.0, 0.0, 0.0,
            0.0, inv_f, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0 / d,
            0.0, 0.0, -1.0, c / d,
        ];
        ParticleCam { eye: [0.0, 0.0, 5.0], vp, inv_vp }
    }
}

thread_local! {
    static PARTICLE_CAM: RefCell<ParticleCam> = RefCell::new(ParticleCam::default());
    /// Global GPU overlay enable flag — toggled from the Settings panel.
    static GPU_OVERLAY_ENABLED: RefCell<bool> = RefCell::new(true);
}

/// Enable or disable the GPU overlay globally.
pub fn set_gpu_overlay_enabled(v: bool) {
    GPU_OVERLAY_ENABLED.with(|e| *e.borrow_mut() = v);
}

/// Returns `true` if the GPU overlay is currently enabled.
pub fn is_gpu_overlay_enabled() -> bool {
    GPU_OVERLAY_ENABLED.with(|e| *e.borrow())
}

/// Called by the HypergraphView render callback every frame to feed the
/// camera matrix into the background smoke shader.
pub fn set_particle_cam(vp: [f32; 16], inv_vp: [f32; 16], eye: [f32; 3]) {
    PARTICLE_CAM.with(|cam| {
        let mut c = cam.borrow_mut();
        c.vp     = vp;
        c.inv_vp = inv_vp;
        c.eye    = eye;
    });
}

// ── GpuInner ─────────────────────────────────────────────────────────────────

pub(crate) struct GpuInner {
    pub device: GpuDevice,
    pub queue: GpuQueue,
    pub canvas: HtmlCanvasElement,
    ctx: GpuCanvasContext,
    pub format: String,

    bg_pipeline: JsValue,
    particle_pipeline: JsValue,
    compute_pipeline: JsValue,
    render_bgl: JsValue,
    compute_bgl: JsValue,

    uniform_buf: GpuBuffer,
    palette_buf: GpuBuffer,
    elem_buf: GpuBuffer,
    particle_buf: GpuBuffer,

    depth_tex: Option<GpuTexture>,
    depth_view: Option<GpuTextureView>,
    depth_size: (u32, u32),

    start_time: f64,
    last_time: f64,
}

// ── JS helpers ────────────────────────────────────────────────────────────────

fn obj() -> Object { Object::new() }

fn jset(o: &Object, key: &str, val: &JsValue) {
    Reflect::set(o, &JsValue::from_str(key), val).unwrap();
}

fn js_str(s: &str) -> JsValue { JsValue::from_str(s) }
fn js_f(n: f64) -> JsValue { JsValue::from_f64(n) }

// ── GPU init ──────────────────────────────────────────────────────────────────

/// Resize the canvas to match the viewport at the current devicePixelRatio.
pub fn resize_canvas_to_viewport(canvas: &HtmlCanvasElement) {
    let Some(win) = web_sys::window() else { return };
    let dpr = win.device_pixel_ratio();
    let css_w = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
    let css_h = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0);
    canvas.set_width((css_w * dpr) as u32);
    canvas.set_height((css_h * dpr) as u32);
}

pub async fn init_gpu(canvas: HtmlCanvasElement) -> Option<GpuInner> {
    // Set HiDPI canvas dimensions before configuring the WebGPU context.
    resize_canvas_to_viewport(&canvas);

    let navigator = web_sys::window()?.navigator();
    let gpu = navigator.gpu();

    let adapter: GpuAdapter = JsFuture::from(gpu.request_adapter())
        .await.ok()?.dyn_into().ok()?;
    let device: GpuDevice = JsFuture::from(adapter.request_device())
        .await.ok()?.dyn_into().ok()?;
    let queue: GpuQueue = device.queue().dyn_into().ok()?;

    let ctx: GpuCanvasContext = canvas.get_context("webgpu").ok()??.dyn_into().ok()?;
    let format_js: JsValue = gpu.get_preferred_canvas_format().into();
    let format = format_js.as_string().unwrap_or_else(|| "bgra8unorm".to_string());

    let cfg = obj();
    jset(&cfg, "device", device.as_ref());
    jset(&cfg, "format", &format_js);
    jset(&cfg, "alphaMode", &js_str("premultiplied"));
    ctx.configure(cfg.unchecked_ref());

    let uniform_buf = mk_buf(&device, UNIFORMS_BYTES as u64, 0x40 | 0x08)?;
    let palette_buf = mk_buf(&device, PALETTE_BYTES as u64, 0x40 | 0x08)?;
    let elem_buf = mk_buf(&device, (MAX_ELEMS * ELEM_FLOATS * 4) as u64, 0x88)?;
    let particle_buf = mk_buf(&device, (NUM_PARTICLES * PARTICLE_FLOATS * 4) as u64, 0x88)?;

    let shared = format!("{}\n{}\n{}\n", PALETTE_WGSL, TYPES_WGSL, NOISE_WGSL);
    let render_shared = format!("{}{}\n", shared, PARTICLE_SHADING_WGSL);
    let bg_shader   = mk_shader(&device, &format!("{}{}", render_shared, BACKGROUND_WGSL));
    let par_shader  = mk_shader(&device, &format!("{}{}", render_shared, PARTICLES_WGSL));
    let comp_shader = mk_shader(&device, &format!("{}{}", shared, COMPUTE_WGSL));

    let render_bgl  = mk_render_bgl(&device);
    let compute_bgl = mk_compute_bgl(&device);
    let r_layout = mk_pipeline_layout(&device, &[&render_bgl]);
    let c_layout = mk_pipeline_layout(&device, &[&compute_bgl]);

    let bg_pipeline = mk_render_pipeline(&device, &r_layout, &bg_shader, "vs_main", "fs_main", &format,
        Some(("src-alpha", "one-minus-src-alpha", "add")));
    let particle_pipeline = mk_render_pipeline(&device, &r_layout, &par_shader, "vs_particle", "fs_particle", &format,
        Some(("one", "one-minus-src-alpha", "add")));

    let cs = obj();
    jset(&cs, "module", comp_shader.as_ref());
    jset(&cs, "entryPoint", &js_str("cs_main"));
    let cd = obj();
    jset(&cd, "layout", &c_layout);
    jset(&cd, "compute", cs.as_ref());
    let compute_pipeline = JsValue::from(device.create_compute_pipeline(cd.unchecked_ref()));

    let now = perf_now();
    Some(GpuInner {
        device, queue, canvas, ctx, format,
        bg_pipeline, particle_pipeline, compute_pipeline,
        render_bgl, compute_bgl,
        uniform_buf, palette_buf, elem_buf, particle_buf,
        depth_tex: None, depth_view: None, depth_size: (0, 0),
        start_time: now, last_time: now,
    })
}

// ── Render frame ──────────────────────────────────────────────────────────────

impl GpuInner {
    pub(crate) fn render_frame(&mut self, callbacks: &[RenderCallback]) {
        if !is_gpu_overlay_enabled() { return; }
        let w = self.canvas.width();
        let h = self.canvas.height();
        if w == 0 || h == 0 { return; }

        self.ensure_depth(w, h);

        let now = perf_now();
        let time = (now - self.start_time) / 1000.0;
        let dt = f64::min((now - self.last_time) / 1000.0, 0.1);
        self.last_time = now;

        let (elem_data, elem_count) = scan_elements();
        if !elem_data.is_empty() { write_f32(&self.queue, &self.elem_buf, &elem_data); }

        let uniforms = build_uniforms(time as f32, dt as f32, w, h, elem_count);
        write_f32(&self.queue, &self.uniform_buf, &uniforms);

        // Upload palette colors from the active theme.
        theme::with_palette_data(|pal| {
            write_f32(&self.queue, &self.palette_buf, pal);
        });

        let render_bg  = mk_bind_group(&self.device, &self.render_bgl,  &[(&self.uniform_buf, 0), (&self.elem_buf, 1), (&self.particle_buf, 2), (&self.palette_buf, 3)]);
        let compute_bg = mk_bind_group(&self.device, &self.compute_bgl, &[(&self.uniform_buf, 0), (&self.elem_buf, 1), (&self.particle_buf, 2), (&self.palette_buf, 3)]);

        let encoder: GpuCommandEncoder = self.device.create_command_encoder().dyn_into().unwrap();

        // Compute pass
        let cp: GpuComputePassEncoder = encoder.begin_compute_pass().dyn_into().unwrap();
        cp.set_pipeline(self.compute_pipeline.unchecked_ref());
        call_set_bind_group(cp.as_ref(), 0, &compute_bg);
        cp.dispatch_workgroups(((NUM_PARTICLES + 63) / 64) as u32);
        cp.end();

        // Render pass
        let color_tex = self.ctx.get_current_texture().unwrap();
        let color_view = color_tex.create_view().unwrap();
        let ca = obj();
        jset(&ca, "view", color_view.as_ref());
        let cv = obj();
        jset(&cv, "r", &js_f(0.0)); jset(&cv, "g", &js_f(0.0));
        jset(&cv, "b", &js_f(0.0)); jset(&cv, "a", &js_f(0.0));
        jset(&ca, "clearValue", cv.as_ref());
        jset(&ca, "loadOp", &js_str("clear"));
        jset(&ca, "storeOp", &js_str("store"));
        let pd = obj();
        jset(&pd, "colorAttachments", Array::of1(ca.as_ref()).as_ref());
        if let Some(ref dv) = self.depth_view {
            let ds = obj();
            jset(&ds, "view", dv.as_ref());
            jset(&ds, "depthClearValue", &js_f(1.0));
            jset(&ds, "depthLoadOp", &js_str("clear"));
            jset(&ds, "depthStoreOp", &js_str("store"));
            jset(&pd, "depthStencilAttachment", ds.as_ref());
        }

        let pass: GpuRenderPassEncoder = encoder.begin_render_pass(pd.unchecked_ref()).unwrap();
        pass.set_pipeline(self.bg_pipeline.unchecked_ref());
        call_set_bind_group(pass.as_ref(), 0, &render_bg);
        pass.draw(6);

        pass.set_pipeline(self.particle_pipeline.unchecked_ref());
        call_set_bind_group(pass.as_ref(), 0, &render_bg);
        pass.draw_with_instance_count(6, NUM_PARTICLES as u32);

        for cb in callbacks {
            cb(&pass, &self.device, &self.queue, time, dt, w, h, self.depth_view.as_ref());
        }
        pass.end();

        let cmd: GpuCommandBuffer = encoder.finish().dyn_into().unwrap();
        self.queue.submit(&[cmd]);
    }

    fn ensure_depth(&mut self, w: u32, h: u32) {
        if self.depth_size == (w, h) { return; }
        if let Some(t) = self.depth_tex.take() { t.destroy(); }
        self.depth_view = None;
        let sz = obj();
        jset(&sz, "width", &js_f(w as f64));
        jset(&sz, "height", &js_f(h as f64));
        let desc = obj();
        jset(&desc, "size", sz.as_ref());
        jset(&desc, "format", &js_str("depth24plus"));
        jset(&desc, "usage", &js_f((0x10u32 | 0x04u32) as f64));
        let tex = self.device.create_texture(desc.unchecked_ref()).unwrap();
        self.depth_view  = Some(tex.create_view().unwrap());
        self.depth_tex   = Some(tex);
        self.depth_size  = (w, h);
    }
}

// ── BGL / pipeline builders ───────────────────────────────────────────────────

fn bgl_entry(binding: u32, vis: f64, ty: &str) -> Object {
    let e = obj();
    jset(&e, "binding", &js_f(binding as f64));
    jset(&e, "visibility", &js_f(vis));
    let b = obj(); jset(&b, "type", &js_str(ty));
    jset(&e, "buffer", b.as_ref());
    e
}

fn mk_render_bgl(device: &GpuDevice) -> JsValue {
    let vf = 3.0; // VERTEX | FRAGMENT
    let entries = Array::of4(
        &bgl_entry(0, vf, "uniform"),
        &bgl_entry(1, vf, "read-only-storage"),
        &bgl_entry(2, vf, "read-only-storage"),
        &bgl_entry(3, 2.0, "uniform"),
    );
    let d = obj(); jset(&d, "entries", entries.as_ref());
    JsValue::from(device.create_bind_group_layout(d.unchecked_ref()).unwrap())
}

fn mk_compute_bgl(device: &GpuDevice) -> JsValue {
    let c = 4.0; // COMPUTE
    let entries = Array::of4(
        &bgl_entry(0, c, "uniform"),
        &bgl_entry(1, c, "read-only-storage"),
        &bgl_entry(2, c, "storage"),
        &bgl_entry(3, c, "uniform"),
    );
    let d = obj(); jset(&d, "entries", entries.as_ref());
    JsValue::from(device.create_bind_group_layout(d.unchecked_ref()).unwrap())
}

fn mk_pipeline_layout(device: &GpuDevice, bgls: &[&JsValue]) -> JsValue {
    let arr = Array::new();
    for b in bgls { arr.push(b); }
    let d = obj(); jset(&d, "bindGroupLayouts", arr.as_ref());
    JsValue::from(device.create_pipeline_layout(d.unchecked_ref()))
}

fn mk_render_pipeline(device: &GpuDevice, layout: &JsValue, shader: &JsValue,
    vs: &str, fs: &str, format: &str, blend: Option<(&str, &str, &str)>) -> JsValue
{
    let vs_s = obj(); jset(&vs_s, "module", shader); jset(&vs_s, "entryPoint", &js_str(vs));
    let tgt = obj(); jset(&tgt, "format", &js_str(format));
    if let Some((src, dst, op)) = blend {
        let bc = obj();
        jset(&bc, "srcFactor", &js_str(src));
        jset(&bc, "dstFactor", &js_str(dst));
        jset(&bc, "operation", &js_str(op));
        let bl = obj(); jset(&bl, "color", bc.as_ref()); jset(&bl, "alpha", bc.as_ref());
        jset(&tgt, "blend", bl.as_ref());
    }
    let fs_s = obj();
    jset(&fs_s, "module", shader); jset(&fs_s, "entryPoint", &js_str(fs));
    jset(&fs_s, "targets", Array::of1(tgt.as_ref()).as_ref());
    let prim = obj(); jset(&prim, "topology", &js_str("triangle-list"));
    let ds = obj();
    jset(&ds, "format", &js_str("depth24plus"));
    jset(&ds, "depthWriteEnabled", &JsValue::from_bool(false));
    jset(&ds, "depthCompare", &js_str("always"));
    let d = obj();
    jset(&d, "layout", layout); jset(&d, "vertex", vs_s.as_ref());
    jset(&d, "fragment", fs_s.as_ref()); jset(&d, "primitive", prim.as_ref());
    jset(&d, "depthStencil", ds.as_ref());
    JsValue::from(device.create_render_pipeline(d.unchecked_ref()).unwrap())
}

fn mk_shader(device: &GpuDevice, code: &str) -> JsValue {
    let d = obj(); jset(&d, "code", &js_str(code));
    JsValue::from(device.create_shader_module(d.unchecked_ref()))
}

fn mk_buf(device: &GpuDevice, size: u64, usage: u32) -> Option<GpuBuffer> {
    let d = obj();
    jset(&d, "size", &js_f(size as f64));
    jset(&d, "usage", &js_f(usage as f64));
    device.create_buffer(d.unchecked_ref()).ok()
}

fn mk_bind_group(device: &GpuDevice, bgl: &JsValue, bindings: &[(&GpuBuffer, u32)]) -> GpuBindGroup {
    let entries = Array::new();
    for (buf, binding) in bindings {
        let res = obj(); jset(&res, "buffer", buf.as_ref());
        let e = obj();
        jset(&e, "binding", &js_f(*binding as f64));
        jset(&e, "resource", res.as_ref());
        entries.push(e.as_ref());
    }
    let d = obj(); jset(&d, "layout", bgl); jset(&d, "entries", entries.as_ref());
    device.create_bind_group(d.unchecked_ref())
}

/// Call `setBindGroup(index, bindGroup)` via Reflect to avoid overload matching.
fn call_set_bind_group(pass: &JsValue, index: u32, bg: &GpuBindGroup) {
    let f = Reflect::get(pass, &js_str("setBindGroup")).unwrap();
    let args = Array::of2(&js_f(index as f64), bg.as_ref());
    let func: &Function = f.unchecked_ref();
    Reflect::apply(func, pass, &args).unwrap_or(JsValue::UNDEFINED);
}

// ── Buffer write ──────────────────────────────────────────────────────────────

pub(crate) fn write_f32(queue: &GpuQueue, buf: &GpuBuffer, data: &[f32]) {
    let arr = Float32Array::from(data);
    let u8_arr = js_sys::Uint8Array::new(&arr.buffer());
    let _ = queue.write_buffer_with_u32_and_u8_array(buf, 0, &u8_arr);
}

// ── Uniforms ──────────────────────────────────────────────────────────────────

fn build_uniforms(time: f32, dt: f32, w: u32, h: u32, elem_count: usize) -> Vec<f32> {
    let mut u = vec![0.0f32; UNIFORMS_BYTES / 4];

    // ── Core ──────────────────────────────────────────────────────────────────
    u[0]  = time;
    u[1]  = w as f32;
    u[2]  = h as f32;
    u[3]  = elem_count as f32;
    u[4]  = w as f32 / 2.0;  // mouse_x (center default)
    u[5]  = h as f32 / 2.0;  // mouse_y
    u[6]  = dt;
    u[7]  = -1.0;             // hover_elem  (none)
    u[9]  = -1.0;             // selected_elem (none)

    // ── Effect settings (percentage / 100) ────────────────────────────────────
    theme::with_effect_settings(|e| {
        let p = |v: f32| v / 100.0;

        // CRT [10‥14]
        if e.crt_enabled {
            u[10] = p(e.crt_scanlines_h);
            u[11] = p(e.crt_scanlines_v);
            u[12] = p(e.crt_edge_shadow);
            u[13] = p(e.crt_flicker);
            u[14] = p(e.crt_line_width);
        }

        // Smoke [15‥19]
        u[15] = if e.smoke_enabled { p(e.smoke_intensity) } else { 0.0 };
        u[16] = p(e.smoke_speed);
        u[17] = p(e.smoke_warm_scale);
        u[18] = p(e.smoke_cool_scale);
        u[19] = p(e.smoke_moss_scale);

        // Grain / vignette / underglow [20‥24]
        u[20] = p(e.grain_intensity);
        u[21] = p(e.grain_coarseness);
        u[22] = p(e.grain_size);
        u[23] = p(e.vignette_strength);
        u[24] = p(e.underglow_strength);

        // Particle speeds [25‥28]
        u[25] = if e.sparks_enabled  { p(e.spark_speed) }   else { 0.0 };
        u[26] = if e.embers_enabled  { p(e.ember_speed) }   else { 0.0 };
        u[27] = if e.beams_enabled   { p(e.beam_speed) }    else { 0.0 };
        u[28] = if e.glitter_enabled { p(e.glitter_speed) } else { 0.0 };

        // Beam params [29‥31]
        u[29] = e.beam_height;
        u[30] = e.beam_count;
        u[31] = p(e.beam_drift);

        // scroll_dx/dy [32,33] = 0

        // Particle counts/sizes [34‥40]
        u[34] = if e.sparks_enabled  { p(e.spark_count) }   else { 0.0 };
        u[35] = if e.sparks_enabled  { p(e.spark_size) }    else { 0.0 };
        u[36] = if e.embers_enabled  { p(e.ember_count) }   else { 0.0 };
        u[37] = if e.embers_enabled  { p(e.ember_size) }    else { 0.0 };
        u[38] = if e.glitter_enabled { p(e.glitter_count) } else { 0.0 };
        u[39] = if e.glitter_enabled { p(e.glitter_size) }  else { 0.0 };
        u[40] = if e.cinder_enabled  { p(e.cinder_size) }   else { 0.0 };

        // CRT color [48‥50]
        u[48] = e.crt_color[0] / 255.0;
        u[49] = e.crt_color[1] / 255.0;
        u[50] = e.crt_color[2] / 255.0;
    });

    // ── Viewport / view ───────────────────────────────────────────────────────
    u[42] = 1.0;              // world_scale
    u[45] = w as f32;         // vp_w
    u[46] = h as f32;         // vp_h
    u[47] = 5.0;              // current_view = 5 (hypergraph — enables 3D smoke)

    // ── Camera VP for 3D triplanar smoke ──────────────────────────────────────
    PARTICLE_CAM.with(|cam| {
        let c = cam.borrow();
        u[52] = c.eye[0]; u[53] = c.eye[1]; u[54] = c.eye[2];
        u[56..72].copy_from_slice(&c.vp);
        u[72..88].copy_from_slice(&c.inv_vp);
    });
    u
}

// ── Element scanner ───────────────────────────────────────────────────────────

pub fn scan_elements() -> (Vec<f32>, usize) {
    let mut data = Vec::with_capacity(MAX_ELEMS * ELEM_FLOATS);
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return (data, 0);
    };
    let Ok(nodes) = doc.query_selector_all(".hg-node") else {
        return (data, 0);
    };
    let count = (nodes.length() as usize).min(MAX_ELEMS);
    for i in 0..count {
        let Some(el) = nodes.item(i as u32) else { continue };
        let rect = el.unchecked_ref::<web_sys::Element>().get_bounding_client_rect();
        data.extend_from_slice(&[
            rect.left() as f32, rect.top() as f32,
            rect.width() as f32, rect.height() as f32,
            i as f32 / count.max(1) as f32, // hue
            0.0, 0.0, 0.0, // kind, depth, pad
        ]);
    }
    (data, count)
}

// ── perf_now ──────────────────────────────────────────────────────────────────

fn perf_now() -> f64 {
    web_sys::window().and_then(|w| w.performance()).map(|p| p.now()).unwrap_or(0.0)
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn start_overlay(overlay: OverlayContext, canvas: HtmlCanvasElement) {
    spawn_local(async move {
        // Clone the canvas handle before init_gpu takes ownership — needed for
        // the window resize listener registered below.
        let canvas_for_resize = canvas.clone();

        let Some(inner) = init_gpu(canvas).await else {
            log::warn!("[overlay] WebGPU unavailable");
            return;
        };
        let gpu_rc = Rc::new(RefCell::new(inner));
        // Guard: if the component was disposed while GPU was initialising, bail out.
        let stored = overlay.gpu.try_update_value(|v| {
            *v = Some(SendWrapper::new(gpu_rc.clone()));
        });
        if stored.is_none() { return; }

        // Activate glass-mode CSS on the document root.
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let _ = doc.document_element().map(|el| {
                let _ = el.class_list().add_1("gpu-active");
            });
        }

        // Window resize listener — keeps canvas pixel dimensions in sync with
        // the viewport and devicePixelRatio.
        let resize_cb = Closure::<dyn FnMut()>::new(move || {
            resize_canvas_to_viewport(&canvas_for_resize);
        });
        if let Some(win) = web_sys::window() {
            let win_js: &JsValue = win.as_ref();
            let add_fn = Reflect::get(win_js, &JsValue::from_str("addEventListener")).unwrap_or_default();
            Reflect::apply(
                add_fn.unchecked_ref::<Function>(),
                win_js,
                &Array::of2(&JsValue::from_str("resize"), resize_cb.as_ref()),
            ).ok();
        }
        resize_cb.forget();

        // Signal to reactive Effects that GPU is now ready.
        overlay.gpu_ready.set(true);
        run_loop(gpu_rc, overlay.callbacks);
    });
}

fn run_loop(gpu: Rc<RefCell<GpuInner>>, cbs: StoredValue<SendWrapper<Rc<RefCell<CallbackVec>>>>) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *f.borrow_mut() = Some(Closure::new(move |_ts: f64| {
        // try_with_value returns None if the StoredValue has been disposed
        // (component unmounted). Stop the loop by not rescheduling.
        let alive = cbs.try_with_value(|cbs_sw| {
            let cbs_borrow = cbs_sw.borrow();
            if let Ok(mut gpu_b) = gpu.try_borrow_mut() {
                gpu_b.render_frame(&cbs_borrow);
            }
        });
        if alive.is_some() {
            raf(&g);
        }
    }));

    raf(&f);
}

fn raf(f: &Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>) {
    if let (Some(win), Some(cb)) = (web_sys::window(), f.borrow().as_ref()) {
        let _ = win.request_animation_frame(cb.as_ref().unchecked_ref());
    }
}
