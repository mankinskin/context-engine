/// Global reactive state, mirroring the Preact Signals store from the TS log-viewer.
///
/// All state lives in Leptos reactive primitives (signals/memos) and is provided
/// to the component tree via Leptos context.
use leptos::prelude::*;
use std::collections::HashMap;

use crate::types::{LogEntry, LogFile, HypergraphSnapshot, ViewTab};

// ── Per-file state ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct FileState {
    pub entries: Vec<LogEntry>,
    pub search_query: String,
    pub level_filter: String,
    pub type_filter: String,
    pub selected_entry: Option<LogEntry>,
    pub active_tab: ViewTab,
    pub active_search_step: i32,
    pub active_path_id: Option<String>,
    pub active_path_step: i32,
}

// ── Global store ──────────────────────────────────────────────────────────────

/// The top-level global store, provided via Leptos context.
#[derive(Clone, Copy)]
pub struct Store {
    pub log_files: RwSignal<Vec<LogFile>>,
    pub current_file: RwSignal<Option<String>>,
    pub is_loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub status_message: RwSignal<String>,
    pub active_tab: RwSignal<ViewTab>,
    pub show_raw: RwSignal<bool>,
    pub auto_layout_enabled: RwSignal<bool>,
    /// Per-file state, keyed by filename.
    pub file_states: RwSignal<HashMap<String, FileState>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            log_files: RwSignal::new(vec![]),
            current_file: RwSignal::new(None),
            is_loading: RwSignal::new(false),
            error: RwSignal::new(None),
            status_message: RwSignal::new("Ready".to_string()),
            active_tab: RwSignal::new(ViewTab::default()),
            show_raw: RwSignal::new(false),
            auto_layout_enabled: RwSignal::new(false),
            file_states: RwSignal::new(HashMap::new()),
        }
    }

    /// Return a reactive memo for the current file's entries.
    pub fn current_entries(&self) -> Memo<Vec<LogEntry>> {
        let file_states = self.file_states;
        let current_file = self.current_file;
        Memo::new(move |_| {
            let filename = current_file.get();
            let states = file_states.get();
            filename
                .and_then(|f| states.get(&f).cloned())
                .map(|s| s.entries)
                .unwrap_or_default()
        })
    }

    /// Extract a hypergraph snapshot from the current file's log entries.
    pub fn hypergraph_snapshot(&self) -> Memo<Option<HypergraphSnapshot>> {
        let entries_memo = self.current_entries();
        Memo::new(move |_| {
            entries_memo.get().into_iter().find_map(|entry| {
                entry
                    .fields
                    .get("graph_snapshot")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
            })
        })
    }

    /// Set entries for the given file, creating a FileState slot if necessary.
    pub fn set_entries(&self, filename: String, entries: Vec<LogEntry>) {
        self.file_states.update(|map| {
            let state = map.entry(filename).or_default();
            state.entries = entries;
        });
    }
}

/// Provide the Store into the Leptos context so any descendant can call
/// `expect_context::<Store>()`.
pub fn provide_store() -> Store {
    let store = Store::new();
    provide_context(store);
    store
}
