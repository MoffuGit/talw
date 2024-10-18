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
        return view! {<ThreadMemberSideBar server_id=server_id thread_id=thread_id/>}.into_view();
    };
    view! { <ServerMemberSideBar server_id=server_id/> }.into_view()
}

#[allow(non_snake_case)]
#[component]
pub fn MemberSideBarTrigger() -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger close_on_click=false on_click=Signal::derive(move || open.update(|open| *open = !*open))>
                <Icon icon=icondata::RiGroup2UserFacesFill class="w-6 h-6 fill-base-content/40" />
            </TooltipTrigger>
            <TooltipContent tooltip_of_side=10.0 tip=Signal::derive(move || match open.get() { true => "Hide Members SideBar".to_string() , false => "Show Members SideBar".to_string()} )  tooltip_side=ToolTipSide::Bottom class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:bottom-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-b-[#dfdfe2] dark:after:border-b-[#0d0d0d]" />
        </TooltipProvider>
    }
}
