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
                        let store_for_click = store;
                        let is_active =
                            move || current_file.get().as_deref() == Some(&name_for_active);
                        let on_click = move |_| {
                            actions::select_file(store_for_click, name_for_click.clone());
                        };

                        // Build metadata badges string
                        let mut badges = Vec::new();
                        if file.has_graph_snapshot { badges.push("⬡"); }
                        if file.has_search_ops     { badges.push("⌕"); }
                        if file.has_insert_ops     { badges.push("+"); }
                        let badge_str = badges.join(" ");

                        // Human-readable size
                        let size_str = format_size(file.size);

                        view! {
                            <li
                                class="lv-file-item"
                                class:lv-active=is_active
                                on:click=on_click
                                title=name.clone()
                            >
                                <span class="lv-file-name">{name_display}</span>
                                <div class="lv-file-meta">
                                    <span>{size_str}</span>
                                    {(!badge_str.is_empty()).then(|| view! {
                                        <span class="lv-file-badges">{badge_str}</span>
                                    })}
                                </div>
                            </li>
                        }
                    }
                />
            </ul>
        </aside>
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
