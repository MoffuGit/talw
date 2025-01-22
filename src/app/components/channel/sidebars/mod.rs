mod server;
mod thread;
use crate::app::components::ui::tool_tip::{
    ToolTipSide, TooltipContent, TooltipProvider, TooltipTrigger,
};
use icondata;
use leptos::*;
use leptos_icons::Icon;
use std::time::Duration;
use uuid::Uuid;

use self::server::ServerMemberSideBar;
use self::thread::ThreadMemberSideBar;

#[derive(Debug, Clone)]
pub struct SideBarContext(pub RwSignal<bool>);

#[allow(non_snake_case)]
#[component]
pub fn MemberSideBar(server_id: Uuid, #[prop(optional)] thread_id: Option<Uuid>) -> impl IntoView {
    if let Some(thread_id) = thread_id {
        return view! { <ThreadMemberSideBar server_id=server_id thread_id=thread_id /> }
            .into_view();
    };
    view! { <ServerMemberSideBar server_id=server_id /> }.into_view()
}

#[allow(non_snake_case)]
#[component]
pub fn MemberSideBarTrigger() -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger
                class="hover:bg-base-content/5 rounded-lg p-1 cursor-pointer"
                on_click=Signal::derive(move || open.update(|open| *open = !*open))
            >
                <Icon icon=icondata::LuUsers class="w-5 h-5" />
            </TooltipTrigger>
            <TooltipContent
                arrow=true
                tooltip_of_side=10.0
                tip="Member List"
                tooltip_side=ToolTipSide::Bottom
                class="rounded-lg w-auto h-auto py-1 px-2 text-sm bg-base-400 border-base-400"
            />
        </TooltipProvider>
    }
}
