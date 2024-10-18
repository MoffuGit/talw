mod inbox;
mod profile;
mod search;
mod servers;
use crate::app::components::theme::{ThemeIcons, Toggle_Theme};
use crate::app::components::ui::tool_tip::*;
use crate::entities::user::User;
use icondata;
use leptos::*;
use leptos_icons::*;
use leptos_router::A;
use std::time::Duration;

use self::inbox::Inbox;
use self::profile::Profile;
use self::search::Search;
use self::servers::Servers;

#[allow(non_snake_case)]
#[component]
pub fn SideBar(user: User) -> impl IntoView {
    view! {
        <div class="w-full h-full flex flex-col items-center py-2.5 bg-base-300 justify-between">
            <div class="flex w-full grow flex-col items-center scrollbar-none overflow-y-scroll overflow-x-hidden">
                <Search/>
                <TooltipProvider delay_duration=Duration::new(0,0)>
                    <TooltipTrigger class="relative my-0.5">
                        <A href="me" class=" flex relative items-center">
                            <div class="flex mx-3 h-[48px] transition-all items-center justify-center  text-base-content  w-[48px]">
                                <Icon icon=icondata::RiChat3CommunicationFill class="h-6 w-6 fill-primary"/>
                            </div>
                        </A>
                    </TooltipTrigger>
                    <TooltipContent tip="Direct Messages" class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
                </TooltipProvider>
                <Inbox/>
                <div class="divider my-0.5 mx-[10px] h-0.5"></div>
                <Servers/>
                <div class="divider my-0.5 mx-[10px] h-0.5"></div>
                <TooltipProvider delay_duration=Duration::new(0,0)>
                    <TooltipTrigger class="group relative flex items-center my-0.5">
                        <div class="absolute left-0 bg-primary rounded-r-full transition-all w-[4px] group-hover:h-[20px] h-[8px]"/>
                            <Toggle_Theme
                                class="relative mx-3 h-[48px] transition-all bg-base-100 text-base-content rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] w-[48px] overflow-hidden"
                                icons=ThemeIcons{dark: icondata::RiSunWeatherFill, light: icondata::RiMoonWeatherFill, class: "fill-primary w-7 h-7 group-hover:fill-base-100"}
                            />
                    </TooltipTrigger>
                    <TooltipContent tip="Toggle Theme" class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
                </TooltipProvider>
            </div>
            <Profile user=user/>
        </div>
    }
}
