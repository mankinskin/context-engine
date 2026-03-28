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

use js_sys::{Array, Float32Array, Object, Reflect};
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

// ── GpuInner ─────────────────────────────────────────────────────────────────

pub(crate) struct GpuInner {
    pub device: GpuDevice,
    pub queue: GpuQueue,
    pub canvas: HtmlCanvasElement,
    ctx: GpuCanvasContext,
    format: String,

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

pub async fn init_gpu(canvas: HtmlCanvasElement) -> Option<GpuInner> {
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
    jset(&cfg, "alphaMode", &js_str("opaque"));
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
        jset(&cv, "b", &js_f(0.0)); jset(&cv, "a", &js_f(1.0));
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
        self.queue.submit(&Array::of1(cmd.as_ref()));
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
    Reflect::apply(f.unchecked_ref(), pass, &args).unwrap_or(JsValue::UNDEFINED);
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
    u[0]  = time;
    u[1]  = w as f32;
    u[2]  = h as f32;
    u[3]  = elem_count as f32;
    u[4]  = w as f32 / 2.0; // mouse centre default
    u[5]  = h as f32 / 2.0;
    u[6]  = dt;
    u[7]  = -1.0; // hover = none
    u[9]  = -1.0; // selected = none
    u[15] = 0.4;  // smoke_intensity
    u[16] = 1.0; u[17] = 1.0; u[18] = 1.0; u[19] = 1.0; // smoke speeds
    u[20] = 0.3; u[21] = 0.5; u[22] = 0.5; // grain
    u[23] = 0.5;  // vignette
    u[25] = 1.0; u[26] = 1.0; u[27] = 1.0; u[28] = 1.0; // particle speeds
    u[29] = 35.0; // beam_height
    u[33] = 1.0; u[34] = 1.0; u[35] = 1.0; u[36] = 1.0; // spark/ember
    u[37] = 1.0; u[38] = 1.0; u[39] = 1.0; // glitter/cinder
    u[41] = 1.0;  // world_scale
    u[44] = w as f32; u[45] = h as f32; // vp dims
    u[46] = 5.0;  // current_view = hypergraph
    for off in [56usize, 72] { // identity particle VP matrices
        u[off] = 1.0; u[off+5] = 1.0; u[off+10] = 1.0; u[off+15] = 1.0;
    }
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
        let Some(inner) = init_gpu(canvas).await else {
            log::warn!("[overlay] WebGPU unavailable");
            return;
        };
        let gpu_rc = Rc::new(RefCell::new(inner));
        overlay.gpu.set_value(Some(SendWrapper::new(gpu_rc.clone())));
        run_loop(gpu_rc, overlay.callbacks);
    });
}

fn run_loop(gpu: Rc<RefCell<GpuInner>>, cbs: StoredValue<SendWrapper<Rc<RefCell<CallbackVec>>>>) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *f.borrow_mut() = Some(Closure::new(move |_ts: f64| {
        let cbs_sw = cbs.get_value();
        let cbs_borrow = cbs_sw.borrow();
        if let Ok(mut gpu_b) = gpu.try_borrow_mut() {
            gpu_b.render_frame(&cbs_borrow);
        }
        raf(&g);
    }));

    raf(&f);
}

fn raf(f: &Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>) {
    if let (Some(win), Some(cb)) = (web_sys::window(), f.borrow().as_ref()) {
        let _ = win.request_animation_frame(cb.as_ref().unchecked_ref());
    }
}
