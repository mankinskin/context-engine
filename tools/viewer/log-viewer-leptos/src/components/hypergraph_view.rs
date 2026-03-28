/// Hypergraph view — renders the snapshot as a flat node list.
///
/// This is a minimal starting point. The full GPU-accelerated rendering (the
/// wgpu pipeline from the TS version) will be added incrementally here,
/// using raw `web_sys` / WebGPU bindings.
use leptos::prelude::*;

use crate::store::Store;
use crate::types::{HypergraphSnapshot, SnapshotNode};

#[component]
pub fn HypergraphView() -> impl IntoView {
    let store = expect_context::<Store>();
    let snapshot = store.hypergraph_snapshot();

    view! {
        <div class="lv-hypergraph-view">
            {move || match snapshot.get() {
                None => {
                    view! {
                        <p class="lv-placeholder">
                            <span class="lv-placeholder-icon">"⬡"</span>
                            "No graph snapshot in this file."
                        </p>
                    }
                        .into_any()
                }
                Some(snap) => view! { <NodeList snapshot=snap /> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn NodeList(snapshot: HypergraphSnapshot) -> impl IntoView {
    let atom_count = snapshot.nodes.iter().filter(|n| n.is_atom).count();
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
                        let is_atom = node.is_atom;
                        let children_count = node.child_indices.len();
                        view! {
                            <div
                                class="hg-node"
                                class:hg-atom=is_atom
                                title=format!(
                                    "idx={} width={} children={}",
                                    node.index,
                                    node.width,
                                    children_count,
                                )
                            >
                                <span class="hg-node-idx">{format!("#{}", node.index)}</span>
                                <span class="hg-node-label">{node.label.clone()}</span>
                                {is_atom.then(|| view! { <span class="hg-atom-tag">"atom"</span> })}
                                {(!is_atom && children_count > 0)
                                    .then(|| {
                                        view! {
                                            <span class="hg-node-children">
                                                {format!("[{}]", children_count)}
                                            </span>
                                        }
                                    })}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
