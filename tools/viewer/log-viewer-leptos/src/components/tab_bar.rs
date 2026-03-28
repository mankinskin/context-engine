/// Tab bar — switches between Logs / Hypergraph / Settings.
use leptos::prelude::*;

use crate::store::Store;
use crate::types::ViewTab;

#[component]
pub fn TabBar() -> impl IntoView {
    let store = expect_context::<Store>();
    let active_tab = store.active_tab;

    let tabs = [ViewTab::Logs, ViewTab::Hypergraph, ViewTab::Settings];

    view! {
        <div class="lv-tab-bar" role="tablist">
            {tabs
                .into_iter()
                .map(|tab| {
                    let label = tab.label();
                    let tab_for_active = tab.clone();
                    let tab_for_click = tab.clone();
                    let is_active = move || active_tab.get() == tab_for_active;
                    let on_click = move |_| active_tab.set(tab_for_click.clone());
                    view! {
                        <button
                            class="lv-tab"
                            class:lv-tab-active=is_active
                            role="tab"
                            on:click=on_click
                        >
                            {label}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}
