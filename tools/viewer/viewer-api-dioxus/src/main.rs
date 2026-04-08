use dioxus::prelude::*;
use viewer_api_dioxus::ViewerShell;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ViewerShell {}
    }
}
