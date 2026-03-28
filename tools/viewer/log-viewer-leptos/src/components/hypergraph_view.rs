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
            {move || {
                match snapshot.get() {
                    None => view! { <p class="lv-placeholder">"No graph snapshot in this file."</p> }.into_any(),
                    Some(snap) => view! { <NodeList snapshot=snap /> }.into_any(),
                }
            }}
        </div>
    }
}

#[component]
fn NodeList(snapshot: HypergraphSnapshot) -> impl IntoView {
    view! {
        <div class="hg-node-list">
            <div class="hg-info-bar">
                {format!(
                    "{} nodes | {} edges | {} atoms",
                    snapshot.nodes.len(),
                    snapshot.edges.len(),
                    snapshot.nodes.iter().filter(|n| n.is_atom).count()
                )}
            </div>
            <div class="hg-nodes">
                <For
                    each=move || snapshot.nodes.clone()
                    key=|n: &SnapshotNode| n.index
                    children=|node| {
                        let kind = if node.is_atom { "hg-atom" } else { "hg-compound" };
                        view! {
                            <div class=format!("hg-node {kind}") data-node-idx=node.index>
                                <span class="hg-node-label">{node.label.clone()}</span>
                                <span class="hg-node-idx">{format!("#{}", node.index)}</span>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
