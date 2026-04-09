//! Layout components — Header, Layout, Sidebar (collapsible + resizable +
//! mobile-drawer), Panel (left/right/top/bottom placement), and GlassPanel.
//!
//! CSS class names match the TypeScript viewer-api package so that the shared
//! `viewer-api.css` stylesheet applies without modification.
use dioxus::prelude::*;

use crate::components::{
    ChevronRightIcon, CloseIcon, HamburgerIcon, ResizeDirection, ResizeEdge, ResizeHandle,
};

// ── Header ────────────────────────────────────────────────────────────────────

/// Slim top bar matching `.header` / `.header-left` / `.header-right` CSS.
///
/// Slot props default to `None` so callers only supply what they need.
#[component]
pub fn Header(
    /// Content for the left slot (icon + title area).
    #[props(default)]
    left: Option<Element>,
    /// Content for the optional centre slot.
    #[props(default)]
    middle: Option<Element>,
    /// Content for the right slot (action buttons, theme toggle, etc.).
    #[props(default)]
    right: Option<Element>,
    /// Extra CSS classes appended to the root `.header` div.
    #[props(default)]
    class: String,
) -> Element {
    let combined = if class.is_empty() {
        "header".to_string()
    } else {
        format!("header {class}")
    };

    rsx! {
        header {
            class: "{combined}",
            div {
                class: "header-left",
                if let Some(l) = left { {l} }
            }
            if let Some(m) = middle {
                div {
                    class: "header-middle",
                    {m}
                }
            }
            div {
                class: "header-right",
                if let Some(r) = right { {r} }
            }
        }
    }
}

// ── Layout ────────────────────────────────────────────────────────────────────

/// Full-page shell: `.app` column wrapping an optional [`Header`] and the
/// `.main-layout` flex row that holds sidebar + content children.
#[component]
pub fn Layout(
    /// Optional header element rendered above the main row.
    #[props(default)]
    header: Option<Element>,
    /// Main content placed inside `.main-layout`.
    children: Element,
    /// Extra CSS classes on the outer `.app` div.
    #[props(default)]
    class: String,
) -> Element {
    let combined = if class.is_empty() {
        "app".to_string()
    } else {
        format!("app {class}")
    };

    rsx! {
        div {
            class: "{combined}",
            if let Some(h) = header { {h} }
            div {
                class: "main-layout",
                {children}
            }
        }
    }
}

// ── Sidebar ───────────────────────────────────────────────────────────────────

/// Collapsible, resizable sidebar with a full-screen drawer at ≤768 px.
///
/// - Desktop: slides to `width: 0` when collapsed, shows a right-edge
///   [`ResizeHandle`] when expanded.
/// - Tablet (≤768 px): becomes a fixed-position overlay panel.  Toggled via
///   `sidebar-hamburger` button that callers usually place inside the
///   [`Header`] `left` slot.
/// - Phone (≤480 px): full-screen when open.
///
/// CSS classes applied:
/// - `.sidebar` always present.
/// - `.sidebar-collapsed` when `collapsed` is `true` (desktop).
/// - `.sidebar-mobile-open` / `.sidebar-mobile-closed` at mobile breakpoint.
/// - `.sidebar-resizing` while a drag gesture is in progress.
#[component]
pub fn Sidebar(
    /// Content rendered inside the sidebar body.
    children: Element,
    /// Optional heading shown in `.sidebar-header`.
    #[props(default)]
    title: Option<String>,
    /// Optional badge text shown next to the title.
    #[props(default)]
    badge: Option<String>,
    /// Collapsed state (desktop).
    #[props(default = false)]
    collapsed: bool,
    /// Called when the collapse/expand button is pressed.
    #[props(default)]
    on_toggle: EventHandler<()>,
    /// Whether the sidebar can be resized by dragging its right edge.
    #[props(default = true)]
    resizable: bool,
    /// Initial width in pixels.
    #[props(default = 280.0)]
    initial_width: f64,
    /// Minimum width constraint when resizing.
    #[props(default = 120.0)]
    min_width: f64,
    /// Extra CSS classes on the root `.sidebar` div.
    #[props(default)]
    class: String,
) -> Element {
    let mut width = use_signal(|| initial_width);
    let mut mobile_open = use_signal(|| false);

    // Build the CSS class string.
    let sidebar_class = use_memo(move || {
        let mut parts = vec!["sidebar"];
        if collapsed {
            parts.push("sidebar-collapsed");
        }
        if *mobile_open.read() {
            parts.push("sidebar-mobile-open");
        } else {
            parts.push("sidebar-mobile-closed");
        }
        let base = parts.join(" ");
        if class.is_empty() {
            base
        } else {
            format!("{base} {class}")
        }
    });

    let inline_style = use_memo(move || {
        if collapsed {
            String::new()
        } else {
            format!("width: {}px; min-width: {}px", *width.read(), min_width)
        }
    });

    rsx! {
        // Mobile: dim overlay backdrop — invisible on desktop via CSS
        div {
            class: if *mobile_open.read() { "sidebar-overlay visible" } else { "sidebar-overlay" },
            onclick: move |_| mobile_open.set(false),
        }

        div {
            class: "{sidebar_class}",
            style: "{inline_style}",

            // Header with title / badge / collapse button
            div {
                class: "sidebar-header",
                if let Some(t) = &title {
                    h2 { "{t}" }
                }
                if let Some(b) = &badge {
                    span { class: "sidebar-badge", "{b}" }
                }
                // Mobile close button (visible only on mobile via CSS `display`)
                button {
                    class: "sidebar-close-btn",
                    aria_label: "Close sidebar",
                    onclick: move |_| mobile_open.set(false),
                    CloseIcon {}
                }
                // Desktop collapse button
                button {
                    class: "sidebar-collapse-btn",
                    aria_label: "Collapse sidebar",
                    onclick: move |_| on_toggle.call(()),
                    // Chevron rotates to indicate state
                    span {
                        style: if collapsed { "transform: rotate(0deg)" } else { "transform: rotate(180deg)" },
                        ChevronRightIcon {}
                    }
                }
            }

            // Scrollable content area
            div {
                class: "sidebar-content",
                {children}
            }

            // Resize handle — hidden on mobile via CSS `.resize-handle` display:none
            if resizable && !collapsed {
                ResizeHandle {
                    edge: ResizeEdge::Right,
                    direction: ResizeDirection::Horizontal,
                    min_size: min_width,
                    on_resize: move |delta: f64| {
                        let new_w = (*width.read() + delta).max(min_width);
                        width.set(new_w);
                    },
                }
            }
        }

        // Hamburger toggle — hidden on desktop, shown on mobile via CSS
        // Exposed as a named slot so callers can place it in the Header.
        // We also render a standalone one here for convenience.
        button {
            class: "sidebar-hamburger",
            aria_label: "Open sidebar",
            onclick: move |_| mobile_open.set(true),
            HamburgerIcon {}
        }
    }
}

// ── Panel ─────────────────────────────────────────────────────────────────────

/// Placement for a [`Panel`].
#[derive(Clone, PartialEq, Default)]
pub enum PanelPlacement {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

impl PanelPlacement {
    fn css_class(&self) -> &'static str {
        match self {
            PanelPlacement::Left => "panel panel-left",
            PanelPlacement::Right => "panel panel-right",
            PanelPlacement::Top => "panel panel-top",
            PanelPlacement::Bottom => "panel panel-bottom",
        }
    }

    fn resize_edge(&self) -> ResizeEdge {
        match self {
            PanelPlacement::Left => ResizeEdge::Right,
            PanelPlacement::Right => ResizeEdge::Left,
            PanelPlacement::Top => ResizeEdge::Bottom,
            PanelPlacement::Bottom => ResizeEdge::Top,
        }
    }

    fn resize_direction(&self) -> ResizeDirection {
        match self {
            PanelPlacement::Left | PanelPlacement::Right => ResizeDirection::Horizontal,
            PanelPlacement::Top | PanelPlacement::Bottom => ResizeDirection::Vertical,
        }
    }

    fn is_horizontal(&self) -> bool {
        matches!(self, PanelPlacement::Left | PanelPlacement::Right)
    }
}

/// A resizable panel anchored to one edge of its container.
///
/// Uses the `.panel`, `.panel-left` / `.panel-right` / `.panel-top` /
/// `.panel-bottom` CSS classes from the shared stylesheet.
#[component]
pub fn Panel(
    /// Content inside the panel.
    children: Element,
    /// Which edge the panel is attached to.
    #[props(default)]
    placement: PanelPlacement,
    /// Initial size (width for Left/Right, height for Top/Bottom) in pixels.
    #[props(default = 300.0)]
    initial_size: f64,
    /// Minimum size constraint.
    #[props(default = 80.0)]
    min_size: f64,
    /// Whether the panel can be resized.
    #[props(default = true)]
    resizable: bool,
    /// Extra CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    let mut size = use_signal(|| initial_size);
    let resizing = use_signal(|| false);

    // Pre-compute placement-derived values before any closures capture ownership.
    let base_class = placement.css_class();
    let is_horizontal = placement.is_horizontal();
    let resize_edge = placement.resize_edge();
    let resize_dir = placement.resize_direction();

    let panel_class = use_memo(move || {
        let r = if *resizing.read() {
            format!("{base_class} panel-resizing")
        } else {
            base_class.to_string()
        };
        if class.is_empty() {
            r
        } else {
            format!("{r} {class}")
        }
    });

    let inline_style = use_memo(move || {
        if is_horizontal {
            format!("width: {}px", *size.read())
        } else {
            format!("height: {}px", *size.read())
        }
    });

    rsx! {
        div {
            class: "{panel_class}",
            style: "{inline_style}",
            {children}
            if resizable {
                ResizeHandle {
                    edge: resize_edge,
                    direction: resize_dir,
                    min_size: min_size,
                    on_resize: move |delta: f64| {
                        let new_size = (*size.read() + delta).max(min_size);
                        size.set(new_size);
                    },
                }
            }
        }
    }
}

// ── GlassPanel ────────────────────────────────────────────────────────────────

/// Frosted-glass card panel — `backdrop-filter: blur` overlay.
///
/// Applies CSS class `glass-panel` which must be defined in the active
/// stylesheet (see `viewer-api.css`).  Accepts an optional title rendered
/// in a `.glass-panel__title` span and arbitrary children content.
#[component]
pub fn GlassPanel(
    /// Optional title displayed at the top of the panel.
    #[props(default)]
    title: Option<String>,
    /// Panel body content.
    children: Element,
    /// Extra CSS classes on the root element.
    #[props(default)]
    class: String,
) -> Element {
    let combined = if class.is_empty() {
        "glass-panel".to_string()
    } else {
        format!("glass-panel {class}")
    };

    rsx! {
        div {
            class: "{combined}",
            if let Some(t) = title {
                div {
                    class: "glass-panel__header",
                    span { class: "glass-panel__title", "{t}" }
                }
            }
            div {
                class: "glass-panel__body",
                {children}
            }
        }
    }
}
