pub mod components;
pub mod effects;
pub mod store;
pub use components::*;
pub use effects::WgpuOverlay;
pub use store::*;
// Explicit re-exports so downstream crates can import without glob.
pub use store::session::{clear_session, get_session_id, with_session};
pub use store::url_state::{get_hash_param, remove_hash_param, set_hash_param, UrlStateManager};

use dioxus::prelude::*;

/// Full-screen root shell used by all viewer applications built on this
/// platform. Renders an absolute-positioned WebGPU canvas underneath a
/// pointer-transparent UI overlay root.
///
/// Downstream crates should mount their own content *inside* `#ui-root` by
/// nesting it as children, or by relying on Dioxus Router to inject route
/// components into the overlay.
#[component]
pub fn ViewerShell(children: Element) -> Element {
    rsx! {
        div {
            style: "width: 100vw; height: 100vh; margin: 0; padding: 0; overflow: hidden; position: relative;",

            // Full-screen WebGPU canvas.
            // WgpuOverlay acquires this element by id to create its GPU
            // surface. Keep the id stable — downstream tickets reference it.
            canvas {
                id: "webgpu-canvas",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; display: block;",
            }

            // UI overlay — viewer components mount here on top of the canvas.
            div {
                id: "ui-root",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%;",
                {children}
            }
        }
    }
}
