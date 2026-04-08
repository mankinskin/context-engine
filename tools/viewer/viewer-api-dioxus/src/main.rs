use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

/// Root application component.
///
/// Renders a full-screen layout with a WebGPU canvas placeholder. The canvas
/// element is required from the start because the WgpuOverlay layer depends on
/// it being present in the DOM at startup; downstream component ports (TreeView,
/// TabBar, etc.) will be added on top of this scaffold.
#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "width: 100vw; height: 100vh; margin: 0; padding: 0; overflow: hidden; position: relative;",

            // Full-screen WebGPU canvas.
            // WgpuOverlay will acquire this element by id to create its GPU
            // surface. Keep the id stable — downstream tickets reference it.
            canvas {
                id: "webgpu-canvas",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; display: block;",
            }

            // UI overlay — viewer components mount here on top of the canvas.
            div {
                id: "ui-root",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none;",
            }
        }
    }
}
