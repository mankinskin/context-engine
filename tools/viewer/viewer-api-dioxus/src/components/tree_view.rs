//! TreeView and FileTree components.
//!
//! [`TreeView`] renders a recursive, keyboard-accessible tree of [`TreeNode`]s
//! with expand/collapse and single-selection state stored in Dioxus signals.
//!
//! [`FileTree`] wraps [`TreeView`] and adds:
//!  - A multi-key sort header (`.file-tree__sort-header`).
//!  - Filter buttons with per-filter item counts and color badges
//!    (`.file-tree__filter-header`).
//!  - Loading and empty states.
//!
//! CSS class names mirror the TypeScript viewer-api package.
use dioxus::prelude::*;

use crate::components::{ChevronRightIcon, FileIcon, FilterIcon, FolderIcon, FolderOpenIcon, Spinner};

// ── Data types ────────────────────────────────────────────────────────────────

/// A single node in the tree.
#[derive(Clone, PartialEq)]
pub struct TreeNode {
    /// Unique identifier used as the React-style `key` and selection value.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional badge text shown after the label (e.g. a count).
    pub badge: Option<String>,
    /// Optional tooltip text.
    pub tooltip: Option<String>,
    /// CSS colour string applied to the badge.
    pub badge_color: Option<String>,
    /// Whether this node is a directory / group or a leaf.
    pub is_dir: bool,
    /// Child nodes (only meaningful when `is_dir` is true).
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    /// Create a leaf node.
    pub fn leaf(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            badge: None,
            tooltip: None,
            badge_color: None,
            is_dir: false,
            children: vec![],
        }
    }

    /// Create a directory node.
    pub fn dir(id: impl Into<String>, label: impl Into<String>, children: Vec<TreeNode>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            badge: None,
            tooltip: None,
            badge_color: None,
            is_dir: true,
            children,
        }
    }
}

// ── Sort keys ─────────────────────────────────────────────────────────────────

/// Multi-key sort descriptor for [`FileTree`].
#[derive(Clone, PartialEq, Debug)]
pub struct SortKey {
    /// Column identifier / label shown in the sort header button.
    pub key: String,
    /// Human-readable display label.
    pub label: String,
    /// Ascending (`true`) or descending.
    pub ascending: bool,
}

impl SortKey {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            ascending: true,
        }
    }
}

// ── Filter definitions ────────────────────────────────────────────────────────

/// A filter button descriptor for [`FileTree`].
#[derive(Clone, PartialEq)]
pub struct FilterDef {
    /// Unique key used to track active state.
    pub key: String,
    /// Display label on the button.
    pub label: String,
    /// Number of items matching this filter.
    pub count: usize,
    /// CSS colour for the count badge (e.g. `"var(--accent-green)"`).
    pub color: Option<String>,
}

impl FilterDef {
    pub fn new(key: impl Into<String>, label: impl Into<String>, count: usize) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            count,
            color: None,
        }
    }
}

// ── Internal recursive item ───────────────────────────────────────────────────

/// Renders a single tree row plus its (optionally visible) children.
/// Uses `use_context` to read shared expanded/selected signals.
#[component]
fn TreeItem(
    node: TreeNode,
    depth: usize,
    expanded_ids: Signal<Vec<String>>,
    selected_id: Signal<Option<String>>,
    on_select: EventHandler<String>,
) -> Element {
    let node_id = node.id.clone();
    let node_id_select = node.id.clone();
    let is_expanded = use_memo(move || expanded_ids.read().contains(&node_id));
    let is_selected = use_memo(move || {
        selected_id
            .read()
            .as_deref()
            .map_or(false, |s| s == node_id_select)
    });

    let row_class = use_memo(move || {
        if *is_selected.read() {
            "tree-item-row selected"
        } else {
            "tree-item-row"
        }
    });

    let toggle_id = node.id.clone();
    let indent = format!("padding-left: {}px", depth * 16);

    // Icon selection
    let icon: Element = if node.is_dir {
        if *is_expanded.read() {
            rsx! { FolderOpenIcon { size: 16, class: "tree-icon folder" } }
        } else {
            rsx! { FolderIcon { size: 16, class: "tree-icon folder" } }
        }
    } else {
        rsx! { FileIcon { size: 16, class: "tree-icon file" } }
    };

    let select_id = node.id.clone();
    let has_children = !node.children.is_empty();

    rsx! {
        div {
            class: "tree-item",
            // Row
            div {
                class: "{row_class}",
                style: "{indent}",
                role: "treeitem",
                aria_expanded: if node.is_dir { Some(*is_expanded.read()) } else { None },
                aria_selected: "{is_selected}",
                title: node.tooltip.as_deref().unwrap_or(""),
                // Toggle chevron
                span {
                    class: if has_children {
                        if *is_expanded.read() { "tree-toggle expanded" } else { "tree-toggle" }
                    } else {
                        "tree-toggle empty"
                    },
                    onclick: move |e| {
                        e.stop_propagation();
                        if has_children {
                            let mut ids = expanded_ids.write();
                            if let Some(pos) = ids.iter().position(|id| id == &toggle_id) {
                                ids.remove(pos);
                            } else {
                                ids.push(toggle_id.clone());
                            }
                        }
                    },
                    ChevronRightIcon { size: 12 }
                }
                // Item icon
                {icon}
                // Label
                span {
                    class: "tree-label",
                    onclick: move |_| {
                        selected_id.set(Some(select_id.clone()));
                        on_select.call(select_id.clone());
                    },
                    "{node.label}"
                }
                // Optional badge
                if let Some(badge) = &node.badge {
                    span {
                        class: "tree-badge",
                        style: node.badge_color.as_deref().map(|c| format!("color: {c}")).unwrap_or_default(),
                        "{badge}"
                    }
                }
            }
            // Children (only rendered when expanded)
            if *is_expanded.read() && !node.children.is_empty() {
                div {
                    class: "tree-children",
                    for child in node.children.clone() {
                        TreeItem {
                            key: "{child.id}",
                            node: child,
                            depth: depth + 1,
                            expanded_ids,
                            selected_id,
                            on_select,
                        }
                    }
                }
            }
        }
    }
}

// ── TreeView ──────────────────────────────────────────────────────────────────

/// Recursive tree with Dioxus-signal-backed expand/collapse and selection.
///
/// Pass pre-built [`TreeNode`] roots in `nodes`.  Optionally control the
/// selected node via `selected_id` and listen to changes with `on_select`.
#[component]
pub fn TreeView(
    /// Root-level tree nodes.
    nodes: Vec<TreeNode>,
    /// Currently selected node id (uncontrolled if `None`).
    #[props(default)]
    selected_id: Option<String>,
    /// Called when a node is clicked.
    #[props(default)]
    on_select: EventHandler<String>,
    /// Initially expanded node ids.
    #[props(default)]
    initially_expanded: Vec<String>,
    /// Extra CSS classes on the `.tree-view` div.
    #[props(default)]
    class: String,
) -> Element {
    let expanded_ids = use_signal(|| initially_expanded);
    let sel_id: Signal<Option<String>> = use_signal(|| selected_id);

    let combined = if class.is_empty() {
        "tree-view".to_string()
    } else {
        format!("tree-view {class}")
    };

    rsx! {
        div {
            class: "{combined}",
            role: "tree",
            for node in nodes {
                TreeItem {
                    key: "{node.id}",
                    node,
                    depth: 0,
                    expanded_ids,
                    selected_id: sel_id,
                    on_select,
                }
            }
        }
    }
}

// ── FileTree ──────────────────────────────────────────────────────────────────

/// FileTree: wraps [`TreeView`] with sort/filter headers and loading/empty states.
///
/// Renders:
/// - `.file-tree__sort-header` — row of sort buttons; active key indicated by
///   `.file-tree__sort-btn--active`.
/// - `.file-tree__filter-header` — column of filter toggle buttons with count
///   badges and per-filter accent colours.
/// - Loading state (`.file-tree__loading`) when `loading` is true.
/// - Empty state (`.file-tree__empty`) when `nodes` is empty and not loading.
/// - [`TreeView`] otherwise.
#[component]
pub fn FileTree(
    /// Tree data.
    nodes: Vec<TreeNode>,
    /// Available sort columns.  The first active SortKey is the primary sort.
    #[props(default)]
    sort_keys: Vec<SortKey>,
    /// Called when a sort key button is clicked.  Callers apply the sort.
    #[props(default)]
    on_sort: EventHandler<String>,
    /// Available filter buttons.
    #[props(default)]
    filters: Vec<FilterDef>,
    /// Currently active filter keys.
    #[props(default)]
    active_filters: Vec<String>,
    /// Called when a filter button is toggled.
    #[props(default)]
    on_filter: EventHandler<String>,
    /// Currently selected node id.
    #[props(default)]
    selected_id: Option<String>,
    /// Called when a node is selected.
    #[props(default)]
    on_select: EventHandler<String>,
    /// Initially expanded node ids.
    #[props(default)]
    initially_expanded: Vec<String>,
    /// Show loading spinner instead of tree content.
    #[props(default = false)]
    loading: bool,
    /// Extra CSS classes on the outermost `.file-tree` div.
    #[props(default)]
    class: String,
) -> Element {
    let combined = if class.is_empty() {
        "file-tree".to_string()
    } else {
        format!("file-tree {class}")
    };

    rsx! {
        div {
            class: "{combined}",

            // ── Sort header ──
            if !sort_keys.is_empty() {
                div {
                    class: "file-tree__sort-header",
                    for sk in &sort_keys {
                        {
                            let key = sk.key.clone();
                            let btn_class = if sk.ascending {
                                "file-tree__sort-btn file-tree__sort-btn--active"
                            } else {
                                "file-tree__sort-btn"
                            };
                            rsx! {
                                button {
                                    key: "{sk.key}",
                                    class: "{btn_class}",
                                    onclick: move |_| on_sort.call(key.clone()),
                                    "{sk.label}"
                                    if sk.ascending { " ↑" } else { " ↓" }
                                }
                            }
                        }
                    }
                }
            }

            // ── Filter header ──
            if !filters.is_empty() {
                div {
                    class: "file-tree__filter-header",
                    for f in &filters {
                        {
                            let fkey = f.key.clone();
                            let is_active = active_filters.contains(&f.key);
                            let btn_class = if is_active {
                                "file-tree__filter-btn file-tree__filter-btn--active"
                            } else {
                                "file-tree__filter-btn"
                            };
                            let badge_style = f.color.as_deref()
                                .map(|c| format!("color: {c}"))
                                .unwrap_or_default();
                            rsx! {
                                button {
                                    key: "{f.key}",
                                    class: "{btn_class}",
                                    onclick: move |_| on_filter.call(fkey.clone()),
                                    span {
                                        class: "file-tree__filter-icon",
                                        FilterIcon { size: 12 }
                                    }
                                    span { class: "file-tree__filter-label", "{f.label}" }
                                    span {
                                        class: "tree-badge",
                                        style: "{badge_style}",
                                        "{f.count}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Body ──
            if loading {
                div {
                    class: "file-tree__loading",
                    Spinner {}
                    span { " Loading…" }
                }
            } else if nodes.is_empty() {
                div {
                    class: "file-tree__empty",
                    "No items to display."
                }
            } else {
                TreeView {
                    nodes,
                    selected_id,
                    on_select,
                    initially_expanded,
                }
            }
        }
    }
}
