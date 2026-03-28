/// Hypergraph view — force-directed 3-D graph.
///
/// # Rendering model
/// - Nodes: DOM `<div class="hg-node">` elements, absolutely positioned.
///   Each frame the render callback projects their 3-D world position to screen
///   coordinates and writes the CSS `transform` + `z-index` imperatively —
///   exactly as `nodePositioner.ts` does in the TypeScript frontend.
/// - Edges + grid: GPU (WebGPU via `hypergraph.wgsl`).
/// - Background smoke + particles: global `OverlayContext` render loop.
///
/// # GPU init flow
/// The global `OverlayContext` is provided by `App` and started on the full-page
/// canvas.  `HypergraphView` reads the context and registers a per-frame GPU
/// render callback once `overlay.gpu_ready` becomes `true` AND the hypergraph
/// snapshot is available.  The callback restricts its GPU operations to the
/// container's bounding rect via `setViewport` / `setScissorRect`.
use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{Array, Function, Reflect};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{GpuRenderPassEncoder, GpuTextureView, MouseEvent, WheelEvent};

use crate::gpu::hypergraph_gpu::{
    build_hypergraph_resources, call_set_bind_group, pack_cam_uniform,
    push_edge_instance, write_f32_buf, EDGE_INSTANCE_FLOATS,
};
use crate::gpu::math3d::{
    mat4_identity, mat4_inverse, mat4_look_at, mat4_multiply, mat4_perspective,
    world_scale_at_depth, world_to_screen, Vec3,
};
use crate::gpu::overlay::{set_particle_cam, write_f32, OverlayContext, RenderCallback};
use crate::store::Store;
use crate::types::{HypergraphSnapshot, SnapshotEdge};

// ── Camera ────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct CameraState {
    yaw: f32, pitch: f32, dist: f32, target: Vec3,
    orbiting: bool, last_mx: f32, last_my: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self { yaw: 0.5, pitch: 0.4, dist: 6.0, target: [0.0, 0.0, 0.0],
               orbiting: false, last_mx: 0.0, last_my: 0.0 }
    }
}

impl CameraState {
    fn eye(&self) -> Vec3 {
        [
            self.target[0] + self.dist * self.pitch.cos() * self.yaw.sin(),
            self.target[1] + self.dist * self.pitch.sin(),
            self.target[2] + self.dist * self.pitch.cos() * self.yaw.cos(),
        ]
    }

    fn view_proj(&self, w: u32, h: u32) -> [f32; 16] {
        let aspect = w as f32 / h.max(1) as f32;
        let proj = mat4_perspective(std::f32::consts::FRAC_PI_4, aspect, 0.1, 200.0);
        let view = mat4_look_at(self.eye(), self.target, [0.0, 1.0, 0.0]);
        mat4_multiply(proj, view)
    }
}

// ── Layout ────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct LayoutNode {
    index: u32, x: f32, y: f32, z: f32, radius: f32,
    edge_color: [f32; 4], is_atom: bool,
}

fn build_layout(snapshot: &HypergraphSnapshot) -> Vec<LayoutNode> {
    let n = snapshot.nodes.len();
    if n == 0 { return Vec::new(); }
    let max_width = snapshot.nodes.iter().map(|nd| nd.width).max().unwrap_or(1);

    let mut pos: Vec<[f32; 3]> = snapshot.nodes.iter().enumerate().map(|(i, node)| {
        let angle = (i as f32 / n as f32) * std::f32::consts::TAU;
        let r = 0.45 + n as f32 * 0.5;
        let jitter = 0.5 + ((i * 17 + 3) % 100) as f32 * 0.01;
        [angle.cos() * r * jitter, (node.width as f32 - 1.0) * 1.67, angle.sin() * r * jitter]
    }).collect();
    let mut vel: Vec<[f32; 3]> = vec![[0.0; 3]; n];

    let repulsion: f32 = 0.6;
    let spring_k:  f32 = 0.020;
    let y_spring:  f32 = 0.005;
    let spring_len:f32 = 0.1;
    let damping:   f32 = 0.85;
    let gravity:   f32 = 0.04;
    let dt:        f32 = 0.4;
    let max_vel:   f32 = 3.0;

    for iter in 0..80usize {
        let temp = 1.0 - iter as f32 / 80.0;
        for i in 0..n {
            for j in (i + 1)..n {
                let mut dx = pos[i][0] - pos[j][0];
                let mut dz = pos[i][2] - pos[j][2];
                let mut d  = (dx*dx + dz*dz).sqrt();
                if d < 0.01 { dx = 0.1; dz = 0.1; d = 0.15; }
                let force = repulsion / (d.max(1.0) * d.max(1.0)) * temp;
                let fx = (dx / d) * force; let fz = (dz / d) * force;
                vel[i][0] += fx; vel[i][2] += fz;
                vel[j][0] -= fx; vel[j][2] -= fz;
            }
        }
        for e in &snapshot.edges {
            let ai = snapshot.nodes.iter().position(|nd| nd.index == e.from);
            let bi = snapshot.nodes.iter().position(|nd| nd.index == e.to);
            if let (Some(ai), Some(bi)) = (ai, bi) {
                let dx = pos[bi][0] - pos[ai][0];
                let dz = pos[bi][2] - pos[ai][2];
                let d  = (dx*dx + dz*dz).sqrt().max(0.01);
                let force = spring_k * (d - spring_len) * temp;
                let fx = (dx / d) * force; let fz = (dz / d) * force;
                vel[ai][0] += fx; vel[ai][2] += fz;
                vel[bi][0] -= fx; vel[bi][2] -= fz;
            }
        }
        for (i, node) in snapshot.nodes.iter().enumerate() {
            let ty = (node.width as f32 - 1.0) * 0.27;
            vel[i][1] += (ty - pos[i][1]) * y_spring;
        }
        if n > 1 {
            let cx = pos.iter().map(|p| p[0]).sum::<f32>() / n as f32;
            let cz = pos.iter().map(|p| p[2]).sum::<f32>() / n as f32;
            for i in 0..n {
                vel[i][0] -= (pos[i][0] - cx) * gravity * temp;
                vel[i][2] -= (pos[i][2] - cz) * gravity * temp;
            }
        }
        for i in 0..n {
            for k in 0..3 {
                vel[i][k] = (vel[i][k] * damping).clamp(-max_vel, max_vel);
                pos[i][k] += vel[i][k] * dt;
            }
        }
    }
    let cx = pos.iter().map(|p| p[0]).sum::<f32>() / n as f32;
    let cz = pos.iter().map(|p| p[2]).sum::<f32>() / n as f32;
    for p in &mut pos { p[0] -= cx; p[2] -= cz; }

    snapshot.nodes.iter().enumerate().map(|(i, node)| {
        let is_atom = node.width == 1;
        let t = ((node.width.saturating_sub(1)) as f32
            / (max_width as i32 - 1).max(1) as f32).min(1.0);
        let edge_color = if is_atom {
            [0.35, 0.55, 0.75, 0.5]
        } else {
            [0.3 + t * 0.6, 0.8 - t * 0.4, 0.3 + (1.0 - t) * 0.3, 0.65]
        };
        LayoutNode {
            index: node.index,
            x: pos[i][0], y: pos[i][1], z: pos[i][2],
            radius: 0.45 + (node.width as f32 * 0.15).min(0.4),
            edge_color, is_atom,
        }
    }).collect()
}

fn edge_color(pattern_idx: u32) -> [f32; 4] {
    const P: [[f32; 3]; 6] = [
        [0.45, 0.55, 0.70], [0.70, 0.45, 0.55], [0.50, 0.70, 0.45],
        [0.65, 0.55, 0.70], [0.70, 0.65, 0.40], [0.40, 0.70, 0.65],
    ];
    let [r, g, b] = P[(pattern_idx as usize) % P.len()];
    [r, g, b, 0.65]
}

// ── DOM node projection ───────────────────────────────────────────────────────

/// Called every frame from the GPU render callback.
/// Queries `.hg-node[data-hg-idx]` elements and updates their CSS transforms.
fn update_node_transforms(layout: &[LayoutNode], vp: [f32; 16], eye: Vec3, w: u32, h: u32) {
    let Some(doc) = web_sys::window().and_then(|win| win.document()) else { return };
    let wf = w as f32;
    let hf = h as f32;
    for node in layout {
        let sel = format!(".hg-node[data-hg-idx='{}']", node.index);
        let Ok(Some(el)) = doc.query_selector(&sel) else { continue };
        let (sx, sy, sz, vis) = world_to_screen([node.x, node.y, node.z], vp, wf, hf);
        if !vis || sx < -300.0 || sx > wf + 300.0 || sy < -300.0 || sy > hf + 300.0 {
            let _ = el.set_attribute("style",
                "position:absolute;top:0;left:0;\
                 transform:translate(-50%,-50%);\
                 visibility:hidden;pointer-events:none;");
            continue;
        }
        let scale = world_scale_at_depth(eye, [node.x, node.y, node.z], hf);
        let pixel_scale = ((scale * node.radius * 2.5) / 80.0).max(0.06);
        let z_idx = (1000.0 * (1.0 - sz.clamp(0.0, 1.0))) as u32;
        let _ = el.set_attribute("style", &format!(
            "position:absolute;top:0;left:0;\
             transform:translate(-50%,-50%) translate({:.1}px,{:.1}px) scale({:.3});\
             z-index:{};visibility:visible;pointer-events:auto;",
            sx, sy, pixel_scale, z_idx,
        ));
    }
}

// ── Default palette (24 × vec4f = 384 bytes) ─────────────────────────────────

fn build_default_palette() -> Vec<f32> {
    let mut p = vec![0f32; 96];
    let slots: &[[f32; 4]] = &[
        [1.0, 0.95, 0.70, 1.0], [1.0, 0.50, 0.10, 0.8], [0.8, 0.85, 0.90, 0.6],
        [1.0, 0.60, 0.10, 0.9], [0.9, 0.80, 0.40, 0.9], [0.7, 0.40, 0.10, 0.4],
        [1.0, 0.90, 0.50, 0.8], [0.5, 0.80, 1.00, 0.8], [1.0, 0.50, 0.10, 0.9],
        [1.0, 0.80, 0.20, 0.9], [0.7, 0.65, 0.60, 0.7], [0.3, 0.50, 0.20, 0.7],
        [0.12,0.16, 0.22, 1.0], [0.22,0.18, 0.14, 1.0], [0.10,0.16, 0.10, 1.0],
        [0.3, 0.40, 0.55, 1.0], [0.9, 0.20, 0.20, 1.0], [0.9, 0.70, 0.10, 1.0],
        [0.2, 0.70, 0.90, 1.0], [0.4, 0.70, 0.40, 1.0], [0.2, 0.80, 0.50, 1.0],
        [0.95,0.55, 0.15, 1.0], [0.95,0.15, 0.15, 1.0], [0.0, 0.0,  0.0,  0.0],
    ];
    for (i, s) in slots.iter().enumerate() {
        p[i * 4..i * 4 + 4].copy_from_slice(s);
    }
    p
}

// ── GPU viewport helpers ──────────────────────────────────────────────────────

/// Call `pass.setViewport(x, y, w, h, 0.0, 1.0)` via Reflect to avoid IDL overload issues.
fn call_set_viewport(pass: &GpuRenderPassEncoder, x: u32, y: u32, w: u32, h: u32) {
    let Ok(f) = Reflect::get(pass.as_ref(), &JsValue::from_str("setViewport")) else { return };
    let args = Array::new();
    args.push(&JsValue::from_f64(x as f64));
    args.push(&JsValue::from_f64(y as f64));
    args.push(&JsValue::from_f64(w as f64));
    args.push(&JsValue::from_f64(h as f64));
    args.push(&JsValue::from_f64(0.0));
    args.push(&JsValue::from_f64(1.0));
    Reflect::apply(f.unchecked_ref::<Function>(), pass.as_ref(), &args).ok();
}

/// Call `pass.setScissorRect(x, y, w, h)` via Reflect.
fn call_set_scissor_rect(pass: &GpuRenderPassEncoder, x: u32, y: u32, w: u32, h: u32) {
    let Ok(f) = Reflect::get(pass.as_ref(), &JsValue::from_str("setScissorRect")) else { return };
    let args = Array::of4(
        &JsValue::from_f64(x as f64),
        &JsValue::from_f64(y as f64),
        &JsValue::from_f64(w as f64),
        &JsValue::from_f64(h as f64),
    );
    Reflect::apply(f.unchecked_ref::<Function>(), pass.as_ref(), &args).ok();
}

// ── Main component ────────────────────────────────────────────────────────────

#[component]
pub fn HypergraphView() -> impl IntoView {
    let store    = expect_context::<Store>();
    let snapshot = store.hypergraph_snapshot();

    // Use the global overlay context provided by App.
    let overlay = expect_context::<OverlayContext>();

    // NodeRef for the hypergraph container — used inside the render callback
    // to restrict GPU drawing to the container's bounding rect.
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Camera state — shared between mouse handlers and render callback.
    let camera:    Rc<RefCell<CameraState>> = Rc::new(RefCell::new(CameraState::default()));
    let cam_down   = camera.clone();
    let cam_move   = camera.clone();
    let cam_up_rc  = camera.clone();
    let cam_wheel  = camera.clone();

    // One-shot GPU init guard and registered callback ID.
    let gpu_init_done: StoredValue<bool>        = StoredValue::new(false);
    let callback_id:   StoredValue<Option<usize>> = StoredValue::new(None);

    // ── Effect: init GPU edges/grid pipeline once gpu_ready + snapshot arrive ──
    let cam_for_effect = camera.clone();
    Effect::new(move |_| {
        // Reactive dependencies — re-run when either changes.
        let gpu_ready = overlay.gpu_ready.get();
        let snap_opt  = snapshot.get();
        if !gpu_ready { return; }
        let Some(snap) = snap_opt else { return; };

        // Check-and-set init guard (non-reactive — prevents double-init).
        let already = gpu_init_done
            .try_update_value(|v| { let was = *v; *v = true; was })
            .unwrap_or(true);
        if already { return; }

        // Borrow GPU device + format from the overlay.
        let Some(gpu_sw) = overlay.gpu.try_with_value(|v| v.clone()).flatten() else {
            gpu_init_done.try_update_value(|v| *v = false); // allow retry
            return;
        };
        let (device, format) = {
            let g = gpu_sw.borrow();
            (g.device.clone(), g.format.clone())
        };

        // Build force-directed layout.
        let nodes = Rc::new(build_layout(&snap));
        let edges = Rc::new(snap.edges.clone());

        // Build GPU pipeline for edges + grid only (max_nodes=1 for minimal buf).
        let max_edges = (edges.len() + 64) as u32;
        let Some(res) = build_hypergraph_resources(&device, &format, 1, max_edges) else {
            gpu_init_done.try_update_value(|v| *v = false);
            return;
        };
        write_f32_buf(&device, &res.palette_ub, &build_default_palette());
        let res_rc = Rc::new(res);

        // Capture state for the render callback.
        let cam_rc     = cam_for_effect.clone();
        let nodes_cb   = nodes.clone();
        let edges_cb   = edges.clone();
        let res_for_cb = res_rc.clone();

        let cb: RenderCallback = Box::new(
            move |pass: &GpuRenderPassEncoder,
                  _dev,
                  queue,
                  time: f64,
                  _dt: f64,
                  cw: u32,
                  ch: u32,
                  _depth: Option<&GpuTextureView>| {
                // ── Container bounding rect (CSS px) ──────────────────────
                // The canvas is full-viewport; restrict 3D drawing to this
                // component's area via setViewport / setScissorRect.
                let (css_x, css_y, css_w, css_h) = container_ref
                    .get_untracked()
                    .map(|el| {
                        let r = el.get_bounding_client_rect();
                        (r.left() as f32, r.top() as f32, r.width() as f32, r.height() as f32)
                    })
                    .unwrap_or((0.0, 0.0, cw as f32, ch as f32));

                // Convert CSS px → physical px using devicePixelRatio.
                let dpr = web_sys::window()
                    .map(|w| w.device_pixel_ratio() as f32)
                    .unwrap_or(1.0);
                let px_x = (css_x * dpr) as u32;
                let px_y = (css_y * dpr) as u32;
                let px_w = ((css_w * dpr) as u32).min(cw.saturating_sub(px_x));
                let px_h = ((css_h * dpr) as u32).min(ch.saturating_sub(px_y));
                if px_w == 0 || px_h == 0 { return; }

                call_set_viewport(pass, px_x, px_y, px_w, px_h);
                call_set_scissor_rect(pass, px_x, px_y, px_w, px_h);

                // Camera uses physical pixel dims for correct aspect ratio.
                let cam = cam_rc.borrow().clone();
                let vp  = cam.view_proj(px_w, px_h);
                let eye = cam.eye();

                // Push camera state for the background 3D smoke shader.
                let inv_vp = mat4_inverse(vp).unwrap_or_else(mat4_identity);
                set_particle_cam(vp, inv_vp, eye);

                // Update GPU camera uniform.
                let res = &res_for_cb;
                write_f32(queue, &res.cam_ub, &pack_cam_uniform(&vp, eye, time as f32));

                // Move DOM nodes to CSS-pixel positions within the container.
                // world_to_screen maps to [0, css_w] × [0, css_h].
                update_node_transforms(&nodes_cb, vp, eye, css_w as u32, css_h as u32);

                // ── Grid ──
                pass.set_pipeline(res.edge_pipeline.unchecked_ref());
                call_set_bind_group(pass.as_ref(), 0, &res.cam_bg);
                pass.set_vertex_buffer(0, Some(res.quad_vb.as_ref()));
                pass.set_vertex_buffer(1, Some(res.grid_ib.as_ref()));
                pass.draw_with_instance_count(6, res.grid_count);

                // ── Edges ──
                let mut edge_data: Vec<f32> =
                    Vec::with_capacity(edges_cb.len() * EDGE_INSTANCE_FLOATS);
                for e in edges_cb.iter() {
                    let a = nodes_cb.iter().find(|n| n.index == e.from);
                    let b = nodes_cb.iter().find(|n| n.index == e.to);
                    if let (Some(a), Some(b)) = (a, b) {
                        push_edge_instance(
                            &mut edge_data,
                            [a.x, a.y, a.z], [b.x, b.y, b.z],
                            edge_color(e.pattern_idx), 0.0, 1.0,
                        );
                    }
                }
                if !edge_data.is_empty() {
                    write_f32(queue, &res.edge_ib, &edge_data);
                    pass.set_vertex_buffer(1, Some(res.edge_ib.as_ref()));
                    pass.draw_with_instance_count(
                        6, (edge_data.len() / EDGE_INSTANCE_FLOATS) as u32,
                    );
                }
            },
        );

        let id = overlay.register(cb);
        callback_id.set_value(Some(id));
    });

    // ── Cleanup: remove our callback from the loop ────────────────────────────
    on_cleanup(move || {
        let _ = callback_id.try_with_value(|id_opt| {
            if let Some(id) = *id_opt {
                let _ = overlay.callbacks.try_with_value(|cbs_sw| {
                    if let Some(slot) = cbs_sw.borrow_mut().get_mut(id) {
                        *slot = Box::new(|_, _, _, _, _, _, _, _| {});
                    }
                });
            }
        });
    });

    // ── Mouse / wheel handlers ────────────────────────────────────────────────

    let on_mousedown = move |e: MouseEvent| {
        let mut cam = cam_down.borrow_mut();
        cam.orbiting = true;
        cam.last_mx  = e.client_x() as f32;
        cam.last_my  = e.client_y() as f32;
        e.prevent_default();
    };
    let on_mousemove = move |e: MouseEvent| {
        let mut cam = cam_move.borrow_mut();
        if !cam.orbiting { return; }
        cam.yaw   += (e.client_x() as f32 - cam.last_mx) * 0.005;
        cam.pitch  = (cam.pitch + (e.client_y() as f32 - cam.last_my) * 0.005)
            .clamp(-1.4, 1.4);
        cam.last_mx = e.client_x() as f32;
        cam.last_my = e.client_y() as f32;
    };
    let on_mouseup = move |_: MouseEvent| {
        cam_up_rc.borrow_mut().orbiting = false;
    };
    let on_wheel = move |e: WheelEvent| {
        let cur_dist = cam_wheel.borrow().dist;
        cam_wheel.borrow_mut().dist =
            (cur_dist + e.delta_y() as f32 * 0.01).clamp(1.5, 60.0);
        e.prevent_default();
    };

    // ── View ─────────────────────────────────────────────────────────────────

    view! {
        <div class="lv-hypergraph-view"
             node_ref=container_ref
             on:mousedown=on_mousedown
             on:mousemove=on_mousemove
             on:mouseup=on_mouseup
             on:contextmenu=|e: web_sys::MouseEvent| e.prevent_default()
             on:wheel=on_wheel>

            // DOM node layer — nodes are absolutely positioned via CSS transforms
            <div class="hg-node-layer">
                {move || snapshot.get().map(|snap| {
                    snap.nodes.iter().map(|node| {
                        let is_atom = node.width == 1;
                        let idx   = node.index;
                        let label = node.label.clone();
                        view! {
                            <div class="hg-node"
                                 class:hg-atom=is_atom
                                 data-hg-idx=idx>
                                <span class="hg-node-idx">{format!("#{idx}")}</span>
                                <span class="hg-node-label">{label}</span>
                                {is_atom.then(|| view! {
                                    <span class="hg-atom-tag">"·"</span>
                                })}
                            </div>
                        }
                    }).collect::<Vec<_>>()
                })}
            </div>

            // Info bar — bottom-left overlay
            {move || snapshot.get().map(|snap| {
                let atoms = snap.nodes.iter().filter(|n| n.width == 1).count();
                view! {
                    <div class="hg-info-bar">
                        {format!("{} nodes ({} atoms) · {} edges · drag: orbit · scroll: zoom",
                            snap.nodes.len(), atoms, snap.edges.len())}
                    </div>
                }
            })}
        </div>
    }
}
