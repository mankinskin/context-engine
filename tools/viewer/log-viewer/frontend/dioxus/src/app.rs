use std::collections::HashMap;

use dioxus::prelude::*;
use viewer_api_dioxus::{
    is_mobile_sidebar_viewport,
    set_gpu_overlay_enabled,
    FileTree,
    FileContentViewer,
    FilterDef,
    GlassPanel,
    HamburgerIcon,
    Header,
    HeaderActions,
    Layout,
    Overlay,
    LogIcon,
    SearchIcon,
    Sidebar,
    StatsIcon,
    TabBar,
    TabItem,
    ThemeProvider,
    ThemeSettings,
    TreeNode,
    ViewerShell,
    WgpuOverlay,
};

use crate::{
    api::{
        HttpLogViewerBackend,
        LogViewerBackend,
    },
    types::{
        LogEntry,
        LogFileInfo,
        Signatures,
    },
};

#[derive(Clone, Debug, Default, PartialEq)]
struct FileViewState {
    all_entries: Vec<LogEntry>,
    visible_entries: Vec<LogEntry>,
    search_query: String,
    jq_filter: String,
    active_tab: String,
    selected_line: Option<usize>,
    code_file: Option<String>,
    code_content: String,
    code_language: Option<String>,
    signatures: Signatures,
}

impl FileViewState {
    fn with_entries(entries: Vec<LogEntry>) -> Self {
        Self {
            all_entries: entries.clone(),
            visible_entries: entries,
            active_tab: "logs".to_string(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy)]
struct Category {
    key: &'static str,
    label: &'static str,
}

const CATEGORIES: [Category; 4] = [
    Category {
        key: "graph",
        label: "Graph",
    },
    Category {
        key: "search",
        label: "Search",
    },
    Category {
        key: "insert",
        label: "Insert",
    },
    Category {
        key: "paths",
        label: "Paths",
    },
];

fn parse_route(hash: &str) -> Option<(String, String)> {
    let raw = hash.trim_start_matches('#');
    let path = raw.trim_start_matches('/');
    let rest = path.strip_prefix("file/")?;

    let valid_tabs = ["logs", "stats", "hypergraph", "settings"];
    if let Some(last_slash) = rest.rfind('/') {
        let possible_tab = &rest[last_slash + 1..];
        if valid_tabs.contains(&possible_tab) {
            let file =
                urlencoding::decode(&rest[..last_slash]).ok()?.to_string();
            return Some((file, possible_tab.to_string()));
        }
    }

    let file = urlencoding::decode(rest).ok()?.to_string();
    Some((file, "logs".to_string()))
}

fn build_route(
    file: &str,
    tab: &str,
) -> String {
    let encoded = urlencoding::encode(file);
    if tab == "logs" {
        format!("/file/{encoded}")
    } else {
        format!("/file/{encoded}/{tab}")
    }
}

fn current_hash() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().and_then(|w| w.location().hash().ok())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

fn update_hash(
    file: &str,
    tab: &str,
) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_hash(&build_route(file, tab));
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (file, tab);
    }
}

fn file_matches_category(
    file: &LogFileInfo,
    category: Option<&str>,
) -> bool {
    match category {
        Some("graph") => file.has_graph_snapshot,
        Some("search") => file.has_search_ops,
        Some("insert") => file.has_insert_ops,
        Some("paths") => file.has_search_paths,
        _ => true,
    }
}

#[component]
pub fn App() -> Element {
    let backend = use_signal(HttpLogViewerBackend::default);
    let mut initialized = use_signal(|| false);

    let mut log_files = use_signal(Vec::<LogFileInfo>::new);
    let mut file_states = use_signal(HashMap::<String, FileViewState>::new);
    let mut active_category = use_signal(|| Option::<String>::None);

    let mut current_file = use_signal(|| Option::<String>::None);
    let mut active_tab = use_signal(|| "logs".to_string());
    let mut is_loading = use_signal(|| false);
    let mut status_message = use_signal(|| "Ready".to_string());
    let mut error_message = use_signal(|| Option::<String>::None);

    let mut search_input = use_signal(String::new);
    let mut jq_input = use_signal(String::new);
    let mut show_filters = use_signal(|| false);
    let mut show_theme_settings = use_signal(|| false);
    let mut fx_enabled = use_signal(|| true);
    let mut sidebar_collapsed = use_signal(|| false);
    let mut mobile_sidebar_open = use_signal(|| false);

    use_effect(move || {
        set_gpu_overlay_enabled(*fx_enabled.read());
    });

    let mut load_file = {
        move |name: String, route_tab: Option<String>| {
            if let Some(existing) = file_states.read().get(&name).cloned() {
                let tab = route_tab.unwrap_or(existing.active_tab);
                current_file.set(Some(name.clone()));
                active_tab.set(tab.clone());
                search_input.set(existing.search_query);
                jq_input.set(existing.jq_filter);
                status_message.set(format!(
                    "Loaded {name} ({} entries)",
                    existing.visible_entries.len()
                ));
                update_hash(&name, &tab);
                return;
            }

            let mut file_states = file_states;
            let mut current_file = current_file;
            let mut active_tab = active_tab;
            let mut search_input = search_input;
            let mut jq_input = jq_input;
            let mut is_loading = is_loading;
            let mut status_message = status_message;
            let mut error_message = error_message;
            let backend = backend.read().clone();

            spawn(async move {
                is_loading.set(true);
                error_message.set(None);
                status_message.set(format!("Loading {name}..."));

                let result = backend.get_log(&name).await;
                match result {
                    Ok(data) => {
                        let signatures = backend
                            .get_signatures(&name)
                            .await
                            .unwrap_or_default();
                        let first_source = data
                            .entries
                            .iter()
                            .find(|entry| entry.file.is_some());

                        let mut state =
                            FileViewState::with_entries(data.entries.clone());
                        state.signatures = signatures;
                        state.active_tab =
                            route_tab.unwrap_or_else(|| "logs".to_string());

                        if let Some(entry) = first_source {
                            if let Some(path) = &entry.file {
                                if let Ok(src) =
                                    backend.get_source_file(path).await
                                {
                                    state.code_file = Some(path.clone());
                                    state.code_content = src.content;
                                    state.code_language = Some(src.language);
                                    state.selected_line =
                                        entry.source_line.map(|v| v as usize);
                                }
                            }
                        }

                        file_states.with_mut(|states| {
                            states.insert(name.clone(), state.clone());
                        });
                        current_file.set(Some(name.clone()));
                        active_tab.set(state.active_tab.clone());
                        search_input.set(state.search_query.clone());
                        jq_input.set(state.jq_filter.clone());
                        update_hash(&name, &state.active_tab);
                        status_message.set(format!(
                            "Loaded {name} ({} entries)",
                            state.visible_entries.len()
                        ));
                    },
                    Err(err) => {
                        error_message.set(Some(err.to_string()));
                        status_message.set("Error loading file".to_string());
                    },
                }

                is_loading.set(false);
            });
        }
    };

    let refresh_all = {
        move || {
            let backend = backend.read().clone();
            let mut log_files = log_files;
            let mut is_loading = is_loading;
            let mut error_message = error_message;
            let mut status_message = status_message;

            spawn(async move {
                is_loading.set(true);
                error_message.set(None);
                match backend.list_logs().await {
                    Ok(files) => {
                        let count = files.len();
                        log_files.set(files);
                        status_message.set(format!("Found {count} log files"));
                    },
                    Err(err) => {
                        error_message.set(Some(err.to_string()));
                        status_message.set("Error loading files".to_string());
                    },
                }
                is_loading.set(false);
            });
        }
    };

    if !*initialized.read() {
        initialized.set(true);
        refresh_all();
        if let Some(hash) = current_hash() {
            if let Some((file, tab)) = parse_route(&hash) {
                load_file(file, Some(tab));
            }
        }
    }

    let current_state = current_file
        .read()
        .as_ref()
        .and_then(|name| file_states.read().get(name).cloned());
    let current_entries = current_state
        .as_ref()
        .map(|state| state.visible_entries.clone())
        .unwrap_or_default();

    let selected_filter = active_category.read().clone();
    let mut tree_nodes = Vec::new();
    for file in log_files
        .read()
        .iter()
        .filter(|file| file_matches_category(file, selected_filter.as_deref()))
    {
        tree_nodes.push(TreeNode::leaf(
            format!("file:{}", file.name),
            file.name.clone(),
        ));
    }

    tree_nodes.sort_by(|a, b| a.label.cmp(&b.label));

    let mut filter_defs = Vec::new();
    for category in CATEGORIES {
        let count = log_files
            .read()
            .iter()
            .filter(|file| file_matches_category(file, Some(category.key)))
            .count();
        filter_defs.push(FilterDef {
            key: category.key.to_string(),
            label: category.label.to_string(),
            count,
            color: None,
        });
    }

    let tabs = vec![
        TabItem::new("logs", "Logs"),
        TabItem {
            id: "stats".to_string(),
            label: "Stats".to_string(),
            icon: Some(rsx! { StatsIcon { size: 12 } }),
            modified: false,
            closeable: false,
        },
        TabItem::new("hypergraph", "Hypergraph"),
    ];

    rsx! {
        style { "html, body, #main {{ overflow: hidden; margin: 0; padding: 0; width: 100%; height: 100%; }}" }
        ThemeProvider {
            ViewerShell {
                WgpuOverlay {}
                Layout {
                    header: rsx! {
                        Header {
                            left: rsx! {
                                button {
                                    class: if *mobile_sidebar_open.read() || !*sidebar_collapsed.read() { "btn btn-icon btn-active" } else { "btn btn-icon" },
                                    aria_label: "Toggle sidebar",
                                    title: "Toggle sidebar",
                                    onclick: move |_| {
                                        if is_mobile_sidebar_viewport() {
                                            let next = !*mobile_sidebar_open.read();
                                            mobile_sidebar_open.set(next);
                                        } else {
                                            sidebar_collapsed.toggle();
                                        }
                                    },
                                    HamburgerIcon {}
                                }
                                div {
                                    class: "header-left",
                                    LogIcon { size: 14, color: "#8b9dc3" }
                                    h1 { class: "header-title", "Log Viewer" }
                                }
                            },
                            middle: rsx! {
                                div {
                                    class: "search-form",
                                    input {
                                        class: "search-input",
                                        r#type: "text",
                                        placeholder: "Search (regex supported)...",
                                        value: "{search_input}",
                                        oninput: move |evt| search_input.set(evt.value()),
                                    }
                                    button {
                                        class: "btn btn-primary",
                                        onclick: move |_| {
                                            let file = current_file.read().clone();
                                            let Some(file) = file else { return; };
                                            let query = search_input.read().trim().to_string();
                                            if query.is_empty() {
                                                file_states.with_mut(|states| {
                                                    if let Some(state) = states.get_mut(&file) {
                                                        state.visible_entries = state.all_entries.clone();
                                                        state.search_query.clear();
                                                        state.jq_filter.clear();
                                                    }
                                                });
                                                return;
                                            }

                                            let backend = backend.read().clone();
                                            let mut file_states = file_states;
                                            let mut is_loading = is_loading;
                                            let mut status_message = status_message;
                                            let mut error_message = error_message;
                                            let mut jq_input = jq_input;

                                            spawn(async move {
                                                is_loading.set(true);
                                                status_message.set(format!("Searching for '{query}'..."));
                                                match backend.search_log(&file, &query, None, None).await {
                                                    Ok(result) => {
                                                        file_states.with_mut(|states| {
                                                            if let Some(state) = states.get_mut(&file) {
                                                                state.visible_entries = result.matches;
                                                                state.search_query = query.clone();
                                                                state.jq_filter.clear();
                                                            }
                                                        });
                                                        jq_input.set(String::new());
                                                        status_message.set(format!(
                                                            "Search matched {} entries",
                                                            result.total_matches
                                                        ));
                                                    }
                                                    Err(err) => {
                                                        error_message.set(Some(err.to_string()));
                                                        status_message.set("Search error".to_string());
                                                    }
                                                }
                                                is_loading.set(false);
                                            });
                                        },
                                        SearchIcon { size: 12 }
                                        " Search"
                                    }
                                    button {
                                        class: "btn",
                                        onclick: move |_| {
                                            let next = !*show_filters.read();
                                            show_filters.set(next);
                                        },
                                        "Filters"
                                    }
                                    button {
                                        class: "btn",
                                        onclick: move |_| {
                                            let file = current_file.read().clone();
                                            let Some(file) = file else { return; };
                                            file_states.with_mut(|states| {
                                                if let Some(state) = states.get_mut(&file) {
                                                    state.visible_entries = state.all_entries.clone();
                                                    state.search_query.clear();
                                                    state.jq_filter.clear();
                                                }
                                            });
                                            search_input.set(String::new());
                                            jq_input.set(String::new());
                                            status_message.set("Cleared filters".to_string());
                                        },
                                        "Clear"
                                    }
                                }
                            },
                            right: rsx! {
                                div {
                                    class: "header-right",
                                    span { class: "status-text", "{status_message}" }
                                    button {
                                        class: "btn",
                                        onclick: move |_| {
                                            let enabled = !*fx_enabled.read();
                                            fx_enabled.set(enabled);
                                        },
                                        if *fx_enabled.read() { "✦ FX" } else { "✧ FX" }
                                    }
                                    HeaderActions {
                                        on_refresh: Some(EventHandler::new(move |_| {
                                            refresh_all();
                                            if let Some(file) = current_file.read().clone() {
                                                file_states.with_mut(|states| {
                                                    states.remove(&file);
                                                });
                                                load_file(file, None);
                                            }
                                        })),
                                        on_filter_toggle: Some(EventHandler::new(move |_| {
                                            let next = !*show_filters.read();
                                            show_filters.set(next);
                                        })),
                                        on_clear: Some(EventHandler::new(move |_| {
                                            if let Some(file) = current_file.read().clone() {
                                                file_states.with_mut(|states| {
                                                    if let Some(state) = states.get_mut(&file) {
                                                        state.visible_entries = state.all_entries.clone();
                                                        state.search_query.clear();
                                                        state.jq_filter.clear();
                                                    }
                                                });
                                            }
                                            search_input.set(String::new());
                                            jq_input.set(String::new());
                                            status_message.set("Cleared filters".to_string());
                                        })),
                                        on_theme_toggle: Some(EventHandler::new(move |_| {
                                            let next = !*show_theme_settings.read();
                                            show_theme_settings.set(next);
                                        })),
                                        filter_active: *show_filters.read(),
                                        has_active_filters: !search_input.read().trim().is_empty()
                                            || !jq_input.read().trim().is_empty(),
                                    }
                                }
                            },
                        }
                    },

                    Sidebar {
                        title: "Log Files".to_string(),
                        badge: log_files.read().len().to_string(),
                        collapsed: *sidebar_collapsed.read(),
                        on_toggle: move |_| {
                            if is_mobile_sidebar_viewport() {
                                mobile_sidebar_open.set(false);
                            } else {
                                sidebar_collapsed.toggle();
                            }
                        },
                        mobile_open: Some(*mobile_sidebar_open.read()),
                        on_mobile_open_change: move |open| mobile_sidebar_open.set(open),

                        FileTree {
                            nodes: tree_nodes,
                            filters: filter_defs,
                            active_filters: active_category.read().as_ref().map(|v| vec![v.clone()]).unwrap_or_default(),
                            selected_id: current_file.read().as_ref().map(|name| format!("file:{name}")),
                            loading: *is_loading.read() && log_files.read().is_empty(),
                            on_filter: move |key: String| {
                                if active_category.read().as_deref() == Some(key.as_str()) {
                                    active_category.set(None);
                                } else {
                                    active_category.set(Some(key));
                                }
                            },
                            on_select: move |id: String| {
                                if let Some(name) = id.strip_prefix("file:") {
                                    load_file(name.to_string(), None);
                                    mobile_sidebar_open.set(false);
                                }
                            },
                        }
                    }

                    main {
                        class: "content",
                        div {
                            class: "center-pane",
                            if *show_filters.read() {
                                div {
                                    class: "filter-panel",
                                    div {
                                        class: "filter-panel-content",
                                        input {
                                            class: "search-input",
                                            r#type: "text",
                                            placeholder: "JQ query...",
                                            value: "{jq_input}",
                                            oninput: move |evt| jq_input.set(evt.value()),
                                        }
                                        button {
                                            class: "btn",
                                            onclick: move |_| {
                                                let file = current_file.read().clone();
                                                let Some(file) = file else { return; };
                                                let jq = jq_input.read().trim().to_string();
                                                if jq.is_empty() {
                                                    return;
                                                }

                                                let backend = backend.read().clone();
                                                let mut file_states = file_states;
                                                let mut is_loading = is_loading;
                                                let mut status_message = status_message;
                                                let mut error_message = error_message;
                                                let mut search_input = search_input;

                                                spawn(async move {
                                                    is_loading.set(true);
                                                    status_message.set("Applying JQ filter...".to_string());
                                                    match backend.query_log(&file, &jq, None).await {
                                                        Ok(result) => {
                                                            file_states.with_mut(|states| {
                                                                if let Some(state) = states.get_mut(&file) {
                                                                    state.visible_entries = result.matches;
                                                                    state.jq_filter = jq.clone();
                                                                    state.search_query.clear();
                                                                }
                                                            });
                                                            search_input.set(String::new());
                                                            status_message.set(format!(
                                                                "JQ matched {} entries",
                                                                result.total_matches
                                                            ));
                                                        }
                                                        Err(err) => {
                                                            error_message.set(Some(err.to_string()));
                                                            status_message.set("JQ query failed".to_string());
                                                        }
                                                    }
                                                    is_loading.set(false);
                                                });
                                            },
                                            "Apply"
                                        }
                                        button {
                                            class: "btn",
                                            onclick: move |_| show_filters.set(false),
                                            "Close"
                                        }
                                    }
                                }
                            }

                            TabBar {
                                tabs,
                                active_id: active_tab.read().clone(),
                                on_select: move |tab_id: String| {
                                    active_tab.set(tab_id.clone());
                                    if let Some(file) = current_file.read().clone() {
                                        file_states.with_mut(|states| {
                                            if let Some(state) = states.get_mut(&file) {
                                                state.active_tab = tab_id.clone();
                                            }
                                        });
                                        update_hash(&file, &tab_id);
                                    }
                                },
                            }

                            div {
                                class: "view-container",
                                if let Some(error) = error_message.read().clone() {
                                    div { class: "alert alert-error", "{error}" }
                                }

                                if *active_tab.read() == "logs" {
                                    GlassPanel {
                                        title: "Log Entries",
                                        if current_entries.is_empty() {
                                            div { class: "empty-state", "Select a log file to view entries." }
                                        } else {
                                            ul {
                                                class: "log-list",
                                                for entry in current_entries.iter() {
                                                    {
                                                        let entry = entry.clone();
                                                        let level = entry.level.to_uppercase();
                                                        let message = entry.message.clone();
                                                        rsx! {
                                                            li {
                                                                key: "entry-{entry.line_number}",
                                                                class: "log-row",
                                                                onclick: move |_| {
                                                                    let Some(path) = entry.file.clone() else { return; };
                                                                    let backend = backend.read().clone();
                                                                    let file = current_file.read().clone();
                                                                    let mut file_states = file_states;
                                                                    let mut error_message = error_message;
                                                                    spawn(async move {
                                                                        match backend.get_source_file(&path).await {
                                                                            Ok(src) => {
                                                                                if let Some(file) = file {
                                                                                    file_states.with_mut(|states| {
                                                                                        if let Some(state) = states.get_mut(&file) {
                                                                                            state.code_file = Some(path.clone());
                                                                                            state.code_content = src.content;
                                                                                            state.code_language = Some(src.language);
                                                                                            state.selected_line = entry.source_line.map(|n| n as usize);
                                                                                        }
                                                                                    });
                                                                                }
                                                                            }
                                                                            Err(err) => error_message.set(Some(err.to_string())),
                                                                        }
                                                                    });
                                                                },
                                                                span { class: "log-row-level", "{level}" }
                                                                span { class: "log-row-line", "#{entry.line_number}" }
                                                                span { class: "log-row-message", "{message}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if *active_tab.read() == "stats" {
                                    GlassPanel {
                                        h3 { "Stats" }
                                        p { "Entries in current view: {current_entries.len()}" }
                                        if let Some(state) = current_state.clone() {
                                            p { "Total entries in file: {state.all_entries.len()}" }
                                            p { "Loaded signatures: {state.signatures.len()}" }
                                        }
                                    }
                                } else if *active_tab.read() == "hypergraph" {
                                    GlassPanel {
                                        h3 { "Hypergraph" }
                                        p { "Hypergraph-specific rendering will be migrated in LOG-5e." }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "log-source-panel",
                        if let Some(state) = current_state {
                            if state.code_content.is_empty() {
                                GlassPanel {
                                    title: "Source",
                                    div {
                                        class: "empty-state",
                                        "Select a log entry with source info to open code context."
                                    }
                                }
                            } else {
                                FileContentViewer {
                                    content: state.code_content,
                                    filename: state.code_file.unwrap_or_else(|| "source.rs".to_string()),
                                    language: state.code_language,
                                    highlighted_line: state.selected_line,
                                }
                            }
                        } else {
                            GlassPanel {
                                title: "Source",
                                div {
                                    class: "empty-state",
                                    "No file selected."
                                }
                            }
                        }
                    }
                }

                Overlay {
                    open: *show_theme_settings.read(),
                    on_close: move |_| show_theme_settings.set(false),
                    ThemeSettings {
                        on_close: move |_| show_theme_settings.set(false),
                    }
                }
            }
        }
    }
}
