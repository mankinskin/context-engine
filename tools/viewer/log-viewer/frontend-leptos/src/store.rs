/// Global reactive state, mirroring the Preact Signals store from the TS log-viewer.
///
/// All state lives in Leptos reactive primitives (signals/memos) and is provided
/// to the component tree via Leptos context.
use leptos::prelude::*;
use std::collections::HashMap;

use crate::theme::{self, EffectSettings, PaletteData, all_presets, default_palette};
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
    /// Active theme name.
    pub active_theme: RwSignal<String>,
    /// Current effect settings (GPU overlay reads these every frame).
    pub effect_settings: RwSignal<EffectSettings>,
    /// Current palette data (GPU overlay reads these every frame).
    pub palette_data: RwSignal<PaletteData>,
}

impl Store {
    pub fn new() -> Self {
        let cinder = theme::preset_cinder();
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
            active_theme: RwSignal::new(cinder.name.to_string()),
            effect_settings: RwSignal::new(cinder.effects),
            palette_data: RwSignal::new(cinder.palette),
        }
    }

    /// Apply a named theme preset. Updates reactive signals and pushes to the
    /// thread-local for the GPU overlay to read next frame.
    pub fn apply_theme(&self, name: &str) {
        if let Some(preset) = all_presets().into_iter().find(|p| p.name == name) {
            self.active_theme.set(preset.name.to_string());
            self.effect_settings.set(preset.effects.clone());
            self.palette_data.set(preset.palette);
            theme::set_effect_settings(&preset.effects);
            theme::set_palette_data(&preset.palette);
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
    ///
    /// Mirrors the TS store: finds the first entry where `message == "graph_snapshot"`
    /// and deserialises `fields.graph_data` (which may be a JSON string or an object).
    pub fn hypergraph_snapshot(&self) -> Memo<Option<HypergraphSnapshot>> {
        let entries_memo = self.current_entries();
        Memo::new(move |_| {
            entries_memo.get().into_iter().find_map(|entry| {
                if entry.message != "graph_snapshot" {
                    return None;
                }
                let graph_data = entry.fields.get("graph_data")?;
                // graph_data may be a JSON string or an already-parsed object.
                let value: serde_json::Value = if let Some(s) = graph_data.as_str() {
                    serde_json::from_str(s).ok()?
                } else {
                    graph_data.clone()
                };
                if value.get("nodes").and_then(|v| v.as_array()).is_none()
                    || value.get("edges").and_then(|v| v.as_array()).is_none()
                {
                    return None;
                }
                serde_json::from_value(value).ok()
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
    // Push initial theme to thread-local so the GPU overlay has values on first frame.
    theme::set_effect_settings(&store.effect_settings.get_untracked());
    theme::set_palette_data(&store.palette_data.get_untracked());
    store
}
