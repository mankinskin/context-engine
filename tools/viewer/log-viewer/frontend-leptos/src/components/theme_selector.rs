/// Theme selector panel — shown in the Settings tab.
use leptos::prelude::*;

use crate::store::Store;
use crate::theme::all_presets;

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let store = expect_context::<Store>();
    let active = store.active_theme;

    let preset_names: Vec<&'static str> = all_presets().iter().map(|p| p.name).collect();

    view! {
        <div class="lv-theme-selector">
            <h3 class="lv-theme-heading">"Theme"</h3>
            <div class="lv-theme-grid">
                {preset_names
                    .into_iter()
                    .map(|name| {
                        let name_owned = name.to_string();
                        let name_for_click = name_owned.clone();
                        let name_for_class = name_owned.clone();
                        view! {
                            <button
                                class:lv-theme-btn=true
                                class:lv-theme-active=move || active.get() == name_for_class
                                on:click=move |_| {
                                    store.apply_theme(&name_for_click);
                                }
                            >
                                {name_owned}
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
