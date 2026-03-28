/// WASM entrypoint.
mod actions;
mod api;
mod app;
mod components;
mod gpu;
mod store;
mod types;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("logger init");
    leptos::mount::mount_to_body(app::App);
}
