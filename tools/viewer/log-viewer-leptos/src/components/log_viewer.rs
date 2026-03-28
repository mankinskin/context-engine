/// Log table — renders the list of log entries for the current file.
use leptos::prelude::*;

use crate::store::Store;
use crate::types::LogEntry;

#[component]
pub fn LogViewer() -> impl IntoView {
    let store = expect_context::<Store>();
    let entries = store.current_entries();

    view! {
        <div class="lv-log-viewer">
            <div class="lv-log-header-row">
                <span class="lv-col-level">"Level"</span>
                <span class="lv-col-target">"Target"</span>
                <span class="lv-col-message">"Message"</span>
            </div>
            <div class="lv-log-rows">
                <For
                    each=move || entries.get()
                    key=|e: &LogEntry| format!("{:?}-{:?}", e.timestamp, e.message)
                    children=|entry| {
                        let level_class = format!(
                            "lv-level-badge lv-level-{}",
                            entry.level.to_lowercase()
                        );
                        view! {
                            <div class="lv-log-row">
                                <span class=level_class>{entry.level.clone()}</span>
                                <span class="lv-col-target">
                                    {entry.target.clone().unwrap_or_default()}
                                </span>
                                <span class="lv-col-message">{entry.message.clone()}</span>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
