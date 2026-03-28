/// Sidebar — lists available log files with a collapsible panel and resize handle.
use leptos::prelude::*;
use viewer_api_leptos::components::{
    resize_handle::ResizeHandle,
    tree_view::{NodeIcon, TreeNode, TreeView},
};

use crate::store::Store;
use crate::{actions, types::LogFile};

/// Default sidebar width in CSS pixels.
const DEFAULT_WIDTH: f64 = 260.0;
/// Minimum sidebar width (prevents collapsing via drag).
const MIN_WIDTH: f64 = 120.0;

#[component]
pub fn Sidebar() -> impl IntoView {
    let store = expect_context::<Store>();
    let log_files = store.log_files;
    let current_file = store.current_file;

    // Sidebar width / collapsed state
    let width = RwSignal::new(DEFAULT_WIDTH);
    let collapsed = RwSignal::new(false);

    let on_resize = move |delta: f64| {
        width.update(|w| *w = (*w + delta).max(MIN_WIDTH));
    };

    // Build TreeNode list from log files
    let nodes = move || {
        log_files
            .get()
            .into_iter()
            .map(|file| TreeNode {
                id: file.name.clone(),
                label: file.name.clone(),
                icon: NodeIcon::File,
                badge: Some(format_size(file.size)),
                children: vec![],
            })
            .collect::<Vec<_>>()
    };

    let on_select: Box<dyn Fn(String) + 'static> = {
        let store = store;
        Box::new(move |id: String| {
            actions::select_file(store, id);
        })
    };

    let sidebar_style = move || {
        if collapsed.get() {
            "width: 0; overflow: hidden; min-width: 0;".to_string()
        } else {
            format!("width: {}px;", width.get())
        }
    };

    let file_count = move || log_files.with(|f| f.len());

    view! {
        <aside class="lv-sidebar" style=sidebar_style>
            <div class="lv-sidebar-header">
                <span class="lv-sidebar-title">
                    "Log Files"
                    {move || {
                        let n = file_count();
                        (n > 0).then(|| view! {
                            <span class="lv-sidebar-badge">{n}</span>
                        })
                    }}
                </span>
                <button
                    class="lv-collapse-btn"
                    title=move || if collapsed.get() { "Expand sidebar" } else { "Collapse sidebar" }
                    on:click=move |_| collapsed.update(|c| *c = !*c)
                >
                    {move || if collapsed.get() { "›" } else { "‹" }}
                </button>
            </div>

            <div class="lv-sidebar-body">
                <TreeView
                    nodes=nodes()
                    on_select=on_select
                />
            </div>

            <ResizeHandle on_resize=on_resize />
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

