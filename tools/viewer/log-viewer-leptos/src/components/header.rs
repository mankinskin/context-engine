/// Header component — search form and status bar.
use leptos::prelude::*;

use crate::store::Store;

#[component]
pub fn Header() -> impl IntoView {
    let store = expect_context::<Store>();
    let status = store.status_message;
    let is_loading = store.is_loading;

    let (query, set_query) = signal(String::new());

    let on_search = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        // TODO: trigger search action
        let _ = query.get();
    };

    view! {
        <header class="lv-header">
            <span class="lv-header-title">"Log Viewer"</span>
            <form class="lv-search-form" on:submit=on_search>
                <input
                    type="text"
                    class="lv-search-input"
                    placeholder="Search logs…"
                    prop:value=query
                    on:input=move |ev| set_query.set(event_target_value(&ev))
                />
                <button type="submit" class="lv-btn">"Search"</button>
            </form>
            <span class="lv-status">
                {move || if is_loading.get() { "Loading…" } else { "" }}
                {move || status.get()}
            </span>
        </header>
    }
}
