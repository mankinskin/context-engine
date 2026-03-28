/// Scene3D — Leptos component that renders hypergraph nodes as 3-D cubes.
///
/// Works entirely inside the shared WebGPU overlay canvas provided by
/// `OverlayContext`.  On mount it registers a `RenderCallback` that is called
/// every frame by the overlay render loop.  On reactive unmount it removes
/// itself from the callback list.
///
/// # Pipeline
/// Uses `scene3d.wgsl` (Blinn-Phong cubes + grid floor) which is independent
/// of the background / particle overlay shaders.
///
/// # Uniforms (per draw)  — must match scene3d.wgsl `Uniforms` struct
/// ```text
/// viewProj  mat4x4  offset   0  (64 bytes)
/// model     mat4x4  offset  64  (64 bytes)
/// color     vec4   offset 128  (16 bytes)
/// lightDir  vec4   offset 144  (16 bytes)
/// cameraPos vec4   offset 160  (16 bytes)
/// flags     vec4   offset 176  (16 bytes)  (x=isGround, y=isHovered, z=time, w=0)
/// ```
/// Total = 192 bytes = 48 f32.
use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{Array, Float32Array, Object, Reflect};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{GpuDevice, GpuQueue, GpuRenderPassEncoder, GpuTextureView};

use crate::gpu::math3d::{
    mat4_look_at, mat4_multiply, mat4_perspective, mat4_scale,
    mat4_translate, Mat4, Vec3,
};
use crate::gpu::overlay::{OverlayContext, RenderCallback};
use crate::types::HypergraphSnapshot;

// ── Embedded scene3d shader ───────────────────────────────────────────────────

const SCENE3D_WGSL: &str =
    include_str!("../../../frontend/src/components/Scene3D/scene3d.wgsl");

// ── Constants ─────────────────────────────────────────────────────────────────

const SCENE3D_UNIFORM_BYTES: usize = 192;
const CAMERA_FOV: f32 = std::f32::consts::FRAC_PI_4; // 45 degrees
const CAMERA_NEAR: f32 = 0.1;
const CAMERA_FAR: f32 = 200.0;
const NODE_SCALE: f32 = 0.9;
const ATOM_SCALE: f32 = 0.6;
const GRID_SPACING: f32 = 2.5;

// ── Camera state ──────────────────────────────────────────────────────────────

#[derive(Clone)]
struct Camera {
    yaw: f32,
    pitch: f32,
    distance: f32,
    target: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            yaw: 0.3,
            pitch: 0.4,
            distance: 12.0,
            target: [0.0, 0.0, 0.0],
        }
    }
}

impl Camera {
    fn eye(&self) -> Vec3 {
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();
        [
            self.target[0] + self.distance * cy * cp,
            self.target[1] + self.distance * sp,
            self.target[2] + self.distance * sy * cp,
        ]
    }

    fn view_proj(&self, width: u32, height: u32) -> Mat4 {
        let aspect = width as f32 / height.max(1) as f32;
        let proj = mat4_perspective(CAMERA_FOV, aspect, CAMERA_NEAR, CAMERA_FAR);
        let view = mat4_look_at(self.eye(), self.target, [0.0, 1.0, 0.0]);
        mat4_multiply(proj, view)
    }
}

// ── Scene3D GPU resources ─────────────────────────────────────────────────────

struct Scene3DResources {
    pipeline: JsValue,
    bgl: JsValue,
    vertex_buf: web_sys::GpuBuffer,
    ground_buf: web_sys::GpuBuffer,
    uniform_buf: web_sys::GpuBuffer,
}

// ── Leptos component ──────────────────────────────────────────────────────────

/// Registers a 3-D cube render callback in the current `OverlayContext`.
///
/// Expects `OverlayContext` to be accessible via `expect_context`.
/// `snapshot` drives the layout: each node becomes one cube.
#[component]
pub fn Scene3D(snapshot: HypergraphSnapshot) -> impl IntoView {
    let overlay = expect_context::<OverlayContext>();
    let resources: StoredValue<Option<SendWrapper<Rc<RefCell<Scene3DResources>>>>> =
        StoredValue::new(None);
    let camera: Rc<RefCell<Camera>> = Rc::new(RefCell::new(Camera::default()));
    let callback_id: StoredValue<Option<usize>> = StoredValue::new(None);

    // Layout: compute a flat 3-D position per node
    let node_positions: Vec<(Vec3, bool, [f32; 4])> = layout_nodes(&snapshot);

    let camera_clone = camera.clone();
    let snapshot_clone = snapshot.clone();

    // Register a render callback once the overlay GPU state is available.
    // We use Effect to re-run if the GPU becomes available (e.g. after async init).
    Effect::new(move |_| {
        let gpu_opt = overlay.gpu.get_value();
        let Some(ref gpu_sw) = gpu_opt else { return };
        let gpu_borrow = gpu_sw.borrow();
        let device = &gpu_borrow.device;

        // Build scene3d resources on first Effect run
        if resources.get_value().is_none() {
            match build_resources(device) {
                Some(r) => {
                    resources.set_value(Some(SendWrapper::new(Rc::new(RefCell::new(r)))));
                }
                None => return,
            }
        }

        let res_sw = resources.get_value().unwrap();
        let positions = node_positions.clone();
        let cam = camera_clone.clone();
        let _sn = snapshot_clone.clone();

        let cb: RenderCallback = Box::new(
            move |pass: &GpuRenderPassEncoder,
                  device: &GpuDevice,
                  queue: &GpuQueue,
                  time: f64,
                  _dt: f64,
                  canvas_w: u32,
                  canvas_h: u32,
                  _depth: Option<&GpuTextureView>| {
                let res = res_sw.borrow();
                let cam_snap = cam.borrow().clone();
                let vp = cam_snap.view_proj(canvas_w, canvas_h);
                let eye = cam_snap.eye();

                pass.set_pipeline(res.pipeline.unchecked_ref());

                // Draw ground plane
                {
                    let mut u = [0f32; SCENE3D_UNIFORM_BYTES / 4];
                    write_mat4(&mut u, 0, &vp);
                    write_mat4(&mut u, 16, &mat4_scale([20.0, 1.0, 20.0]));
                    write_vec4(&mut u, 32, [0.06, 0.07, 0.09, 1.0]); // color
                    write_vec4(&mut u, 36, [0.5, -1.0, 0.8, 0.0]); // lightDir
                    write_vec4(&mut u, 40, [eye[0], eye[1], eye[2], 0.0]); // cameraPos
                    write_vec4(&mut u, 44, [1.0, 0.0, time as f32, 0.0]); // flags: isGround=1

                    write_uniform(queue, &res.uniform_buf, &u);
                    let bg = make_bind_group(device, &res.bgl, &res.uniform_buf);
                    call_set_bind_group(pass.as_ref(), 0, &bg);
                    pass.set_vertex_buffer(0, Some(res.ground_buf.as_ref()));
                    pass.draw(6); // ground quad (2 triangles)
                }

                // Draw one cube per node
                for (pos, is_atom, color) in &positions {
                    let scale_v = if *is_atom { ATOM_SCALE } else { NODE_SCALE };
                    let model = mat4_multiply(
                        mat4_translate(*pos),
                        mat4_scale([scale_v; 3]),
                    );

                    let mut u = [0f32; SCENE3D_UNIFORM_BYTES / 4];
                    write_mat4(&mut u, 0, &vp);
                    write_mat4(&mut u, 16, &model);
                    write_vec4(&mut u, 32, *color);
                    write_vec4(&mut u, 36, [0.5, -1.0, 0.8, 0.0]);
                    write_vec4(&mut u, 40, [eye[0], eye[1], eye[2], 0.0]);
                    write_vec4(&mut u, 44, [0.0, 0.0, time as f32, 0.0]);

                    write_uniform(queue, &res.uniform_buf, &u);
                    let bg = make_bind_group(device, &res.bgl, &res.uniform_buf);
                    call_set_bind_group(pass.as_ref(), 0, &bg);
                    pass.set_vertex_buffer(0, Some(res.vertex_buf.as_ref()));
                    pass.draw(36); // 12 triangles × 3 vertices
                }
            },
        );

        let id = overlay.register(cb);
        callback_id.set_value(Some(id));
    });

    // Unregister on cleanup
    on_cleanup(move || {
        if let Some(id) = callback_id.get_value() {
            overlay.unregister(id);
        }
    });

    // The component renders no DOM — the GPU canvas is managed by HypergraphView.
    view! { <></> }
}

// ── Layout algorithm ──────────────────────────────────────────────────────────

/// Compute 3-D positions for each node in a grid layout.
/// Returns `Vec<(position, is_atom, rgba_color)>`.
fn layout_nodes(snapshot: &HypergraphSnapshot) -> Vec<(Vec3, bool, [f32; 4])> {
    let n = snapshot.nodes.len();
    if n == 0 {
        return Vec::new();
    }
    let cols = (n as f32).sqrt().ceil() as usize;
    snapshot
        .nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let col = i % cols;
            let row = i / cols;
            let x = (col as f32 - cols as f32 / 2.0) * GRID_SPACING;
            let z = (row as f32 - (n / cols) as f32 / 2.0) * GRID_SPACING;
            let is_atom = node.width == 1;
            let y = if is_atom { 0.3 } else { 0.5 };
            let color = node_color(node.index, is_atom);
            ([x, y, z], is_atom, color)
        })
        .collect()
}

fn node_color(index: u32, is_atom: bool) -> [f32; 4] {
    let h = (index as f32 * 137.508) % 360.0; // golden angle hue
    let (r, g, b) = hsl_to_rgb(h / 360.0, 0.5, if is_atom { 0.55 } else { 0.45 });
    [r, g, b, 1.0]
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

// ── GPU resource construction ─────────────────────────────────────────────────

fn build_resources(device: &GpuDevice) -> Option<Scene3DResources> {
    let shader = create_shader_module(device, SCENE3D_WGSL);

    let bgl = make_bgl(device);
    let layout = make_pipeline_layout(device, &bgl);
    let pipeline = make_pipeline(device, &layout, &shader, &shader)?;

    let vertex_buf = create_vertex_buffer(device, &cube_vertices())?;
    let ground_buf = create_vertex_buffer(device, &ground_quad())?;
    let uniform_buf = create_uniform_buffer(device, SCENE3D_UNIFORM_BYTES as u64)?;

    Some(Scene3DResources {
        pipeline,
        bgl,
        vertex_buf,
        ground_buf,
        uniform_buf,
    })
}

fn create_shader_module(device: &GpuDevice, code: &str) -> JsValue {
    let desc = obj();
    set(&desc, "code", &js_str(code));
    JsValue::from(device.create_shader_module(desc.unchecked_ref()))
}

fn make_bgl(device: &GpuDevice) -> JsValue {
    let entry = obj();
    set(&entry, "binding", &JsValue::from_f64(0.0));
    set(
        &entry,
        "visibility",
        &JsValue::from_f64((1u32 | 2u32) as f64), // VERTEX | FRAGMENT
    );
    let buf = obj();
    set(&buf, "type", &js_str("uniform"));
    set(&entry, "buffer", buf.as_ref());
    let entries = Array::of1(entry.as_ref());
    let desc = obj();
    set(&desc, "entries", entries.as_ref());
    JsValue::from(device.create_bind_group_layout(desc.unchecked_ref()).unwrap())
}

fn make_pipeline_layout(device: &GpuDevice, bgl: &JsValue) -> JsValue {
    let bgls = Array::of1(bgl);
    let desc = obj();
    set(&desc, "bindGroupLayouts", bgls.as_ref());
    JsValue::from(device.create_pipeline_layout(desc.unchecked_ref()))
}

fn make_pipeline(
    device: &GpuDevice,
    layout: &JsValue,
    vs_module: &JsValue,
    fs_module: &JsValue,
) -> Option<JsValue> {
    // Vertex buffer layout: position (3×f32) + normal (3×f32) = stride 24
    let pos_attr = obj();
    set(&pos_attr, "shaderLocation", &JsValue::from_f64(0.0));
    set(&pos_attr, "offset", &JsValue::from_f64(0.0));
    set(&pos_attr, "format", &js_str("float32x3"));
    let norm_attr = obj();
    set(&norm_attr, "shaderLocation", &JsValue::from_f64(1.0));
    set(&norm_attr, "offset", &JsValue::from_f64(12.0)); // after 3×f32
    set(&norm_attr, "format", &js_str("float32x3"));
    let vbl = obj();
    set(&vbl, "arrayStride", &JsValue::from_f64(24.0));
    set(&vbl, "stepMode", &js_str("vertex"));
    set(
        &vbl,
        "attributes",
        Array::of2(pos_attr.as_ref(), norm_attr.as_ref()).as_ref(),
    );

    let vertex_state = obj();
    set(&vertex_state, "module", vs_module);
    set(&vertex_state, "entryPoint", &js_str("vs_main"));
    set(
        &vertex_state,
        "buffers",
        Array::of1(vbl.as_ref()).as_ref(),
    );

    let target = obj();
    // No explicit format here — will be resolved at runtime from the canvas preferred format.
    // We keep it as "bgra8unorm" as a safe default (overridden below).
    let preferred = web_sys::window()
        .map(|w| {
            let f: JsValue = w.navigator().gpu().get_preferred_canvas_format().into();
            f.as_string().unwrap_or_else(|| "bgra8unorm".to_string())
        })
        .unwrap_or_else(|| "bgra8unorm".to_string());
    set(&target, "format", &js_str(&preferred));

    let targets = Array::of1(target.as_ref());
    let frag_state = obj();
    set(&frag_state, "module", fs_module);
    set(&frag_state, "entryPoint", &js_str("fs_main"));
    set(&frag_state, "targets", targets.as_ref());

    let primitive = obj();
    set(&primitive, "topology", &js_str("triangle-list"));
    set(&primitive, "cullMode", &js_str("back"));

    let ds = obj();
    set(&ds, "format", &js_str("depth24plus"));
    set(&ds, "depthWriteEnabled", &JsValue::from_bool(true));
    set(&ds, "depthCompare", &js_str("less"));

    let desc = obj();
    set(&desc, "layout", layout);
    set(&desc, "vertex", vertex_state.as_ref());
    set(&desc, "fragment", frag_state.as_ref());
    set(&desc, "primitive", primitive.as_ref());
    set(&desc, "depthStencil", ds.as_ref());

    Some(JsValue::from(
        device.create_render_pipeline(desc.unchecked_ref()).unwrap(),
    ))
}

fn create_vertex_buffer(device: &GpuDevice, data: &[f32]) -> Option<web_sys::GpuBuffer> {
    let bytes = (data.len() * 4) as u64;
    let desc = obj();
    set(&desc, "size", &JsValue::from_f64(bytes as f64));
    set(&desc, "usage", &JsValue::from_f64((0x20u32 | 0x08u32) as f64)); // VERTEX | COPY_DST
    let buf = device.create_buffer(desc.unchecked_ref()).ok()?;
    let arr = Float32Array::from(data);
    let u8_arr = js_sys::Uint8Array::new(&arr.buffer());
    let _ = device.queue().write_buffer_with_u32_and_u8_array(
        &buf,
        0,
        &u8_arr,
    );
    Some(buf)
}

fn create_uniform_buffer(device: &GpuDevice, size: u64) -> Option<web_sys::GpuBuffer> {
    let desc = obj();
    set(&desc, "size", &JsValue::from_f64(size as f64));
    set(&desc, "usage", &JsValue::from_f64((0x40u32 | 0x08u32) as f64)); // UNIFORM | COPY_DST
    device.create_buffer(desc.unchecked_ref()).ok()
}

fn make_bind_group(device: &GpuDevice, bgl: &JsValue, uniform_buf: &web_sys::GpuBuffer) -> JsValue {
    let resource = obj();
    set(&resource, "buffer", uniform_buf.as_ref());
    let entry = obj();
    set(&entry, "binding", &JsValue::from_f64(0.0));
    set(&entry, "resource", resource.as_ref());
    let entries = Array::of1(entry.as_ref());
    let desc = obj();
    set(&desc, "layout", bgl);
    set(&desc, "entries", entries.as_ref());
    JsValue::from(device.create_bind_group(desc.unchecked_ref()))
}

/// Call `setBindGroup(index, bindGroup)` via Reflect to avoid IDL overload issues.
fn call_set_bind_group(pass: &JsValue, index: u32, bg: &JsValue) {
    let f = Reflect::get(pass, &JsValue::from_str("setBindGroup")).unwrap();
    let args = Array::of2(&JsValue::from_f64(index as f64), bg);
    Reflect::apply(f.unchecked_ref(), pass, &args).unwrap_or(JsValue::UNDEFINED);
}

// ── Uniform write helpers ─────────────────────────────────────────────────────

fn write_mat4(u: &mut [f32], offset: usize, m: &Mat4) {
    u[offset..offset + 16].copy_from_slice(m);
}

fn write_vec4(u: &mut [f32], offset: usize, v: [f32; 4]) {
    u[offset..offset + 4].copy_from_slice(&v);
}

fn write_uniform(queue: &GpuQueue, buf: &web_sys::GpuBuffer, data: &[f32]) {
    let bytes: &[u8] =
        unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4) };
    let arr = js_sys::Uint8Array::from(bytes);
    let _ = queue.write_buffer_with_u32_and_u8_array(buf, 0, &arr);
}

// ── Cube geometry ──────────────────────────────────────────────────────────────

/// Unit cube centered at origin: 36 vertices × 6 floats (xyz + normal xyz).
fn cube_vertices() -> Vec<f32> {
    #[rustfmt::skip]
    let faces: &[(Vec3, Vec3, Vec3, Vec3, Vec3)] = &[
        // +X
        ([ 0.5,  0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5, -0.5, -0.5], [ 0.5,  0.5, -0.5], [1.0, 0.0, 0.0]),
        // -X
        ([-0.5,  0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, -0.5,  0.5], [-0.5,  0.5,  0.5], [-1.0, 0.0, 0.0]),
        // +Y
        ([-0.5,  0.5, -0.5], [-0.5,  0.5,  0.5], [ 0.5,  0.5,  0.5], [ 0.5,  0.5, -0.5], [0.0, 1.0, 0.0]),
        // -Y
        ([-0.5, -0.5,  0.5], [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5], [ 0.5, -0.5,  0.5], [0.0, -1.0, 0.0]),
        // +Z
        ([-0.5,  0.5,  0.5], [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5], [0.0, 0.0, 1.0]),
        // -Z
        ([ 0.5,  0.5, -0.5], [ 0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5,  0.5, -0.5], [0.0, 0.0, -1.0]),
    ];
    let mut verts = Vec::with_capacity(36 * 6);
    for (a, b, c, d, n) in faces {
        // Triangle 1: a, b, c
        verts.extend_from_slice(a);
        verts.extend_from_slice(n);
        verts.extend_from_slice(b);
        verts.extend_from_slice(n);
        verts.extend_from_slice(c);
        verts.extend_from_slice(n);
        // Triangle 2: a, c, d
        verts.extend_from_slice(a);
        verts.extend_from_slice(n);
        verts.extend_from_slice(c);
        verts.extend_from_slice(n);
        verts.extend_from_slice(d);
        verts.extend_from_slice(n);
    }
    verts
}

/// Ground plane quad (2 triangles, 6 vertices × 6 floats, normal = +Y).
fn ground_quad() -> Vec<f32> {
    let n = [0.0f32, 1.0, 0.0];
    let verts: &[([f32; 3], [f32; 3])] = &[
        ([-1.0, 0.0, -1.0], n),
        ([-1.0, 0.0,  1.0], n),
        ([ 1.0, 0.0,  1.0], n),
        ([-1.0, 0.0, -1.0], n),
        ([ 1.0, 0.0,  1.0], n),
        ([ 1.0, 0.0, -1.0], n),
    ];
    verts.iter().flat_map(|(p, nor)| [p[0], p[1], p[2], nor[0], nor[1], nor[2]]).collect()
}

// ── JS Object helpers (local copies to avoid cross-module dependency) ─────────

fn obj() -> Object { Object::new() }
fn set(o: &Object, key: &str, val: &JsValue) {
    Reflect::set(o, &JsValue::from_str(key), val).unwrap();
}
fn js_str(s: &str) -> JsValue { JsValue::from_str(s) }
