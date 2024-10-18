use crate::app::components::navigation::context_server_menu::ContextServerMenu;
use crate::entities::server::Server;
use leptos_router::use_router;
use std::time::Duration;

use crate::app::api::server::{get_user_servers, use_server};
use crate::app::components::modal::create_server::CreateServerModal;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::*;
use leptos_icons::*;
use leptos_router::A;

#[component]
pub fn Servers() -> impl IntoView {
    let use_server = use_server();
    let servers = create_resource(
        move || {
            (
                use_server.leave_server.version().get(),
                use_server.join_with_invitation.version().get(),
                use_server.create_server.version().get(),
            )
        },
        move |_| get_user_servers(),
    );

    let open = create_rw_signal(false);

    view! {
        <ContextMenuProvider open=open>
            <ContextMenuTrigger>
                <TooltipProvider delay_duration=Duration::new(0,0)>
                    <TooltipTrigger class="relative my-0.5">
                        <A href="" class=" flex relative items-center">
                            <div class="flex mx-3 h-[48px] transition-all items-center justify-center  text-base-content   w-[48px]">
                                <Icon icon=icondata::RiAppsSystemLine class="h-6 w-6 fill-primary"/>
                            </div>
                        </A>
                    </TooltipTrigger>
                    <TooltipContent tip="Servers" class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
                </TooltipProvider>
            </ContextMenuTrigger>
            <ContextMenuContent class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] z-40 rounded".to_string()>
                <CreateServerModal class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded">
                    <div>"Create Server"</div>
                    <Icon icon=icondata::RiAddSystemFill class="w-5 h-5"/>
                </CreateServerModal>
                <A href="discover" class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on:click=move |_| open.set(false)>
                    <div>"Discover Servers"</div>
                    <Icon icon=icondata::RiCompass3MapFill class="w-5 h-5"/>
                </A>
                <A href="" class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on:click=move |_| open.set(false)>
                    <div>"Show Servers"</div>
                    <Icon icon=icondata::RiCheckboxCircleSystemFill class="w-5 h-5"/>
                </A>
            </ContextMenuContent>
        </ContextMenuProvider>
        <Transition fallback=move || ()>
            {
                move || {
                    servers.with(|servers|
                        match  servers {
                            Some(Ok(servers)) => {
                                servers.iter().map(|server| {
                                    view! {
                                        <ServerNavigation server=server.clone()/>
                                    }
                                }).collect_view()
                            },
                            _ => view!{<div/>}.into_view()
                        }
                    )
                }
            }
        </Transition>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn ServerNavigation(server: Server) -> impl IntoView {
    let current_server = move || {
        use_router()
            .pathname()
            .with(|path| path.split('/').nth(2).map(|path| path.to_string()))
    };
    let image_url = store_value(server.image_url.clone());
    let name = server.name.clone();
    view! {
        <TooltipProvider delay_duration=Duration::new(0,0)>
            <TooltipTrigger class="relative my-0.5">
                <A href=server.id.simple().to_string() class="group flex relative items-center">
                    <div class=move || format!("absolute left-0 bg-white rounded-r-full transition-all w-[4px] {}", {
                        match current_server().is_some_and(|current| current == server.id.simple().to_string()) {
                            false => "group-hover:h-[20px] h-[8px]",
                            true =>"h-[36px]",
                        }
                    })
                    />
                    <ContextServerMenu  server=server>
                        {
                            move || match image_url.get_value() {
                                None => ().into_view(),
                                Some(url) => view!{
                                    <img class="w-full h-full object-cover " src=url/>

                                }.into_view()
                            }
                        }
                    </ContextServerMenu>
                </A>
            </TooltipTrigger>
            <TooltipContent tip=name class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
        </TooltipProvider>
    }
}
