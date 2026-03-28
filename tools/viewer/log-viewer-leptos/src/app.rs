/// Root App component.
use leptos::prelude::*;

use crate::components::{Header, HypergraphView, LogViewer, Sidebar, TabBar};
use crate::store::provide_store;
use crate::types::ViewTab;
use crate::actions;

#[component]
pub fn App() -> impl IntoView {
    let store = provide_store();

    // Load file list on mount.
    Effect::new(move |_| {
        actions::load_log_files(store);
    });

    let active_tab = store.active_tab;

    view! {
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
                                view! { <p class="lv-placeholder">"Settings — coming soon"</p> }
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
    }
}
