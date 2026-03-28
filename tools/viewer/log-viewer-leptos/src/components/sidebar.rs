/// Sidebar — lists available log files.
use leptos::prelude::*;

use crate::store::Store;
use crate::{actions, types::LogFile};

#[component]
pub fn Sidebar() -> impl IntoView {
    let store = expect_context::<Store>();
    let log_files = store.log_files;
    let current_file = store.current_file;

    view! {
        <aside class="lv-sidebar">
            <div class="lv-sidebar-section-title">"Log Files"</div>
            <ul class="lv-file-list">
                <For
                    each=move || log_files.get()
                    key=|f: &LogFile| f.name.clone()
                    children=move |file| {
                        let name = file.name.clone();
                        let name_display = name.clone();
                        let name_for_active = name.clone();
                        let name_for_click = name.clone();
                        let is_active =
                            move || current_file.get().as_deref() == Some(&name_for_active);
                        let on_click = move |_| {
                            actions::select_file(name_for_click.clone());
                        };
                        view! {
                            <li
                                class="lv-file-item"
                                class:lv-active=is_active
                                on:click=on_click
                            >
                                <span class="lv-file-name">{name_display}</span>
                            </li>
                        }
                    }
                />
            </ul>
        </aside>
    }
}
