pub mod app;
pub mod entities;
pub mod messages;
pub mod open_graph;
#[cfg(feature = "ssr")]
pub mod state;
pub mod sync;
pub mod uploadthing;
#[cfg(feature = "ssr")]
pub mod ws;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
