pub mod context_server_menu;
pub mod navbar;
pub mod server;
pub mod sidebar;

// use leptos::{Signal, SignalWith};
// use leptos_router::use_router;
// use uuid::Uuid;
//
// pub fn use_current_server() -> Signal<Option<Uuid>> {
//     Signal::derive(move || {
//         use_router().pathname().with(|path| {
//             path.split('/')
//                 .nth(2)
//                 .and_then(|path| Uuid::parse_str(path).ok())
//         })
//     })
// }
