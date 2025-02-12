pub mod sidebar;
use crate::app::api::thread::JoinThread;
use crate::app::api::thread::LeaveThread;
use leptos::prelude::*;
use uuid::Uuid;

use crate::app::api::thread::use_thread;

#[component]
pub fn LeaveThread(thread_id: Uuid, server_id: Uuid, class: &'static str) -> impl IntoView {
    view! {
        <div
            on:click=move |_| {
                use_thread()
                    .leave_thread
                    .dispatch(LeaveThread {
                        thread_id,
                        server_id,
                    });
            }
            class=class
        >
            "Leave Thread"
        </div>
    }
}

#[component]
pub fn JoinThread(thread_id: Uuid, server_id: Uuid, class: &'static str) -> impl IntoView {
    view! {
        <div
            on:click=move |_| {use_thread().join_thread.dispatch(JoinThread { thread_id, server_id });}
            class=class
        >
            "Join Thread"
        </div>
    }
}
