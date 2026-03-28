/// Root App component.
use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

use crate::components::{Header, HypergraphView, LogViewer, Sidebar, TabBar, ThemeSelector};
use crate::gpu::overlay::{start_overlay, OverlayContext};
use crate::store::provide_store;
use crate::types::ViewTab;
use crate::actions;

#[component]
pub fn App() -> impl IntoView {
    let store = provide_store();

    // ── Global GPU overlay — runs on every tab, canvas covers the full page. ──
    let overlay = OverlayContext {
        gpu:       StoredValue::new(None),
        gpu_ready: RwSignal::new(false),
        callbacks: StoredValue::new(SendWrapper::new(Rc::new(RefCell::new(Vec::new())))),
    };
    provide_context(overlay);

    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    // Start the overlay once the canvas element mounts.
    Effect::new(move |_| {
        let Some(el) = canvas_ref.get() else { return };
        let canvas: web_sys::HtmlCanvasElement = el.unchecked_into();
        start_overlay(overlay, canvas);
    });

    // Load file list on mount.
    Effect::new(move |_| {
        actions::load_log_files(store);
    });

    let active_tab = store.active_tab;

    view! {
        <>
            // Full-page GPU canvas — position:fixed, behind all UI (z-index:-1).
            <canvas id="gpu-overlay" node_ref=canvas_ref />

            <div class="lv-app">
                <Header />
                <div class="lv-body">
                    <Sidebar />
                    <main class="lv-main">
                        <TabBar />
                        <div class="lv-view-container">
                            {move || match active_tab.get() {
                                ViewTab::Logs => view! { <LogViewer /> }.into_any(),
                                ViewTab::Hypergraph => view! { <HypergraphView /> }.into_any(),
                                ViewTab::Settings => {
                                    view! { <ThemeSelector /> }
                                        .into_any()
                                }
                                ViewTab::Debug => {
                                    view! { <p class="lv-placeholder">"Debug — coming soon"</p> }
                                        .into_any()
                                }
                            }}
                        </div>
                        {move || {
                            store.error.get().map(|e| {
                                view! { <div class="lv-error-banner">{e}</div> }
                            })
                        }}
                    </main>
                </div>
            </div>
        </>
    }
}
