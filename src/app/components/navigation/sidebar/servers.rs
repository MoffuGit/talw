use crate::app::components::navigation::context_server_menu::ContextServerMenu;
use crate::entities::server::Server;
// use leptos_router::use_router;
use std::time::Duration;

use crate::app::api::server::{get_user_servers, use_server};
use crate::app::components::modal::create_server::CreateServerModal;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::logging;
use leptos::prelude::*;
use leptos_icons::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[component]
pub fn Servers() -> impl IntoView {
    let use_server = use_server();
    let servers = Resource::new(
        move || {
            (
                use_server.edit_server_image.version().get(),
                use_server.edit_server_name.version().get(),
                use_server.leave_server.version().get(),
                use_server.join_with_invitation.version().get(),
                use_server.create_server.version().get(),
            )
        },
        move |_| get_user_servers(),
    );

    let open = RwSignal::new(false);
    let hidden = RwSignal::new(false);
    let create_server_modal_ref = NodeRef::new();

    view! {
        <ContextMenuProvider open=open hidden=hidden>
            <ContextMenuTrigger>
                <TooltipProvider delay_duration=Duration::new(0, 0)>
                    <TooltipTrigger class="relative my-0.5">
                        <A href="" {..} class=" flex relative items-center">
                            <div class="flex transition-all items-center justify-center text-base-content w-8 h-8">
                                <Icon
                                    icon=icondata::LuCommand
                                    // class="h-5 w-5 stroke-base-content"
                                />
                            </div>
                        </A>
                    </TooltipTrigger>
                    <TooltipContent
                        tip="Servers"
                        tooltip_of_side=10.0
                        arrow=true
                        class="rounded-lg w-auto h-auto py-1.5 px-2.5 text-sm bg-base-400 border-base-400"
                    />
                </TooltipProvider>
            </ContextMenuTrigger>
            <ContextMenuContent
                ignore=vec![create_server_modal_ref]
                class="transition-all ease-out w-56 flex flex-col h-auto p-1 bg-base-400 z-40 rounded-md border border-base-100"
            >
                <CreateServerModal
                    on_open=Signal::derive(move || hidden.set(true))
                    content_ref=create_server_modal_ref
                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                >
                    <div>"Create Server"</div>
                    <Icon icon=icondata::RiAddSystemFill /* class="w-5 h-5" */ />
                </CreateServerModal>
                <A
                    href="discover"
                    {..}
                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                    on:click=move |_| open.set(false)
                >
                    <div>"Discover Servers"</div>
                    <Icon icon=icondata::RiCompass3MapFill /* class="w-5 h-5" */ />
                </A>
                <A
                    href=""
                    {..}
                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                    on:click=move |_| open.set(false)
                >
                    <div>"Show Servers"</div>
                    <Icon icon=icondata::RiCheckboxCircleSystemFill /* class="w-5 h-5" */ />
                </A>
            </ContextMenuContent>
        </ContextMenuProvider>
        <Transition fallback=move || view!{}>
            <For
                each=move || servers.get().and_then(Result::ok).unwrap_or_default()
                key=|server| server.id
                children=move |server: Server| {
                    view!{
                        <ServerNavigation server=server/>
                    }
                }
            />
        </Transition>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn ServerNavigation(server: Server) -> impl IntoView {
    let current_server =
        move || use_params_map().with(|params| params.get("id").map(|id| id.to_string()));
    let image_url = server.image_url.clone();
    let name = server.name.clone();
    view! {
        <div class="group flex relative items-center justify-center w-full my-0.5">
            <div class=move || {
                format!(
                    "absolute left-0 bg-white rounded-r-full transition-all duration-100 ease-linear w-0.5 {}",
                    {
                        match current_server()
                            .is_some_and(|current| { current == server.id.simple().to_string() })
                        {
                            false => "group-hover:h-3 h-1",
                            true => "h-6",
                        }
                    },
                )
            } />
            <TooltipProvider delay_duration=Duration::new(0, 0)>
                <TooltipTrigger class="relative my-0.5">
                    <A href=server.id.simple().to_string() {..} class="group flex relative items-center">
                        <ContextServerMenu server=server>
                            {image_url.map(|url| view! { <img class="w-full h-full object-cover " src=url /> })}
                        </ContextServerMenu>
                    </A>
                </TooltipTrigger>
                <TooltipContent
                    tip=name
                    tooltip_of_side=10.0
                    arrow=true
                    class="rounded-lg w-auto h-auto py-1.5 px-2.5 text-sm bg-base-400 border-base-400"
                />
            </TooltipProvider>
        </div>
    }
}
