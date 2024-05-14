use std::time::Duration;

use crate::app::api::server::use_server;
use crate::app::components::create_server::Create_server_modal;
use crate::app::components::theme::{ThemeIcons, Toggle_Theme};
use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::*;
use leptos_icons::*;
use leptos_router::{use_router, A};
use uuid::Uuid;

#[component]
pub fn SideBar() -> impl IntoView {
    let servers = use_server().servers;
    view! {
        <div class="w-full h-full flex flex-col items-center pt-3 bg-base-300 scrollbar-none overflow-y-scroll overflow-x-hidden">
            <Transition fallback=move || ()>
                {move || servers.and_then(|servers| servers.iter().map(|server| {
                    let server = server.clone();
                    view! {
                        <Navigation_server id=server.id name=server.name/>
                    }
                }).collect_view())}

                <Create_server_modal/>

                <Navigation_action tip="Toggle theme".into()>
                    <Toggle_Theme
                        class="relative mx-3 h-[48px] transition-all bg-base-100 text-base-content rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] w-[48px] overflow-hidden"
                        icons=ThemeIcons{dark: icondata::RiSunWeatherFill, light: icondata::RiMoonWeatherFill, class: "fill-primary w-7 h-7 group-hover:fill-base-100"}
                    />
                </Navigation_action>
            </Transition>
        </div>
    }
}

#[component]
pub fn Navigation_action(tip: String, children: Children) -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=Duration::new(0,0)>
            <TooltipTrigger class="group relative flex items-center mb-1">
                <div class="absolute left-0 bg-primary rounded-r-full transition-all w-[4px] group-hover:h-[20px] h-[8px]"/>
                {children()}
            </TooltipTrigger>
            <TooltipContent tip=tip class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#c6d2d2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#c6d2d2] dark:after:border-r-[#0d0d0d]"/>
        </TooltipProvider>
    }
}

#[component]
pub fn Navigation_server(id: Uuid, name: String) -> impl IntoView {
    let current_server = move || {
        use_router().pathname().with(|path| {
            Uuid::parse_str(path.split('/').nth(2).unwrap_or_default()).unwrap_or_default()
        })
    };
    view! {
        <TooltipProvider delay_duration=Duration::new(0,0)>
            <TooltipTrigger class="relative mb-1">
                <A href=id.simple().to_string() class="group flex relative items-center">
                    <div class=move || format!("absolute left-0 bg-primary rounded-r-full transition-all w-[4px] {}", {
                        match current_server() == id {
                            false => "group-hover:h-[20px] h-[8px]",
                            true =>"h-[36px]",
                        }
                    })
                    />
                    <div class=" mx-3 h-[48px] transition-all bg-base-100 text-base-content rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] w-[48px]"/>
                </A>
            </TooltipTrigger>
            <TooltipContent tip=name class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#c6d2d2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#c6d2d2] dark:after:border-r-[#0d0d0d]"/>
        </TooltipProvider>
    }
}
