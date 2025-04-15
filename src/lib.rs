pub mod app;
pub mod entities;
pub mod messages;
#[cfg(feature = "ssr")]
pub mod msg_sender;
#[cfg(feature = "ssr")]
pub mod state;
#[cfg(feature = "ssr")]
pub mod subs;
pub mod topic;
pub mod uploadthing;
pub mod ws;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
