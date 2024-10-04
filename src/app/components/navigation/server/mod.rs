use std::str::FromStr;

use leptos::{Signal, SignalWith};
use leptos_router::use_router;
use uuid::Uuid;

pub mod category;
pub mod channel;
pub mod server_menu;
pub mod sidebar;
pub mod thread;

pub fn use_current_channel() -> Signal<Option<Uuid>> {
    Signal::derive(move || {
        use_router().pathname().with(|path| {
            path.split('/')
                .nth(3)
                .and_then(|path| Uuid::parse_str(path).ok())
        })
    })
}

pub fn use_current_thread() -> Signal<Option<(Uuid, Uuid)>> {
    Signal::derive(move || {
        use_router().pathname().with(|path| {
            if path.contains("thread") {
                let mut path = path.split('/').skip(4);
                let channel = path.next();
                let thread = path.next();
                match (
                    channel.and_then(|channel| Uuid::from_str(channel).ok()),
                    thread.and_then(|thread| Uuid::from_str(thread).ok()),
                ) {
                    (Some(channel), Some(thread)) => Some((channel, thread)),
                    _ => None,
                }
            } else {
                None
            }
        })
    })
}

pub fn use_current_split_thread() -> Signal<Option<Uuid>> {
    Signal::derive(move || {
        use_router().pathname().with(|path| {
            path.split('/')
                .nth(4)
                .and_then(|path| Uuid::parse_str(path).ok())
        })
    })
}
