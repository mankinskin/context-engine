/// GPU resources for HypergraphView rendering.
///
/// Renders hypergraph nodes as instanced billboard sphere impostors and edges
/// as instanced energy-beam quads, using `hypergraph.wgsl`.
///
/// # Shader layout (after `palette.wgsl` prepend)
/// - `vs_node` / `fs_node` — billboard sphere per node
/// - `vs_edge` / `fs_edge` — billboard quad per edge or grid line
///
/// # Camera uniform (binding 0, 128 bytes)
/// ```text
/// viewProj : mat4x4<f32>   offset  0  (64 bytes)
/// eye      : vec4<f32>     offset 64  (16 bytes)
/// time     : vec4<f32>     offset 80  (16 bytes)  x=time
/// ```
/// Total struct: 96 bytes, buffer allocated 128 bytes.
///
/// # Node instance (12 floats = 48 bytes, locations 2–5)
/// ```text
/// center  : vec3<f32>   offset  0
/// radius  : f32         offset 12
/// color   : vec4<f32>   offset 16
/// flags   : vec4<f32>   offset 32  (x=selected, y=hovered, z=isAtom)
/// ```
///
/// # Edge instance (12 floats = 48 bytes, locations 6–10)
/// ```text
/// posA    : vec3<f32>   offset  0
/// posB    : vec3<f32>   offset 12
/// color   : vec4<f32>   offset 24
/// flags   : f32         offset 40  (highlighted)
/// edgeType: f32         offset 44
/// ```
use js_sys::{Array, Float32Array, Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{GpuBuffer, GpuDevice};

use crate::gpu::math3d::{Mat4, Vec3};

// ── Embedded shaders ──────────────────────────────────────────────────────────

const PALETTE_WGSL: &str =
    include_str!("../../../../viewer-api/frontend/ts/src/effects/palette.wgsl");
const HYPERGRAPH_WGSL: &str =
    include_str!("../../../../viewer-api/frontend/ts/src/components/HypergraphView/hypergraph.wgsl");

// ── Constants ─────────────────────────────────────────────────────────────────

pub const NODE_INSTANCE_FLOATS: usize = 12;
pub const EDGE_INSTANCE_FLOATS: usize = 12;

// Camera uniform buffer size (96 bytes data, 128 allocated for alignment)
pub const CAM_UB_BYTES: usize = 128;

// Grid parameters (match TypeScript constants.ts)
const GRID_EXTENT: i32 = 20;
const GRID_STEP: i32 = 2;

// Quad vertices for instanced billboard rendering (6 vertices × 2 floats)
const QUAD_VERTS: [f32; 12] = [-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0];

// ── Resources struct ──────────────────────────────────────────────────────────

pub struct HypergraphGpuResources {
    /// Node billboard pipeline (vs_node / fs_node).
    pub node_pipeline: JsValue,
    /// Edge billboard pipeline (vs_edge / fs_edge) — used for both edges and grid.
    pub edge_pipeline: JsValue,
    /// Shared quad vertex buffer (6 verts × vec2).
    pub quad_vb: GpuBuffer,
    /// Node instance buffer — sized for `max_nodes × NODE_INSTANCE_FLOATS × 4` bytes.
    pub node_ib: GpuBuffer,
    /// Edge instance buffer — sized for `max_edges × EDGE_INSTANCE_FLOATS × 4` bytes.
    pub edge_ib: GpuBuffer,
    /// Grid instance buffer (static, written once).
    pub grid_ib: GpuBuffer,
    /// Camera + time uniform buffer.
    pub cam_ub: GpuBuffer,
    /// Palette buffer (384 bytes = 24 × vec4f).
    pub palette_ub: GpuBuffer,
    /// Bind group (cam_ub at 0, palette_ub at 1).
    pub cam_bg: JsValue,
    /// Bind group layout (shared by both pipelines).
    pub bgl: JsValue,
    /// Number of grid line instances.
    pub grid_count: u32,
    pub max_nodes: u32,
    pub max_edges: u32,
}

// ── Build resources ───────────────────────────────────────────────────────────

pub fn build_hypergraph_resources(
    device: &GpuDevice,
    format: &str,
    max_nodes: u32,
    max_edges: u32,
) -> Option<HypergraphGpuResources> {
    let full_wgsl = format!("{}\n{}", PALETTE_WGSL, HYPERGRAPH_WGSL);
    let shader = mk_shader(device, &full_wgsl);

    // BGL: binding 0 = cam uniform (vert+frag), binding 1 = palette uniform (frag)
    let bgl = mk_bgl(device);
    let layout = mk_pipeline_layout(device, &bgl);

    // Quad vertex buffer layout (location 0, float32x2, vertex step)
    let quad_vb_layout = mk_quad_vb_layout();

    // Node instance layout (locations 2–5, float32x*, instance step)
    let node_ib_layout = mk_node_ib_layout();

    // Edge instance layout (locations 6–10, float32x*, instance step)
    let edge_ib_layout = mk_edge_ib_layout();

    let node_pipeline = mk_node_pipeline(device, &layout, &shader, &quad_vb_layout, &node_ib_layout, format)?;
    let edge_pipeline = mk_edge_pipeline(device, &layout, &shader, &quad_vb_layout, &edge_ib_layout, format)?;

    let quad_vb = create_vertex_buffer(device, &QUAD_VERTS)?;
    let node_ib = create_buffer(device, (max_nodes as usize * NODE_INSTANCE_FLOATS * 4) as u64, 0x20 | 0x08)?;
    let edge_ib = create_buffer(device, (max_edges as usize * EDGE_INSTANCE_FLOATS * 4).max(48) as u64, 0x20 | 0x08)?;

    let (grid_data, grid_count) = build_grid_data();
    let grid_ib = create_vertex_buffer(device, &grid_data)?;

    let cam_ub = create_buffer(device, CAM_UB_BYTES as u64, 0x40 | 0x08)?;
    let palette_ub = create_buffer(device, 384, 0x40 | 0x08)?;

    let cam_bg = mk_bind_group(device, &bgl, &cam_ub, &palette_ub);

    Some(HypergraphGpuResources {
        node_pipeline,
        edge_pipeline,
        quad_vb,
        node_ib,
        edge_ib,
        grid_ib,
        cam_ub,
        palette_ub,
        cam_bg,
        bgl,
        grid_count,
        max_nodes,
        max_edges,
    })
}

// ── Uniform packing ───────────────────────────────────────────────────────────

/// Pack camera uniform data into a 32-f32 array (128 bytes).
pub fn pack_cam_uniform(view_proj: &Mat4, eye: Vec3, time: f32) -> [f32; 32] {
    let mut u = [0f32; 32];
    u[0..16].copy_from_slice(view_proj);
    u[16] = eye[0]; u[17] = eye[1]; u[18] = eye[2]; u[19] = 0.0;
    u[20] = time;
    u
}

// ── Instance buffer filling ───────────────────────────────────────────────────

/// Append one edge instance to `buf` (12 floats).
pub fn push_edge_instance(
    buf: &mut Vec<f32>,
    pos_a: Vec3,
    pos_b: Vec3,
    color: [f32; 4],
    flags: f32,
    edge_type: f32,
) {
    buf.extend_from_slice(&pos_a);
    buf.extend_from_slice(&pos_b);
    buf.extend_from_slice(&color);
    buf.push(flags);
    buf.push(edge_type);
}

/// Append one node instance to `buf` (12 floats).
pub fn push_node_instance(
    buf: &mut Vec<f32>,
    center: Vec3,
    radius: f32,
    color: [f32; 4],
    selected: bool,
    hovered: bool,
    is_atom: bool,
) {
    buf.extend_from_slice(&center);
    buf.push(radius);
    buf.extend_from_slice(&color);
    buf.extend_from_slice(&[
        if selected { 1.0 } else { 0.0 },
        if hovered  { 1.0 } else { 0.0 },
        if is_atom  { 1.0 } else { 0.0 },
        0.0,
    ]);
}

// ── Grid data builder ─────────────────────────────────────────────────────────

fn build_grid_data() -> (Vec<f32>, u32) {
    let mut lines: Vec<f32> = Vec::new();

    // Regular grid lines
    let mut i = -GRID_EXTENT;
    while i <= GRID_EXTENT {
        let fi = i as f32;
        let fe = GRID_EXTENT as f32;
        // X-parallel (constant X, varying Z)
        push_edge_instance(&mut lines,
            [fi, 0.0, -fe], [fi, 0.0, fe],
            [0.22, 0.24, 0.28, 0.10], 0.0, 0.0);
        // Z-parallel (varying X, constant Z)
        push_edge_instance(&mut lines,
            [-fe, 0.0, fi], [fe, 0.0, fi],
            [0.22, 0.24, 0.28, 0.10], 0.0, 0.0);
        i += GRID_STEP;
    }
    // Axis lines (highlighted)
    let fe = GRID_EXTENT as f32;
    push_edge_instance(&mut lines,
        [-fe, 0.0, 0.0], [fe, 0.0, 0.0],
        [0.60, 0.22, 0.18, 0.30], 1.0, 0.0); // X axis: red
    push_edge_instance(&mut lines,
        [0.0, 0.0, -fe], [0.0, 0.0, fe],
        [0.18, 0.22, 0.60, 0.30], 1.0, 0.0); // Z axis: blue

    // Return the full slice but report count WITHOUT axis lines for normal draw
    // (axis lines are included in the buffer but drawn with the same call for simplicity)
    let total_count = (lines.len() / EDGE_INSTANCE_FLOATS) as u32;
    (lines, total_count)
}

// ── Pipeline builders ─────────────────────────────────────────────────────────

fn mk_shader(device: &GpuDevice, code: &str) -> JsValue {
    let d = obj();
    set(&d, "code", &JsValue::from_str(code));
    JsValue::from(device.create_shader_module(d.unchecked_ref()))
}

fn mk_bgl(device: &GpuDevice) -> JsValue {
    let vf = (1u32 | 2u32) as f64; // VERTEX | FRAGMENT
    let f  = 2.0f64;               // FRAGMENT

    let e0 = obj();
    set(&e0, "binding", &js_f(0.0));
    set(&e0, "visibility", &js_f(vf));
    let b0 = obj(); set(&b0, "type", &js_str("uniform"));
    set(&e0, "buffer", b0.as_ref());

    let e1 = obj();
    set(&e1, "binding", &js_f(1.0));
    set(&e1, "visibility", &js_f(f));
    let b1 = obj(); set(&b1, "type", &js_str("uniform"));
    set(&e1, "buffer", b1.as_ref());

    let entries = Array::of2(e0.as_ref(), e1.as_ref());
    let d = obj(); set(&d, "entries", entries.as_ref());
    JsValue::from(device.create_bind_group_layout(d.unchecked_ref()).unwrap())
}

fn mk_pipeline_layout(device: &GpuDevice, bgl: &JsValue) -> JsValue {
    let bgls = Array::of1(bgl);
    let d = obj(); set(&d, "bindGroupLayouts", bgls.as_ref());
    JsValue::from(device.create_pipeline_layout(d.unchecked_ref()))
}

fn mk_quad_vb_layout() -> Object {
    let attr = obj();
    set(&attr, "shaderLocation", &js_f(0.0));
    set(&attr, "offset", &js_f(0.0));
    set(&attr, "format", &js_str("float32x2"));
    let vbl = obj();
    set(&vbl, "arrayStride", &js_f(8.0));
    set(&vbl, "stepMode", &js_str("vertex"));
    set(&vbl, "attributes", Array::of1(attr.as_ref()).as_ref());
    vbl
}

fn mk_node_ib_layout() -> Object {
    let a2 = mk_attr(2, 0,  "float32x3");
    let a3 = mk_attr(3, 12, "float32");
    let a4 = mk_attr(4, 16, "float32x4");
    let a5 = mk_attr(5, 32, "float32x4");
    let attrs = Array::new();
    attrs.push(a2.as_ref());
    attrs.push(a3.as_ref());
    attrs.push(a4.as_ref());
    attrs.push(a5.as_ref());
    let vbl = obj();
    set(&vbl, "arrayStride", &js_f(48.0)); // 12 × f32
    set(&vbl, "stepMode", &js_str("instance"));
    set(&vbl, "attributes", attrs.as_ref());
    vbl
}

fn mk_edge_ib_layout() -> Object {
    let a6  = mk_attr(6,  0,  "float32x3");
    let a7  = mk_attr(7,  12, "float32x3");
    let a8  = mk_attr(8,  24, "float32x4");
    let a9  = mk_attr(9,  40, "float32");
    let a10 = mk_attr(10, 44, "float32");
    let attrs = Array::new();
    attrs.push(a6.as_ref());
    attrs.push(a7.as_ref());
    attrs.push(a8.as_ref());
    attrs.push(a9.as_ref());
    attrs.push(a10.as_ref());
    let vbl = obj();
    set(&vbl, "arrayStride", &js_f(48.0)); // 12 × f32
    set(&vbl, "stepMode", &js_str("instance"));
    set(&vbl, "attributes", attrs.as_ref());
    vbl
}

fn mk_attr(loc: u32, offset: u32, format: &str) -> Object {
    let a = obj();
    set(&a, "shaderLocation", &js_f(loc as f64));
    set(&a, "offset", &js_f(offset as f64));
    set(&a, "format", &js_str(format));
    a
}

fn mk_fragment_state(shader: &JsValue, entry: &str, format: &str, blend: bool) -> Object {
    let tgt = obj();
    set(&tgt, "format", &js_str(format));
    if blend {
        let bc = obj();
        set(&bc, "srcFactor", &js_str("src-alpha"));
        set(&bc, "dstFactor", &js_str("one-minus-src-alpha"));
        set(&bc, "operation", &js_str("add"));
        let ba = obj();
        set(&ba, "srcFactor", &js_str("one"));
        set(&ba, "dstFactor", &js_str("one-minus-src-alpha"));
        set(&ba, "operation", &js_str("add"));
        let bl = obj();
        set(&bl, "color", bc.as_ref());
        set(&bl, "alpha", ba.as_ref());
        set(&tgt, "blend", bl.as_ref());
    }
    let fs = obj();
    set(&fs, "module", shader);
    set(&fs, "entryPoint", &js_str(entry));
    set(&fs, "targets", Array::of1(tgt.as_ref()).as_ref());
    fs
}

fn mk_depth_stencil(write: bool) -> Object {
    let ds = obj();
    set(&ds, "format", &js_str("depth24plus"));
    set(&ds, "depthWriteEnabled", &JsValue::from_bool(write));
    set(&ds, "depthCompare", &js_str(if write { "less" } else { "always" }));
    ds
}

fn mk_node_pipeline(
    device: &GpuDevice,
    layout: &JsValue,
    shader: &JsValue,
    quad_vbl: &Object,
    node_vbl: &Object,
    format: &str,
) -> Option<JsValue> {
    let vs = obj();
    set(&vs, "module", shader);
    set(&vs, "entryPoint", &js_str("vs_node"));
    set(&vs, "buffers", Array::of2(quad_vbl.as_ref(), node_vbl.as_ref()).as_ref());

    let fs = mk_fragment_state(shader, "fs_node", format, true);
    let prim = obj();
    set(&prim, "topology", &js_str("triangle-list"));
    let ds = mk_depth_stencil(false); // depth compare always, no write (impostor)

    let d = obj();
    set(&d, "layout", layout);
    set(&d, "vertex", vs.as_ref());
    set(&d, "fragment", fs.as_ref());
    set(&d, "primitive", prim.as_ref());
    set(&d, "depthStencil", ds.as_ref());

    Some(JsValue::from(device.create_render_pipeline(d.unchecked_ref()).unwrap()))
}

fn mk_edge_pipeline(
    device: &GpuDevice,
    layout: &JsValue,
    shader: &JsValue,
    quad_vbl: &Object,
    edge_vbl: &Object,
    format: &str,
) -> Option<JsValue> {
    let vs = obj();
    set(&vs, "module", shader);
    set(&vs, "entryPoint", &js_str("vs_edge"));
    set(&vs, "buffers", Array::of2(quad_vbl.as_ref(), edge_vbl.as_ref()).as_ref());

    let fs = mk_fragment_state(shader, "fs_edge", format, true);
    let prim = obj();
    set(&prim, "topology", &js_str("triangle-list"));
    let ds = mk_depth_stencil(false);

    let d = obj();
    set(&d, "layout", layout);
    set(&d, "vertex", vs.as_ref());
    set(&d, "fragment", fs.as_ref());
    set(&d, "primitive", prim.as_ref());
    set(&d, "depthStencil", ds.as_ref());

    Some(JsValue::from(device.create_render_pipeline(d.unchecked_ref()).unwrap()))
}

fn mk_bind_group(device: &GpuDevice, bgl: &JsValue, cam_ub: &GpuBuffer, palette_ub: &GpuBuffer) -> JsValue {
    let r0 = obj(); set(&r0, "buffer", cam_ub.as_ref());
    let e0 = obj(); set(&e0, "binding", &js_f(0.0)); set(&e0, "resource", r0.as_ref());

    let r1 = obj(); set(&r1, "buffer", palette_ub.as_ref());
    let e1 = obj(); set(&e1, "binding", &js_f(1.0)); set(&e1, "resource", r1.as_ref());

    let entries = Array::of2(e0.as_ref(), e1.as_ref());
    let d = obj(); set(&d, "layout", bgl); set(&d, "entries", entries.as_ref());
    JsValue::from(device.create_bind_group(d.unchecked_ref()))
}

// ── Buffer utilities ──────────────────────────────────────────────────────────

fn create_vertex_buffer(device: &GpuDevice, data: &[f32]) -> Option<GpuBuffer> {
    let bytes = (data.len() * 4) as u64;
    let buf = create_buffer(device, bytes, 0x20 | 0x08)?; // VERTEX | COPY_DST
    write_f32_to_buffer(device, &buf, data);
    Some(buf)
}

pub fn create_buffer(device: &GpuDevice, size: u64, usage: u32) -> Option<GpuBuffer> {
    let d = obj();
    set(&d, "size", &js_f(size as f64));
    set(&d, "usage", &js_f(usage as f64));
    device.create_buffer(d.unchecked_ref()).ok()
}

fn write_f32_to_buffer(device: &GpuDevice, buf: &GpuBuffer, data: &[f32]) {
    let arr = Float32Array::from(data);
    let u8arr = js_sys::Uint8Array::new(&arr.buffer());
    let _ = device.queue().write_buffer_with_u32_and_u8_array(buf, 0, &u8arr);
}

pub fn write_f32_buf(device: &GpuDevice, buf: &GpuBuffer, data: &[f32]) {
    let arr = Float32Array::from(data);
    let u8arr = js_sys::Uint8Array::new(&arr.buffer());
    let _ = device.queue().write_buffer_with_u32_and_u8_array(buf, 0, &u8arr);
}

/// Call `setBindGroup(index, bindGroup)` via Reflect to avoid IDL overload issues.
pub fn call_set_bind_group(pass: &JsValue, index: u32, bg: &JsValue) {
    let f = Reflect::get(pass, &JsValue::from_str("setBindGroup")).unwrap();
    let args = Array::of2(&JsValue::from_f64(index as f64), bg);
    let func: &Function = f.unchecked_ref();
    Reflect::apply(func, pass, &args).unwrap_or(JsValue::UNDEFINED);
}

// ── JS helpers ────────────────────────────────────────────────────────────────

fn obj() -> Object { Object::new() }
fn set(o: &Object, key: &str, val: &JsValue) {
    Reflect::set(o, &JsValue::from_str(key), val).unwrap();
}
fn js_f(v: f64) -> JsValue { JsValue::from_f64(v) }
fn js_str(s: &str) -> JsValue { JsValue::from_str(s) }
