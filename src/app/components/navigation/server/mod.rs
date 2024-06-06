use leptos::{Signal, SignalWith};
use leptos_router::use_router;
use uuid::Uuid;

pub mod category;
pub mod channel;
pub mod invite_people;
pub mod server_menu;
pub mod sidebar;

pub fn use_current_channel() -> Signal<Option<Uuid>> {
    Signal::derive(move || {
        use_router().pathname().with(|path| {
            path.split('/')
                .nth(3)
                .and_then(|path| Uuid::parse_str(path).ok())
        })
    })
}
