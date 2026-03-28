/// Log table — renders the list of log entries for the current file.
use leptos::prelude::*;

use crate::store::Store;
use crate::types::LogEntry;

#[component]
pub fn LogViewer() -> impl IntoView {
    let store = expect_context::<Store>();
    let entries = store.current_entries();

    let count = Memo::new(move |_| entries.get().len());

    view! {
        <div class="lv-log-viewer">
            <div class="lv-log-toolbar">
                <span>{move || format!("{} entries", count.get())}</span>
            </div>
            <div class="lv-log-header-row">
                <span>"Level"</span>
                <span>"Target"</span>
                <span>"Message"</span>
            </div>
            <div class="lv-log-rows">
                {move || {
                    if entries.get().is_empty() {
                        view! {
                            <p class="lv-placeholder">
                                <span class="lv-placeholder-icon">"📋"</span>
                                "No entries — select a log file from the sidebar"
                            </p>
                        }
                            .into_any()
                    } else {
                        view! {
                            <For
                                each=move || entries.get()
                                key=|e: &LogEntry| {
                                    format!("{:?}{}", e.timestamp, e.message)
                                }
                                children=|e| view! { <LogRow entry=e /> }
                            />
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn LogRow(entry: LogEntry) -> impl IntoView {
    let (expanded, set_expanded) = signal(false);

    let level = entry.level.clone();
    let level_class = format!("lv-level-badge lv-level-{}", level.to_lowercase());
    let target = entry.span_name.clone().unwrap_or_else(|| entry.event_type.clone());
    let message = entry.message.clone();

    // Build detail text for expanded view.
    let detail = {
        let mut parts = Vec::new();
        if let Some(ref file) = entry.file {
            let loc = match entry.source_line {
                Some(l) => format!("{}:{}", file, l),
                None => file.clone(),
            };
            parts.push(format!("location: {}", loc));
        }
        if let Some(ref ts) = entry.timestamp {
            parts.push(format!("timestamp: {}", ts));
        }
        if !entry.fields.is_null() {
            if let Ok(pretty) = serde_json::to_string_pretty(&entry.fields) {
                parts.push(format!("fields:\n{}", pretty));
            }
        }
        parts.join("\n")
    };

    let has_detail = !detail.is_empty();

    view! {
        <div
            class="lv-log-row"
            class:lv-expanded=expanded
            on:click=move |_| {
                if has_detail {
                    set_expanded.update(|v| *v = !*v);
                }
            }
        >
            <span class=level_class.clone()>{level}</span>
            <span class="lv-col-target">{target}</span>
            <span class="lv-col-message">{message}</span>
            <Show when=move || expanded.get() && has_detail>
                <div class="lv-entry-details">{detail.clone()}</div>
            </Show>
        </div>
    }
}