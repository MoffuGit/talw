use leptos::prelude::{Memo, With};
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

pub mod category;
pub mod channel;
pub mod server_menu;
pub mod sidebar;
pub mod thread;

pub fn use_current_channel() -> Memo<Option<Uuid>> {
    Memo::new(move |_| {
        use_params_map().with(|map| {
            map.get("channel_id")
                .and_then(|channel| Uuid::parse_str(&channel).ok())
        })
    })
}

pub fn use_current_thread() -> Memo<Option<Uuid>> {
    Memo::new(move |_| {
        use_params_map().with(|map| {
            map.get("thread_id")
                .and_then(|channel| Uuid::parse_str(&channel).ok())
        })
    })
}
