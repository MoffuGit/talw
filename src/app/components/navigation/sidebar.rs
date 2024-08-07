use crate::app::api::server::{get_user_servers_with_members, use_server};
use crate::app::components::modal::create_server::CreateServerModal;
use crate::app::components::navigation::context_server_menu::ContextServerMenu;
use crate::app::components::theme::{ThemeIcons, Toggle_Theme};
use crate::entities::server::Server;
use crate::{app::components::ui::tool_tip::*, entities::member::Member};
use icondata;
use leptos::*;
use leptos_icons::*;
use leptos_router::{use_router, A};
use std::time::Duration;

#[allow(non_snake_case)]
#[component]
pub fn SideBar() -> impl IntoView {
    let use_server = use_server();
    let servers = create_resource(
        move || {
            (
                use_server.leave_server.version().get(),
                use_server.join_with_invitation.version().get(),
                use_server.create_server.version().get(),
            )
        },
        move |_| get_user_servers_with_members(),
    );
    view! {
        <div class="w-full h-full flex flex-col items-center pt-3 bg-base-300 scrollbar-none overflow-y-scroll overflow-x-hidden">
                <Navigation id="me".to_string() name="Direct messages".to_string()>
                    <Icon icon=icondata::RiEmotionUserFacesFill class="h-8 w-8 group-hover:fill-base-100 fill-primary"/>
                </Navigation>
                <div class="divider my-0.5 mx-[10px] h-0.5"></div>
                <Transition fallback=move || ()>
                    {
                        move || {
                            servers.with(|servers|
                                match  servers {
                                    Some(Ok(servers)) => {
                                        servers.iter().map(|(server, member)| {
                                            view! {
                                                <Navigation_server server=server.clone() member=member.clone()/>
                                            }
                                        }).collect_view()
                                    },
                                    _ => view!{<div/>}.into_view()
                                }
                            )
                        }
                    }
                </Transition>

                <CreateServerModal/>

                <Navigation id="search_servers".to_string() name="Explore Discoverable Servers".to_string()>
                    <Icon icon=icondata::RiCompassMapLine class="h-8 w-8 group-hover:fill-base-100 fill-primary"/>
                </Navigation>

                <div class="divider my-0.5 mx-[10px] h-0.5"></div>
                <Navigation_action tip="Toggle theme".into()>
                    <Toggle_Theme
                        class="relative mx-3 h-[48px] transition-all bg-base-100 text-base-content rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] w-[48px] overflow-hidden"
                        icons=ThemeIcons{dark: icondata::RiSunWeatherFill, light: icondata::RiMoonWeatherFill, class: "fill-primary w-7 h-7 group-hover:fill-base-100"}
                    />
                </Navigation_action>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Navigation_action(tip: String, children: Children) -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=Duration::new(0,0)>
            <TooltipTrigger class="group relative flex items-center my-0.5">
                <div class="absolute left-0 bg-primary rounded-r-full transition-all w-[4px] group-hover:h-[20px] h-[8px]"/>
                {children()}
            </TooltipTrigger>
            <TooltipContent tip=tip class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
        </TooltipProvider>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Navigation(
    id: String,
    name: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let current_server = move || {
        use_router()
            .pathname()
            .with(|path| path.split('/').nth(2).map(|path| path.to_string()))
    };
    view! {
        <TooltipProvider delay_duration=Duration::new(0,0)>
            <TooltipTrigger class="relative my-0.5">
                <A href=id.clone() class="group flex relative items-center">
                    <div class=move || format!("absolute left-0 bg-primary rounded-r-full transition-all w-[4px] {}", {
                        match current_server().is_some_and(|current| current == id) {
                            false => "group-hover:h-[20px] h-[8px]",
                            true =>"h-[36px]",
                        }
                    })
                    />
                    <div class="flex mx-3 h-[48px] transition-all items-center justify-center bg-base-100 text-base-content rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] w-[48px]">
                        {children.map(|children| children())}
                    </div>
                </A>
            </TooltipTrigger>
            <TooltipContent tip=name class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
        </TooltipProvider>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Navigation_server(server: Server, member: Member) -> impl IntoView {
    let current_server = move || {
        use_router()
            .pathname()
            .with(|path| path.split('/').nth(2).map(|path| path.to_string()))
    };
    let server = store_value(server);
    view! {
        <TooltipProvider delay_duration=Duration::new(0,0)>
            <TooltipTrigger class="relative my-0.5">
                <A href=server.get_value().id.simple().to_string() class="group flex relative items-center">
                    <div class=move || format!("absolute left-0 bg-primary rounded-r-full transition-all w-[4px] {}", {
                        match current_server().is_some_and(|current| current == server.get_value().id.simple().to_string()) {
                            false => "group-hover:h-[20px] h-[8px]",
                            true =>"h-[36px]",
                        }
                    })
                    />
                    <ContextServerMenu  server=server.get_value() member=member>
                        {
                            move || match server.get_value().image_url {
                                None => ().into_view(),
                                Some(url) => view!{
                                    <img class="w-full h-full object-cover " src=url/>

                                }.into_view()
                            }
                        }
                    </ContextServerMenu>
                </A>
            </TooltipTrigger>
            <TooltipContent tip=server.get_value().name class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
        </TooltipProvider>
    }
}
