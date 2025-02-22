mod server;
mod thread;
use crate::app::components::ui::tool_tip::{
    ToolTipSide, TooltipContent, TooltipProvider, TooltipTrigger,
};
use icondata;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_icons::Icon;
use std::time::Duration;
use uuid::Uuid;

use self::server::ServerMemberSideBar;
use self::thread::ThreadMemberSideBar;

#[derive(Debug, Clone)]
pub struct SideBarContext(pub RwSignal<bool>);

#[component]
pub fn MemberSideBar(server_id: Uuid, #[prop(optional)] thread_id: Option<Uuid>) -> impl IntoView {
    if let Some(thread_id) = thread_id {
        Either::Left(view! { <ThreadMemberSideBar server_id=server_id thread_id=thread_id /> })
    } else {
        Either::Right(view! { <ServerMemberSideBar server_id=server_id /> })
    }
}

#[component]
pub fn MemberSideBarTrigger() -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger
                class="hover:bg-base-100 rounded-md p-1 cursor-pointer"
                on_click=Signal::derive(move || open.update(|open| *open = !*open))
            >
                <Icon icon=icondata::LuUsers /* class="w-5 h-5" */ />
            </TooltipTrigger>
            <TooltipContent
                arrow=true
                tooltip_of_side=10.0
                tip="Member List"
                tooltip_side=ToolTipSide::Bottom
                class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
            />
        </TooltipProvider>
    }
}
