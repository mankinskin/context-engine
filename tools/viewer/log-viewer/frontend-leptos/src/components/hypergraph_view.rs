/// Hypergraph view — 3-D GPU-accelerated node graph.
///
/// Renders hypergraph nodes as Blinn-Phong cubes on a WebGPU canvas with a
/// particle overlay.  DOM `<div class="hg-node">` labels are laid out over the
/// GPU canvas for interaction.
///
/// # GPU architecture
/// - `OverlayContext` is provided here and consumed by `Scene3D`.
/// - The canvas occupies the full container (`position: absolute; inset: 0`).
/// - DOM node labels are positioned with `position: absolute` CSS transforms
///   matching the 3-D layout grid.
use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

use crate::gpu::overlay::{start_overlay, OverlayContext};
use crate::gpu::scene3d::Scene3D;
use crate::store::Store;
use crate::types::{HypergraphSnapshot, SnapshotNode};

#[component]
pub fn HypergraphView() -> impl IntoView {
    let store = expect_context::<Store>();
    let snapshot = store.hypergraph_snapshot();

    // Provide the overlay context so child components (Scene3D) can register
    // render callbacks.
    let overlay = OverlayContext {
        gpu: StoredValue::new(None),
        callbacks: StoredValue::new(SendWrapper::new(Rc::new(RefCell::new(Vec::new())))),
    };
    provide_context(overlay);

    // Canvas node ref — the WebGPU overlay renders into this element.
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    // Start the GPU overlay as soon as the canvas mounts.
    Effect::new(move |_| {
        let Some(canvas_el) = canvas_ref.get() else { return };
        let canvas: web_sys::HtmlCanvasElement = canvas_el.unchecked_into();
        start_overlay(overlay, canvas);
    });

    view! {
        <div class="lv-hypergraph-view" style="position: relative; overflow: hidden;">
            // GPU canvas — behind the DOM labels.
            <canvas
                node_ref=canvas_ref
                class="hg-gpu-canvas"
                style="position: absolute; inset: 0; width: 100%; height: 100%; z-index: 0; pointer-events: none;"
            />
            // Node labels + 3-D scene, rendered above canvas.
            <div style="position: relative; z-index: 1;">
                {move || match snapshot.get() {
                    None => view! {
                        <p class="lv-placeholder">
                            <span class="lv-placeholder-icon">"⬡"</span>
                            "No graph snapshot in this file."
                        </p>
                    }.into_any(),
                    Some(snap) => view! {
                        <Scene3D snapshot=snap.clone() />
                        <NodeLabels snapshot=snap />
                    }.into_any(),
                }}
            </div>
        </div>
    }
}

/// DOM overlay labels for the 3-D node grid (one `<div>` per node).
/// These are positioned over the GPU canvas to show text labels.
#[component]
fn NodeLabels(snapshot: HypergraphSnapshot) -> impl IntoView {
    let atom_count = snapshot.nodes.iter().filter(|n| n.width == 1).count();
    let compound_count = snapshot.nodes.len() - atom_count;
    let info = format!(
        "{} nodes ({} atoms, {} compound) · {} edges",
        snapshot.nodes.len(),
        atom_count,
        compound_count,
        snapshot.edges.len(),
    );

    view! {
        <div class="hg-node-list">
            <div class="hg-info-bar">{info}</div>
            <div class="hg-nodes">
                <For
                    each=move || snapshot.nodes.clone()
                    key=|n: &SnapshotNode| n.index
                    children=|node| {
                        let is_atom = node.width == 1;
                        view! {
                            <div
                                class="hg-node"
                                class:hg-atom=is_atom
                                title=format!(
                                    "idx={} width={}",
                                    node.index,
                                    node.width,
                                )
                            >
                                <span class="hg-node-idx">{format!("#{}", node.index)}</span>
                                <span class="hg-node-label">{node.label.clone()}</span>
                                {is_atom.then(|| view! { <span class="hg-atom-tag">"atom"</span> })}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
